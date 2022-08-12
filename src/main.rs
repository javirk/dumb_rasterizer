use image::{imageops, Rgb, RgbImage};
use std::{cmp, mem};
mod model;
use nalgebra::{SVector, Vector2, Vector3};

const width: f32 = 800.0;
const height: f32 = 800.;

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
    color: Rgb<u8>,
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
                image.put_pixel(P.x as u32, P.y as u32, color);
            }
            P.y += 1.;
        }
        P.x += 1.;
    }
}

fn world2screen(v: SVector<f32, 3>) -> SVector<f32, 3> {
    return Vector3::new(
        ((v.x + 1.) * width / 2. + 0.5).floor(),
        ((v.y + 1.) * height / 2. + 0.5).floor(),
        v.z
    );
}

fn main() {
    let mut zbuffer: Vec<f32> = vec![-std::f32::MAX; (width * height) as usize];
    let light_dir: SVector<f32, 3> = Vector3::new(0., 0., -1.);

    let mut imgbuf: RgbImage = image::ImageBuffer::new(width as u32, height as u32);

    let model = match model::Model::from_file("./obj/african_head.obj") {
        Ok(m) => m,
        Err(e) => {
            println!("Error {}", e.to_string());
            std::process::exit(1)
        }
    };

    let mut intensity_color: u8 = 0;

    // Render
    for i in 0..model.nfaces as usize {
        let face = &model.faces[i];
        let mut screen_coords: Vec<SVector<f32, 3>> = Vec::new();
        let mut world_coords: Vec<SVector<f32, 3>> = Vec::new();
        for j in 0..3 as usize {
            let v: SVector<f32, 3> = model.verts[face[j] as usize];
            screen_coords.push(world2screen(v));
            world_coords.push(v);
        }

        let mut n: SVector<f32, 3> = (world_coords[2] - world_coords[0]).cross(&(world_coords[1] - world_coords[0]));
        n = n.normalize();

        let intensity: f32 = n.dot(&light_dir);

        if intensity > 0. {
            intensity_color = (intensity * 255.) as u8;
            triangle(
                screen_coords,
                &mut zbuffer,
                &mut imgbuf,
                Rgb([intensity_color, intensity_color, intensity_color]),
            );
        }
    }

    imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("test.png").unwrap();
}


fn maindebug() {
    let mut zbuffer: Vec<f32> = vec![-std::f32::MAX; (width * height) as usize];
    let light_dir: SVector<f32, 3> = Vector3::new(0., 0., -1.);

    let mut imgbuf: RgbImage = image::ImageBuffer::new(width as u32, height as u32);

    let model = match model::Model::from_file("./obj/african_head.obj") {
        Ok(m) => m,
        Err(e) => {
            println!("Error {}", e.to_string());
            std::process::exit(1)
        }
    };

    let mut screen_coords: Vec<SVector<f32, 3>> = vec![
        Vector3::new(286.985504, 323.728149, 0.493568987),
        Vector3::new(287.443481, 325.356567, 0.480883005),
        Vector3::new(286.608154, 326.354736, 0.484221011),
    ];

    triangle(
        screen_coords,
        &mut zbuffer,
        &mut imgbuf,
        Rgb([255, 255, 255]),
    );

    imgbuf = imageops::flip_vertical(&imgbuf);
    imgbuf.save("test.png").unwrap();
}
