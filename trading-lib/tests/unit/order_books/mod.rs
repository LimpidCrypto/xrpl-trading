#[cfg(test)]
mod test_sorting {
    use crate::common::_static::order_books::{ORDER_BOOKS, ORDER_BOOKS_LIST};

    #[test]
    fn test_order_book_sorting() {
        let mut order_book = ORDER_BOOKS_LIST[0].clone();
        assert!(order_book.bids.orders[0].rate < order_book.bids.orders[1].rate);
        order_book.sort();
        assert!(order_book.bids.orders[0].rate > order_book.bids.orders[1].rate);
    }

    #[test]
    fn test_order_books_sorting() {
        let mut order_books = ORDER_BOOKS.clone();
        assert!(
            order_books.order_books[0].bids.orders[0].rate
                < order_books.order_books[0].bids.orders[1].rate
        );
        order_books.sort();
        assert!(
            order_books.order_books[0].bids.orders[0].rate
                > order_books.order_books[0].bids.orders[1].rate
        );
    }
}

#[cfg(test)]
mod test_liquidity {
    use trading_lib::order_books::IsLiquid;

    use crate::common::_static::order_books::{ORDER_BOOKS, ORDER_BOOKS_LIST};

    #[test]
    fn test_order_book_liquidity() {
        let mut order_book = ORDER_BOOKS_LIST[0].clone();
        order_book.sort();
        dbg!(&order_book.calculate_spread_pct());
        assert!(order_book.is_liquid(ORDER_BOOKS.liquidity_spread));
    }

    #[test]
    fn test_get_liquid_order_books() {
        let order_books = ORDER_BOOKS.get_liquid_order_books();
        dbg!(&order_books[0].calculate_spread_pct());
        assert_eq!(order_books.len(), 1);
    }
}
