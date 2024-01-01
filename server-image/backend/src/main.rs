mod server_handler;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde::Deserialize;
use server_handler::ServerHandler;
use std::{
    fs::{read_to_string, File},
    io::Write,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct AppState {
    handler: Arc<Mutex<Option<ServerHandler>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState {
        handler: Arc::new(Mutex::new(None)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(start_server)
            .service(stop_server)
            .service(properties)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[derive(Deserialize)]
struct Properties {
    data: std::collections::BTreeMap<String, String>,
}

#[post("/properties")]
async fn properties(properties: web::Json<Properties>) -> impl Responder {
    let mut handlebars = Handlebars::new();

    let template = read_to_string("server.properties.template").unwrap();
    let _ = handlebars.register_template_string("t", template);

    let rendered_template = handlebars.render("t", &properties.data).unwrap();

    let mut server_properties_file = File::create("server.properties").unwrap();
    let _ = server_properties_file.write_all(rendered_template.as_bytes());

    HttpResponse::Ok()
}

#[post("/start")]
async fn start_server(data: web::Data<AppState>) -> impl Responder {
    let mut state = data.handler.lock().unwrap();
    *state = Some(ServerHandler::new());

    if let Some(ref mut handler) = *state {
        match handler.wait_for("[Server thread/INFO]: Done") {
            Ok(bool) => HttpResponse::Ok().body(format!("Success: {}", bool)),
            Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        }
    } else {
        HttpResponse::InternalServerError().body("Failed to initialize Server handler")
    }
}

#[post("/stop")]
async fn stop_server(data: web::Data<AppState>) -> impl Responder {
    let mut state = data.handler.lock().unwrap();

    if let Some(ref mut handler) = *state {
        let res = handler.stop_server();
        println!("stop res {:?}", res);

        return HttpResponse::Ok().body(format!("{:?}", res));
    } else {
        return HttpResponse::InternalServerError().body("Server handler not initialized");
    }
}
