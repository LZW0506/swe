
#[path ="types/mod.rs"]
mod types;


use types::source_type;
// 查询服务器信息入口
#[tauri::command]
pub async fn query_table_info(source_type: source_type::Source,username:&str,password:&str,address:source_type::AddressType,database:&str)->Result<Vec<source_type::TbaleInfoType>,String>{
    let test_re:Result<Vec<source_type::TbaleInfoType>, sqlx::Error>;
    match source_type {
        source_type::Source::Mysql =>test_re = query_tabel(username,password,address,database).await
    }
    match test_re {
        Ok(value) => {
            print!("{:?}",value);
            Ok(value)
        },
        Err(err) => Err(format!("{}",err))
    }
}

use sqlx::{mysql::MySqlPoolOptions, Column, Row};
async fn query_tabel(username:&str,password:&str,address:source_type::AddressType,database:&str)-> Result<Vec<source_type::TbaleInfoType>, sqlx::Error> {
    let url = format!("mysql://{}:{}@{}:{}/{}",username,password,address.url,address.port,database);
    let pool = MySqlPoolOptions::new().max_connections(5).connect(&url).await?;

    let rows = sqlx::query_as::<_,source_type::TbaleInfoType>("show table status")  
        .fetch_all(&pool)  
        .await?;  
    
    pool.close().await;
    Ok(rows)
}