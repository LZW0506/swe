
#[path ="types/mod.rs"]
mod types;
use types::source_type;
// 查询服务器信息入口
#[tauri::command]
pub async fn query_table_info(source_type: source_type::Source,username:&str,password:&str,address:source_type::AddressType,database:&str,name:&str)->Result<Vec<source_type::TbaleListType>,String>{
    let test_re:Result<Vec<source_type::TbaleListType>, sqlx::Error>;
    match source_type {
        source_type::Source::Mysql =>test_re = query_tabel(username,password,address,database,name).await
    }
    match test_re {
        Ok(value) =>  Ok(value),
        Err(err) => Err(format!("{}",err))
    }
}
// 连接mysql并导出表信息
use sqlx::mysql::MySqlPoolOptions;
async fn query_tabel(username:&str,password:&str,address:source_type::AddressType,database:&str,name:&str)-> Result<Vec<source_type::TbaleListType>, sqlx::Error> {
    let url = format!("mysql://{}:{}@{}:{}/{}",username,password,address.url,address.port,database);
    let pool = MySqlPoolOptions::new().max_connections(5).connect(&url).await?;
    let sql = format!("show table status like '%{}%'",name);
    
    let rows = sqlx::query_as::<_,source_type::TbaleListType>(sql.as_str())  
        .fetch_all(&pool)  
        .await?;  
    
    pool.close().await;
    Ok(rows)
}