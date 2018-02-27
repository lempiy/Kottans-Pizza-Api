use uuid::Uuid;
use chrono::{DateTime};
use chrono::offset::Utc;
use std::error::Error;
use std::sync::MutexGuard;
use postgres::Connection;

pub struct User {
    uuid: Uuid,
    username: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
    last_login: Option<DateTime<Utc>>,
}

type Result<T> = result::Result<T, Error>;

impl User {
    pub fn new(
        db: &MutexGuard<Connection>,
        username: &str,
        email: &str,
        password: &str)->Result<User> {
        let user = User{
            uuid: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            created_at: Utc::now(),
            last_login: None,
        };

        if let Err(e) = db.execute(
            "INSERT INTO person (\
                uuid, \
                username, \
                email, \
                password, \
                created_at,\
                last_login\
            ) VALUES (\
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            )",
            &[&user.uuid, &user.username,
                &user.email, &user.password,
                &user.created_at, &user.last_login]
        ) {
            Err(e)
        } else {
            Ok(user)
        }
    }

    pub fn find(
        db: &MutexGuard<Connection>,
        username: &str,
        password: &str,
    )->Result<Option<User>> {
        let mut uuid:Option<Uuid> = None;
        let mut email = String::new();
        let mut created_at: Option<DateTime<Utc>> = None;
        let mut last_login: Option<DateTime<Utc>> = None;
        match &db.query(
            "SELECT uuid, email, created_at, last_login \
                FROM person WHERE username = $1 AND password=$2",
            &[&username, &password]) {
            Ok(query) => {
                for row in query {
                    uuid = Some(row.get("uuid"));
                    email = row.get("email");
                    created_at = Some(row.get("created_at"));
                    last_login = Some(row.get("last_login"));
                    break
                }
                if let Some(uuid) = uuid {
                    Ok(Some(User {
                        uuid,
                        username,
                        email,
                        password,
                        created_at,
                        last_login,
                    }))
                } else {
                    Ok(None)
                }
            },
            Err(err) => {
                Err(::std::convert::From::from(err))
            }
        }
    }

    pub fn get(
        db: &MutexGuard<Connection>,
        uuid: Uuid,
    )->Result<Option<User>> {
        let mut username: Option<String> = None;
        let mut password = String::new();
        let mut email = String::new();
        let mut created_at: Option<DateTime<Utc>> = None;
        let mut last_login: Option<DateTime<Utc>> = None;
        match &db.query(
            "SELECT username, email, password, created_at, last_login \
                FROM person WHERE uuid = $1",
            &[&uuid]) {
            Ok(query) => {
                for row in query {
                    username = Some(row.get("username"));
                    email = row.get("email");
                    password = row.get("password");
                    created_at = Some(row.get("created_at"));
                    last_login = Some(row.get("last_login"));
                    break
                }
                if let Some(uuid) = uuid {
                    Ok(Some(User {
                        uuid,
                        username,
                        email,
                        password,
                        created_at,
                        last_login,
                    }))
                } else {
                    Ok(None)
                }
            },
            Err(err) => {
                Err(::std::convert::From::from(err))
            }
        }
    }
}
