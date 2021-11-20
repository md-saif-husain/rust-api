#[macro_use]
extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
mod model;
mod schema;
use self::model::*;
use self::schema::cats::dsl::*; // provides alias like "cats"
use actix_files::Files;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Result};
use dotenv::dotenv;

async fn get_cats(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Error getting connection from pool");
    let cats_data = web::block(move || cats.limit(100).load::<Cat>(&connection))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    return Ok(HttpResponse::Ok().json(cats_data));
}
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_connection_url = env::var("DATABASE_URL").expect("DATABASE URL in env must be set");
    let manager = ConnectionManager::<PgConnection>::new(&db_connection_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Pool must be created");

    println!("Listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::scope("/api").route("/cats", web::get().to(get_cats)))
            .service(Files::new("/", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
