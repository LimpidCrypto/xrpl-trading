use std::borrow::Cow;

use rust_decimal::Decimal;
use trading_lib::{
    models::currency::Currency,
    order_books::{
        order::Order,
        order_book::{OrderBook, OrderBookSide, OrderBookSideType},
        OrderBooks,
    },
};

pub static ORDERS: &[&[&[Order<'_>]]] = &[
    // 1. XRP/USD:rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS
    &[
        // 1.1 XRP/USD bids
        &[
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(80, 0, 0, false, 0),
                rate: Decimal::from_parts(23, 0, 0, false, 2),
            },
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(24, 0, 0, false, 2),
            },
        ],
        // 1.2 XRP/USD asks
        &[
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(26, 0, 0, false, 2),
            },
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(28, 0, 0, false, 2),
            },
        ],
    ],
    // 2. EUR:rAPKsP3tt7fV9Vj2QWzBk1r4Fg5vY1YhZ/USD:rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS
    &[
        // 2.1 EUR/USD bids
        &[
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("EUR"),
                    issuer: Cow::Borrowed("rAPKsP3tt7fV9Vj2QWzBk1r4Fg5vY1YhZ"),
                    transfer_fee: 0.1,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(80, 0, 0, false, 0),
                rate: Decimal::from_parts(103, 0, 0, false, 2),
            },
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("EUR"),
                    issuer: Cow::Borrowed("rAPKsP3tt7fV9Vj2QWzBk1r4Fg5vY1YhZ"),
                    transfer_fee: 0.1,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(104, 0, 0, false, 2),
            },
        ],
        // 2.2 EUR/USD asks
        &[
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("EUR"),
                    issuer: Cow::Borrowed("rAPKsP3tt7fV9Vj2QWzBk1r4Fg5vY1YhZ"),
                    transfer_fee: 0.1,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(106, 0, 0, false, 2),
            },
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("EUR"),
                    issuer: Cow::Borrowed("rAPKsP3tt7fV9Vj2QWzBk1r4Fg5vY1YhZ"),
                    transfer_fee: 0.1,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(108, 0, 0, false, 2),
            },
        ],
    ],
    // 3. XRP/EUR:r5m7tZjQoEzD7dZSdNfjXxK9z4r7zgA8v
    &[
        // 3.1 XRP/EUR bids
        &[
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("EUR"),
                    issuer: Cow::Borrowed("r5m7tZjQoEzD7dZSdNfjXxK9z4r7zgA8v"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(80, 0, 0, false, 0),
                rate: Decimal::from_parts(23, 0, 0, false, 2),
            },
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("EUR"),
                    issuer: Cow::Borrowed("r5m7tZjQoEzD7dZSdNfjXxK9z4r7zgA8v"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(24, 0, 0, false, 2),
            },
        ],
        // 3.2 XRP/EUR asks
        &[
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("EUR"),
                    issuer: Cow::Borrowed("r5m7tZjQoEzD7dZSdNfjXxK9z4r7zgA8v"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(26, 0, 0, false, 2),
            },
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("EUR"),
                    issuer: Cow::Borrowed("r5m7tZjQoEzD7dZSdNfjXxK9z4r7zgA8v"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(28, 0, 0, false, 2),
            },
        ],
    ],
    // 4. XRP/USD:rPVMhWBsfF9iMXYj3aAzJVkPDTFNSyWdKy
    &[
        // 4.1 XRP/USD bids
        &[
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rPVMhWBsfF9iMXYj3aAzJVkPDTFNSyWdKy"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(80, 0, 0, false, 0),
                rate: Decimal::from_parts(23, 0, 0, false, 2),
            },
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rPVMhWBsfF9iMXYj3aAzJVkPDTFNSyWdKy"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(24, 0, 0, false, 2),
            },
        ],
        // 4.2 XRP/USD asks
        &[
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rPVMhWBsfF9iMXYj3aAzJVkPDTFNSyWdKy"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(26, 0, 0, false, 2),
            },
            Order {
                base: Currency {
                    currency_code: Cow::Borrowed("XRP"),
                    issuer: Cow::Borrowed(""),
                    transfer_fee: 0.0,
                },
                counter: Currency {
                    currency_code: Cow::Borrowed("USD"),
                    issuer: Cow::Borrowed("rPVMhWBsfF9iMXYj3aAzJVkPDTFNSyWdKy"),
                    transfer_fee: 0.1,
                },
                base_quantity: Decimal::from_parts(100, 0, 0, false, 0),
                rate: Decimal::from_parts(28, 0, 0, false, 2),
            },
        ],
    ],
];

pub static ORDER_BOOKS_LIST: &[OrderBook<'_>] = &[
    OrderBook {
        base: Currency {
            currency_code: Cow::Borrowed("XRP"),
            issuer: Cow::Borrowed(""),
            transfer_fee: 0.0,
        },
        counter: Currency {
            currency_code: Cow::Borrowed("USD"),
            issuer: Cow::Borrowed("rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS"),
            transfer_fee: 0.1,
        },
        bids: OrderBookSide {
            side_type: OrderBookSideType::Bids,
            orders: Cow::Borrowed(ORDERS[0][0]),
        },
        asks: OrderBookSide {
            side_type: OrderBookSideType::Asks,
            orders: Cow::Borrowed(ORDERS[0][1]),
        },
    },
    OrderBook {
        base: Currency {
            currency_code: Cow::Borrowed("EUR"),
            issuer: Cow::Borrowed("rAPKsP3tt7fV9Vj2QWzBk1r4Fg5vY1YhZ"),
            transfer_fee: 0.1,
        },
        counter: Currency {
            currency_code: Cow::Borrowed("USD"),
            issuer: Cow::Borrowed("rDk7FQvkQxQQNGTtfM2Fr66s7Nm3k87vdS"),
            transfer_fee: 0.1,
        },
        bids: OrderBookSide {
            side_type: OrderBookSideType::Bids,
            orders: Cow::Borrowed(ORDERS[1][0]),
        },
        asks: OrderBookSide {
            side_type: OrderBookSideType::Asks,
            orders: Cow::Borrowed(ORDERS[1][1]),
        },
    },
    OrderBook {
        base: Currency {
            currency_code: Cow::Borrowed("XRP"),
            issuer: Cow::Borrowed(""),
            transfer_fee: 0.0,
        },
        counter: Currency {
            currency_code: Cow::Borrowed("EUR"),
            issuer: Cow::Borrowed("r5m7tZjQoEzD7dZSdNfjXxK9z4r7zgA8v"),
            transfer_fee: 0.1,
        },
        bids: OrderBookSide {
            side_type: OrderBookSideType::Bids,
            orders: Cow::Borrowed(ORDERS[2][0]),
        },
        asks: OrderBookSide {
            side_type: OrderBookSideType::Asks,
            orders: Cow::Borrowed(ORDERS[2][1]),
        },
    },
    OrderBook {
        base: Currency {
            currency_code: Cow::Borrowed("XRP"),
            issuer: Cow::Borrowed(""),
            transfer_fee: 0.0,
        },
        counter: Currency {
            currency_code: Cow::Borrowed("USD"),
            issuer: Cow::Borrowed("rPVMhWBsfF9iMXYj3aAzJVkPDTFNSyWdKy"),
            transfer_fee: 0.1,
        },
        bids: OrderBookSide {
            side_type: OrderBookSideType::Bids,
            orders: Cow::Borrowed(ORDERS[3][0]),
        },
        asks: OrderBookSide {
            side_type: OrderBookSideType::Asks,
            orders: Cow::Borrowed(ORDERS[3][1]),
        },
    },
];

pub static ORDER_BOOKS: OrderBooks<'_> = OrderBooks {
    order_books: Cow::Borrowed(ORDER_BOOKS_LIST),
    liquidity_spread: 0.05,
};
