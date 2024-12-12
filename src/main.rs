#[macro_use] extern crate rocket;


#[get("/")]
fn default() -> String {
    "Hello, world!".to_string()
}

#[get("/<x>", rank = 2)]
    fn index(x: &str) -> String {
        format!("Hello, {}!",x)
    }

#[launch]
fn rocket() -> _ {
    rocket::build()
                .mount("/", routes![index, default])
}
