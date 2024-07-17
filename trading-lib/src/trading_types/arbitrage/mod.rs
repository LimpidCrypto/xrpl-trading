pub mod swap;
pub mod triangular;

pub trait IsProfitable {
    fn is_profitable(&self) -> bool;
}
