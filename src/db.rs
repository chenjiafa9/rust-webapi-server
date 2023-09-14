use sqlx::postgres::Postgres;
use sqlx::{postgres::PgPoolOptions, Pool};

const SQL_FILE: &str = "sql/create_table.sql";

const PG_HOST: &str = "localhost";
const PG_USER: &str = "postgres";
const PG_PWD: &str = "CJFCJF";
const PG_DB: &str = "postgres";
const PG_MAX_NUM: u32 = 5;

pub type Db = Pool<Postgres>;

//初始化PG连接池
async fn new_db_pool(
    host: &str,
    user: &str,
    pwd: &str,
    db: &str,
    max_num: u32,
) -> Result<Db, sqlx::error::Error> {
    let conn_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_num)
        .connect(&conn_string)
        .await
}

//读取sql并执行
async fn sql_exec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    let sql_str = std::fs::read_to_string(file)?;
    let sqls: Vec<&str> = sql_str.split(";").collect();
    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }
    Ok(())
}

//暴露接口
pub async fn init_db() -> Result<Db, sqlx::Error> {
    {
        let db = new_db_pool(PG_HOST, PG_USER, PG_PWD, PG_DB, 1).await?;
        sql_exec(&db, SQL_FILE).await?;
    }

    new_db_pool(PG_HOST, PG_USER, PG_PWD, PG_DB, PG_MAX_NUM).await
}
