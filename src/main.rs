use image::{ImageBuffer, Rgb, RgbImage};

fn line(x0: u32, y0: u32, x1:u32, y1:u32, image: &mut RgbImage, color: Rgb<u8>) {
    let y0 = y0 as f32;
    let y1 = y1 as f32;
    for x in x0..x1 {
        let t = ((x-x0) as f32)/((x1-x0) as f32);
        let y = y0*(1.-t)+ y1*t;
        image.put_pixel(x as u32, y as u32, color);
    }
}


fn main() {
    let imgx = 100;
    let imgy = 100;
    let red = Rgb([255 as u8, 0, 0]);
    let white = Rgb([255, 255, 255]);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf: RgbImage = image::ImageBuffer::new(imgx, imgy);

    line(13, 20, 80, 40, &mut imgbuf, white);
    line(20, 13, 40, 80, &mut imgbuf, red);
    line(80, 40, 13, 20, &mut imgbuf, red);

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fig.tga").unwrap();

}