use mysql::{Pool, PooledConn, prelude::*};
use once_cell::sync::OnceCell;
use tracing::{ info};

static DB_POOL: OnceCell<Pool> = OnceCell::new();

// 初始化数据库链接池
pub fn init_mysql_pool(db_url: &str) {
    info!("初始化数据库线程池--------开始-------");
    DB_POOL.set(mysql::Pool::new(&db_url).expect(&format!("Error connecting to {}", &db_url))).
        unwrap_or_else(|_| { info!("try insert pool cell failure!") });
    info!("初始化数据库线程池--------结束-------");
}

// 从链接链接池里面获取链接
pub fn get_connect() -> PooledConn {
    info!("从链接池获取数据库链接----------开始----------");
    let conn = DB_POOL.get().expect("Error get pool from OneCell<Pool>").get_conn().expect("Error get_connect from db pool");
    info!("从链接池获取数据库链接----------结束----------");
    conn
}



fn main() {
    let mut conn = get_connect();
    let mut index = 0;
    let size = 2000;
    let mut addr = "";
    let mut coinType = "";
    let mut blockHeight = 0;
    let query_result = conn.exec("SELECT addr,coin_type,block_height from address limit :index,:size", params!("index"=>id,"size"=>""))
        .map(|row| {
            row.map(|(addr, coin_type, block_height)| addr, coinType, blockHeight)
        });
    match query_result {
        Ok(result) => {
            print!("res:{}",result);
        },
        Err(_)=>{
            
        }
    }
           
}
