mod webscraper;
mod camera;
mod GPIO;
use std::sync::mpsc::{channel, Receiver};
use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, FixedOffset, Local, Utc, Timelike};

//SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

struct Restart{
    restarts: u8,
    first_restart: time::Instant,
}

fn notify(){
    // maybe send mail or ideally something else, maybe light somewhere in the house
    println!("NOTIFY");
}

fn check_burner_err(mut restart: Restart, vec: Vec<bool>) -> Restart{
    let mut on_off_count: u32 = 0;
    for i in 3..vec.len(){
        if (vec[i-1] || vec[i-2] || vec[i-3]) != vec[i]{
            on_off_count += 1;
            println!("{}", on_off_count);
        }
    }
    if on_off_count > 10{
        crate::GPIO::punch_relay();
        if restart.restarts == 0{
            restart.first_restart = time::Instant::now();
        }
        restart.restarts += 1;
        
    }
    if restart.restarts == 4{
        notify();
    } else if (restart.restarts < 4) && ((restart.first_restart.elapsed().as_millis())/1000 > 60*60) {
        restart.restarts = 0;
    }
    return restart;
}

fn main(){
    let (tx, rx)= channel();
    let mut restart = Restart{restarts: 0, first_restart: time::Instant::now()};
    
    let hour = Local::now().hour();
    println!("{}", hour);
    
    //let prices = crate::webscraper::scrape_prices();
    //for i in 0..prices.len(){
    //    println!("{}", prices[i].date);
    //    println!("{}", prices[i].price);
    //}
    thread::spawn(move ||{
        crate::camera::run_camera(tx);
    });

    loop {
        thread::sleep(time::Duration::from_secs(30));
        restart = check_burner_err(restart, rx.try_iter().collect());
        println!("{}", restart.restarts);
    }
}