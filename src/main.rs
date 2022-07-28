use image::{ImageBuffer, Rgb, RgbImage};
use std::mem;
mod model;

fn line(mut x0: i32, mut y0: i32, mut x1:i32, mut y1:i32, image: &mut RgbImage, color: Rgb<u8>) {
    let mut steep: bool = false;
    // This line is terrible.
    //let (mut x0, mut x1, mut y0, mut y1): (f32, f32, f32, f32) = (x0 as f32, x1 as f32, y0 as f32, y1 as f32); 

    
    if (x0-x1).abs() < (y0-y1).abs() {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror = dy.abs()*2;
    let mut error = 0;
    let mut y = y0;

    for x in x0..x1+1 {        
        if steep {
            image.put_pixel(y as u32, x as u32, color);
        } else {
            image.put_pixel(x as u32, y as u32, color);
        }
        error += derror;
        if error > dx {
            y += if y1 > y0 {1} else {-1};
            error -= dx*2;
        }
    }
}

fn make_image() {
    // This is just for me to remember
    let imgx = 100;
    let imgy = 100;
    let white = Rgb([255, 255, 255]);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf: RgbImage = image::ImageBuffer::new(imgx, imgy);

    line(13, 20, 80, 40, &mut imgbuf, white);
    
    imgbuf.save("fig.tga").unwrap();
}


fn main() {
    let model = match model::Model::from_file("./obj/african_head.obj") {
        Ok(m) => {
            println!("Ole");
            m
        },
        Err(e) => {
            println!("Error {}", e.to_string());
            std::process::exit(1)
        }
    };

    println!("{}", model.nfaces)
}