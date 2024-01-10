mod get_user;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use aws_config::{profile::Property, BehaviorVersion};
use aws_sdk_dynamodb as dynamodb;
use dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct AppState {
    db_client: Arc<Mutex<dynamodb::Client>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let db_client = dynamodb::Client::new(&config);

    let state = AppState {
        db_client: Arc::new(Mutex::new(db_client)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(new_user)
            .service(user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Deserialize)]
struct NewUserRequestData {
    email: String,
    name: String,
}

#[post("/new-user")]
async fn new_user(
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

#[derive(Deserialize)]
struct GetUserRequestData {
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetUserResponseData {
    item: Option<UserDetails>,
}

#[derive(Deserialize, Debug)]
struct UserDetails {
    email: AttributeValue,
    name: AttributeValue,
}

#[get("/user")]
async fn user(
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
                let user: GetUserResponseData = serde_json::from_str(item).unwrap();
                return HttpResponse::Ok(web::Json(user));
            }
        }
        Err(e) => {
            println!("{:?}", e);

            return HttpResponse::InternalServerError();
        }
    }

    HttpResponse::Ok()
}
