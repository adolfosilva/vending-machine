use actix::*;
use rust_decimal::Decimal;

use std::collections::HashMap;

use product::Product;

type Quantity = u8;

#[derive(Debug, Clone)]
pub struct VendingMachine {
    pub products: HashMap<Product, Quantity>,
    pub balance: Decimal,
}

impl VendingMachine {
    pub fn new(products: HashMap<Product, Quantity>, balance: Decimal) -> Self {
        VendingMachine { products, balance }
    }
}

impl Actor for VendingMachine {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Vending Machine running!");
    }
}
