pub mod exceptions;
pub mod order;
pub mod order_book;

use order_book::OrderBook;

#[derive(Debug)]
pub struct OrderBooks<'a> {
    pub order_books: Vec<OrderBook<'a>>,
    pub liquidity_spread: f64,
}

impl<'a> OrderBooks<'a> {
    pub fn sort(&mut self) {
        self.order_books
            .iter_mut()
            .for_each(|order_book| order_book.sort());
    }
}

pub trait IsLiquid {
    /// Returns true if the order book is liquid determained based on the provided `liquidity_spread`.
    fn is_liquid(&self, liquidity_spread: f64) -> bool;
}

pub trait Flip {
    fn flip(&mut self);

    fn get_flipped(&self) -> Self
    where
        Self: Sized + Clone,
    {
        let mut flipped = self.clone();
        flipped.flip();

        flipped
    }
}
