# CarDealershipRUST
Sistema para concessionária de carros usando RUST, Diesel and Actix Web

![screenshot](assets/banner.png)

## Dependencies
- Diesel ORM depends on the following client library:   libmysqlclient
- Run the comand to get it:
> apt install -y default-libmysqlclient-dev

## ~~Steps to run the project~~
- ~~docker compose up~~
- ~~follow the link  http://localhost:3500~~

## Steps to run the project without Docker
- Run the SQL script at  /database/create.sql
- Set DATABASE_URL in the .env file
- cargo build
- cargo run
- use INSOMNIA to test the endpoints (
    http://127.0.0.1:8080/api/cars
    http://127.0.0.1:8080/api/customers
    http://127.0.0.1:8080/api/salespeople
)
