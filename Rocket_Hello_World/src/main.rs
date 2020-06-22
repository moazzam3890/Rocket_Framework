#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/hello")]
fn index_0() ->&'static str {
    "Hello"
}
mod world;

fn main () {
    rocket::ignite()
    .mount("/", routes![index_0, world::others::index_1])
    .launch();
}