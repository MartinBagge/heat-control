mod webscraper;
mod camera;

fn main(){
    //let prices = crate::webscraper::scrape_prices();
    //for i in 0..prices.len(){
    //    println!("{}", prices[i].date);
    //    println!("{}", prices[i].price);
    //}
    crate::camera::run_camera();
}