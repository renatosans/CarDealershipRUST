mod models;
mod schema;
use models::*;
use schema::cars_for_sale::dsl::cars_for_sale;

use dotenv::dotenv;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

fn main() {
    let mut catalogo = Catalogo {..Default::default()};
    let cars: Vec<CarsForSale> = catalogo.retrieve_cars();
    if cars.is_empty() {
        catalogo.insert_car();
        return;
    }
    cars.into_iter().for_each(|car: CarsForSale| {
        println!("Car: {:?}", car);
    });
}

struct Catalogo {
    pool: DbPool,
}

impl Default for Catalogo {
    fn default() -> Self {
        dotenv().ok();
        let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        // let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
        let manager: ConnectionManager<MysqlConnection> = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            std::process::exit(0); // don´t panic
        });

        Self {
            pool: pool,
        }
    }
}

impl Catalogo {
    fn insert_car(&mut self) {
        let mut conn = self.pool.get().unwrap(); // TODO: fix unwrap

        let new_car = CarsForSale {
            id: 0,
            brand: "Fiat".to_string(),
            model: "Pulse".to_string(),
            year: 2019,
            img: None,
            color: Some("Azul".to_string()),
            mileage: None,
            category: Some("SUV compacto".to_string()),
            price: 92500.00
        };

        diesel::insert_into(cars_for_sale).values(new_car).execute(&mut conn).unwrap_or_else(|e| {
            println!("Error: {}", e);
            std::process::exit(0); // don´t panic
        });
    }

    fn retrieve_cars(&mut self) -> Vec<CarsForSale> {
        let mut conn = self.pool.get().unwrap(); // TODO: fix unwrap

        let db_result: Result<Vec<CarsForSale>, diesel::result::Error> = cars_for_sale.load::<CarsForSale>(&mut conn);

        let catalogo = db_result.unwrap();
        catalogo
    }
}
