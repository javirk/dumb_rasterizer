use std::fs::File;
use std::io::{BufReader, prelude::*, Error};
use nalgebra::{SVector, Vector3};
use image::{Pixel, Rgb, ImageBuffer};


type Result<T> = std::result::Result<T, Error>;

pub struct Model {
    // TODO: use index and vertex buffers
    pub nfaces: i32,
    pub nverts: i32,
    pub faces: Vec<Vec<i32>>,
    pub faces_diffuse_coords: Vec<Vec<i32>>,
    pub faces_normal_coords: Vec<Vec<i32>>,
    pub verts: Vec<SVector<f32, 3>>,
    pub uv_: Vec<SVector<f32, 3>>,
    pub norms: Vec<SVector<f32, 3>>,
    pub diffuse_map: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    pub normal_map: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    pub specular_map: ImageBuffer<image::Rgb<u8>, Vec<u8>>
}

impl Model {
    pub fn from_file(obj_file: &str, diffuse_file: &str, normal_file: &str, specular_file: &str) -> Result<Self> {
        let file = File::open(obj_file)?;//.expect("file not found!");
        let diffuse_map = image::open(diffuse_file).unwrap().to_rgb8();
        let normal_map = image::open(normal_file).unwrap().to_rgb8();
        let specular_map = image::open(specular_file).unwrap().to_rgb8();

        let mut model = Model {
            nfaces: 0,
            nverts: 0,
            faces: Vec::new(),
            faces_diffuse_coords: Vec::new(),
            faces_normal_coords: Vec::new(),
            verts: Vec::new(),
            uv_: Vec::new(),
            norms: Vec::new(),
            diffuse_map: diffuse_map,
            normal_map: normal_map,
            specular_map: specular_map
        };

        let buf_reader = BufReader::new(file);
      
        for line in buf_reader.lines() {
            let l = line?;
            if l.len() == 0 {
                continue;
            }
            match &l[..2] {
                "v " => Model::add_line_float_vector(&mut model.verts, &l),
                "f " => Model::add_face_from_line(&mut model, &l),
                "vt" => Model::add_line_float_vector(&mut model.uv_, &l),
                "vn" => Model::add_line_float_vector(&mut model.norms, &l),
                _ => (),
            }
        }
        
        model.nverts = model.verts.len() as i32;
        
        assert_eq!(model.faces.len() as i32, model.nfaces);
        Ok(model)
    }

    fn add_line_float_vector(vec_to_append: &mut Vec<SVector<f32, 3>>, line: &str) {
        let mut vector = Vec::new();
        let line_vec = trim_whitespace(line);
        for value in line_vec {
            match value == "v" || value == "vt" || value == "vn" {
                false => vector.push(value.parse::<f32>().unwrap()),
                true => ()
            }
        }

        let vertex: SVector<f32, 3> = Vector3::new(vector[0], vector[1], vector[2]);

        vec_to_append.push(vertex);
    }

    fn add_face_from_line(&mut self, face_line: &str) {
        let mut face: Vec<i32> = Vec::new();
        let mut face_texture: Vec<i32> = Vec::new();
        let mut face_normal: Vec<i32> = Vec::new();
        let mut vertex_info: Vec<&str>;
        let mut vertex_num: i32;
        let mut texture_num: i32;
        let mut normal_num: i32;
        for value in face_line.split(" ") {
            match value == "f" {
                false => {
                    vertex_info = value.split("/").collect();
                    vertex_num = (vertex_info[0].parse::<i32>().unwrap()) - 1;
                    texture_num = (vertex_info[1].parse::<i32>().unwrap()) - 1;
                    normal_num = (vertex_info[2].parse::<i32>().unwrap()) - 1;
                    face.push(vertex_num);
                    face_texture.push(texture_num);
                    face_normal.push(normal_num);
                },
                true => ()
            }
        }
        self.faces.push(face);
        self.faces_diffuse_coords.push(face_texture);
        self.faces_normal_coords.push(face_normal);
        self.nfaces += 1;
    }

    pub fn uv_normal(&self, iface: usize, nthvert: usize) -> SVector<f32, 3> {
        // Normals should be taken from the normal map, just like the diffuse
        let idx: i32 = self.faces_normal_coords[iface][nthvert];
        return self.norms[idx as usize]
    }

    pub fn normal(&self, uvw: SVector<f32, 3>) -> SVector<f32, 3> {
        let c = self.normal_map
            .get_pixel(
                (uvw[0] * (self.normal_map.width() as f32)) as u32,
                ((1. - uvw[1]) * (self.normal_map.height() as f32)) as u32,
            )
            .to_rgb().0;
        
        let n: SVector<f32, 3> = Vector3::new(
            c[2] as f32/255.*2. - 1.,
            c[1] as f32/255.*2. - 1.,
            c[0] as f32/255.*2. - 1.,
        );
        return n;
    }

    pub fn diffuse(&self, uvw: SVector<f32, 3>) -> Rgb<u8> {
        // Discard w coord and reverse one of the dimensions
        let pixel_color = self.diffuse_map
            .get_pixel(
                (uvw[0] * (self.diffuse_map.width() as f32)) as u32,
                ((1. - uvw[1]) * (self.diffuse_map.height() as f32)) as u32,
            )
            .to_rgb();
        return pixel_color;
    }

    pub fn specular(&self, uvw: SVector<f32, 3>) -> f32 {
        let s: u8 = self.specular_map
            .get_pixel(
                (uvw[0] * (self.specular_map.width() as f32)) as u32,
                ((1. - uvw[1]) * (self.specular_map.height() as f32)) as u32,
            )
            .to_rgb().0[0];
        
        return s as f32;
    }

    pub fn uv(&self, iface: usize, nthvert: usize) -> SVector<f32, 3> {
        return self.uv_[self.faces_diffuse_coords[iface][nthvert] as usize]
    }
}

pub fn trim_whitespace(s: &str) -> Vec<&str> {
    let words: Vec<&str> = s.split_whitespace().collect();
    return words;
}