use std::{
    borrow::{Borrow, Cow},
    mem::swap,
    sync::{Arc, Mutex, MutexGuard},
};

use anyhow::{bail, Result};
use rust_decimal::prelude::ToPrimitive;

use crate::{models::currency::Currency, utils::anyhow_mutex};

use super::{exceptions::OrderBookException, order::Order, Flip, IsLiquid};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderBookSideType {
    Bids,
    Asks,
}

#[derive(Debug, Clone)]
pub struct OrderBookSide<'a> {
    pub side_type: OrderBookSideType,
    pub orders: Cow<'a, [Order<'a>]>,
}

impl<'a> From<(Cow<'a, [Order<'a>]>, OrderBookSideType)> for OrderBookSide<'a> {
    fn from((orders, side_type): (Cow<'a, [Order<'a>]>, OrderBookSideType)) -> Self {
        Self { side_type, orders }
    }
}

#[derive(Debug, Clone)]
pub struct OrderBook<'a> {
    // The currency that is being sold.
    pub base: Currency<'a>,
    // The currency that is being bought.
    pub counter: Currency<'a>,
    pub bids: Arc<Mutex<OrderBookSide<'a>>>,
    pub asks: Arc<Mutex<OrderBookSide<'a>>>,
}

impl Flip for OrderBook<'_> {
    fn flip(&mut self) -> Result<()> {
        let mut bids = self.get_bids()?;
        for order in bids.orders.to_mut() {
            order.flip()?;
        }
        // for order in self.get_asks()?.orders.to_mut() {
        //     order.flip()?;
        // }
        // swap(&mut self.bids, &mut self.asks);
        // swap(&mut self.base, &mut self.counter);
        Ok(())
    }
}

impl<'a> IsLiquid<'a> for OrderBook<'a> {
    fn is_liquid(&'a self, liquidity_spread: f64) -> Result<bool> {
        let order_book_spread = self.calculate_spread_pct()?;

        Ok(order_book_spread <= liquidity_spread)
    }
}

impl<'a> OrderBook<'a> {
    pub fn get_asks(&'a self) -> Result<MutexGuard<'_, OrderBookSide<'_>>> {
        anyhow_mutex(&self.asks)
    }

    pub fn get_bids(&'a self) -> Result<MutexGuard<'_, OrderBookSide<'_>>> {
        anyhow_mutex(&self.bids)
    }

    /// Sorts the bids in descending order and the asks in ascending order.
    pub fn sort(&'a mut self) -> Result<()> {
        self.get_bids()?.orders.to_mut().sort_by(|a, b| b.cmp(a));
        self.get_asks()?.orders.to_mut().sort();

        Ok(())
    }

    pub fn calculate_spread_pct(&'a self) -> Result<f64> {
        let bids = &self.get_bids()?.orders;
        let best_bid = bids.first().unwrap();
        let asks = &self.get_asks()?.orders;
        let best_ask = asks.first().unwrap();
        let spread = best_ask.rate - best_bid.rate;
        let spread_pct = spread / best_bid.rate;
        Ok(spread_pct.to_f64().unwrap())
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
            Some(OrderBookSideType::Bids) => self.bids.orders.lock().unwrap().to_mut().push(order),
            Some(OrderBookSideType::Asks) => {
                order.flip();
                self.asks.orders.lock().unwrap().to_mut().push(order)
            }
            None => bail!(OrderBookException::InvalidOrder),
        }
        self.sort();
        Ok(())
    }
}

// #[cfg(test)]
// mod order_book_tests {
//     use rust_decimal::{prelude::FromPrimitive, Decimal};

//     use super::*;

//     #[test]
//     #[cfg(feature = "xrpl")]
//     fn test_sort_order_book() {
//         use xrpl::models::currency::{Currency as XRPLCurrency, IssuedCurrency, XRP};
//         let mut order_book = OrderBook {
//             base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
//             counter: Currency::from_xrpl(
//                 XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
//                     "USD".into(),
//                     "rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS".into(),
//                 )),
//                 None,
//             ),
//             bids: OrderBookSide {
//                 side_type: OrderBookSideType::Bids,
//                 orders: vec![
//                     Order {
//                         base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
//                         counter: Currency::from_xrpl(
//                             XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
//                                 "USD".into(),
//                                 "rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS".into(),
//                             )),
//                             None,
//                         ),
//                         base_quantity: Decimal::from(80),
//                         rate: Decimal::from_f32(0.23).unwrap(),
//                     },
//                     Order {
//                         base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
//                         counter: Currency::from_xrpl(
//                             XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
//                                 "USD".into(),
//                                 "rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS".into(),
//                             )),
//                             None,
//                         ),
//                         base_quantity: Decimal::from(100),
//                         rate: Decimal::from_f32(0.24).unwrap(),
//                     },
//                 ],
//             },
//             asks: OrderBookSide {
//                 side_type: OrderBookSideType::Asks,
//                 orders: vec![
//                     Order {
//                         base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
//                         counter: Currency::from_xrpl(
//                             XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
//                                 "USD".into(),
//                                 "rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS".into(),
//                             )),
//                             None,
//                         ),
//                         base_quantity: Decimal::from(100),
//                         rate: Decimal::from_f32(0.26).unwrap(),
//                     },
//                     Order {
//                         base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
//                         counter: Currency::from_xrpl(
//                             XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
//                                 "USD".into(),
//                                 "rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS".into(),
//                             )),
//                             None,
//                         ),
//                         base_quantity: Decimal::from(90),
//                         rate: Decimal::from_f32(0.25).unwrap(),
//                     },
//                 ],
//             },
//         };
//         order_book.sort();
//         assert_eq!(
//             order_book.bids.orders[0].rate,
//             Decimal::from_f32(0.24).unwrap()
//         );
//         assert_eq!(
//             order_book.asks.orders[0].rate,
//             Decimal::from_f32(0.25).unwrap()
//         );
//     }
// }
