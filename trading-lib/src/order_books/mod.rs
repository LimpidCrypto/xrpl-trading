pub mod exceptions;
pub mod order;
pub mod order_book;

use std::borrow::Cow;

use anyhow::{Ok, Result};
use order_book::OrderBook;

#[derive(Debug, Clone)]
pub struct OrderBooks<'a> {
    pub order_books: Cow<'a, [OrderBook<'a>]>,
    pub liquidity_spread: f64,
}

impl<'a> OrderBooks<'a> {
    pub fn sort(&'a mut self) -> Result<()> {
        self.order_books
            .to_mut()
            .iter_mut()
            .try_for_each(|order_book| order_book.sort())
    }

    pub fn get_liquid_order_books(&'a self) -> Result<Vec<&OrderBook<'_>>> {
        let mut liquid_order_books = Vec::new();
        for order_book in self.order_books.iter() {
            if order_book.is_liquid(self.liquidity_spread)? {
                liquid_order_books.push(order_book);
            }
        }

        Ok(liquid_order_books)
    }

    pub fn get_illiquid_order_books(&'a self) -> Result<Vec<&OrderBook<'_>>> {
        let mut illiquid_order_books = Vec::new();
        for order_book in self.order_books.iter() {
            if !order_book.is_liquid(self.liquidity_spread)? {
                illiquid_order_books.push(order_book);
            }
        }

        Ok(illiquid_order_books)
    }
}

pub trait IsLiquid<'a> {
    /// Returns true if the order book is liquid determained based on the provided `liquidity_spread`.
    fn is_liquid(&'a self, liquidity_spread: f64) -> Result<bool>;
}

pub trait Flip {
    fn flip(&mut self) -> Result<()>;

    // fn get_flipped(&'a self) -> Self
    // where
    //     Self: Sized + Clone,
    // {
    //     let mut flipped = self.clone();
    //     flipped.flip();

    //     flipped
    // }
}
