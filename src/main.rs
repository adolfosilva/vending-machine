extern crate actix;
extern crate futures;
extern crate rust_decimal;

use actix::*;
use futures::{future, Future};
use rust_decimal::Decimal;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Product {
    name: &'static str,
    price: Decimal,
}

#[derive(Debug, PartialEq, Eq)]
struct Buy(Product);

impl Message for Buy {
    type Result = VendingResult;
}

type Quantity = u8;

#[derive(Debug, Clone)]
struct VendingMachine {
    products: HashMap<Product, Quantity>,
    balance: Decimal,
}

impl VendingMachine {
    fn new(products: HashMap<Product, Quantity>) -> Self {
        VendingMachine {
            products: products,
            balance: Decimal::new(450, 2),
        }
    }
}

impl Actor for VendingMachine {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Vending Machine running!");
    }
}

#[derive(Debug, PartialEq)]
enum VendingMachineError {
    OutOfProduct(String),
}

type VendingResult = Result<Product, VendingMachineError>;

impl Handler<Buy> for VendingMachine {
    type Result = VendingResult;

    fn handle(&mut self, msg: Buy, _ctx: &mut Context<Self>) -> Self::Result {
        let product = msg.0;
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

fn main() {
    let system = System::new("");

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

    let machine = VendingMachine::new(products).start();

    machine.do_send(Buy(coke));
    machine.do_send(Buy(coke));
    machine.do_send(Buy(coke));
    machine.do_send(Buy(coke));
    machine.do_send(Buy(apple));
    machine.do_send(Buy(apple));
    machine.do_send(Buy(apple));
    let res = machine.send(Buy(apple));

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
