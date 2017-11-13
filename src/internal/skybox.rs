use ::internal::{ Vertex, runtime_readbytes };

use image;

use glium;
use glium::{ VertexBuffer, IndexBuffer, Program };

use std::io::Cursor;

pub struct Skybox {
    pub image_paths: Vec<String>,
    pub textures: Vec<glium::Texture2d>,
    pub vb: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u8>,
    pub program: Program,
    pub cubemap: glium::texture::Cubemap,
    pub distance: f32
}

impl Skybox {
    // FIXME
    pub fn new(dist: f32, display_ref: &glium::Display) -> Skybox {
        let paths: Vec<String> = vec!["posx.jpg".into(), "negx.jpg".into(),
                                      "posy.jpg".into(), "negy.jpg".into(),
                                      "posz.jpg".into(), "negz.jpg".into()];

        let paths2 = paths.clone();

        let textures = paths2.into_iter().map(|path| {
            let image = image::load(Cursor::new(&runtime_readbytes(
                &format!("../resources/textures/skybox/orig/{}", path)
            )[..]),
                                    image::JPEG).unwrap().to_rgba();
            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
            glium::Texture2d::new(display_ref, image).unwrap()
        }).collect();

        let vertices = [
            // front
            Vertex::with_position_only([-dist, -dist, dist]),
            Vertex::with_position_only([ dist, -dist, dist]),
            Vertex::with_position_only([ dist,  dist, dist]),
            Vertex::with_position_only([-dist,  dist, dist]),

            // r
            Vertex::with_position_only([ dist, -dist,  dist]),
            Vertex::with_position_only([ dist, -dist, -dist]),
            Vertex::with_position_only([ dist,  dist, -dist]),
            Vertex::with_position_only([ dist,  dist,  dist]),

            // back
            Vertex::with_position_only([-dist, -dist, -dist]),
            Vertex::with_position_only([-dist,  dist, -dist]),
            Vertex::with_position_only([ dist,  dist, -dist]),
            Vertex::with_position_only([ dist, -dist, -dist]),

            // l
            Vertex::with_position_only([ -dist, -dist,  dist]),
            Vertex::with_position_only([ -dist,  dist,  dist]),
            Vertex::with_position_only([ -dist,  dist, -dist]),
            Vertex::with_position_only([ -dist, -dist, -dist]),

            // bottom
            Vertex::with_position_only([ -dist, -dist,  dist]),
            Vertex::with_position_only([ -dist, -dist, -dist]),
            Vertex::with_position_only([  dist, -dist, -dist]),
            Vertex::with_position_only([  dist, -dist,  dist]),

            // top
            Vertex::with_position_only([ -dist, dist,  dist]),
            Vertex::with_position_only([  dist, dist,  dist]),
            Vertex::with_position_only([  dist, dist, -dist]),
            Vertex::with_position_only([ -dist, dist, -dist]),
        ];

        let program = glium::Program::from_source(
            display_ref,
//         " #version 140

//             in vec3 position;
//             out vec3 ReflectDir;

//             uniform mat4 model;
//             uniform mat4 view;
//             uniform mat4 perspective;

//             void main() {
//                 ReflectDir = position;
//                 gl_Position = perspective * view * model * vec4(position, 1.0);
//             }
//         ",
//         " #version 140
//             in vec3 ReflectDir;
//             out vec4 color;

//             uniform samplerCube cubetex;

//             void main() {
// // reflect_path = dot(ReflectDir,
//                 color = texture(cubetex, ReflectDir);
//             }
            //         ",
            r#"#version 400

in vec3 position;
out vec3 textureCoords;

uniform mat4 perspective; // projectionMatrix;
uniform mat4 view; // viewMatrix;

void main(void){

	gl_Position = perspective * view * vec4(position, 1.0);
	textureCoords = position;

}"#,
            "
#version 400

in vec3 textureCoords;
out vec4 out_Color;

uniform samplerCube cubetex;

void main(void){
    out_Color = texture(cubetex, textureCoords);
}
",
        None).unwrap();

        let indices = glium::IndexBuffer::new(display_ref,
            glium::index::PrimitiveType::TrianglesList,
            &[
                // Front
                0u8, 2, 1, 0, 3, 2,
                // Right
                4, 6, 5, 4, 7, 6,
                // Back
                8, 10, 9, 8, 11, 10,
                // Left
                12, 14, 13, 12, 15, 14,
                // Bottom
                16, 18, 17, 16, 19, 18,
                // Top
                20, 22, 21, 20, 23, 22,
            ]).unwrap();

        Skybox {
            image_paths: paths,
            vb: glium::VertexBuffer::new(display_ref, &vertices).unwrap(),
            program: program,
            indices: indices,
            cubemap: glium::texture::Cubemap::empty(display_ref, 2048).unwrap(),
            distance: dist,
            textures: textures
        }
    }
}
