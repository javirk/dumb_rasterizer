use image::{imageops, RgbImage, ImageBuffer};
mod my_gl;
mod model;
mod rgb;
mod shaders;
use nalgebra::{SVector, SMatrix, Vector3};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.;

const light_dir: SVector<f32, 3> = Vector3::new(0., 0., -1.);
const eye: SVector<f32, 3> = Vector3::new(1., 1., 3.);
const center: SVector<f32, 3> = Vector3::new(0., 0., 0.);
const up: SVector<f32, 3> = Vector3::new(0., 1., 0.);


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

    // Declare variables to use later
    let mut face: &Vec<i32>;
    let mut face_texture: &Vec<i32>;
    let mut intensity: f32;
    let mut v: SVector<f32, 3>;
    let mut n: SVector<f32, 3>;
    let mut t: SVector<f32, 3>;

    let modelview: SMatrix<f32, 4, 4> = my_gl::lookat(eye, center, up);
    let projection: SMatrix<f32, 4, 4> = my_gl::projection(-1. / (eye - center).z);
    let viewport: SMatrix<f32, 4, 4> = my_gl::viewport(WIDTH / 8., HEIGHT / 8., WIDTH * 3./4., HEIGHT* 3./4.);

    // Render
    for i in 0..model.nfaces as usize {
        face_texture = &model.faces_diffuse_coords[i];
        face = &model.faces[i];

        let mut screen_coords: Vec<SVector<f32, 3>> = Vec::new(); // Is it bad to use let inside a for loop? @TODO: Investigate
        //let mut world_coords: Vec<SVector<f32, 3>> = Vec::new();
        let mut texture_coords: Vec<SVector<f32, 3>> = Vec::new();
        let shader = shaders::GouraudShader.init();

        for j in 0..3 as usize {
            v = model.verts[face[j] as usize];
            t = model.verts_diffuse[face_texture[j] as usize];

            texture_coords.push(t);
            screen_coords.push(
                my_gl::m2v(viewport * projection * modelview * my_gl::v2m(v))
            );
            //world_coords.push(v);
        }

        //n = (world_coords[2] - world_coords[0]).cross(&(world_coords[1] - world_coords[0]));
        //n = n.normalize();

        intensity = n.dot(&light_dir);

        if intensity > 0. {
            my_gl::triangle(
                screen_coords,
                &mut zbuffer,
                &mut imgbuf,
                &texture_map,
                texture_coords,
                intensity,
            );
        }
    }

    imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("test.png").unwrap();
}