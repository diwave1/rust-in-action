mod app;
pub mod mysql;
mod xredis;

// use重新导出
pub use app::{APP_CONFIG, REDIS_POOL};
