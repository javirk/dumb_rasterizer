use image::Rgb;
use nalgebra::{Matrix4x1, SVector, Vector3, SMatrix};
use crate::model::Model;

struct IShader;

// trait IShader {
//     fn vertex(iface: u32, nthvert: u32) -> SVector<f32, 3>;
//     fn fragment(var: SVector<f32, 3>, color: Rgb<u8>) -> bool;
// }

pub struct GouraudShader {
    varying_intensity: SVector<f32, 3>,
}

impl GouraudShader {
    pub fn init() -> GouraudShader {
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
        let gl_vertex = model.verts[model.faces[iface][nthvert]];
        gl_vertex = transformation.dot(&gl_vertex);
        return gl_vertex;
    }

    fn fragment(&self, bar: SVector<f32, 3>) -> (bool, Rgb<u8>) {
        let intensity: f32 = self.varying_intensity * bar;  // Probably wrong
        let color: Rgb<u8> = Rgb([
            (255. * intensity) as u8,
            (255. * intensity) as u8,
            (255. * intensity) as u8
        ]);
        return (false, color)
    }
}
