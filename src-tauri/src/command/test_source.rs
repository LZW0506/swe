use serde::Deserialize;



#[derive(Deserialize)]
pub struct AddressType {
  url: String,
  port: String,
}



#[tauri::command]
pub fn test_data_source(source_type: &str,username:&str,password:&str,address:AddressType,database:&str) ->  Result<(), String> {
    match test_mysql(username,password,address,database) {
    Ok(_) =>   Ok(()),
    Err(err) => Err(format!("{}",err))
    }

}
// 测试mysql是否可以连接成功
use sqlx::mysql::MySqlPoolOptions;
#[tokio::main]
async fn test_mysql(username:&str,password:&str,address:AddressType,database:&str) -> Result<(), sqlx::Error> {
    let url = format!("mysql://{}:{}@{}:{}/{}",username,password,address.url,address.port,database);
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await?;
    drop(pool);
    Ok(())
}