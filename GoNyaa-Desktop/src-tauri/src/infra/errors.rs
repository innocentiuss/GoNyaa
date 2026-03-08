use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("云端服务不可达: {0}")]
    Network(String),
    #[error("云端返回数据解析失败: {0}")]
    Deserialize(String),
    #[error("本地文件读写失败: {0}")]
    Io(String),
    #[error("外部链接打开失败: {0}")]
    Open(String),
    #[error("应用路径解析失败: {0}")]
    Path(String),
    #[error("配置无效: {0}")]
    Config(String),
}

pub type AppResult<T> = Result<T, AppError>;
