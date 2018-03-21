use uuid::Uuid;
use chrono::DateTime;
use chrono::offset::Utc;
use std::sync::MutexGuard;
use postgres::Connection;
use postgres::transaction::Transaction;
use postgres::Error;
use std::result;

use super::tag::Tag;
use super::ingredient::Ingredient;

pub struct CreatePizzaInput {
    pub uuid: Uuid,
    pub name: String,
    pub store_id: i32,
    pub user_uuid: Uuid,
    pub price: f64,
    pub size: i32,
    pub description: Option<String>,
    pub tags: Vec<i32>,
    pub img_url: String,
    pub ingredients: Vec<i32>,
    pub time_prepared: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct Pizza {
    pub uuid: Uuid,
    pub name: String,
    pub store_id: i32,
    pub user_uuid: Uuid,
    pub size: i32,
    pub deleted: bool,
    pub accepted: bool,
    pub price: f64,
    pub description: Option<String>,
    pub tags: Vec<Tag>,
    pub img_url: String,
    pub ingredients: Vec<Ingredient>,
    pub created_date: DateTime<Utc>,
    pub time_prepared: DateTime<Utc>,
}

type Result<T> = result::Result<T, Error>;

impl Pizza {
    pub fn create(db: &MutexGuard<Connection>, data: CreatePizzaInput) -> Result<()> {
        match db.transaction() {
            Ok(tx) => {
                if let Err(err) = Pizza::insert_pizza(&tx, data) {
                    tx.set_rollback();
                    if let Err(e) = tx.finish() {
                        Err(Error::from(e))
                    } else {
                        Err(Error::from(err))
                    }
                } else {
                    tx.commit()
                }
            },
            Err(err) => Err(Error::from(err))
        }
    }

    fn insert_pizza(tx: &Transaction, data: CreatePizzaInput) -> Result<()> {
        if let Err(err) = tx.execute(format!(
            "INSERT INTO pizza_{} (uuid, name, store_id, user_uuid, \
                size, price, description, img_url, now(), time_prepared) \
                VALUES ($1,$2,$3,$4,$5,$6,$7,$8, $9)",
            data.store_id).as_ref(),
        &[
            &data.uuid,
            &data.name,
            &data.store_id,
            &data.user_uuid,
            &data.size,
            &data.price,
            &data.description,
            &data.img_url,
            &data.time_prepared,
        ]) {
            return Err(Error::from(err))
        };
        match tx.prepare(format!(
            "INSERT INTO pizza_ingredient_{} (store_id, ingredient_id, pizza_uuid)\
                VALUES ($1, $2, $3);",
            data.store_id
        ).as_ref()) {
            Ok(st) => {
                for ingredient_id in data.ingredients.iter() {
                    if let Err(err) = st.execute(
                        &[
                            &data.store_id,
                            ingredient_id,
                            &data.uuid
                        ]) {
                        return Err(Error::from(err))
                    };
                };
            },
            Err(err) => return Err(Error::from(err))
        };
        match tx.prepare(format!(
            "INSERT INTO pizza_tag_{} (store_id, tag_id, pizza_uuid)\
                VALUES ($1, $2, $3);",
            data.store_id
        ).as_ref()) {
            Ok(st) => {
                for tag_id in data.tags.iter() {
                    if let Err(err) = st.execute(
                        &[
                            &data.store_id,
                            tag_id,
                            &data.uuid
                        ]) {
                        return Err(Error::from(err))
                    };
                };
            },
            Err(err) => return Err(Error::from(err))
        };
        Ok(())
    }
}
