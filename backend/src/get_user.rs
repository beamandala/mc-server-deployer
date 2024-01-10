use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct NewUserRequestData {
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
pub async fn user(
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
