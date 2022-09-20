use std::process;

use image::Rgb;
use nalgebra::{SVector, Vector3, SMatrix, Matrix3, Matrix4, Matrix4x3};
use crate::model::Model;
use crate::{my_gl::{proj4_3, m2v, v2m, m2v_floor}, LIGHT_DIR};


pub trait IShader {
    fn init() -> Self where Self: Sized;  // Because we want IShader to be an object type
    fn vertex(&mut self,
        model: &Model,
        transformation: SMatrix<f32, 4, 4>,
        iface: usize,
        nthvert: usize,) -> SVector<f32, 4>;
    fn fragment(&self, model: &Model, bar: SVector<f32, 3>, base_color: Rgb<u8>) -> (bool, Rgb<u8>); 
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
        transformation: SMatrix<f32, 4, 4>,
        iface: usize,
        nthvert: usize,
    ) -> SVector<f32, 4> {
        self.varying_intensity[nthvert] = f32::max(0., model.uv_normal(iface, nthvert).dot(&LIGHT_DIR));
        let mut gl_vertex: SMatrix<f32, 4, 1> = v2m(model.verts[model.faces[iface][nthvert] as usize]);
        gl_vertex = transformation * gl_vertex;
        return m2v(gl_vertex);
    }

    fn fragment(&self, model: &Model, bar: SVector<f32, 3>, base_color: Rgb<u8>) -> (bool, Rgb<u8>) {
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
        transformation: SMatrix<f32, 4, 4>,
        iface: usize,
        nthvert: usize,
    ) -> SVector<f32, 4> {
        self.varying_intensity[nthvert] = f32::max(0., model.uv_normal(iface, nthvert).dot(&LIGHT_DIR));
        let mut gl_vertex: SMatrix<f32, 4, 1> = v2m(model.verts[model.faces[iface][nthvert] as usize]);
        gl_vertex = transformation * gl_vertex;
        return m2v(gl_vertex);
    }

    fn fragment(&self, model: &Model, bar: SVector<f32, 3>, base_color: Rgb<u8>) -> (bool, Rgb<u8>) {
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

pub struct Shader {
    varying_uv: SMatrix<f32, 3, 3>,
    varying_nrm: SMatrix<f32, 3, 3>,
    varying_tri: SMatrix<f32, 4, 3>,
    uniform_M: SMatrix<f32, 4, 4>,
    uniform_MIT: SMatrix<f32, 4, 4>,
    ndc_tri: SMatrix<f32, 3, 3> //Not used at all now
}

impl Shader {
    pub fn new(uniform_m: SMatrix<f32, 4, 4>) -> Self {
        let inv_matrix = uniform_m.try_inverse().unwrap();
        return Shader {
            varying_uv: Matrix3::<f32>::zeros(),
            varying_nrm: Matrix3::<f32>::zeros(),
            varying_tri: Matrix4x3::<f32>::zeros(),
            uniform_M: uniform_m,
            uniform_MIT: inv_matrix.transpose(),
            ndc_tri: Matrix3::<f32>::zeros()
        }
    }

    pub fn vertex(&mut self, model: &Model, transformation: SMatrix<f32, 4, 4>, iface: usize, nthvert: usize) -> SVector<f32, 4> {
        self.varying_uv.set_column(nthvert, &model.uv(iface, nthvert));
        self.varying_nrm.set_column(nthvert,
            &proj4_3(m2v(self.uniform_MIT * v2m(model.uv_normal(iface, nthvert))))
        );
        let mut gl_vertex: SMatrix<f32, 4, 1> = v2m(model.verts[model.faces[iface][nthvert] as usize]);
        gl_vertex = transformation * gl_vertex;
        self.varying_tri.set_column(nthvert, &gl_vertex);
        self.ndc_tri.set_column(nthvert, &proj4_3(m2v_floor(gl_vertex)));
        return m2v_floor(gl_vertex);
    }

    pub fn fragment(&self, model: &Model, bar: SVector<f32, 3>, base_color: Rgb<u8>) -> (bool, Rgb<u8>) {
        let bn:SMatrix<f32, 3, 1> = (self.varying_nrm * bar).normalize();
        let uvw: SMatrix<f32, 3, 1> = self.varying_uv * bar;

        let A: SMatrix<f32, 3, 3> = SMatrix::from_rows(&[
            (self.ndc_tri.column(1) - self.ndc_tri.column(0)).transpose(),
            (self.ndc_tri.column(2) - self.ndc_tri.column(0)).transpose(),
            bn.transpose()
        ]);

        let AI: SMatrix<f32, 3, 3> = A.try_inverse().unwrap();

        let i: SVector<f32, 3> = AI * Vector3::new(
            self.varying_uv[(0, 1)] - self.varying_uv[(0, 0)],
            self.varying_uv[(0, 2)] - self.varying_uv[(0, 0)],
            0.,
        );

        let j: SVector<f32, 3> = AI * Vector3::new(
            self.varying_uv[(1, 1)] - self.varying_uv[(1, 0)],
            self.varying_uv[(1, 2)] - self.varying_uv[(1, 0)],
            0.,
        );

        let B: SMatrix<f32, 3, 3> = SMatrix::from_columns(&[
            i.normalize(),
            j.normalize(),
            bn
        ]);

        let n: SVector<f32, 3> = (B * model.normal(uvw)).normalize();
        let l: SVector<f32, 4> = m2v(self.uniform_M * v2m(LIGHT_DIR));
        let l_norm: SVector<f32, 3> = proj4_3(l).normalize();
        let r: SVector<f32, 3> = (2.*n*(n.dot(&l_norm)) - l_norm).normalize();

        let spec: f32 = f32::powf(f32::max(r.z, 0.), model.specular(uvw));
        let diffuse: f32 = f32::max(0., n.dot(&l_norm));

        let mut color: Rgb<u8> = model.diffuse(uvw);
        color = Rgb([
            (5. + color.0[0] as f32 * (diffuse + 0.3 * spec)) as u8,
            (5. + color.0[1] as f32 * (diffuse + 0.3 * spec)) as u8,
            (5. + color.0[2] as f32 * (diffuse + 0.3 * spec)) as u8,
        ]);
        return (false, color)
    }

}

pub enum AnyShader {
    Shader(Shader)
}

impl From<Shader> for AnyShader {
    fn from(shader: Shader) -> Self {
        AnyShader::Shader(shader)
    }
}

impl AnyShader {
    pub fn vertex(&mut self, model: &Model, transformation: SMatrix<f32, 4, 4>, iface: usize, nthvert: usize) -> SVector<f32, 4> {
        match self {
            AnyShader::Shader(f) => f.vertex(model, transformation, iface, nthvert),
        }
    }

    pub fn fragment(&self, model: &Model, bar: SVector<f32, 3>, base_color: Rgb<u8>) -> (bool, Rgb<u8>) {
        match self {
            AnyShader::Shader(f) => f.fragment(model, bar, base_color),
        }
    }
}