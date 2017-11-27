#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
#![allow(dead_code, unused_variables)]

extern crate rocket;

use rocket::{Rocket, State};

#[get("/")]
fn index(state: State<u32>) {  }

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .manage(100u32)
}

#[test]
fn main() {
    if false {
        rocket().launch();
    }

    let instance = rocket();
    if false {
        instance.launch();
    }
}
