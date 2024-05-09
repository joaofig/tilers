use std::result::Result;
use sqlx::{SqlitePool, Sqlite, Pool, Error};
use crate::models::{LevelRange, TileQuadKey};
use crate::qk_tools::qk_str_to_i64;


pub async fn connect() -> Result<Pool<Sqlite>, Error> {
    let db_url = String::from("sqlite:///Users/joafigu/data/ved/tile.db");
    SqlitePool::connect(&db_url).await
}


pub async fn get_tile_quad_keys(qk: &str) -> Vec<TileQuadKey> {
    let db = connect().await;

    match db {
        Ok(pool) => {
            let sql = format!("select qk, intensity from l{} where tile=$1", qk.len() + 8);
            let quad_keys: Result<Vec<TileQuadKey>, sqlx::Error> = sqlx::query_as(&sql)
                .bind(qk_str_to_i64(qk))
                .fetch_all(&pool)
                .await;
            quad_keys.unwrap_or_else(|_err| vec!())
        }
        Err(_err) => {
            vec!()
        }
    }
}


pub async fn get_level_range(level: i32) -> LevelRange {
    let db = connect().await;
    
    match db {
        Ok(pool) => {
            let sql= "select level_min, level_max from level_range where level_num=$1";
            let result = sqlx::query_as(sql)
                .bind(level)
                .fetch_one(&pool)
                .await;
            result.unwrap_or_else(|_| LevelRange::new(0.0, 0.0))
        }
        Err(_) => LevelRange::new(0.0, 0.0),
    }
}
