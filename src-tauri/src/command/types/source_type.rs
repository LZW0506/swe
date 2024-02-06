
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


  
#[derive(Debug, FromRow,Serialize, Deserialize)]
pub struct TbaleListType {
  #[sqlx(rename = "Name")] // 表名
  pub name:String,
  #[sqlx(rename = "Engine")] 
  pub engine:String,
  // pub Version:u64,
  #[sqlx(rename = "Row_format")]
  pub row_format:String,
  #[sqlx(rename = "Rows")]
  pub rows: u64,
  #[serde(with = "date_format")]
  #[sqlx(rename = "Create_time")]
  pub create_time:DateTime<Utc>,
  #[sqlx(rename = "Collation")]
  pub collation:String,
  #[sqlx(rename = "Comment")]
  pub comment:String,
}



#[derive(Debug, FromRow,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableInfoType {
  #[sqlx(rename = "Field")]
  pub field:String,
  #[sqlx(rename = "Type")]
  pub field_type:String,
  #[sqlx(rename = "Default")]
  pub default:Option<String>,
  #[sqlx(rename = "Null")]
  pub null:String,
  #[sqlx(rename = "Comment")]
  pub comment:String,
}


#[derive(Debug, FromRow,Serialize, Deserialize)]
pub struct TbaleInfoShowType {
  pub table_name:String,
  pub comment:String,
  pub info:Vec<TableInfoType>,
}


// #[derive(Debug,Deserialize,Serialize)]
// pub struct TbaleInfoTypeOut {
//   pub data:TbaleInfoType,
//   pub total:usize
// }


mod date_format {
  use chrono::{DateTime, Utc, NaiveDateTime};
  use serde::{self, Deserialize, Serializer, Deserializer};

  const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

  pub fn serialize<S>(
      date: &DateTime<Utc>,
      serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let s = format!("{}", date.format(FORMAT));
      serializer.serialize_str(&s)
  }

  pub fn deserialize<'de, D>(
      deserializer: D,
  ) -> Result<DateTime<Utc>, D::Error>
  where
      D: Deserializer<'de>,
  {
      let s = String::deserialize(deserializer)?;
      let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
      Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
  }
}
