
#[path ="types/mod.rs"]
mod types;
use types::source_type;
use docx_rs::*;


// 查询服务器信息入口
#[tauri::command]
pub async fn down_word(source_type: source_type::Source,username:&str,password:&str,address:source_type::AddressType,database:&str,names:Vec<String>,path:&str)->Result<(), String>{
    let file_path = String::from(format!("{}/table.docx",path));
    let path = std::path::Path::new(file_path.as_str());
    let file = std::fs::File::create(&path).unwrap();
    let table = Table::new(vec![TableRow::new(vec![
        TableCell::new().add_paragraph(Paragraph::new())
    ])]);
    let _ = Docx::new().add_table(table).build().pack(file);
    Ok(())
}
// // 连接mysql并导出表信息
// use sqlx::mysql::MySqlPoolOptions;
// async fn query_tabel(username:&str,password:&str,address:source_type::AddressType,database:&str,name:&str)-> Result<Vec<source_type::TbaleInfoType>, sqlx::Error> {
//     let url = format!("mysql://{}:{}@{}:{}/{}",username,password,address.url,address.port,database);
//     let pool = MySqlPoolOptions::new().max_connections(5).connect(&url).await?;
//     let sql = format!("show table status like '%{}%'",name);
    
//     let rows = sqlx::query_as::<_,source_type::TbaleInfoType>(sql.as_str())  
//         .fetch_all(&pool)  
//         .await?;  
    
//     pool.close().await;
//     Ok(rows)
// }