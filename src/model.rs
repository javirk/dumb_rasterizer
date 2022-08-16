use std::fs::File;
use std::env;
use std::io::{BufReader, prelude::*, Error};
use nalgebra::{SVector, Vector3};

type Result<T> = std::result::Result<T, Error>;

// TODO: re-implement Vec to have always three components and with names x, y, z
pub struct Model {
    pub nfaces: i32,
    pub nverts: i32,
    pub faces: Vec<Vec<i32>>,
    pub faces_texture_coords: Vec<Vec<i32>>,
    pub verts: Vec<SVector<f32, 3>>,
    pub verts_texture: Vec<SVector<f32, 3>>,
}

impl Model {
    pub fn from_file(filename: &str) -> Result<Self> {
        let mut model = Model {
            nfaces: 0,
            nverts: 0,
            faces: Vec::new(),
            faces_texture_coords: Vec::new(),
            verts: Vec::new(),
            verts_texture: Vec::new()
        };
        let path = env::current_dir()?;
        let file = File::open(filename)?;//.expect("file not found!");
        let buf_reader = BufReader::new(file);
      
        for line in buf_reader.lines() {
            let l = line?;
            if l.len() == 0 {
                continue;
            }
            match &l[..2] {
                "v " => Model::add_line_float_vector(&mut model.verts, &l),
                "f " => Model::add_face_from_line(&mut model, &l),
                "vt" => Model::add_line_float_vector(&mut model.verts_texture, &l),
                _ => (), // There are vn lines that we are not interested in yet
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
            match value == "v" || value == "vt" {
                false => vector.push(value.parse::<f32>().unwrap()),
                true => ()
            }
        }

        let vertex: SVector<f32, 3> = Vector3::new(vector[0], vector[1], vector[2]);

        vec_to_append.push(vertex);
    }

    fn add_face_from_line(&mut self, face_line: &str) {
        let mut face = Vec::new();
        let mut face_texture = Vec::new();
        let mut vertex_info: Vec<&str>;
        let mut vertex_num: i32;
        let mut texture_num: i32;
        for value in face_line.split(" ") {
            match value == "f" {
                false => {
                    vertex_info = value.split("/").collect();
                    vertex_num = (vertex_info[0].parse::<i32>().unwrap()) - 1;
                    texture_num = (vertex_info[1].parse::<i32>().unwrap()) - 1;
                    face.push(vertex_num);
                    face_texture.push(texture_num);
                },
                true => ()
            }
        }
        self.faces.push(face);
        self.faces_texture_coords.push(face_texture);
        self.nfaces += 1;  // I don't know if I like this here, it's maybe too many memory access
    }
}

pub fn trim_whitespace(s: &str) -> Vec<&str> {
    let words: Vec<&str> = s.split_whitespace().collect();
    return words;
}