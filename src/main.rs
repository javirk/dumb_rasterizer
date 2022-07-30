use image::{ImageBuffer, Rgb, RgbImage, imageops};
use std::mem;
mod model;

fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, image: &mut RgbImage, color: Rgb<u8>) {
    let mut steep: bool = false;

    if (x0 - x1).abs() < (y0 - y1).abs() {
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
    let derror = dy.abs() * 2;
    let mut error = 0;
    let mut y = y0;

    for x in x0..x1 + 1 {
        if steep {
            image.put_pixel(y as u32, x as u32, color);
        } else {
            image.put_pixel(x as u32, y as u32, color);
        }
        error += derror;
        if error > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

fn main() {
    let width: f32 = 512.0;
    let height: f32 = 512.;

    let white = Rgb([255, 255, 255]);

    let mut imgbuf: RgbImage = image::ImageBuffer::new(width as u32, height as u32);

    let model = match model::Model::from_file("./obj/african_head.obj") {
        Ok(m) => m,
        Err(e) => {
            println!("Error {}", e.to_string());
            std::process::exit(1)
        }
    };

    println!("{}", model.nfaces);
    // Render
    for i in 0..model.nfaces as usize {
        let face = &model.faces[i];
        for j in 0..3 as usize {
            let v0 = &model.verts[face[j] as usize];
            let v1 = &model.verts[face[(j + 1) % 3] as usize];
            // Obj vertices are [-1, 1]. We want to transform to [0, w)
            let x0 = ((v0[0] + 1.) * (width - 1.) / 2.) as i32;
            let y0 = ((v0[1] + 1.) * (height - 1.) / 2.) as i32;
            let x1 = ((v1[0] + 1.) * (width - 1.) / 2.) as i32;
            let y1 = ((v1[1] + 1.) * (height - 1.) / 2.) as i32;
            line(x0, y0, x1, y1, &mut imgbuf, white);
        }
    }

    imgbuf = imageops::flip_vertical(&imgbuf);

    imgbuf.save("test.png").unwrap();
}
