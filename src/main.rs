
#[macro_use] extern crate rocket;

mod routes;
mod models;
mod db;

use rocket::fs::FileServer;
use db::load_tasks_from_file;
use std::sync::Mutex;
use std::path::Path;

// Функція запуску Rocket-сервера
#[launch]
fn rocket() -> _ {

    let tasks = load_tasks_from_file(Path::new("tasks.json")).expect("Не вдалося завантажити завдання");

    rocket::build()
        .manage(Mutex::new(tasks)) //Щоб лише один потік міг змінювати дані одночасно
        .mount("/", FileServer::from("static"))
        .mount("/", routes::routes())
}
