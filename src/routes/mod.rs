use crate::models::*;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post("/", format = "json", data = "<data>")]
pub async fn login(data: Json<Login>) -> String {
    String::from("login")
}

#[put("/", format = "json", data = "<data>")]
pub async fn refresh(data: Json<Token>) -> String {
    String::from("refresh")
}

#[delete("/", format = "json", data = "<data>")]
pub async fn logout(data: Json<Token>) -> String {
    String::from("logout")
}

#[post("/register", format = "json", data = "<data>")]
pub async fn register(data: Json<Login>) -> String {
    String::from("register")
}

#[get("/register/<verification_id>")]
pub async fn verify(verification_id: &str) -> String {
    String::from("verify")
}

pub fn get_routes() -> Vec<Route> {
    routes![login, refresh, logout, register, verify]
}
