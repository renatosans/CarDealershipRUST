mod models;
mod schema;
use diesel::expression::is_aggregate::No;
use models::*;
use schema::cars_for_sale::dsl::cars_for_sale;


fn main() {
    let catalogo = Catalogo::retrieve_cars();
    println!("Cars for sale: ");
}

struct Catalogo {
    pool: DbPool,
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

    fn retrieve_cars(&mut self) {
        let mut conn = self.pool.get().unwrap(); // TODO: fix unwrap

        let db_result: Result<Vec<CarsForSale>, diesel::result::Error> = cars_for_sale.load::<CarsForSale>(&mut conn);

        let catalogo = db_result.unwrap();
        catalogo.into_iter().for_each(|car: CarsForSale| println!("Car: "));
    }
}
