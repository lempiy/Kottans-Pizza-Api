use chrono::DateTime;
use chrono::offset::Utc;
use std::sync::MutexGuard;
use postgres::Connection;
use postgres::Error;
use std::result;

const DEFAULT_LIMIT:i64 = 100;

#[derive(Serialize, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub price: f64,
    pub created_date: DateTime<Utc>,
}

type Result<T> = result::Result<T, Error>;

#[derive(Serialize, Debug)]
pub struct IngredientSet {
    offset: i64,
    limit: i64,
    count: i64,
    results: Vec<Ingredient>
}

impl Ingredient {
    pub fn get_some(
        db: &MutexGuard<Connection>,
        offset: Option<i64>,
        limit: Option<i64>)
        -> Result<IngredientSet> {
        let offset = if let Some(n) = offset{n}else{0i64};
        let limit = if let Some(n) = limit{
            if n < DEFAULT_LIMIT {n} else {DEFAULT_LIMIT}
        } else {
            DEFAULT_LIMIT
        };
        match db.query(
            "SELECT id, name, description, image_url, price, created_date \
             FROM ingredient ORDER BY id LIMIT $1 OFFSET $2;",
            &[&limit, &offset],
        ) {
            Ok(query) => {
                let count = match Ingredient::get_records_count(db) {
                    Ok(n) => n,
                    Err(err) => return Err(Error::from(err)),
                };
                let mut set = IngredientSet{
                    offset,
                    limit,
                    count,
                    results: Vec::new(),
                };
                for row in query.iter() {
                    let ingredient = Ingredient{
                        id: row.get("id"),
                        name: row.get("name"),
                        description: row.get("description"),
                        image_url: row.get("image_url"),
                        price: row.get("price"),
                        created_date: row.get("created_date"),
                    };
                    set.results.push(ingredient);
                }
                Ok(set)
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn get_records_count(db: &MutexGuard<Connection>)-> Result<i64> {
        match db.query(
            "SELECT get_count($1);",
            &[&"ingredient"],
        ) {
            Ok(query) => {
                for row in query.iter() {
                    let count = Ok(row.get(0));
                    return count
                }
                Ok(0)
            }
            Err(err) => Err(Error::from(err)),
        }
    }
}
