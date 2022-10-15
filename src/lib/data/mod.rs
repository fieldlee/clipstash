pub mod models;
pub mod query;

use derive_more::{Display, From};
use serde::{Deserialize,Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug,thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type AppDatabase = Database<sqlx::MySql>;
pub type DatabasePool = sqlx::mysql::MySqlPool;
pub type Transaction<'t> = sqlx::Transaction<'t,sqlx::MySql>;
pub type AppDatabaseRow = sqlx::mysql::MySqlRow;
pub type AppQueryResult = sqlx::mysql::MySqlQueryResult;

pub struct Database<D:sqlx::Database> (sqlx::Pool<D>);

impl Database<sqlx::MySql> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::mysql::MySqlPoolOptions::new().connect(connection_str).await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!("If the database has not yet been created, run: \n    $ sqlx database setup\n");
                panic!("database connection error");
            }
        }

    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}


#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);


impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    pub fn nil() ->DbId {
        Self(Uuid::nil())
    }
}

impl From<DbId> for String {
    fn from(id: DbId) -> Self {
        format!("{}", id.0)
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}
 

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}
