use scraper::Html;
use serde_json::{json, Result};
use serde::{Deserialize, Serialize};
use std::string::String;
//use scraper

#[derive(Serialize, Deserialize)]
struct Column {
    behavior: u8,
    combinedname: String,
    datetimefordata: String,
    displayname: String,
    displaynameordominatingdirection: String,
    displaynegativevalueinblue: bool,
    groupheader: String,
    index: u8,
    isadditionaldata: bool,
    isdominatingdirection: bool,
    isofficial: bool,
    isvalid: bool,
    name: String,
    scale: u8,
    secondaryvalue: String,
    usedashdisplaystyle: bool,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct Row {
    columns: Vec<Column>,
    datetimefordata: String,
    daynumber: u8,
    emptyvalue: String,
    endtime: String,
    isextrarow: bool,
    isntcrow: bool,
    name: String,
    parent: String,
    starttime: String,
    starttimedate: String,
}

#[derive(Serialize, Deserialize)]
struct Rows {
    rows: Vec<Row>
}

pub struct date_price{
    pub date: String,
    pub price: f32
}

fn get_prices() -> serde_json::Value{
    let res = reqwest::blocking::get("https://www.nordpoolgroup.com/api/marketdata/page/41?currency=,,DKK,EUR").unwrap().text().unwrap();
    return serde_json::from_str(&res).unwrap();
}

fn find_prices(full_json: serde_json::Value) -> Vec<date_price>{
    let rows: Rows = serde_json::from_str(&(serde_json::to_string(full_json.get("data").unwrap()).unwrap().to_lowercase().replace("null", "\"\""))).unwrap();
    let mut prices = Vec::new();
    for i in 0..24 {
        let row: &Row = &rows.rows[i];
        let price_str: String = (row.columns[0].value).replace(",",".").replace(" ", "");
        let price: f32 = price_str.parse().unwrap();
        let price_obj = date_price{
            date: row.starttime.clone(),
            price: price
        };
        prices.push(price_obj);
    }
    return prices;
}

pub fn scrape_prices() -> Vec<date_price>{
    let res = get_prices();
    let prices = find_prices(res);
    return prices;
}