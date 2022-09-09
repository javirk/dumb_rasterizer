use nalgebra::{Matrix4x1, SVector, SMatrix, Vector2, Vector3};
use image::{Pixel, Rgb, RgbImage};
use crate::shaders::IShader;

const DEPTH: f32 = 255.;

pub fn projection(coeff: f32) -> SMatrix<f32, 4, 4> {
    // Coeff: -1. / (eye - center).z;
    let mut proj: SMatrix<f32, 4, 4> = SMatrix::identity();
    proj[(3, 2)] = coeff;
    return proj;
}


pub fn viewport(x: f32, y: f32, w: f32, h: f32) -> SMatrix<f32, 4, 4> {
    let mut m: SMatrix<f32, 4, 4> = SMatrix::identity();
    m[(0, 3)] = x + w / 2.;
    m[(1, 3)] = y + h / 2.;
    m[(2, 3)] = DEPTH / 2.;

    m[(0, 0)] = w / 2.;
    m[(1, 1)] = h / 2.;
    m[(2, 2)] = DEPTH / 2.;
    return m;
}

pub fn lookat(eye: SVector<f32, 3>, center: SVector<f32, 3>, up: SVector<f32, 3>) -> SMatrix<f32, 4, 4> {
    let z: SVector<f32, 3> = (eye - center).normalize();
    let x: SVector<f32, 3> = up.cross(&z).normalize();
    let y: SVector<f32, 3> = z.cross(&x).normalize();
    let mut res: SMatrix<f32, 4, 4> = SMatrix::identity();
    for i in 0..3 {
        res[(0, i)] = x[i];
        res[(1, i)] = y[i];
        res[(2, i)] = z[i];
        res[(i, 3)] = -center[i];
    }
    return res;
}

pub fn v2m(v: SVector<f32, 3>) -> SMatrix<f32, 4, 1> {
    let m = Matrix4x1::from_column_slice(&[v.x, v.y, v.z, 1.]);
    return m;
}

pub fn m2v(m: SMatrix<f32, 4, 1>) -> SVector<f32, 3> {
    let v = Vector3::new(
        (m[(0, 0)] / m[(3, 0)]).floor(),
        (m[(1, 0)] / m[(3, 0)]).floor(),
        (m[(2, 0)] / m[(3, 0)]).floor(),
    );
    return v;
}


fn barycentric(pts: &Vec<SVector<f32, 3>>, p: SVector<f32, 3>) -> SVector<f32, 3> {
    let v1: SVector<f32, 3> = Vector3::new(
        pts[2][0] - pts[0][0],
        pts[1][0] - pts[0][0],
        pts[0][0] - p[0],
    );
    let v2: SVector<f32, 3> = Vector3::new(
        pts[2][1] - pts[0][1],
        pts[1][1] - pts[0][1],
        pts[0][1] - p[1],
    );

    let u = v1.cross(&v2);

    if u.z.abs() < 1e-2 {
        return Vector3::new(-1., 1., 1.);
    }

    return Vector3::new(1.0f32 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
}

pub fn triangle(
    pts: Vec<SVector<f32, 3>>,
    shader: Box<dyn IShader>,
    zbuffer: &mut Vec<f32>,
    image: &mut RgbImage,
) {
    let (imwidth, imheight) = (image.width() as f32, image.height() as f32);
    let (texwidth, texheight) = (texture.width() as f32, texture.height() as f32);

    let mut bboxmin: SVector<f32, 2> = Vector2::new(std::f32::MAX, std::f32::MAX);
    let mut bboxmax: SVector<f32, 2> = Vector2::new(-std::f32::MAX, -std::f32::MAX);
    let clamp: SVector<f32, 2> = Vector2::new(imwidth - 1., imheight - 1.);

    for i in 0..3 {
        for j in 0..=1 {
            bboxmin[j] = f32::max(0., f32::min(bboxmin[j], pts[i][j]));
            bboxmax[j] = f32::min(clamp[j], f32::max(bboxmax[j], pts[i][j]));
        }
    }

    let mut p: SVector<f32, 3> = Vector3::new(bboxmin.x, bboxmin.y, 0.);
    let mut texture_coords: SVector<f32, 3>;
    let mut pixel_color: Rgb<u8>;

    while p.x <= bboxmax.x {
        p.y = bboxmin.y;
        while p.y <= bboxmax.y {
            let bc_screen = barycentric(&pts, p);
            if bc_screen.x < 0. || bc_screen.y < 0. || bc_screen.z < 0. {
                p.y += 1.;
                continue;
            }

            p.z = 0.;
            for i in 0..3 {
                p.z += pts[i][2] * bc_screen[i];
            }

            if zbuffer[(p.x + p.y * imwidth) as usize] < p.z {
                zbuffer[(p.x + p.y * imwidth) as usize] = p.z;

                texture_coords = vertices_texture_coords[0] * bc_screen.x
                    + vertices_texture_coords[1] * bc_screen.y
                    + vertices_texture_coords[2] * bc_screen.z;

                pixel_color = texture
                    .get_pixel(
                        (texture_coords[0] * texwidth) as u32,
                        ((1. - texture_coords[1]) * texheight) as u32,
                    )
                    .to_rgb();
                pixel_color = Rgb([
                    ((pixel_color.0[0] as f32) * intensity) as u8,
                    ((pixel_color.0[1] as f32) * intensity) as u8,
                    ((pixel_color.0[2] as f32) * intensity) as u8,
                ]);

                image.put_pixel(p.x as u32, p.y as u32, pixel_color)
            }
            p.y += 1.;
        }
        p.x += 1.;
    }
}
