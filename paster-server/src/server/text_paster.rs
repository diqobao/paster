#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct TextEntry {
    value: String,
    uuid: String,
}

#[get("/")]
fn index_handler() -> &'static str {
    "Paster"
}

#[get("/<uuid>")]
fn get_text_handler(uuid: String) -> String {
    format!("get_text_handler, {}!", uuid)
}

#[get("/new")]
fn new_text_entry_page() -> &'static str {
    "new_entry_handler"
}

#[post("/new", data = "<entry>")]
fn new_entry_handler(entry: Form<Entry>) -> &'static str {
    "new_entry_handler"
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' doesn't exist.", req.uri())
}