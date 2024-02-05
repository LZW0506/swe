
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
  pub Name:String,
  pub Engine:String,
  pub Version:u64,
  pub Row_format:String,
  pub Rows: u64,
  #[serde(with = "date_format")]
  pub Create_time:DateTime<Utc>,
  pub Collation:String,
  pub Comment:String,
}

#[derive(Debug, FromRow,Serialize, Deserialize)]
pub struct TbaleInfoType {
  pub Field:String,
  pub Type:String,
  pub Default:Option<String>,
  pub Null:String,
  pub Comment:String,
}

#[derive(Debug, FromRow,Serialize, Deserialize)]
pub struct TbaleInfoShowType {
  pub table_name:String,
  pub comment:String,
  pub info:Vec<TbaleInfoType>,
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

