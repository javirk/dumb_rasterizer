use std::fs::File;
use std::env;
use std::io::{BufReader, prelude::*, Error};

type Result<T> = std::result::Result<T, Error>;

pub struct Model {
    pub nfaces: i32,
    pub nverts: i32,
    pub faces: Vec<Vec<i32>>,
    pub verts: Vec<Vec<f32>>
}

//https://blog.logrocket.com/fundamentals-for-using-structs-in-rust/

impl Model {
    pub fn from_file(filename: &str) -> Result<(Self)> {
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
            match l.chars().next() {
                Some('v') => Model::add_vertex_from_line(&mut model, &l),
                Some('f') => Model::add_face_from_line(&mut model, &l),
                Some(_) => println!("Other"),
                None => println!("Empty")
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