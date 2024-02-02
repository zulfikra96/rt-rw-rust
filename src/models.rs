use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::Serialize;
use crate::schema::users;
use bigdecimal::BigDecimal;

#[derive(Queryable, Selectable, Debug )]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub token: Option<String>,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub ip: Option<String>,
    pub mac_addr: Option<String>,
    pub devices: Option<String>,
    pub internet_service_id: Option<i32>,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String, 
    pub phone: String,
    pub password : String,
    pub role: String,
    pub password_plain: String,
    pub internet_service_id: i32,
    pub mikrotik_id: String
}

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::internet_services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InternetServices {
    pub id: i32,
    pub name: Option<String>,
    pub price: Option<BigDecimal>
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetUserDetail {
    pub id: i32,
    pub mikrotik_id: Option<String>
}
