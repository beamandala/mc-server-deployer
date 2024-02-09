#![allow(unused)]

mod server;
mod user;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb as dynamodb;
use aws_sdk_ec2 as ec2;
use aws_sdk_ssm as ssm;
use dotenv::dotenv;
use server::{create_server, list_servers};
use std::{
    env,
    sync::{Arc, Mutex},
};
use user::{create_user, get_user};

// TODO: Parse env variables into a struct

#[derive(Clone)]
pub struct AppState {
    db_client: Arc<Mutex<dynamodb::Client>>,
    ec2_client: Arc<Mutex<ec2::Client>>,
    ssm_client: Arc<Mutex<ssm::Client>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let db_client = dynamodb::Client::new(&aws_config);
    let ec2_client = ec2::Client::new(&aws_config);
    let ssm_client = ssm::Client::new(&aws_config);

    let state = AppState {
        db_client: Arc::new(Mutex::new(db_client)),
        ec2_client: Arc::new(Mutex::new(ec2_client)),
        ssm_client: Arc::new(Mutex::new(ssm_client)),
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
    config.service(
        web::scope("")
            .service(ping)
            .service(create_user)
            .service(get_user)
            .service(list_servers)
            .service(create_server)
    );
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok()
}
