mod schema;
mod models;
mod handlers;

use dotenv::dotenv;
use handlers::salesperson;
use handlers::customer;
use handlers::cars_for_sale;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_cors::Cors;
use actix_web::{web, middleware, App, HttpServer};


pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(
                web::scope("/api")
                    .service(cars_for_sale::index)
                    .service(cars_for_sale::select)
                    .service(cars_for_sale::create)
                    .service(cars_for_sale::update)
                    .service(cars_for_sale::delete)
                    .service(customer::index)
                    .service(customer::select)
                    .service(customer::create)
                    .service(customer::update)
                    .service(customer::delete)
                    .service(salesperson::index)
                    // .service(salesperson::select)
                    // .service(salesperson::create)
                    .service(salesperson::update)
                    .service(salesperson::delete)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await  
}
