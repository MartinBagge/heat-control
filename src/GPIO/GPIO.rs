use gpio;
use gpio::GpioOut;

pub fn punch_relay(){
    /*
    let mut burner_relay = gpio::sysfs::SysFsGpioOutput::open(23).unwrap();
    burner_relay.set_high().unwrap();
    thread::sleep(time::Duration::from_millis(500));
    burner_relay.set_low().unwrap();
    */
    println!("punch_relay");
}

pub fn start_heater(){
    let mut heater_relay = gpio::sysfs::SysFsGpioOutput::open(24).unwrap();
    heater_relay.set_high().unwrap();
}

pub fn stop_heater(){
    let mut heater_relay = gpio::sysfs::SysFsGpioOutput::open(24).unwrap();
    heater_relay.set_low().unwrap();
}