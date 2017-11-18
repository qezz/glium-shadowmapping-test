
extern crate glium;
use glium::Display;

use image;
use glium::Texture2d;
use std::io::Cursor;

pub mod skybox;
pub mod program;

pub trait Reload {
    fn reload(&mut self);
}

pub struct Shaders {
    pub vert: String,
    pub frag: String,
    pub geom: Option<String>,

    vert_path: String,
    frag_path: String,
    geom_path: Option<String>,
}

impl Shaders {
    pub fn new(vert_path: &str, frag_path: &str) -> Shaders {
        Shaders {
            vert: load_shader(vert_path),
            frag: load_shader(frag_path),
            geom: None,

            vert_path: vert_path.into(),
            frag_path: frag_path.into(),
            geom_path: None,
        }
    }
}

impl Reload for Shaders {
    fn reload(&mut self) {
        self.vert = load_shader(&self.vert_path);
        self.frag = load_shader(&self.frag_path);
        if let Some(ref path) = self.geom_path {
            self.geom = Some(load_shader(&path))
        }
    }
}

#[allow(dead_code)]
pub fn load_png_texture(display: &glium::backend::Facade, path: &str) -> glium::Texture2d {
    let image = image::load(Cursor::new(&runtime_readbytes(path)[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_texture = glium::texture::Texture2d::new(display, image).unwrap(); // glium::texture::SrgbTexture2d::new(&display, image).

    diffuse_texture
}

pub fn load_jpg_texture(display: &glium::backend::Facade, path: &str) -> glium::texture::SrgbTexture2d {
    let image = image::load(Cursor::new(&runtime_readbytes(path)[..]),
                            image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

    diffuse_texture
}

#[cfg(feature = "genmesh")]
extern crate genmesh;

extern crate obj;

use glium::vertex::VertexBuffer;

use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);

impl Vertex {
    pub fn with_position_only(pos: [f32; 3]) -> Vertex {
        Vertex {
            position: pos,
            normal: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0]
        }
    }
}

pub fn load_shader(path: &str) -> String {
    let mut f = match File::open(path) {
        Ok(res) => res,
        Err(e) => panic!("not found: {}: {:#?}", path, e)
    };
    let mut string = String::new();
    let _ = f.read_to_string(&mut string);
    string
}

extern crate wavefront_obj;
use wavefront_obj::obj::ObjSet;

#[allow(dead_code)]
pub fn piston_parse(path: &str) -> ObjSet {
    let mut f = match File::open(path) {
        Ok(res) => res,
        Err(e) => panic!("not found: {}: {:#?}", path, e)
    };
    let mut string = String::new();
    let _ = f.read_to_string(&mut string);

    wavefront_obj::obj::parse(string.to_owned()).unwrap()
}

pub fn runtime_readbytes(path: &str) -> Vec<u8> {

    let mut f = File::open(path).unwrap();
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).unwrap();

    buffer.to_owned()
}

use obj::SimplePolygon;



pub fn load_wavefront(display: &Display, data: &[u8]) -> VertexBuffer<Vertex> {

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::Obj::<SimplePolygon>::load_buf(&mut data).unwrap();

    let mut vertex_data = Vec::new();

    for object in &data.objects {
        for shape in object.groups.iter().flat_map(|g| g.polys.iter()) {
            // println!("{:?}", shape);
            match &shape.len() {
                // &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
                // &[v1, v2, v3] => {
                &3 => {
                    let v1 = shape[0];
                    let v2 = shape[1];
                    let v3 = shape[2];
                    for v in [v1, v2, v3].iter() {
                        let position = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);

                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        vertex_data.push(Vertex {
                            position: position,
                            normal: normal,
                            tex_coords: texture,
                        })
                    }
                },
                _ => unimplemented!()
            }
        }
    }

    // panic!();

    // println!("{:#?}", vertex_data);

    glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap() // .into_vertex_buffer_any()
}
