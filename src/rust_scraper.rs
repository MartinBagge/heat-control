use scraper::Html;
use serde_json::json;
//use scraper

fn get_prices() -> serde_json::Value{
    let res = reqwest::blocking::get("https://www.nordpoolgroup.com/api/marketdata/page/41?currency=,,DKK,EUR").unwrap().text().unwrap();
    return serde_json::from_str(&res).unwrap();
}

fn find_prices(full_json: serde_json::Value){
    let rows = serde_json::to_vec(full_json.get("data").unwrap().get("Rows").unwrap()).unwrap();
    let prices = [0.0;24];
    for i in 0..rows.len() {
        let row = rows[i];
        println!("{}", row);
        let price = row[0].get("Value");
        prices[i] = price;
    }
    return prices;
}

pub fn scrape_prices(){
    let res = get_prices();
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}