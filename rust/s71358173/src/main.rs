#[macro_use] extern crate rocket;
use std::collections::HashMap;
use rocket_dyn_templates::Template;
use rocket::form::Form;
// use rocket::response::{Flash, Redirect};
use rocket::serde::{Serialize, json::Json};

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", &context)
}

#[derive(Debug, FromForm, Serialize)]
struct Task<'r> {
   description: &'r str,
   completed: bool
}

#[post("/", data = "<task>")]
fn new(task: Form<Task<'_>>) -> Template {
    println!("{:?}", *task);
    if task.description.is_empty() {
        // Flash::error(Redirect::to(uri!(index)), "Cannot be empty.")
        Template::render("index", &*task)
    } else {
        // Flash::success(Redirect::to(uri!(home)), "Task added.")
        // let mut context: HashMap<&str, &str> = HashMap::new();
        Template::render("home", &*task)
    }
}

#[get("/home")]
fn home() -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    Template::render("home", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, new, home])
    .attach(Template::fairing())
}
