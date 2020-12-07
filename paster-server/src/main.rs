#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index_handler, get_text_handler, new_entry_handler],
        )
        .register(catchers![not_found])
        .launch();

    let rand_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .collect();

    println!("{}", rand_string);
}
