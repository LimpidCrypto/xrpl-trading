use std::borrow::Cow;

use trading_lib::order_books::{
    order::Order,
    order_book::{OrderBook, OrderBookSideType},
    Flip, OrderBooks,
};

use super::dummy_data::{
    generate_currency, generate_random_currency_code, generate_random_decimal,
};

fn generate_orders<'a>(
    num: usize,
    base_currency: Option<Cow<'a, str>>,
    counter_currency: Option<Cow<'a, str>>,
) -> Vec<Order<'a>> {
    let mut base = generate_currency();
    if let Some(base_currency) = base_currency {
        base.currency_code = base_currency;
    }
    let mut counter = generate_currency();
    if let Some(counter_currency) = counter_currency {
        counter.currency_code = counter_currency;
    }
    let mut rate = generate_random_decimal(0.01, 1.0);
    let mut orders = Vec::new();
    for _ in 0..num {
        rate *= generate_random_decimal(1.05, 1.2);
        orders.push(Order {
            base: base.clone(),
            counter: counter.clone(),
            base_quantity: generate_random_decimal(1.0, 100.0),
            rate,
        });
    }

    orders
}

fn generate_order_books_with_same_currency_codes<'a>(
    num: usize,
    num_orders: usize,
) -> Vec<OrderBook<'a>> {
    let base_currency_code = generate_random_currency_code();
    let counter_currency_code = generate_random_currency_code();
    let mut order_books = Vec::new();
    for _ in 0..num {
        let orders = generate_orders(
            num_orders,
            Some(base_currency_code.clone().into()),
            Some(counter_currency_code.clone().into()),
        );
        let base_currency = orders[0].base.clone();
        let counter_currency = orders[0].counter.clone();
        let (bids, asks) = orders.split_at(num_orders / 2);
        let mut order_book = OrderBook {
            base: base_currency,
            counter: counter_currency,
            bids: (bids, OrderBookSideType::Bids).into(),
            asks: (asks, OrderBookSideType::Asks).into(),
        };
        if rand::random() {
            order_book.flip();
        }
        order_books.push(order_book);
    }

    order_books
}

pub fn generate_order_books<'a>(
    num_currency_pairs: usize,
    num_order_books_per_currency_pair: usize,
) -> OrderBooks<'a> {
    let mut order_books = Vec::new();
    for _ in 0..num_currency_pairs {
        let order_books_with_same_currency_codes =
            generate_order_books_with_same_currency_codes(num_order_books_per_currency_pair, 10);
        order_books.extend(order_books_with_same_currency_codes);
    }

    OrderBooks {
        order_books,
        liquidity_spread: 0.1,
    }
}
