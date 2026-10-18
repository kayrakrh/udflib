use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UdfError {
    #[error("Could not find the selected file: {}", .0.display())]
    FileNotFound(PathBuf),
    #[error("zip hatası: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("G/Ç hatası: {0}")]
    Io(#[from] std::io::Error),
    #[error("XML hatası: {0}")]
    Xml(#[from] roxmltree::Error),
    #[error("JSON hatası: {0}")]
    Json(#[from] serde_json::Error),
    #[error("arşivde zorunlu dosya yok: {0}")]
    Missing(&'static str),
    #[error("geçersiz içerik: {0}")]
    Invalid(String),
}
