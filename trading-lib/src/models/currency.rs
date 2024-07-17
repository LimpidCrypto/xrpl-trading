use std::borrow::Cow;

extern crate xrpl;

use xrpl::models::amount::{Amount, IssuedCurrencyAmount, XRPAmount};
#[cfg(feature = "xrpl")]
use xrpl::models::currency::Currency as XRPLCurrency;

#[derive(Debug, Clone, PartialEq)]
pub struct Currency<'a> {
    pub currency_code: Cow<'a, str>,
    pub issuer: Cow<'a, str>,
    pub transfer_fee: f32,
}

#[cfg(feature = "xrpl")]
impl<'a> Currency<'a> {
    pub fn get_xrpl_amount(&self, amount: Cow<'a, str>) -> Amount<'a> {
        match self.currency_code.as_ref() {
            "XRP" => XRPAmount(amount).into(),
            _ => IssuedCurrencyAmount::new(self.currency_code.clone(), self.issuer.clone(), amount)
                .into(),
        }
    }
}

#[cfg(feature = "xrpl")]
impl<'a> Currency<'a> {
    pub fn from_xrpl(currency: XRPLCurrency<'a>, transfer_fee: Option<f32>) -> Self {
        match currency {
            XRPLCurrency::XRP(_) => Self {
                currency_code: "XRP".into(),
                issuer: "".into(),
                transfer_fee: 0.0,
            },
            XRPLCurrency::IssuedCurrency(issued_currency) => Self {
                currency_code: issued_currency.currency,
                issuer: issued_currency.issuer,
                transfer_fee: transfer_fee.unwrap_or(0.0),
            },
        }
    }
}

impl Currency<'_> {
    pub fn is_same_currency(&self, other: &Self) -> bool {
        self.currency_code == other.currency_code
    }
}
