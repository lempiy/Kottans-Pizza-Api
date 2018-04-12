use uuid::Uuid;
use chrono::DateTime;
use chrono::offset::Utc;
use std::sync::{MutexGuard,Mutex,Arc};
use postgres::Connection;
use postgres::transaction::Transaction;
use postgres::Error;
use std::result;
use utils::itob;
use std::{thread, time};
use rand::{Rng, thread_rng};
use redis::{self, Commands};
use postgres::types::ToSql;
use utils::pubsub::{Manager, PubSubEvent};
use utils::constants::{NOTIFICATION_THREAD_NAME, ACCEPT_PIZZA_EVENT_NAME};
use std::collections::HashMap;
use serde_json;

use super::tag::Tag;
use super::ingredient::Ingredient;

const DEFAULT_LIMIT: i64 = 100;

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

#[derive(Serialize, Debug)]
pub struct PizzaListOutput {
    pub uuid: Uuid,
    pub name: String,
    pub store_id: i32,
    pub user_uuid: Uuid,
    pub size: i32,
    pub accepted: bool,
    pub price: f64,
    pub description: Option<String>,
    pub img_url: String,
    pub created_date: DateTime<Utc>,
    pub time_prepared: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct PizzaSet {
    offset: i64,
    limit: i64,
    count: i64,
    results: Vec<PizzaListOutput>,
}

#[derive(Serialize)]
struct AcceptedPizzaNotification<'a> {
    store_id: i32,
    payload: AcceptedPizzaNotificationPayload<'a>,
}

#[derive(Serialize)]
struct AcceptedPizzaNotificationPayload<'a> {
    event_name: &'a str,
    data: Vec<String>,
}

type Result<T> = result::Result<T, Error>;

impl Pizza {
    pub fn create(db: &MutexGuard<Connection>, data: CreatePizzaInput) -> Result<()> {
        match db.transaction() {
            Ok(tx) => if let Err(err) = Pizza::insert_pizza(&tx, data) {
                tx.set_rollback();
                if let Err(e) = tx.finish() {
                    Err(Error::from(e))
                } else {
                    Err(Error::from(err))
                }
            } else {
                tx.commit()
            },
            Err(err) => Err(Error::from(err)),
        }
    }

    fn insert_pizza(tx: &Transaction, data: CreatePizzaInput) -> Result<()> {
        if let Err(err) = tx.execute(
            format!(
                "INSERT INTO pizza_{} (uuid, name, store_id, user_uuid, \
                 size, price, description, img_url, created_date, time_prepared) \
                 VALUES ($1,$2,$3,$4,$5,$6,$7,$8,now(),$9)",
                data.store_id
            ).as_ref(),
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
            ],
        ) {
            return Err(Error::from(err));
        };
        match tx.prepare(
            format!(
                "INSERT INTO pizza_ingredient_{} (store_id, ingredient_id, pizza_uuid)\
                 VALUES ($1, $2, $3);",
                data.store_id
            ).as_ref(),
        ) {
            Ok(st) => for ingredient_id in data.ingredients.iter() {
                if let Err(err) = st.execute(&[&data.store_id, ingredient_id, &data.uuid]) {
                    return Err(Error::from(err));
                };
            },
            Err(err) => return Err(Error::from(err)),
        };
        if data.tags.len() > 0 {
            match tx.prepare(
                format!(
                    "INSERT INTO pizza_tag_{} (store_id, tag_id, pizza_uuid)\
                     VALUES ($1, $2, $3);",
                    data.store_id
                ).as_ref(),
            ) {
                Ok(st) => for tag_id in data.tags.iter() {
                    if let Err(err) = st.execute(&[&data.store_id, tag_id, &data.uuid]) {
                        return Err(Error::from(err));
                    };
                },
                Err(err) => return Err(Error::from(err)),
            };
        };
        Ok(())
    }

    pub fn get_non_accepted(
        db: &MutexGuard<Connection>,
        offset: Option<i64>,
        limit: Option<i64>,
        store_id: i32,
    ) -> Result<PizzaSet> {
        let offset = if let Some(n) = offset { n } else { 0i64 };
        let limit = if let Some(n) = limit {
            if n < DEFAULT_LIMIT {
                n
            } else {
                DEFAULT_LIMIT
            }
        } else {
            DEFAULT_LIMIT
        };
        match db.query(
            format!(
                "SELECT uuid, user_uuid, store_id, price, \
                 name, size, description, img_url, accepted, created_date, time_prepared \
                 FROM pizza_{} WHERE deleted=0 AND accepted=0 ORDER BY time_prepared \
                 LIMIT $1 OFFSET $2;",
                store_id
            ).as_ref(),
            &[&limit, &offset],
        ) {
            Ok(query) => {
                let count = match Pizza::get_records_count(db, store_id) {
                    Ok(n) => n,
                    Err(err) => return Err(Error::from(err)),
                };
                let mut set = PizzaSet {
                    offset,
                    limit,
                    count,
                    results: Vec::new(),
                };
                for row in query.iter() {
                    let ingredient = PizzaListOutput {
                        uuid: row.get("uuid"),
                        name: row.get("name"),
                        store_id: row.get("store_id"),
                        user_uuid: row.get("user_uuid"),
                        size: row.get("size"),
                        accepted: itob(row.get("accepted")),
                        price: row.get("price"),
                        description: row.get("description"),
                        img_url: row.get("img_url"),
                        created_date: row.get("created_date"),
                        time_prepared: row.get("time_prepared"),
                    };
                    set.results.push(ingredient);
                }
                Ok(set)
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn get_records_count(db: &MutexGuard<Connection>, store_id: i32) -> Result<i64> {
        match db.query(
            "SELECT get_count($1);",
            &[&format!("pizza_{}_non_accepted", store_id)],
        ) {
            Ok(query) => {
                for row in query.iter() {
                    let count = Ok(row.get(0));
                    return count;
                }
                Ok(0)
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn emulate_accept(db: Arc<Mutex<Connection>>,
      redis: Arc<Mutex<redis::Connection>>, mng: Arc<Mutex<Manager>>) {
        thread::spawn(move||{
            let mut rng = thread_rng();

            loop {
                let sleep_time_sec:u64 = rng.gen_range(30, 60);
                let millis = time::Duration::from_millis(sleep_time_sec*1000);
                thread::sleep(millis);
                let db = db.lock().unwrap();
                let rds = redis.lock().unwrap();
                let list:Vec<String> =
                    match rds.lrange::<String, Vec<String>>("pizza-created".to_string(),
                                                            0, -1) {
                    Ok(list) => list,
                    Err(e) => {
                        println!("redis get from list error: {:?}", e);
                        panic!(e)
                    }
                };
                if list.len() > 0 {
                    let marks = list.iter()
                        .map(|_|{
                            "?".to_string()
                        })
                        .collect::<Vec<String>>()
                        .join(",");

                    let uuids: Vec<&ToSql> = list
                        .iter()
                        .map(|x| {
                            let sq: &ToSql = x;
                            sq
                        })
                        .collect();
                    let mut pizzas = Vec::new();
                    match db.query(
                        format!("UPDATE pizza \
                         SET accepted=1 \
                         WHERE uuid IN({}) \
                         RETURNING uuid, store_id;", marks).as_ref(),
                        &uuids,
                    ) {
                        Ok(q) => {
                            for row in q.iter() {
                                let value:(String, i32) = (row.get("uuid"), row.get("store_id"));
                                pizzas.push(value);
                            }
                            if let Err(e) = rds.del::<String, i32>("pizza-created".to_string()) {
                                println!("cannot clear pizza-created from redis, error: {:?}", e);
                            };
                            Pizza::emit_pizzas_accepted(pizzas, mng.clone());
                            println!("emulate_accept successful {}", sleep_time_sec);

                        },
                        Err(e) => println!("emulate_accept error {:?}", e)
                    }
                }
            }
        });
    }

    fn emit_pizzas_accepted(pizzas:Vec<(String,i32)>, mng: Arc<Mutex<Manager>>) {
        let m = mng.lock().unwrap();
        let mut hm:HashMap<i32,Vec<String>> = HashMap::new();
        for (uuid, store_id) in pizzas.into_iter() {
            if let Some(v) = hm.get_mut(&store_id) {
                v.push(uuid);
                continue
            };
            hm.insert(store_id, vec![uuid]);
        }
        for (store_id, uuids) in hm.into_iter() {
            let event = AcceptedPizzaNotification{
                store_id,
                payload: AcceptedPizzaNotificationPayload {
                    event_name: ACCEPT_PIZZA_EVENT_NAME,
                    data: uuids,
                },
            };
            let message = serde_json::to_string(&event).unwrap();
            m.send(PubSubEvent {
                channel: NOTIFICATION_THREAD_NAME.to_string(),
                message,
            });
        }
    }

    pub fn get_pizza_by_uuid(
        db: &MutexGuard<Connection>,
        uuid: Uuid,
        store_id: i32,
    ) -> Option<PizzaListOutput> {
        match db.query(
            &format!(
                "SELECT uuid, user_uuid, store_id, price, \
                 name, size, description, img_url, accepted, created_date, \
                 time_prepared from pizza_{} WHERE uuid=$1 LIMIT 1;",
                store_id
            ),
            &[&uuid],
        ) {
            Ok(query) => if query.len() > 0 {
                let row = query.iter().last().unwrap();
                Some(PizzaListOutput {
                    uuid: row.get("uuid"),
                    name: row.get("name"),
                    store_id: row.get("store_id"),
                    user_uuid: row.get("user_uuid"),
                    size: row.get("size"),
                    accepted: itob(row.get("accepted")),
                    price: row.get("price"),
                    description: row.get("description"),
                    img_url: row.get("img_url"),
                    created_date: row.get("created_date"),
                    time_prepared: row.get("time_prepared"),
                })
            } else {
                None
            },
            _ => None,
        }
    }
}
