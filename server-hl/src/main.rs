

#[macro_use] extern crate rocket;

#[get("/")]
fn get_all() -> &'static str {
    "get_all"
}

#[get("/<id>")]
fn get(id: usize) -> &'static str {
    "get"
}

#[post("/")]
fn create() -> &'static str {
    "create"
}

#[put("/<id>")]
fn update(id: usize) -> &'static str {
    "update"
}

#[delete("/<id>")]
fn delete(id: usize) -> &'static str {
    "delete"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_all, get, create, update, delete])
}