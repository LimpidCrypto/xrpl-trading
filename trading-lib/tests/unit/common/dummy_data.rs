use rand::Rng;
use rust_decimal::prelude::*;
use trading_lib::models::currency::Currency;

pub fn generate_currency<'a>() -> Currency<'a> {
    let currency = Currency {
        currency_code: generate_random_currency_code().into(),
        issuer: generate_random_issuer().into(),
        transfer_fee: generate_random_f32(0.0, 0.5),
    };
    currency
}

pub fn generate_random_currency_code() -> String {
    let currency_code = generate_random_string(3);
    currency_code.to_uppercase()
}

pub fn generate_random_issuer() -> String {
    let issuer = generate_random_string(10);
    issuer
}

/// Generate a random string of a given length with both uppercase and lowercase characters.
pub fn generate_random_string(len: u8) -> String {
    let mut rng = rand::thread_rng();
    let s: String = (0..len)
        .map(|_| {
            let choice = rng.gen_range(0..2);
            if choice == 0 {
                rng.gen_range(b'A'..=b'Z') as char
            } else {
                rng.gen_range(b'a'..=b'z') as char
            }
        })
        .collect();
    s
}

pub fn generate_random_decimal(min: f32, max: f32) -> Decimal {
    let random_f32 = generate_random_f32(min, max);
    Decimal::from_f32(random_f32).unwrap()
}

pub fn generate_random_f32(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
