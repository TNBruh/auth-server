use crate::models::*;
use crate::CONFIG;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::{routes, Route};

#[post("/", format = "json", data = "<data>")]
pub async fn login(data: Form<Login>) -> String {
    //fetch necessary data from the server or db
    let conf = &CONFIG;

    //custom logic here. it must return result to indicate successful processing

    //bcrypt hash check

    //create refresh token and access token

    //send refresh token to storage

    //if all passes; return all fetched data, access token, and refresh token
    String::from("login")
}

#[put("/", format = "json", data = "<data>")]
pub async fn refresh(data: Form<Token>) -> String {
    //check jwt key and extract value

    //if CHECK_HAS_IS_REFRESH is true, check if "is_refresh" is true in the jwt key

    //custom logic here

    //extend refresh token age in storage

    //create new access token

    //return access token
    String::from("refresh")
}

#[delete("/", format = "json", data = "<data>")]
pub async fn logout(data: Form<Token>) -> String {
    //custom login here

    //sends delete request to storage with matching id

    //return status
    String::from("logout")
}

#[post("/register", format = "json", data = "<data>")]
pub async fn register(data: Form<Login>) -> String {
    String::from("register")
}

#[get("/register/<verification_id>")]
pub async fn verify(verification_id: &str) -> String {
    String::from("verify")
}

pub fn get_routes() -> Vec<Route> {
    routes![login, refresh, logout, register, verify]
}
