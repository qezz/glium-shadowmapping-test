extern crate glium;
use glium::{ Display, Program };

use ::internal::Shaders;

pub fn program_from_shader_paths(display: &Display,
                                 vert_shader_path: &str,
                                 frag_shader_path: &str) -> glium::Program {
    let shaders = Shaders::new(vert_shader_path, frag_shader_path);

    glium::Program::from_source(display, &shaders.vert, &shaders.frag, None).unwrap()
}
