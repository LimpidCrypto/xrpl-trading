use std::{cmp::Ordering, mem::swap};

use rust_decimal::{prelude::FromPrimitive, Decimal};
#[cfg(feature = "xrpl")]
use xrpl::models::{
    amount::Amount,
    currency::{Currency as XRPLCurrency, IssuedCurrency, XRP},
    ledger::Offer,
    transactions::{OfferCreate, OfferCreateFlag},
    FlagCollection,
};

use crate::models::currency::Currency;

use super::Flip;

#[derive(Debug, Clone)]
pub struct Order<'a> {
    pub base: Currency<'a>,
    pub counter: Currency<'a>,
    pub base_quantity: Decimal,
    pub rate: Decimal,
}

impl Flip for Order<'_> {
    fn flip(&mut self) {
        self.rate = Decimal::from(1) / self.rate;
        self.base_quantity = self.base_quantity * self.rate;
        swap(&mut self.base, &mut self.counter);
    }
}

impl PartialEq for Order<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.rate == other.rate
    }
}

impl Eq for Order<'_> {}

impl PartialOrd for Order<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rate.partial_cmp(&other.rate)
    }
}

impl Ord for Order<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rate.cmp(&other.rate)
    }
}

impl Order<'_> {
    pub fn calculate_counter_quantity_after_fee(&self) -> Decimal {
        (self.base_quantity * self.rate)
            * Decimal::from_f32(1.0 - self.counter.transfer_fee).unwrap()
    }
}

#[cfg(feature = "xrpl")]
impl<'a> From<Offer<'a>> for Order<'a> {
    fn from(offer: Offer<'a>) -> Self {
        let (taker_gets_currency, taker_gets_amount): (XRPLCurrency, String) =
            match offer.taker_gets {
                Amount::XRPAmount(xrp) => (XRP::new().into(), xrp.0.into()),
                Amount::IssuedCurrencyAmount(issued_currency) => (
                    IssuedCurrency::new(issued_currency.currency, issued_currency.issuer).into(),
                    issued_currency.value.into(),
                ),
            };
        let (taker_pays_currency, taker_pays_amount): (XRPLCurrency, String) =
            match offer.taker_pays {
                Amount::XRPAmount(xrp) => (XRP::new().into(), xrp.0.into()),
                Amount::IssuedCurrencyAmount(issued_currency) => (
                    IssuedCurrency::new(issued_currency.currency, issued_currency.issuer).into(),
                    issued_currency.value.into(),
                ),
            };
        let taker_gets_amount: Decimal = taker_gets_amount.parse().unwrap();
        let taker_pays_amount: Decimal = taker_pays_amount.parse().unwrap();

        Self {
            base: Currency::from_xrpl(taker_pays_currency, None),
            counter: Currency::from_xrpl(taker_gets_currency, None),
            base_quantity: taker_pays_amount,
            rate: taker_gets_amount / taker_pays_amount,
        }
    }
}

#[cfg(feature = "xrpl")]
impl<'a> Into<OfferCreate<'a>> for Order<'a> {
    fn into(self) -> OfferCreate<'a> {
        let base_qty = self.base_quantity.to_string();
        let counter_qty = self.calculate_counter_quantity_after_fee().to_string();

        OfferCreate::new(
            "".into(),
            None,
            None,
            Some(FlagCollection::new(vec![
                OfferCreateFlag::TfImmediateOrCancel,
                OfferCreateFlag::TfSell,
            ])),
            None,
            None,
            None,
            None,
            None,
            None,
            self.counter.get_xrpl_amount(counter_qty.into()),
            self.base.get_xrpl_amount(base_qty.into()),
            None,
            None,
        )
    }
}

#[cfg(test)]
mod order_tests {
    use rust_decimal::{prelude::FromPrimitive, Decimal};
    use xrpl::models::{
        amount::{IssuedCurrencyAmount, XRPAmount},
        FlagCollection,
    };

    use super::*;

    #[test]
    #[cfg(feature = "xrpl")]
    fn test_from_offer() {
        let offer = Offer::new(
            FlagCollection::new(Vec::new()),
            None,
            None,
            "r".into(),
            "1".into(),
            "10".into(),
            "".into(),
            "".into(),
            0,
            0,
            IssuedCurrencyAmount::new("USD".into(), "issuer".into(), "10".into()).into(),
            XRPAmount("20".into()).into(),
            None,
        );
        let order = Order::from(offer);
        assert_eq!(
            order,
            Order {
                base: Currency::from_xrpl(XRPLCurrency::XRP(XRP::new()), None),
                counter: Currency::from_xrpl(
                    XRPLCurrency::IssuedCurrency(IssuedCurrency::new(
                        "USD".into(),
                        "issuer".into()
                    )),
                    None
                ),
                base_quantity: Decimal::from(20),
                rate: Decimal::from_f32(0.5).unwrap(),
            }
        );
    }
}
