#[path ="./types/mod.rs"]
mod types;
use types::source_type;
// 调用测试连接入口
#[tauri::command]
pub async fn test_data_source(source_type: source_type::Source,username:&str,password:&str,address:source_type::AddressType,database:&str) ->  Result<(), String> {
    let test_re:Result<(), sqlx::Error>;
    match source_type {
        source_type::Source::Mysql =>test_re = test_mysql(username,password,address,database).await
    }
    match test_re {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}",err))
    }
}
// 测试mysql是否可以连接成功
use sqlx::mysql::MySqlPoolOptions;
async fn test_mysql(username:&str,password:&str,address:source_type::AddressType,database:&str) -> Result<(), sqlx::Error> {
    let url = format!("mysql://{}:{}@{}:{}/{}",username,password,address.url,address.port,database);
    let pool = MySqlPoolOptions::new().max_connections(5).connect(&url).await?;
    pool.close().await;
    Ok(())
}