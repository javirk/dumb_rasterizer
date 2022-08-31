use nalgebra::{Matrix4x1, SMatrix, SVector, Vector2, Vector3};

const DEPTH: f32 = 255.;

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