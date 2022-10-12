use nokhwa::{CameraFormat, FrameFormat, Camera};
use image::{DynamicImage, GrayImage, RgbImage, ImageBuffer};
use chrono::Utc;
use std::{thread, time};

fn analyze_frame(frame: GrayImage) -> bool{
    let mut sum:u64 = 0;

    for (x, y, p) in luma.enumerate_pixels() {
        sum = sum + p[0] as u64;
    }

    let avg = sum as f32 / (frame.width()*frame.height()) as f32;
    
    if avg > 50.0 {
        return true;
    }
    return false;
}

pub fn run_camera(){

    // set up the Camera
    let mut camera = Camera::new(
        0, Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)), // format
    ).unwrap();
    camera.set_frame_rate(2);
    // open stream
    camera.open_stream().unwrap();

    loop {

        let loop_time = time::Instant::now();

        let frame = camera.frame().unwrap();
        
        let rgb = RgbImage::from_vec(frame.width(), frame.height(), frame.to_vec());
        if rgb.is_none(){
            continue; // check to make sure camera is started
        }

        let luma: GrayImage = DynamicImage::ImageRgb8(rgb.unwrap()).into_luma8();

        if analyze_frame() {
            
        }

        thread::sleep(time::Duration::from_millis((500 as u64)-(loop_time.elapsed().as_millis() as u64)));

    }

}