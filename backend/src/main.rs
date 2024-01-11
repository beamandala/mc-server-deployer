mod user;

use actix_web::{web, App, HttpServer};
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb as dynamodb;
use std::sync::{Arc, Mutex};
use user::{create_user, get_user};

#[derive(Clone)]
pub struct AppState {
    db_client: Arc<Mutex<dynamodb::Client>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let db_client = dynamodb::Client::new(&aws_config);

    let state = AppState {
        db_client: Arc::new(Mutex::new(db_client)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(app_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(web::scope("").service(create_user).service(get_user));
}
