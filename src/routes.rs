use std::collections::HashMap;

use crate::models::*;
use crate::services::structs;
use either::Either;
use rocket::form::Form;
use rocket::serde::de::value::MapDeserializer;
use rocket::serde::json::serde_json;
use rocket::{routes, Route, State};

#[post("/", format = "json", data = "<data>")]
pub async fn login(data: Form<Login>, host: &State<structs::Host>) -> String {
    //extract login data
    let login_data = data.into_inner();

    //map login data into hash map
    let login_map_wrapped = login_data.extract();
    if login_map_wrapped.is_err() {
        return String::from("invalid data");
    }
    let login_map = login_map_wrapped.unwrap();

    let login_resp_wrapped = host.login(&login_map).await;
    if login_resp_wrapped.is_err() {
        return String::from("login failed");
    }
    let login_resp = login_resp_wrapped.unwrap();

    let reply = match login_resp {
        Either::Right(map) => {}
        Either::Left(strn) => {}
    };

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
