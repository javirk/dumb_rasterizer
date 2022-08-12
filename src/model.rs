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
    pub verts: Vec<SVector<f32, 3>>
}

impl Model {
    pub fn from_file(filename: &str) -> Result<Self> {
        let mut model = Model {
            nfaces: 0,
            nverts: 0,
            faces: Vec::new(),
            verts: Vec::new()
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
                "v " => Model::add_vertex_from_line(&mut model, &l),
                "f " => Model::add_face_from_line(&mut model, &l),
                _ => (), // There are vt and vn lines that we are not interested in yet
            }
        }
        assert_eq!(model.verts.len() as i32, model.nverts);
        assert_eq!(model.faces.len() as i32, model.nfaces);
        Ok(model)
    }

    fn add_vertex_from_line(&mut self, vertex_line: &str) {
        let mut vertex = Vec::new();
        for value in vertex_line.split(" ") {
            match value == "v" {
                false => vertex.push(value.parse::<f32>().unwrap()),
                true => ()
            }
        }

        let vertex = Vector3::new(vertex[0], vertex[1], vertex[2]);

        self.verts.push(vertex);
        self.nverts += 1;  // I don't know if I like this here, it's maybe too many memory access
    }

    fn add_face_from_line(&mut self, face_line: &str) {
        let mut face = Vec::new();
        for value in face_line.split(" ") {
            match value == "f" {
                false => {
                    let face_nums: Vec<&str> = value.split("/").collect();
                    let face_num = (face_nums[0].parse::<i32>().unwrap()) - 1;
                    face.push(face_num);
                },
                true => ()
            }
        }
        self.faces.push(face);
        self.nfaces += 1;  // I don't know if I like this here, it's maybe too many memory access
    }
}