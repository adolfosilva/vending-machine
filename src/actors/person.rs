use actix::{Actor, Context};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Person {
    pub money: Decimal,
}

impl Actor for Person {
    type Context = Context<Self>;
}
