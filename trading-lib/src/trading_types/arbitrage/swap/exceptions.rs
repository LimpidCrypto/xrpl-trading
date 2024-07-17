use thiserror::Error;

#[derive(Debug, Error)]
pub enum SwapArbitrageException {
    #[error("Invalid order book combination. The base currency of the first order book must be the counter currency of the second order book.")]
    InvalidOrderBookCombo,
}
