
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;

mod dynamics;
mod multiple_segments;
mod forwarding;

fn main() {
    rocket::ignite()
    .mount("/", routes![dynamics::dyn_path, forwarding::user_ssb, forwarding::user_sus])
    .mount("/public", StaticFiles::from("multiple_segments::mult_seg"))
    .launch();
}