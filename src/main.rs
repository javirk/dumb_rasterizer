use image::{imageops, RgbImage, Rgb};
mod my_gl;
mod model;
mod rgb;
mod shaders;
use my_gl::triangle;
use nalgebra::{SVector, SMatrix, Vector3};
use shaders::IShader;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.;

const LIGHT_DIR: SVector<f32, 3> = Vector3::new(0., 0., -1.);
const EYE: SVector<f32, 3> = Vector3::new(1., 1., 3.);
const CENTER: SVector<f32, 3> = Vector3::new(0., 0., 0.);
const UP: SVector<f32, 3> = Vector3::new(0., 1., 0.);
const BASE_COLOR: Rgb<u8> = Rgb([255 as u8, 155 as u8, 0 as u8]);


fn main() {
    let mut zbuffer: Vec<f32> = vec![-std::f32::MAX; (WIDTH * HEIGHT) as usize];

    let mut imgbuf: RgbImage = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    let model = match model::Model::from_file(
        "./obj/african_head.obj",
        "./obj/african_head_diffuse.tga",
        "./obj/african_head_nm.tga"
    ) {
        Ok(m) => m,
        Err(e) => {
            println!("Error {}", e.to_string());
            std::process::exit(1)
        }
    };

    let modelview: SMatrix<f32, 4, 4> = my_gl::lookat(EYE, CENTER, UP);
    let projection: SMatrix<f32, 4, 4> = my_gl::projection(-1. / (EYE - CENTER).z);
    let viewport: SMatrix<f32, 4, 4> = my_gl::viewport(WIDTH / 8., HEIGHT / 8., WIDTH * 3./4., HEIGHT* 3./4.);

    let mut shader: Box<dyn IShader> = Box::new(shaders::CartoonShader::init());
    let transformation: SMatrix<f32, 4, 4> = viewport * projection * modelview;

    for i in 0..model.nfaces as usize {
        let mut screen_coords: Vec<SVector<f32, 4>> = Vec::new(); // Is it bad to use let inside a for loop? @TODO: Investigate
        for j in 0..3 as usize {
            screen_coords.push(shader.vertex(&model, LIGHT_DIR, transformation, i, j));
        }
        triangle(screen_coords, &shader, &mut zbuffer, &mut imgbuf, BASE_COLOR);
    }

    imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("test.png").unwrap();
}