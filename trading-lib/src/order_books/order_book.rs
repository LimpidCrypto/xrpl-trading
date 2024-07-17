use std::mem::swap;

use anyhow::{bail, Result};
use rust_decimal::prelude::ToPrimitive;

use crate::models::currency::Currency;

use super::{exceptions::OrderBookException, order::Order, Flip, IsLiquid};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderBookSideType {
    Bids,
    Asks,
}

#[derive(Debug, Clone)]
pub struct OrderBookSide<'a> {
    pub side_type: OrderBookSideType,
    pub orders: Vec<Order<'a>>,
}

impl<'a> From<(&[Order<'a>], OrderBookSideType)> for OrderBookSide<'a> {
    fn from((orders, side_type): (&[Order<'a>], OrderBookSideType)) -> Self {
        Self {
            side_type,
            orders: orders.to_vec(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderBook<'a> {
    // The currency that is being sold.
    pub base: Currency<'a>,
    // The currency that is being bought.
    pub counter: Currency<'a>,
    pub bids: OrderBookSide<'a>,
    pub asks: OrderBookSide<'a>,
}

impl<'a> Flip for OrderBook<'a> {
    fn flip(&mut self) {
        self.bids.orders.iter_mut().for_each(|order| order.flip());
        self.asks.orders.iter_mut().for_each(|order| order.flip());
        swap(&mut self.bids, &mut self.asks);
        swap(&mut self.base, &mut self.counter);
    }
}

// impl IsProfitable for OrderBook<'_> {
//     /// Returns true if any bid in self is profitable against any ask in other.
//     fn is_profitable(&self, other: &OrderBook<'_>) -> bool {
//         self.bids
//             .orders
//             .iter()
//             .any(|bid| other.asks.orders.iter().any(|ask| bid.is_profitable(ask)))
//     }
// }

impl IsLiquid for OrderBook<'_> {
    fn is_liquid(&self, liquidity_spread: f64) -> bool {
        let order_book_spread = self.calculate_spread_pct();
        order_book_spread <= liquidity_spread
    }
}

impl PartialEq for OrderBook<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.bids.orders == other.bids.orders
    }
}

impl Eq for OrderBook<'_> {}

impl<'a> OrderBook<'a> {
    /// Sorts the bids in descending order and the asks in ascending order.
    pub fn sort(&mut self) {
        self.bids.orders.sort_by(|a, b| b.cmp(a));
        self.asks.orders.sort();
    }

    pub fn calculate_spread_pct(&self) -> f64 {
        let best_bid = self.bids.orders.first().unwrap();
        let best_ask = self.asks.orders.first().unwrap();
        let spread = best_ask.rate - best_bid.rate;
        let spread_pct = spread / best_bid.rate;
        spread_pct.to_f64().unwrap()
    }

    pub fn determain_order_book_side_type(&self, order: &Order<'_>) -> Option<OrderBookSideType> {
        if order.base == self.base && order.counter == self.counter {
            Some(OrderBookSideType::Bids)
        } else if order.base == self.counter && order.counter == self.base {
            Some(OrderBookSideType::Asks)
        } else {
            None
        }
    }

    pub fn is_order_for_order_book(&self, order: &Order<'_>) -> bool {
        self.determain_order_book_side_type(order).is_some()
    }

    pub fn add_order<'b: 'a>(&mut self, mut order: Order<'b>) -> Result<()> {
        let order_book_side = self.determain_order_book_side_type(&order);
        match order_book_side {
            Some(OrderBookSideType::Bids) => self.bids.orders.push(order),
            Some(OrderBookSideType::Asks) => {
                order.flip();
                self.asks.orders.push(order)
            }
            None => bail!(OrderBookException::InvalidOrder),
        }
        self.sort();
        Ok(())
    }
}

#[cfg(test)]
mod order_book_tests {
    use rust_decimal::{prelude::FromPrimitive, Decimal};

    use super::*;

    #[test]
    #[cfg(feature = "xrpl")]
    fn test_sort_order_book() {
        use xrpl::models::currency::{Currency as XRPLCurrency, IssuedCurrency, XRP};
        let mut order_book = OrderBook {
            base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
            counter: Currency::from_xrpl(
                XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
                    "USD".into(),
                    "rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS".into(),
                )),
                None,
            ),
            bids: OrderBookSide {
                side_type: OrderBookSideType::Bids,
                orders: vec![
                    Order {
                        base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
                        counter: Currency::from_xrpl(
                            XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
                                "USD".into(),
                                "rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS".into(),
                            )),
                            None,
                        ),
                        base_quantity: Decimal::from(80),
                        rate: Decimal::from_f32(0.23).unwrap(),
                    },
                    Order {
                        base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
                        counter: Currency::from_xrpl(
                            XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
                                "USD".into(),
                                "rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS".into(),
                            )),
                            None,
                        ),
                        base_quantity: Decimal::from(100),
                        rate: Decimal::from_f32(0.24).unwrap(),
                    },
                ],
            },
            asks: OrderBookSide {
                side_type: OrderBookSideType::Asks,
                orders: vec![
                    Order {
                        base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
                        counter: Currency::from_xrpl(
                            XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
                                "USD".into(),
                                "rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS".into(),
                            )),
                            None,
                        ),
                        base_quantity: Decimal::from(100),
                        rate: Decimal::from_f32(0.26).unwrap(),
                    },
                    Order {
                        base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
                        counter: Currency::from_xrpl(
                            XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
                                "USD".into(),
                                "rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS".into(),
                            )),
                            None,
                        ),
                        base_quantity: Decimal::from(90),
                        rate: Decimal::from_f32(0.25).unwrap(),
                    },
                ],
            },
        };
        order_book.sort();
        assert_eq!(
            order_book.bids.orders[0].rate,
            Decimal::from_f32(0.24).unwrap()
        );
        assert_eq!(
            order_book.asks.orders[0].rate,
            Decimal::from_f32(0.25).unwrap()
        );
    }
}
