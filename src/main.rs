extern crate actix;
extern crate futures;
extern crate rust_decimal;

mod actors;
mod messages;
mod product;

use actors::VendingMachine;
use product::Product;
use messages::Buy;

use actix::*;
use futures::{future, Future};
use rust_decimal::Decimal;

use std::collections::HashMap;

fn main() {
    let system = System::new("VendingMachine");

    let coke = Product {
        name: "Coke",
        price: Decimal::new(8, 1),
    };
    let apple = Product {
        name: "Apple",
        price: Decimal::new(45, 2),
    };
    let mut products = HashMap::new();
    products.insert(coke, 5); // 4
    products.insert(apple, 4); // 1.8

    let machine = VendingMachine::new(products, Decimal::new(450, 2)).start();

    machine.do_send(Buy(coke, Decimal::new(45, 2)));
    machine.do_send(Buy(coke, Decimal::new(45, 2)));
    machine.do_send(Buy(coke, Decimal::new(45, 2)));
    machine.do_send(Buy(coke, Decimal::new(45, 2)));
    machine.do_send(Buy(apple, Decimal::new(235, 2)));
    machine.do_send(Buy(apple, Decimal::new(235, 2)));
    machine.do_send(Buy(apple, Decimal::new(28, 1)));
    let res = machine.send(Buy(apple, Decimal::new(28, 1)));

    Arbiter::spawn(res.then(|res| {
        match res {
            Ok(vm) => println!("VendingMachine: {:?}", vm),
            _ => println!("Something wrong"),
        }

        System::current().stop();
        future::result(Ok(()))
    }));

    system.run();
}
