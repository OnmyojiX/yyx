use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime, TimeZone, Timelike, Utc};
use lazy_static::lazy_static;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use url::Url;
use yyx_types::Snapshot;

const URL_PREFIX: &str = "https://yys.cbg.163.com/cgi/mweb/equip/";
const URL_GET_DETAIL: &str = "https://yys.cbg.163.com/cgi/api/get_equip_detail";
const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.87 Safari/537.36";

use crate::result::*;
use crate::types::CbgEquipDesc;

lazy_static! {
  static ref HTTP_CLIENT: Client = { Client::new() };
}

pub fn pull(cbg_url: &str, version: &str) -> CbgResult<CbgSnapshot> {
  use reqwest::header::USER_AGENT;
  let params = get_params_from_url(cbg_url)?;

  let res: Response = HTTP_CLIENT
    .post(URL_GET_DETAIL)
    .header(USER_AGENT, UA)
    .form(&params)
    .send()?
    .json()?;

  res.parse_snapshot(cbg_url, version)
}

#[derive(Debug, Deserialize)]
pub struct Response {
  status: i64,
  equip: Equip,
}

#[derive(Debug, Deserialize)]
struct Equip {
  equip_desc: String,
  create_time_desc: String,
  server_name: String,
  price: i64,
}

fn parse_datetime(v: &str) -> CbgResult<DateTime<Utc>> {
  let naive = NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S")?;
  Ok(
    FixedOffset::east(3600 * 8)
      .ymd(naive.year(), naive.month(), naive.day())
      .and_hms(naive.hour(), naive.minute(), naive.second())
      .into(),
  )
}

impl Response {
  fn parse_snapshot(&self, url: &str, version: &str) -> CbgResult<CbgSnapshot> {
    use std::convert::TryFrom;

    let desc: CbgEquipDesc = serde_json::from_str(&self.equip.equip_desc)?;
    let listing_info = CbgListingInfo {
      server_name: self.equip.server_name.clone(),
      player_name: desc.player_name.clone(),
      player_level: desc.player_level.clone(),
      price: format!("{:.2}", self.equip.price as f64 / 100.0),
      create_time: parse_datetime(&self.equip.create_time_desc)?,
    };
    let timestamp = listing_info.create_time.clone().into();
    Ok(CbgSnapshot {
      listing_info,
      snapshot: Snapshot {
        version: format!("{}-cbg", version),
        timestamp,
        data: TryFrom::try_from(desc).map_err(CbgError::Convert)?,
        cbg_url: Some(url.to_string()),
      },
    })
  }
}

#[derive(Debug, Serialize)]
pub struct CbgSnapshot {
  pub listing_info: CbgListingInfo,
  pub snapshot: Snapshot,
}

#[derive(Debug, Serialize)]
pub struct CbgListingInfo {
  pub server_name: String,
  pub player_name: String,
  pub player_level: i64,
  pub price: String,
  pub create_time: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct Params {
  #[serde(rename = "serverid")]
  pub server_id: String,
  #[serde(rename = "ordersn")]
  pub order_sn: String,
}

pub fn get_params_from_url(cbg_url: &str) -> Result<Params, CbgError> {
  lazy_static! {
    static ref URL: Url = { Url::parse(URL_PREFIX).unwrap() };
  }

  if !cbg_url.starts_with(URL_PREFIX) {
    return Err(CbgError::UrlPrefix(URL_PREFIX));
  }
  let url = Url::parse(cbg_url)?;
  let segments: Vec<&str> = url
    .path_segments()
    .map(|segments| {
      segments
        .skip(
          URL
            .path_segments()
            .map(|s| s.filter(|s| !s.is_empty()).count())
            .unwrap_or(0),
        )
        .collect()
    })
    .ok_or(CbgError::InvalidUrl)?;
  if segments.len() != 2 {
    return Err(CbgError::InvalidUrl);
  }
  Ok(Params {
    server_id: segments[0].to_string(),
    order_sn: segments[1].to_string(),
  })
}

#[test]
fn test_deserialize_desc() {
  use crate::types::CbgEquipDesc;
  use serde_json;
  let json_content = include_str!("./test_data_res.json");
  let res: Response = serde_json::from_str(&json_content).unwrap();

  {
    use serde_json::Value;
    use std::fs::write;
    let v: Value = serde_json::from_str(&res.equip.equip_desc).unwrap();
    write(
      "./test_data_res.json",
      serde_json::to_string_pretty(&v).unwrap(),
    )
    .unwrap();
  }

  let _: CbgEquipDesc = serde_json::from_str(&res.equip.equip_desc).unwrap();
}
