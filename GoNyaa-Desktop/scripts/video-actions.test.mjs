import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { test } from 'node:test'
import { runInNewContext } from 'node:vm'
import ts from 'typescript'

// Exercise the real composable with controlled IPC promises, including overlapping clicks.
const compiled = ts.transpileModule(
  readFileSync(new URL('../src/composables/useVideoActions.ts', import.meta.url), 'utf8'),
  { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2020 } },
).outputText

function setup(overrides = {}, autoViewed = true) {
  const opened = []
  const toggled = []
  const errors = []
  const rows = { value: [{ fanHao: 'ABC-123', viewed: false, viewLink: 'https://example.com/official',
    viewLink2: 'https://example.com/first', viewLink3: 'https://example.com/second' }] }
  const exports = {}
  runInNewContext(compiled, {
    exports,
    require(name) {
      if (name === '@/api/desktop') return { desktopApi: {
        openExternal: async (...args) => { opened.push(args); return overrides.open?.(...args) },
        toggleViewed: async (id) => { toggled.push(id); return overrides.toggle?.(id) },
      } }
      if (name === '@/composables/useNotifications') return { useNotifications: () => ({
        notifyError: (...args) => errors.push(args), notifySuccess() {},
      }) }
      throw new Error(`Unexpected dependency: ${name}`)
    },
  })
  const actions = exports.useVideoActions({ rows, autoViewed: { value: autoViewed } })
  return { actions, rows, opened, toggled, errors }
}

test('left and middle clicks pass their focus preference to IPC', async () => {
  const state = setup({}, false)
  await state.actions.handleOpenLink(state.rows.value[0], 'viewLink2')
  await state.actions.handleOpenLink(state.rows.value[0], 'viewLink3', true)
  assert.deepEqual(state.opened, [
    ['https://example.com/first', false], ['https://example.com/second', true],
  ])
  assert.equal(state.toggled.length, 0)
})

test('rapid clicks open every link but mark the same item viewed only once', async () => {
  let finishToggle
  let beginToggle
  const togglePending = new Promise(resolve => { finishToggle = resolve })
  const toggleStarted = new Promise(resolve => { beginToggle = resolve })
  const state = setup({ toggle: () => { beginToggle(); return togglePending } })
  const row = state.rows.value[0]
  const clicks = ['viewLink', 'viewLink2', 'viewLink3'].map(field =>
    state.actions.handleOpenLink(row, field, true))
  // Allow IPC continuations to overlap while the first viewed update is pending.
  await toggleStarted
  assert.equal(state.opened.length, 3)
  assert.deepEqual(state.toggled, ['ABC-123'])
  finishToggle()
  await Promise.all(clicks)
  assert.equal(state.rows.value[0].viewed, true)
  // A captured row from before syncViewed must not toggle the record back off.
  await state.actions.handleOpenLink(row, 'viewLink2', true)
  assert.equal(state.opened.length, 4)
  assert.equal(state.toggled.length, 1)
})

test('a failed browser launch does not mark viewed', async () => {
  const state = setup({ open: () => { throw new Error('launch failed') } })
  await state.actions.handleOpenLink(state.rows.value[0], 'viewLink2', true)
  assert.equal(state.toggled.length, 0)
  assert.equal(state.rows.value[0].viewed, false)
  assert.equal(state.errors.length, 1)
})

test('failed viewed updates can be retried on the next click', async () => {
  let failed = false
  const state = setup({ toggle: () => {
    if (!failed) { failed = true; throw new Error('save failed') }
  } })
  const row = state.rows.value[0]
  await state.actions.handleOpenLink(row, 'viewLink2', true)
  assert.equal(state.rows.value[0].viewed, false)
  await state.actions.handleOpenLink(row, 'viewLink3', true)
  assert.equal(state.toggled.length, 2)
  assert.equal(state.rows.value[0].viewed, true)
})
