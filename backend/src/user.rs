use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct GetUserRequestData {
    email: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct GetUserResponseData {
//     item: Option<UserDetails>,
// }
//
// #[derive(Deserialize, Debug)]
// struct UserDetails {
//     email: AttributeValue,
//     name: AttributeValue,
// }

#[get("/user")]
pub async fn get_user(
    data: web::Data<AppState>,
    properties: web::Json<NewUserRequestData>,
) -> impl Responder {
    let db = data.db_client.lock().unwrap();

    let get_user_res = db
        .get_item()
        .table_name("mine-auth")
        .key("email", AttributeValue::S(properties.email.to_owned()))
        .send()
        .await;

    match get_user_res {
        Ok(res) => {
            println!("{:?}", res);

            if let item = Some(res.item) {
                // let user: GetUserResponseData = serde_json::from_str(item).unwrap();
                // return HttpResponse::Ok(web::Json(user));
            }
        }
        Err(e) => {
            println!("{:?}", e);

            return HttpResponse::InternalServerError();
        }
    }

    HttpResponse::Ok()
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
