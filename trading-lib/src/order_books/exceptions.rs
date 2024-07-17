use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum OrderBookException {
    #[error("Invalid order")]
    InvalidOrder,
}
