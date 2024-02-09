use crate::AppState;

use actix_web::{get, post, web, HttpResponse, Responder};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use serde_dynamo::from_item;
use std::collections::HashMap;

#[derive(Deserialize)]
struct GetUserRequestData {
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetUserResponseData {
    user: Option<UserDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserDetails {
    email: String,
    name: String,
}

#[get("/user")]
pub async fn get_user(
    data: web::Data<AppState>,
    properties: web::Json<GetUserRequestData>,
) -> impl Responder {
    let db = data.db_client.lock().unwrap();

    let get_user_res = db
        .get_item()
        .table_name("mine-auth")
        .key("email", AttributeValue::S(properties.email.to_owned()))
        .send()
        .await;

    drop(db);

    match get_user_res {
        Ok(res) => {
            println!("{:?}", res);

            if let Some(item) = res.item {
                let user: UserDetails = from_item(item).unwrap();
                println!("{:?}", user);

                return web::Json(GetUserResponseData { user: Some(user) });
            } else {
                return web::Json(GetUserResponseData { user: None });
            }
        }
        Err(e) => {
            println!("{:?}", e);

            return web::Json(GetUserResponseData { user: None });
        }
    }
}

#[derive(Deserialize)]
struct NewUserRequestData {
    email: String,
    name: String,
}

#[post("/new-user")]
pub async fn create_user(
    data: web::Data<AppState>,
    properties: web::Json<NewUserRequestData>,
) -> impl Responder {
    let db = data.db_client.lock().unwrap();

    let mut user_item: HashMap<String, AttributeValue> = HashMap::new();
    user_item.insert(
        "email".to_string(),
        AttributeValue::S(properties.email.to_owned()),
    );
    user_item.insert(
        "name".to_string(),
        AttributeValue::S(properties.name.to_owned()),
    );

    let db_res = db
        .put_item()
        .table_name("mine-auth")
        .set_item(Some(user_item))
        .send()
        .await;

    match db_res {
        Ok(o) => {
            println!("{:?}", o);

            return HttpResponse::Ok().body(format!("{:?}", o));
        }
        Err(e) => {
            println!("{:?}", e);

            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    }
}
