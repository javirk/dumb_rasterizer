use image::{imageops, Rgb, RgbImage, Pixel};
use std::{mem};
mod model;
mod rgb;
use rgb::RgbExt;
use nalgebra::{SVector, Vector2, Vector3};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.;

fn barycentric(pts: &Vec<SVector<f32, 3>>, P: SVector<f32, 3>) -> SVector<f32, 3> {
    let v1: SVector<f32, 3> = Vector3::new(
        pts[2][0] - pts[0][0],
        pts[1][0] - pts[0][0],
        pts[0][0] - P[0],
    );
    let v2: SVector<f32, 3> = Vector3::new(
        pts[2][1] - pts[0][1],
        pts[1][1] - pts[0][1],
        pts[0][1] - P[1],
    );

    let u = v1.cross(&v2);
    //println!("{} {} {}", u.x, u.y, u.z);

    /* `pts` and `P` has integer value as coordinates
    so `abs(u[2])` < 1 means `u[2]` is 0, that means
    triangle is degenerate, in this case return something with negative coordinates */
    if u.z.abs() < 1e-2 {
        return Vector3::new(-1., 1., 1.);
    }

    return Vector3::new(1.0f32 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
}

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

fn triangle(
    pts: Vec<SVector<f32, 3>>,
    zbuffer: &mut Vec<f32>,
    image: &mut RgbImage,
    vertices_colors: &mut Vec<RgbExt<f32>>,
) {
    let (imwidth, imheight) = (image.width() as f32, image.height() as f32);

    let mut bboxmin: SVector<f32, 2> = Vector2::new(std::f32::MAX, std::f32::MAX);
    let mut bboxmax: SVector<f32, 2> = Vector2::new(-std::f32::MAX, -std::f32::MAX);
    let clamp: SVector<f32, 2> = Vector2::new(imwidth - 1., imheight - 1.);

    for i in 0..3 {
        for j in 0..=1 {
            bboxmin[j] = f32::max(0., f32::min(bboxmin[j], pts[i][j]));
            bboxmax[j] = f32::min(clamp[j], f32::max(bboxmax[j], pts[i][j]));
        }
    }

    let mut P: SVector<f32, 3> = Vector3::new(bboxmin.x, bboxmin.y, 0.);
    let mut color: RgbExt<f32>;
    let mut den_color: f32;
    let mut color_rgb8: Rgb<u8>;

    while P.x <= bboxmax.x {
        P.y = bboxmin.y;
        while P.y <= bboxmax.y {
            let bc_screen = barycentric(&pts, P);
            if bc_screen.x < 0. || bc_screen.y < 0. || bc_screen.z < 0. {
                P.y += 1.;
                continue;
            }

            P.z = 0.;
            for i in 0..3 {
                P.z += pts[i][2] * bc_screen[i];
            }

            if zbuffer[(P.x + P.y * imwidth) as usize] < P.z {
                zbuffer[(P.x + P.y * imwidth) as usize] = P.z;

                den_color = bc_screen.x + bc_screen.y + bc_screen.z;
                
                color = (vertices_colors[0]*bc_screen.x + vertices_colors[1]*bc_screen.y + vertices_colors[2]*bc_screen.z) / den_color;
                // TODO: Implement this as an extension to the trait. I don't know how to do it yet. RgbExt<f32> -> Rgb<u8>
                color_rgb8 = Rgb([
                    (color.0[0]*255.) as u8, 
                    (color.0[1]*255.) as u8, 
                    (color.0[2]*255.) as u8
                    ]); 
                    
                image.put_pixel(P.x as u32, P.y as u32, color_rgb8)
            }
            P.y += 1.;
        }
        P.x += 1.;
    }
}

fn world2screen(v: SVector<f32, 3>) -> SVector<f32, 3> {
    return Vector3::new(
        ((v.x + 1.) * WIDTH / 2. + 0.5).floor(),
        ((v.y + 1.) * HEIGHT / 2. + 0.5).floor(),
        v.z
    );
}

fn main() {
    let mut zbuffer: Vec<f32> = vec![-std::f32::MAX; (WIDTH * HEIGHT) as usize];
    let light_dir: SVector<f32, 3> = Vector3::new(0., 0., -1.);

    let mut imgbuf: RgbImage = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    let model = match model::Model::from_file("./obj/african_head.obj") {
        Ok(m) => m,
        Err(e) => {
            println!("Error {}", e.to_string());
            std::process::exit(1)
        }
    };

    let texture = image::open("./obj/african_head_diffuse.tga").unwrap().into_rgba32f();

    let mut intensity_color: u8 = 0;

    // Declare variables to use later
    let mut face: &Vec<i32> = &Vec::new();
    let mut face_texture: &Vec<i32> = &Vec::new();
    let mut intensity: f32 = 0.;
    let mut v: SVector<f32, 3>;
    let mut n: SVector<f32, 3>;
    let mut vertex_color: Rgb<f32>;

    // Render
    for i in 0..model.nfaces as usize {
        face = &model.faces[i];
        face_texture = &model.faces_texture_coords[i];
        let mut screen_coords: Vec<SVector<f32, 3>> = Vec::new();  // Is it bad to use let inside a for loop? @TODO: Investigate
        let mut world_coords: Vec<SVector<f32, 3>> = Vec::new();
        let mut texture_coords: Vec<SVector<f32, 3>> = Vec::new();
        let mut vertices_colors: Vec<RgbExt<f32>> = Vec::new();

        for j in 0..3 as usize {
            v = model.verts[face[j] as usize];
            texture_coords.push(model.verts_texture[face_texture[j] as usize]);
            screen_coords.push(world2screen(v));
            world_coords.push(v);
        }
        
        // There has to be a better way to do this. 
        for j in 0..3 as usize {
            texture_coords[j][0] = texture_coords[j][0] * (texture.dimensions().0 as f32);
            texture_coords[j][1] = texture_coords[j][1] * (texture.dimensions().1 as f32);

            vertex_color = texture.get_pixel(texture_coords[j][0] as u32, texture_coords[j][1] as u32).to_rgb();
            vertices_colors.push(RgbExt(vertex_color))
        }

        n = (world_coords[2] - world_coords[0]).cross(&(world_coords[1] - world_coords[0]));
        n = n.normalize();

        intensity = n.dot(&light_dir);

        if intensity > 0. {
            intensity_color = (intensity * 255.) as u8;
            triangle(
                screen_coords,
                &mut zbuffer,
                &mut imgbuf,
                &mut vertices_colors
            );
        }
    }

    imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("test.png").unwrap();
}


fn main_test() {
    let mut zbuffer: Vec<f32> = vec![-std::f32::MAX; (WIDTH * HEIGHT) as usize];

    let mut imgbuf: RgbImage = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    let screen_coords: Vec<SVector<f32, 3>> = vec![
        Vector3::new(45., 45., 0.),
        Vector3::new(256., 467., 0.),
        Vector3::new(467., 45., 0.),
    ];

    let mut vertices_colors: Vec<RgbExt<f32>> = vec![
        RgbExt(Rgb([1., 0., 0.])),
        RgbExt(Rgb([0., 1., 0.])),
        RgbExt(Rgb([0., 0., 1.])),
    ];

    triangle(
        screen_coords,
        &mut zbuffer,
        &mut imgbuf,
        &mut vertices_colors
    );

    imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("test.png").unwrap();
}