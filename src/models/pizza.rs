use uuid::Uuid;
use chrono::DateTime;
use chrono::offset::Utc;
use std::sync::MutexGuard;
use postgres::Connection;
use postgres::Error;
use std::result;

use super::tag::Tag;
use super::ingredient::Ingredient;

pub struct CreatePizzaInput {
    pub name: String,
    pub store_id: i32,
    pub user_uuid: Uuid,
    pub size: i32,
    pub description: String,
    pub tags: Vec<i32>,
    pub img_url: String,
    pub ingredients: Vec<i32>,
}

pub struct Pizza {
    pub id: i32,
    pub name: String,
    pub store_id: i32,
    pub user_uuid: Uuid,
    pub size: i32,
    pub deleted: bool,
    pub accepted: bool,
    pub price: f32,
    pub description: String,
    pub tags: Vec<Tag>,
    pub img_url: String,
    pub ingredients: Vec<Ingredient>,
    pub created_date: DateTime<Utc>,
}

impl Pizza {
    pub fn create(db: &MutexGuard<Connection>) {

    }
}