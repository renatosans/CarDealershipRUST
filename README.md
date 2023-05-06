# CarDealershipRUST
Sistema para concessionária de carros usando RUST, Diesel and Actix Web

## Dependencies
- Diesel ORM depends on the following client library:   libmysqlclient

## Steps to run the project
- create the database using the script located in the database folder (create.sql)
- seed the database with cars_for_sale.json
![screenshot](docs/seed.png)
- cargo build  // diesel ORM needs the path of the client lib for the database
- cargo run
- use INSOMNIA to test the endpoints (
    http://127.0.0.1:8080/api/cars
    http://127.0.0.1:8080/api/customers
    http://127.0.0.1:8080/api/salespeople
)
