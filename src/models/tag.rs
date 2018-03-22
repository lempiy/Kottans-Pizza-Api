use std::sync::MutexGuard;
use postgres::Connection;
use postgres::Error;
use std::result;
use validator::ValidationError;
use std::collections::HashMap;
use std::borrow::Cow;
use postgres::types::ToSql;

const DEFAULT_LIMIT:i64 = 100;

#[derive(Serialize, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

type Result<T> = result::Result<T, Error>;

#[derive(Serialize, Debug)]
pub struct TagSet {
    offset: i64,
    limit: i64,
    count: i64,
    results: Vec<Tag>
}

impl Tag {
    pub fn get_some(
        db: &MutexGuard<Connection>,
        offset: Option<i64>,
        limit: Option<i64>)
        -> Result<TagSet> {
        let offset = if let Some(n) = offset{n}else{0i64};
        let limit = if let Some(n) = limit{
            if n < DEFAULT_LIMIT {n} else {DEFAULT_LIMIT}
        } else {
            DEFAULT_LIMIT
        };
        match db.query(
            "SELECT id, name, description \
             FROM tag ORDER BY id LIMIT $1 OFFSET $2;",
            &[&limit, &offset],
        ) {
            Ok(query) => {
                let count = match Tag::get_records_count(db) {
                    Ok(n) => n,
                    Err(err) => return Err(Error::from(err)),
                };
                let mut set = TagSet{
                    offset,
                    limit,
                    count,
                    results: Vec::new(),
                };
                for row in query.iter() {
                    let tag = Tag{
                        id: row.get("id"),
                        name: row.get("name"),
                        description: row.get("description")
                    };
                    set.results.push(tag);
                }
                Ok(set)
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    pub fn get_records_count(db: &MutexGuard<Connection>)-> Result<i64> {
        match db.query(
            "SELECT get_count($1);",
            &[&"tag"],
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

    pub fn validate_tags_exist(
        db: &MutexGuard<Connection>,
        tag_ids: &Vec<i32>,
    ) -> result::Result<(), ValidationError> {
        if tag_ids.len() == 0 {
            return Err(ValidationError {
                code: Cow::from("wrong_tags"),
                message: Some(Cow::from("Tags cannot be empty")),
                params: HashMap::new(),
            })
        };
        let mut query = tag_ids
            .iter()
            .enumerate()
            .fold("SELECT id FROM tag WHERE id IN (".to_string(),
                  |acc, x| {
                      let (i, _) = x;
                      acc + &format!("${},", i+1)
                  });
        query.pop();
        query += ") ORDER BY id;";
        let ids:Vec<&ToSql> = tag_ids
            .iter()
            .map(|x|{
                let sq:&ToSql = x;
                sq
            })
            .collect();
        match db.query(&query, &ids) {
            Ok(query) => {
                if query.len() == tag_ids.len() {
                    Ok(())
                } else {
                    let result_ids:Vec<i32> = query
                        .iter()
                        .map(|row|{
                            row.get("id")
                        }).collect();
                    let missing:Vec<i32> = tag_ids
                        .iter()
                        .filter_map(|id| {
                            match result_ids.iter().find(|x|{
                                *id == **x
                            }) {
                                Some(_) => None,
                                None => Some(*id)
                            }
                        })
                        .collect();
                    Err(ValidationError {
                        code: Cow::from("wrong_tags"),
                        message: Some(Cow::from(
                            format!("Tags with ids {:?} are not exist", missing)
                        )),
                        params: HashMap::new(),
                    })
                }
            }
            Err(_) => Err(ValidationError {
                code: Cow::from("wrong_tags"),
                message: Some(Cow::from("Cannot tag ids")),
                params: HashMap::new(),
            }),
        }
    }
}
