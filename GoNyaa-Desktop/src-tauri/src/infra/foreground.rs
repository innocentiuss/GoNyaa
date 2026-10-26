//! Briefly protect the foreground window while an external browser is starting.
//! Windows automatically releases the lock on an explicit user window switch.

#[cfg(target_os = "windows")]
use std::sync::Mutex;

#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, LockSetForegroundWindow, LSFW_LOCK, LSFW_UNLOCK,
};

#[cfg(target_os = "windows")]
#[derive(Default)]
struct ForegroundState {
    generation: u64,
    locked: bool,
}

#[cfg(target_os = "windows")]
static FOREGROUND: Mutex<ForegroundState> = Mutex::new(ForegroundState {
    generation: 0,
    locked: false,
});

pub struct ForegroundLease {
    #[cfg(target_os = "windows")]
    generation: Option<u64>,
}

impl ForegroundLease {
    pub fn cancel() {
        #[cfg(target_os = "windows")]
        {
            let mut state = FOREGROUND.lock().expect("foreground lock");
            state.generation = state.generation.wrapping_add(1);
            if state.locked {
                // SAFETY: Release only this process's temporary foreground lock.
                unsafe { LockSetForegroundWindow(LSFW_UNLOCK) };
                state.locked = false;
            }
        }
    }

    pub fn for_open(window: &tauri::WebviewWindow, keep_foreground: bool) -> Self {
        #[cfg(target_os = "windows")]
        {
            // Serialize release/acquire with delayed cleanup. An earlier click's timer
            // must not unlock protection acquired by a newer middle click.
            let mut state = FOREGROUND.lock().expect("foreground lock");
            state.generation = state.generation.wrapping_add(1);
            if state.locked {
                // SAFETY: This process owns the temporary foreground lock.
                unsafe { LockSetForegroundWindow(LSFW_UNLOCK) };
                state.locked = false;
            }

            let acquired = keep_foreground
                && window.hwnd().is_ok_and(|handle| {
                    // SAFETY: The handle comes from the live Tauri window. Both Win32
                    // calls have no pointer arguments requiring writable storage.
                    unsafe {
                        GetForegroundWindow() == handle.0 && LockSetForegroundWindow(LSFW_LOCK) != 0
                    }
                });
            state.locked = acquired;
            Self {
                generation: acquired.then_some(state.generation),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (window, keep_foreground);
            Self {}
        }
    }

    pub fn release_after_launch(self) {
        #[cfg(target_os = "windows")]
        if self.generation.is_some() {
            tauri::async_runtime::spawn(async move {
                // Covers warm and cold browser starts, without delaying the IPC reply.
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                drop(self);
            });
        }
    }
}

impl Drop for ForegroundLease {
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        if let Some(generation) = self.generation {
            let mut state = FOREGROUND.lock().expect("foreground lock");
            if state.generation == generation && state.locked {
                // SAFETY: Only the most recent lease may release our foreground lock.
                unsafe { LockSetForegroundWindow(LSFW_UNLOCK) };
                state.locked = false;
            }
        }
    }
}
