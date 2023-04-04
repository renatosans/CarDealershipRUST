// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use chrono::NaiveDate;
use diesel::prelude::*;


#[derive(Queryable, Debug)]
pub struct CarsForSale {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub year: i32,
    pub img: Option<String>,
    pub color: Option<String>,
    pub mileage: Option<i32>,
    pub category: Option<String>,
    pub price: f64,
}

#[derive(Queryable, Debug)]
pub struct CarsForService {
    pub id: i32,
    pub customer_id: i32,
    pub car_details: String,
    pub mechanic: String,
}

#[derive(Queryable, Debug)]
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Invoice {
    pub id: i32,
    pub customer_id: i32,
    pub salesperson_id: i32,
    pub car_id: i32,
    pub amount: i32,
}

#[derive(Queryable, Debug)]
pub struct Salesperson {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub commission: f64,
}
