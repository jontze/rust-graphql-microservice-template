use actix_web::{web, HttpResponse};
use juniper::http::{graphiql, GraphQLRequest};
use sqlx::PgPool;

use super::{create_schema, Context, Schema};

async fn graphql(
    pool: web::Data<PgPool>,
    st: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let context = Context {
        pool: pool.as_ref().to_owned(),
    };
    let test = serde_json::to_string(&data.execute(&st, &context).await)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(test))
}

async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql::graphiql_source("/graphql", None))
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .data(create_schema())
        .route("/graphql/", web::post().to(graphql))
        .route("/graphiql/", web::get().to(graphiql));
}
