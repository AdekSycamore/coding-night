use super::context::GraphQLContext;
use super::db::PostgresPool;
use super::graphql::create_schema;
use super::graphql::Schema;

use actix_web::{web::{self, Data}, Error, HttpResponse, HttpRequest};
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

pub fn graphql_endpoints(config: &mut web::ServiceConfig) {
    let schema = Arc::new(create_schema());
    config
        .app_data(Data::new(schema))
        .route("/graphql", web::post().to(graphql))
        .route("/graphiql", web::get().to(graphql_playground));
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql", None))
}
async fn graphql(
    pool: web::Data<PostgresPool>,
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    
    let usertoken = match req.headers().get("auth") {
        Some(token) => String::from(token.to_str().expect("")),
        None => String::from(""),
    };


    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
        token: usertoken
    };

    let res = data.execute(&schema, &ctx).await;
    let result = Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)).unwrap();

    let x = match result {
        Ok(n) => n,
        Err(e) => e.to_string(),
    };

    Ok(HttpResponse::Ok().content_type("application/json").body(x))
}
