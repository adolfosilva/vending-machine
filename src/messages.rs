use actix::{Context, Handler, Message};
use rust_decimal::Decimal;

pub type Money = Decimal;

use actors::VendingMachine;
use product::Product;

#[derive(Debug, PartialEq, Eq)]
pub struct Buy(pub Product, pub Money);

impl Message for Buy {
    type Result = VendingResult;
}

impl Handler<Buy> for VendingMachine {
    type Result = VendingResult;

    fn handle(&mut self, msg: Buy, _ctx: &mut Context<Self>) -> Self::Result {
        let (product, money) = (msg.0, msg.1);
        match self.products.get_mut(&product) {
            Some(0) => {
                let product = product.name.to_string();
                Err(VendingMachineError::OutOfProduct(product))
            }
            Some(qt) => {
                *qt = *qt - 1;
                println!("{} remaining: {:?}", product.name, qt);
                self.balance -= product.price;
                Ok(product)
            }
            None => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VendingMachineError {
    OutOfProduct(String),
}

pub type VendingResult = Result<Product, VendingMachineError>;
