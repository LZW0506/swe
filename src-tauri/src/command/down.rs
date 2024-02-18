
#[path ="types/mod.rs"]
mod types;

use docx_rs::*;
use rfd::FileDialog;
use serde::Serialize;
use types::source_type;

#[derive(Serialize)]
pub enum DownCode {
    Success,Cancel
}


// 查询服务器信息入口
#[tauri::command]
pub async fn down_word(source_type: source_type::Source,username:&str,password:&str,address:source_type::AddressType,database:&str,name:&str,names:Vec<String>)->Result<DownCode, String>{
   

    // 先打开选择窗口
    if let Some(folder_path) = FileDialog::new().pick_folder() {
        let file_path = String::from(format!("{}/table_info.docx",folder_path.display()));
        let test_re:Result<Vec<source_type::TableInfoShowType>, sqlx::Error>;
        // 查询表结构
        match source_type {
            source_type::Source::Mysql =>test_re = query_table_info(username,password,address,database,name,names).await
        }
        match test_re {
            Ok(table_list) => {
                export_word(table_list,file_path);
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
async fn query_table_info(username:&str,password:&str,address:source_type::AddressType,database:&str,name:&str,names:Vec<String>)-> Result<Vec<source_type::TableInfoShowType>, sqlx::Error> {
    let url = format!("mysql://{}:{}@{}:{}/{}",username,password,address.url,address.port,database);
    let pool = MySqlPoolOptions::new().max_connections(5).connect(&url).await?;
     // 查询数据库中的所有表
    let tables_query  = format!("show table status like '%{}%'",name);

    let tables_result = query_as::<_,source_type::TableListType>(tables_query.as_str()).fetch_all(&pool).await?;
    let mut table_info_list:Vec<source_type::TableInfoShowType> = vec![];
    // 遍历所有表，查询表结构信息并导出到文件
    for table_row in tables_result {
        let table_name: String = table_row.name; 
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

        let columns_result = query_as::<_,source_type::TableInfoType>(&columns_query).fetch_all(&pool).await?;
        // 获取到是哪个表， 他的备注和表名
        let info: source_type::TableInfoShowType = source_type::TableInfoShowType{
            table_name,
            comment: table_row.comment,
            info: columns_result,
        };
        table_info_list.push(info);
    }
    
    pool.close().await;
    Ok(table_info_list)
}

fn export_word(table_list:Vec<source_type::TableInfoShowType>,file_path:String){
    let file = std::fs::File::create(&file_path).unwrap();
    let mut document = Document::new();
    for table_item in table_list{
        // 标题（表名加备注）
        let title = Paragraph::new().add_run(Run::new().add_text(format!("{} {}",table_item.table_name,table_item.comment)).size(24));
        document.children.push(docx_rs::DocumentChild::Paragraph(Box::new(title)));
        // 表格
        let mut row_list :Vec<TableRow> = vec![];
        // 添加表头
        let header_row:TableRow = TableRow::new(vec![
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("名称"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("类型"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("默认值"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("是否可空"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("备注"))),
        ]);
        // 添加每一行的数据
        row_list.push(header_row.clone());
        for table_info in table_item.info{
            let mut cell:Vec<TableCell> = vec![];
            cell.push(TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(table_info.field))));
            cell.push(TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(table_info.field_type))));
            cell.push(TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(
                match table_info.default {
                    Some(s) => s.to_string(),
                    None => "".to_string(),
                }
            ))));
            cell.push(TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(table_info.null))));
            cell.push(TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(table_info.comment))));
            row_list.push(TableRow::new(cell))
        }
        let table = Table::new(row_list);
        document.children.push(docx_rs::DocumentChild::Table(Box::new(table)));
        // 加个空白行
        document.children.push(docx_rs::DocumentChild::Paragraph(Box::new(Paragraph::new())));

    }

    let _ = Docx::new().document(document).build().pack(file);
}
