extern crate actix_rt;
extern crate actix_web;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate juniper;
extern crate r2d2;
extern crate todos;

use std::{env, io};

use actix_cors::Cors;
use actix_web::web;
use actix_web::{middleware, App, HttpServer, web::Data, HttpResponse};

use todos::db::get_pool;
use todos::endpoints::{self, graphql_endpoints};

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("ok")
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();

    let pool = get_pool();

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
        .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(graphql_endpoints)
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}

fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
