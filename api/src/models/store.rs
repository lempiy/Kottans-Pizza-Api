use std::sync::MutexGuard;
use postgres::Connection;
use validator::ValidationError;
use postgres::Error;
use std::result;
use std::collections::HashMap;
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct Store {
    pub id: i32,
    pub name: String,
}

type Result<T> = result::Result<T, Error>;

impl Store {
    pub fn get_all(db: &MutexGuard<Connection>) -> Result<Vec<Store>> {
        match db.query(
            "SELECT id, name \
             FROM store ORDER BY id;",
            &[],
        ) {
            Ok(query) => {
                let mut vector = Vec::new();
                for row in query.iter() {
                    let store = Store {
                        id: row.get("id"),
                        name: row.get("name"),
                    };
                    vector.push(store);
                }
                Ok(vector)
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn find(db: &MutexGuard<Connection>, id: i32, password: &str) -> Result<Option<Store>> {
        let mut name: Option<String> = None;
        match db.query(
            "SELECT id, name, password \
             FROM store WHERE id = $1 AND password=$2",
            &[&id, &password],
        ) {
            Ok(query) => {
                for row in query.iter() {
                    name = Some(row.get("name"));
                    break;
                }
                if let Some(name) = name {
                    Ok(Some(Store { id, name }))
                } else {
                    Ok(None)
                }
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn validate_correct_store(
        db: &MutexGuard<Connection>,
        id: i32,
        password: &str,
    ) -> result::Result<(), ValidationError> {
        match Store::find(db, id, password) {
            Ok(result) => if let Some(_) = result {
                Ok(())
            } else {
                Err(ValidationError {
                    code: Cow::from("wrong_store"),
                    message: Some(Cow::from("Wrong store credentials")),
                    params: HashMap::new(),
                })
            },
            Err(_) => Err(ValidationError {
                code: Cow::from("wrong_store"),
                message: Some(Cow::from("Cannot check store credentials")),
                params: HashMap::new(),
            }),
        }
    }
}
