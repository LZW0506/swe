
use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
// 连接地址及端口
#[derive(Deserialize)]
pub struct AddressType {
  pub url: String,
  pub port: String,
}

// 枚举数据源
#[derive(Deserialize)]
pub enum Source {
    Mysql
}

#[derive(Debug, FromRow,Deserialize,Serialize)]
pub struct TbaleInfoType {
  pub Name:String,
  pub Engine:String,
  pub Version:i32,
  pub Row_format:String,
  pub Rows: u64,
  pub Collation:String,
  pub Comment:String,
}
