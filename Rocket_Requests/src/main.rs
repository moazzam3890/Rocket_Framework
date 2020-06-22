
#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;

mod dynamics;

fn main() {
    rocket::ignite()
    .mount("/", routes![dynamics::dyn_path])
    .launch();
}