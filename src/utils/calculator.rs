use rust_decimal::Decimal;
use chrono::DateTime;
use chrono::offset::Utc;
use num_traits::{FromPrimitive, ToPrimitive};
use postgres::{Connection, Error};
use chrono::TimeZone;
use postgres::types::ToSql;
use std::sync::MutexGuard;
use std::result;

const PIZZA_BASIC_TIME:i64 = 300;
const PIZZA_SIZE_TIME_FACTOR:i64 = 5;
const PIZZA_TIME_PER_INGREDIENT:i64 = 5;
const PIZZA_CRUST_PRICE_FACTOR:i64 = 5;
type Result<T> = result::Result<T, Error>;

pub fn calculate_pizza_price(
    db: &MutexGuard<Connection>,
    ingredient_ids: &Vec<i32>,
    size: &i64,
)-> Result<f64> {
    let base_price = Decimal::from(size / PIZZA_CRUST_PRICE_FACTOR);
    let mut query = ingredient_ids
        .iter()
        .enumerate()
        .fold("SELECT price FROM ingredient WHERE id IN (".to_string(),
              |acc, x| {
                  let (i, _) = x;
                  acc + &format!("${},", i+1)
              });
    query.pop();
    query += ") ORDER BY id;";
    let ids:Vec<&ToSql> = ingredient_ids
        .iter()
        .map(|x|{
            let sq:&ToSql = x;
            sq
        })
        .collect();
    match db.query(&query, &ids) {
        Ok(query) => {
            Ok(query
                .iter()
                .filter_map(|row|{
                    Decimal::from_f64(row.get("price"))
                })
                .fold(base_price, |acc, x| {
                    acc + x
                })
                .to_f64()
                .unwrap())
        }
        Err(err) => Err(Error::from(err)),
    }
}

pub fn calculate_preparation_time(size: &i64, ingredient_count: usize) -> DateTime<Utc> {
    let base_add_size_time = size / PIZZA_SIZE_TIME_FACTOR * 10;
    let base_add_ingredients_time = ingredient_count as i64 * PIZZA_TIME_PER_INGREDIENT;
    let result = PIZZA_BASIC_TIME+base_add_size_time+base_add_ingredients_time;
    let ready_timestamp = Utc::now().naive_utc().timestamp() + result;
    Utc.timestamp(ready_timestamp, 0)
}
