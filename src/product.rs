use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Product {
    pub name: &'static str,
    pub price: Decimal,
}
