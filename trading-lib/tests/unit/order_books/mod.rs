use super::common::order_books::generate_order_books;
use trading_lib::trading_types::arbitrage::swap::GetProfitableTrades;

#[test]
fn test_() {
    let mut order_books = generate_order_books(1, 5);
    order_books.sort();
    dbg!(order_books.get_profitable_trades());
}
