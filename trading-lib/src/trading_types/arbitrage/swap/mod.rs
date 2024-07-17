pub mod exceptions;

use std::borrow::Cow;

use anyhow::{bail, Result};
use exceptions::SwapArbitrageException;
#[cfg(feature = "xrpl")]
use xrpl::models::transactions::OfferCreate;

use crate::{
    models::currency::Currency,
    order_books::{order::Order, order_book::OrderBook, Flip, OrderBooks},
};

use super::IsProfitable;

/// Represents a profitable swap trade between two order books from the perspective of the trader.
/// There are different combinations of how orders of two order books can be swapped:
/// ### 1. Example (Trade for more XRP)
/// *Order Book 1*: XRP/USD:GateHub <br>
/// *Order Book 2*: XRP/USD:Bitstamp <br>
/// *Consuming Bid of XRP/USD:GateHub (counterparty wants to buy XRP with USD)*: XRP -> USD:GateHub (you sell XRP for USD) <br>
/// *Consuming Ask of XRP/USD:Bitstamp (counterparty wants to buy USD with XRP)*: USD:Bitstamp -> XRP (you buy XRP with USD)
///
/// ### 2. Example (Trade for more USD)
/// *Order Book 1*: XRP/USD:GateHub <br>
/// *Order Book 2*: XRP/USD:Bitstamp <br>
/// *Consuming Ask of XRP/USD:GateHub (counterparty wants to buy USD for XRP)*: USD:GateHub -> XRP (you sell USD for XRP) <br>
/// *Consuming Bid of XRP/USD:Bitstamp (counterparty wants to buy XRP with USD)*: XRP -> USD:Bitstamp (you buy USD with XRP)
///
/// ### 3. Example (Trade for more XRP with second order book flipped)
/// *Order Book 1*: XRP/USD:GateHub <br>
/// *Order Book 2*: USD:Bitstamp/XRP <br>
/// *Consuming Bid of XRP/USD:GateHub (counterparty wants to buy XRP with USD)*: XRP -> USD:GateHub (you sell XRP for USD) <br>
/// *Consuming Bid of USD:Bitstamp/XRP (counterparty wants to buy XRP with USD)*: USD:Bitstamp -> XRP (you buy XRP with USD)
///
/// ### 4. Example (Trade for more USD with second order book flipped)
/// *Order Book 1*: XRP/USD:GateHub <br>
/// *Order Book 2*: USD:Bitstamp/XRP <br>
/// *Consuming Ask of XRP/USD:GateHub (counterparty wants to buy USD for XRP)*: USD:GateHub -> XRP (you sell USD for XRP) <br>
/// *Consuming Ask of USD:Bitstamp/XRP (counterparty wants to buy XRP with USD)*: XRP -> USD:Bitstamp (you buy USD with XRP)
///
/// ### 5. Example (Trade for more XRP with first order books flipped)
/// *Order Book 1*: USD:GateHub/XRP <br>
/// *Order Book 2*: XRP/USD:Bitstamp <br>
/// *Consuming Ask of USD:GateHub/XRP (counterparty wants to buy XRP with USD)*: XRP -> USD:GateHub (you sell XRP for USD) <br>
/// *Consuming Ask of XRP/USD:Bitstamp (counterparty wants to buy USD with XRP)*: USD:Bitstamp -> XRP (you buy XRP with USD)
///
/// ### 6. Example (Trade for more USD with first order books flipped)
/// *Order Book 1*: USD:GateHub/XRP <br>
/// *Order Book 2*: XRP/USD:Bitstamp <br>
/// *Consuming Bid of USD:GateHub/XRP (counterparty wants to buy USD for XRP)*: USD:GateHub -> XRP (you sell USD for XRP) <br>
/// *Consuming Bid of XRP/USD:Bitstamp (counterparty wants to buy XRP with USD)*: XRP -> USD:Bitstamp (you buy USD with XRP)
#[derive(Debug, Clone, PartialEq)]
pub struct SwapTrade<'a> {
    pub sell_order: Order<'a>,
    pub buy_order: Order<'a>,
    pub starting_currency: Currency<'a>,
}

#[cfg(feature = "xrpl")]
impl<'a> Into<(OfferCreate<'a>, OfferCreate<'a>)> for SwapTrade<'a> {
    fn into(self) -> (OfferCreate<'a>, OfferCreate<'a>) {
        (self.sell_order.into(), self.buy_order.into())
    }
}

impl IsProfitable for SwapTrade<'_> {
    fn is_profitable(&self) -> bool {
        let counter_quantity_after_fee = self.sell_order.calculate_counter_quantity_after_fee();
        let mut other = self.buy_order.clone();
        if counter_quantity_after_fee < other.base_quantity {
            other.base_quantity = counter_quantity_after_fee;
        }

        counter_quantity_after_fee < other.calculate_counter_quantity_after_fee()
    }
}

impl<'a> TryFrom<(&mut OrderBook<'a>, &mut OrderBook<'a>, Cow<'a, str>)> for SwapTrade<'a> {
    type Error = anyhow::Error;

    fn try_from(
        (sell_order_book, buy_order_book, trading_currency): (
            &mut OrderBook<'a>,
            &mut OrderBook<'a>,
            Cow<'a, str>,
        ),
    ) -> Result<Self> {
        sell_order_book.sort();
        buy_order_book.sort();
        if sell_order_book.base.is_same_currency(&buy_order_book.base)
            && sell_order_book
                .counter
                .is_same_currency(&buy_order_book.counter)
            && sell_order_book.base.currency_code == trading_currency
            && buy_order_book.base.currency_code == trading_currency
        {
            // 1. Example (Trade for more XRP)
            // *Order Book 1*: XRP/USD:GateHub <br>
            // *Order Book 2*: XRP/USD:Bitstamp <br>
            // *Consuming Bid of XRP/USD:GateHub*: XRP -> USD:GateHub (sell XRP) <br>
            // *Consuming Ask of XRP/USD:Bitstamp*: USD:Bitstamp -> XRP (buy XRP)
            let mut buy_order = buy_order_book.asks.orders[0].clone();
            buy_order.flip();
            Ok(SwapTrade {
                sell_order: sell_order_book.bids.orders[0].clone(),
                buy_order,
                starting_currency: sell_order_book.base.clone(),
            })
        } else if sell_order_book.base.is_same_currency(&buy_order_book.base)
            && sell_order_book
                .counter
                .is_same_currency(&buy_order_book.counter)
            && sell_order_book.counter.currency_code == trading_currency
            && buy_order_book.counter.currency_code == trading_currency
        {
            // 2. Example (Trade for more USD)
            // *Order Book 1*: XRP/USD:GateHub <br>
            // *Order Book 2*: XRP/USD:Bitstamp <br>
            // *Consuming Ask of XRP/USD:GateHub*: USD:GateHub -> XRP (sell USD) <br>
            // *Consuming Bid of XRP/USD:Bitstamp*: XRP -> USD:Bitstamp (buy USD)
            let mut sell_order = sell_order_book.asks.orders[0].clone();
            sell_order.flip();
            Ok(SwapTrade {
                sell_order,
                buy_order: buy_order_book.bids.orders[0].clone(),
                starting_currency: sell_order_book.counter.clone(),
            })
        } else if sell_order_book
            .base
            .is_same_currency(&buy_order_book.counter)
            && sell_order_book
                .counter
                .is_same_currency(&buy_order_book.base)
            && sell_order_book.base.currency_code == trading_currency
            && buy_order_book.counter.currency_code == trading_currency
        {
            // ### 3. Example (Trade for more XRP with second order book flipped)
            // *Order Book 1*: XRP/USD:GateHub <br>
            // *Order Book 2*: USD:Bitstamp/XRP <br>
            // *Consuming Bid of XRP/USD:GateHub*: XRP -> USD:GateHub (sell XRP) <br>
            // *Consuming Bid of USD:Bitstamp/XRP*: USD:Bitstamp -> XRP (buy XRP)
            Ok(SwapTrade {
                sell_order: sell_order_book.bids.orders[0].clone(),
                buy_order: buy_order_book.bids.orders[0].clone(),
                starting_currency: sell_order_book.base.clone(),
            })
        } else if sell_order_book
            .base
            .is_same_currency(&buy_order_book.counter)
            && sell_order_book
                .counter
                .is_same_currency(&buy_order_book.base)
            && sell_order_book.counter.currency_code == trading_currency
            && buy_order_book.base.currency_code == trading_currency
        {
            // ### 4. Example (Trade for more USD with second order book flipped)
            // *Order Book 1*: XRP/USD:GateHub <br>
            // *Order Book 2*: USD:Bitstamp/XRP <br>
            // *Consuming Ask of XRP/USD:GateHub*: USD:GateHub -> XRP (sell USD) <br>
            // *Consuming Ask of USD:Bitstamp/XRP*: XRP -> USD:Bitstamp (buy USD)
            let mut sell_order = sell_order_book.asks.orders[0].clone();
            sell_order.flip();
            let mut buy_order = buy_order_book.asks.orders[0].clone();
            buy_order.flip();
            Ok(SwapTrade {
                sell_order,
                buy_order,
                starting_currency: sell_order_book.counter.clone(),
            })
        } else {
            bail!(SwapArbitrageException::InvalidOrderBookCombo)
        }
    }
}

impl GetProfitableTrades for OrderBooks<'_> {
    fn get_profitable_trades(&self) -> Vec<SwapTrade<'_>> {
        let mut profitable_trades = Vec::new();
        for i in 0..self.order_books.len() {
            for j in i + 1..self.order_books.len() {
                let mut order_book_1 = self.order_books[i].clone();
                order_book_1.sort();
                let mut order_book_2 = self.order_books[j].clone();
                order_book_2.sort();
                let trading_base_currency = order_book_1.base.clone();
                let trading_counter_currency = order_book_1.counter.clone();
                let trade_1 = SwapTrade::try_from((
                    &mut order_book_1,
                    &mut order_book_2,
                    trading_base_currency.currency_code.clone(),
                ))
                .unwrap();
                let trade_2 = SwapTrade::try_from((
                    &mut order_book_1,
                    &mut order_book_2,
                    trading_counter_currency.currency_code.clone(),
                ))
                .unwrap();
                let trade_3 = SwapTrade::try_from((
                    &mut order_book_2,
                    &mut order_book_1,
                    trading_base_currency.currency_code.clone(),
                ))
                .unwrap();
                let trade_4 = SwapTrade::try_from((
                    &mut order_book_2,
                    &mut order_book_1,
                    trading_counter_currency.currency_code.clone(),
                ))
                .unwrap();
                if trade_1.is_profitable() {
                    profitable_trades.push(trade_1);
                }
                if trade_2.is_profitable() {
                    profitable_trades.push(trade_2);
                }
                if trade_3.is_profitable() {
                    profitable_trades.push(trade_3);
                }
                if trade_4.is_profitable() {
                    profitable_trades.push(trade_4);
                }
            }
        }

        profitable_trades
    }
}

pub trait GetProfitableTrades {
    fn get_profitable_trades(&self) -> Vec<SwapTrade<'_>>;
}
