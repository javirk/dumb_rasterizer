use image::Rgb;
use nalgebra::{Matrix4x1, SVector, Vector3, SMatrix};
use crate::model::Model;
use crate::my_gl;

pub trait IShader {
    fn init(&self) -> Self where Self: Sized;  // Because we want IShader to be an object type
    fn vertex(&mut self,
        model: Model,
        light_dir: SVector<f32, 3>,
        transformation: SMatrix<f32, 4, 4>,
        iface: usize,
        nthvert: usize,) -> SVector<f32, 3>;
    fn fragment(&self, bar: SVector<f32, 3>) -> (bool, Rgb<u8>);
}

pub struct GouraudShader {
    varying_intensity: SVector<f32, 3>,
}

impl IShader for GouraudShader {
    fn init(&self) -> Self {
        return GouraudShader{
            varying_intensity: Vector3::new(0., 0., 0.)
        }
    }

    fn vertex(
        &mut self,
        model: Model,
        light_dir: SVector<f32, 3>,
        transformation: SMatrix<f32, 4, 4>,
        iface: usize,
        nthvert: usize,
    ) -> SVector<f32, 3> {

        self.varying_intensity[nthvert] = f32::max(0., model.normal(iface, nthvert).dot(&light_dir));
        let gl_vertex: SMatrix<f32, 4, 1> = my_gl::v2m(model.verts[model.faces[iface][nthvert] as usize]);
        gl_vertex = transformation * gl_vertex;
        return my_gl::m2v(gl_vertex);
    }

    fn fragment(&self, bar: SVector<f32, 3>) -> (bool, Rgb<u8>) {
        let intensity: f32 = self.varying_intensity.dot(&bar);  // Probably wrong
        let color: Rgb<u8> = Rgb([
            (255. * intensity) as u8,
            (255. * intensity) as u8,
            (255. * intensity) as u8
        ]);
        return (false, color)
    }
}
