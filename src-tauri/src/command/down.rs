
#[path ="types/mod.rs"]
mod types;
use rfd::FileDialog;
use serde::Serialize;
use types::source_type;
use docx_rs::*;

#[derive(Serialize)]
pub enum DownCode {
    Success,Cancel
}


// 查询服务器信息入口
#[tauri::command]
pub async fn down_word(source_type: source_type::Source,username:&str,password:&str,address:source_type::AddressType,database:&str,name:&str,names:Vec<String>)->Result<DownCode, String>{
   

    // 先打开选择窗口
    if let Some(folder_path) = FileDialog::new().pick_folder() {
        let file_path = String::from(format!("{}/table.docx",folder_path.display()));
        let path = std::path::Path::new(file_path.as_str());
        let file = std::fs::File::create(&path).unwrap();
        let test_re:Result<Vec<source_type::TbaleInfoShowType>, sqlx::Error>;
        // 查询表结构
        match source_type {
            source_type::Source::Mysql =>test_re = query_tabel_info(username,password,address,database,name,names).await
        }
        match test_re {
            Ok(table_list) => {
                export_word(table_list);
                Ok(DownCode::Success)
            },
            Err(err) => Err(format!("{}",err))
        }
    } else {
        Ok(DownCode::Cancel)
    }
 
    
}
// // 连接mysql并导出表信息
use sqlx::{mysql::MySqlPoolOptions, query_as};
async fn query_tabel_info(username:&str,password:&str,address:source_type::AddressType,database:&str,name:&str,names:Vec<String>)-> Result<Vec<source_type::TbaleInfoShowType>, sqlx::Error> {
    let url = format!("mysql://{}:{}@{}:{}/{}",username,password,address.url,address.port,database);
    let pool = MySqlPoolOptions::new().max_connections(5).connect(&url).await?;
     // 查询数据库中的所有表
    let tables_query  = format!("show table status like '%{}%'",name);

    let tables_result = query_as::<_,source_type::TbaleListType>(tables_query.as_str()).fetch_all(&pool).await?;
    let mut table_info_list:Vec<source_type::TbaleInfoShowType> = vec![];
    // 遍历所有表，查询表结构信息并导出到文件
    for table_row in tables_result {
        let table_name: String = table_row.Name; 
        // 如果没选中就不查询这个表结构
        if names.len() >0 && !names.contains(&table_name){
            continue;
        }
        // 查询表结构信息
        let columns_query = format!(
            "
            SHOW FULL COLUMNS FROM {};
            ",
            table_name
        );

        let columns_result = query_as::<_,source_type::TbaleInfoType>(&columns_query).fetch_all(&pool).await?;
        // 获取到是哪个表， 他的备注和表名
        let info: source_type::TbaleInfoShowType = source_type::TbaleInfoShowType{
            table_name:table_name,
            comment: table_row.Comment,
            info: columns_result,
        };
        table_info_list.push(info);
    }
    
    pool.close().await;
    Ok(table_info_list)
}

fn export_word(table_list:Vec<source_type::TbaleInfoShowType>){
    let docx = Docx::new();
    for table_item in table_list  {
        
    }
}