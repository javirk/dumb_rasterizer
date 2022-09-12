use image::Rgb;
use nalgebra::{SVector, Vector3, SMatrix};
use crate::model::Model;
use crate::my_gl;


pub trait IShader {
    fn init() -> Self where Self: Sized;  // Because we want IShader to be an object type
    fn vertex(&mut self,
        model: &Model,
        light_dir: SVector<f32, 3>,
        transformation: SMatrix<f32, 4, 4>,
        iface: usize,
        nthvert: usize,) -> SVector<f32, 4>;
    fn fragment(&self, bar: SVector<f32, 3>, base_color: Rgb<u8>) -> (bool, Rgb<u8>); 
}

pub struct GouraudShader {
    varying_intensity: SVector<f32, 3>,
}

impl IShader for GouraudShader {
    fn init() -> Self {
        return GouraudShader{
            varying_intensity: Vector3::new(0., 0., 0.)
        }
    }

    fn vertex(
        &mut self,
        model: &Model,
        light_dir: SVector<f32, 3>,
        transformation: SMatrix<f32, 4, 4>,
        iface: usize,
        nthvert: usize,
    ) -> SVector<f32, 4> {
        self.varying_intensity[nthvert] = f32::max(0., model.normal(iface, nthvert).dot(&light_dir));
        let mut gl_vertex: SMatrix<f32, 4, 1> = my_gl::v2m(model.verts[model.faces[iface][nthvert] as usize]);
        gl_vertex = transformation * gl_vertex;
        return my_gl::m2v(gl_vertex);
    }

    fn fragment(&self, bar: SVector<f32, 3>, base_color: Rgb<u8>) -> (bool, Rgb<u8>) {
        let intensity: f32 = self.varying_intensity.dot(&bar);
        let color: Rgb<u8> = Rgb([
            (base_color.0[0] as f32 * intensity) as u8,
            (base_color.0[1] as f32 * intensity) as u8,
            (base_color.0[2] as f32 * intensity) as u8
        ]);
        return (false, color)
    }
}

pub struct CartoonShader {
    varying_intensity: SVector<f32, 3>,
}

impl IShader for CartoonShader {
    fn init() -> Self {
        return CartoonShader {
            varying_intensity: Vector3::new(0., 0., 0.)
        }
    }

    // Is there a way to take this implementation from GoraudShader?
    fn vertex(
        &mut self,
        model: &Model,
        light_dir: SVector<f32, 3>,
        transformation: SMatrix<f32, 4, 4>,
        iface: usize,
        nthvert: usize,
    ) -> SVector<f32, 4> {
        self.varying_intensity[nthvert] = f32::max(0., model.normal(iface, nthvert).dot(&light_dir));
        let mut gl_vertex: SMatrix<f32, 4, 1> = my_gl::v2m(model.verts[model.faces[iface][nthvert] as usize]);
        gl_vertex = transformation * gl_vertex;
        return my_gl::m2v(gl_vertex);
    }

    fn fragment(&self, bar: SVector<f32, 3>, base_color: Rgb<u8>) -> (bool, Rgb<u8>) {
        let mut intensity: f32 = self.varying_intensity.dot(&bar);
        intensity = match intensity {
            x if (0.85..1.00).contains(&x) => 1.,
            x if (0.60..0.85).contains(&x) => 0.80,
            x if (0.45..0.60).contains(&x) => 0.60,
            x if (0.30..0.45).contains(&x) => 0.45,
            x if (0.15..0.30).contains(&x) => 0.30,
            _ => 0.,
        };

        let color: Rgb<u8> = Rgb([
            (base_color.0[0] as f32 * intensity) as u8,
            (base_color.0[1] as f32 * intensity) as u8,
            (base_color.0[2] as f32 * intensity) as u8
        ]);
        return (false, color)
    }

}