#![feature(slice_patterns)]

#[macro_use]
extern crate glium;
extern crate image;

#[cfg(feature = "genmesh")]
extern crate genmesh;
extern crate obj;

#[cfg(feature = "genmesh")]
use obj::{Obj, SimplePolygon};
use std::path::Path;

extern crate wavefront_obj;
use wavefront_obj::obj::parse;

use glium::{glutin, Surface};

use std::io::Cursor;

mod internals;
use internals::{ load_shader, Vertex, piston_parse, runtime_readbytes,
                 load_wavefront };

mod support;
use support::Action;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

//     // #[derive(Copy, Clone)]
//     // struct Vertex {
//     //     position: [f32; 3],
//     //     normal: [f32; 3],
//     //     tex_coords: [f32; 2],
//     // }

//     // implement_vertex!(Vertex, position, normal, tex_coords);

//     let test_tex_cube2 = glium::vertex::VertexBuffer::new(&display, &[
//         Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.25, 0.75] },
//         Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.5, 0.75] },
//         Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.25, 0.5] },
//         Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.5, 0.5] },

//         Vertex { position: [ -1.0,  -1.0, -2.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.25, 0.25] },
//         Vertex { position: [ 1.0,  -1.0, -2.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.5, 0.25] },
// //        Vertex { position: [ -1.0, -1.0, -0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.25, 0.5] },
// //        Vertex { position: [ 1.0,  -1.0, -0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.5, 0.5] },

//         Vertex { position: [ -1.0,  1.0, -2.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.5, 0.75] },
//         Vertex { position: [ 1.0,  1.0, -2.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.5, 0.75] },

//         Vertex { position: [ -1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.25, 0.75] },
//         Vertex { position: [ 1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.5, 0.75] },
//     ]).unwrap();

//     let test_cube_bytes = runtime_readbytes("../resources/plain_cube.obj");
//     // let test_cube_bytes = runtime_readbytes("../resources/cube.obj");
//     // let test_tex_cube = glium::vertex::VertexBuffer::new(&display,
//     //                                                      &load_wavefront("../resources/plain_cube.obj")[..]).unwrap();
//     let test_tex_cube = load_wavefront(&display, &test_cube_bytes);

//     let image = image::load(Cursor::new(
//         // &include_bytes!("../resources/textures/cube.png")[..]
//         &runtime_readbytes("../resources/textures/cube.png")[..]
//     ),
//                             image::PNG).unwrap().to_rgba();
//     let image_dimensions = image.dimensions();
//     let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
//     let cube_texture = glium::texture::Texture2d::new(&display, image).unwrap(); // glium::texture::SrgbTexture2d::new(&display, image).

//     let image = image::load(Cursor::new(&runtime_readbytes("../resources/wall-normal.png")[..]),
//                             image::PNG).unwrap().to_rgba();
//     let image_dimensions = image.dimensions();
//     let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
//     let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();

    let mut vertex_shader_src: String = load_shader("shaders/lightstest.vert");
    let mut fragment_shader_src: String = load_shader("shaders/lightstest.frag");
    let mut program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src,
                                              None).unwrap();

//     let mut camera = support::camera::CameraState::new(0.1);

//     support::start_loop(|| {
//         camera.update();

//         let mut target = display.draw();
//         target.clear_color_and_depth((0.9, 0.9, 0.9, 1.0), 1.0);

//         let model = [
//             [1.0, 0.0, 0.0, 0.0],
//             [0.0, 1.0, 0.0, 0.0],
//             [0.0, 0.0, 1.0, 0.0],
//             [0.0, 0.0, 0.0, 1.0f32]
//         ];

//         let light = [1.0, -1.0, 0.10f32];

//         let params = glium::DrawParameters {
//             depth: glium::Depth {
//                 test: glium::DepthTest::IfLess, // glium::draw_parameters::DepthTest::IfLess,
//                 write: true,
//                 .. Default::default()
//             },
//             polygon_mode: glium::PolygonMode::Line,
//             .. Default::default()
//         };

//         target.draw(&test_tex_cube, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
//                     &uniform!{ model: model, view: camera.get_view(),
//                                // cameraPosition: camera.get_view(),
//                                perspective: camera.get_perspective(),
//                                u_light: light,
//                                light: light,
//                                diffuse_tex: &cube_texture, normal_tex: &normal_map,
//                                light_position: light,
//                                light_intensities: [0.0, 0.9, 0.0f32],
//                                light_attenuation: 0.7f32,
//                                light_ambientCoefficient: 0.8f32,

//                                specular_color: [0.0, 0.9, 0.0f32],

//                                persp_matrix: camera.get_perspective(),
//                                view_matrix: camera.get_view(),
//                     },
//                     &params).unwrap();
//         target.draw(&test_tex_cube2, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
//                     &uniform!{ model: model, view: camera.get_view(),
//                                // cameraPosition: camera.get_view(),
//                                perspective: camera.get_perspective(),
//                                u_light: light,
//                                light: light,
//                                diffuse_tex: &cube_texture, normal_tex: &normal_map,
//                                light_position: light,
//                                light_intensities: [0.0, 0.9, 0.0f32],
//                                light_attenuation: 0.7f32,
//                                light_ambientCoefficient: 0.8f32,

//                                specular_color: [0.0, 0.9, 0.0f32],

//                                persp_matrix: camera.get_perspective(),
//                                view_matrix: camera.get_view(),
//                     },
//                     &params).unwrap();

//         target.finish().unwrap();

//         let mut action = support::Action::Continue;

//         events_loop.poll_events(|event| {
//             match event {
//                 glutin::Event::WindowEvent { event, .. } => match event {
//                     glutin::WindowEvent::Closed => action = support::Action::Stop,
//                     // #\Space to reload shaders
//                     glutin::WindowEvent::KeyboardInput { input, ..} if input.virtual_keycode == Some(glutin::VirtualKeyCode::Space) => {
//                         vertex_shader_src = load_shader("shaders/prev.vert");
//                         fragment_shader_src = load_shader("shaders/prev.frag");
//                         program = glium::Program::from_source(&display,& vertex_shader_src, &fragment_shader_src,
//                                                               None).unwrap();
//                     }
//                     glutin::WindowEvent::KeyboardInput { input, ..} if input.virtual_keycode == Some(glutin::VirtualKeyCode::Escape) => {
//                         action = Action::Stop;
//                     }
//                     ev => camera.process_input(&ev),
//                 },
//                 _ => (),
//             }
//         });

//         action
//     });
// }


// // fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
// //     let f = {
// //         let f = direction;
// //         let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
// //         let len = len.sqrt();
// //         [f[0] / len, f[1] / len, f[2] / len]
// //     };

// //     let s = [up[1] * f[2] - up[2] * f[1],
// //              up[2] * f[0] - up[0] * f[2],
// //              up[0] * f[1] - up[1] * f[0]];

// //     let s_norm = {
// //         let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
// //         let len = len.sqrt();
// //         [s[0] / len, s[1] / len, s[2] / len]
// //     };

// //     let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
// //              f[2] * s_norm[0] - f[0] * s_norm[2],
// //              f[0] * s_norm[1] - f[1] * s_norm[0]];

// //     let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
// //              -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
// //              -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

// //     [
// //         [s_norm[0], u[0], f[0], 0.0],
// //         [s_norm[1], u[1], f[1], 0.0],
// //         [s_norm[2], u[2], f[2], 0.0],
// //         [p[0], p[1], p[2], 1.0],
// //     ]
// // }




// #[macro_use]
// extern crate glium;

// use glium::{glutin, Surface};
// use glium::vertex::IntoVerticesSource;

// mod support;
// mod internals;

// use internals::{Vertex, ShapeTransitionIterator, dist};


// #![feature(slice_patterns)]

// #[macro_use]
// extern crate glium;
// extern crate image;

// #[cfg(feature = "genmesh")]
// extern crate genmesh;
// extern crate obj;

// #[cfg(feature = "genmesh")]
// use obj::{Obj, SimplePolygon};
// use std::path::Path;

// extern crate wavefront_obj;
// use wavefront_obj::obj::parse;



// use std::io::Cursor;

// mod internals;
// use internals::{ load_shader, Vertex, piston_parse, runtime_readbytes,
//                  load_wavefront };

// mod support;
// use support::Action;

// use glium::{glutin, Surface};


// fn main() {

//     let mut events_loop = glutin::EventsLoop::new();
//     let window = glutin::WindowBuilder::new();
//     let context = glutin::ContextBuilder::new().with_depth_buffer(24);
//     let display = glium::Display::new(window, context, &events_loop).unwrap();

//     // building the vertex and index buffers
//     // let vertex_buffer = support::load_wavefront(&display,
//     //                                             include_bytes!("support/teapot.obj"));

//     let shape = glium::vertex::VertexBuffer::new(&display, &[
//         Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
//         Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
//         Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
//         Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
//     ]).unwrap();

//     // let shape2 = support::load_wavefront(&display, include_bytes!("../resources/cube.obj");

//     // let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &[
//     //     Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0] },
//     //     Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0] },
//     //     Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0] },
//     //     Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0] },
//     // ]).unwrap();

//     let cube_vertices = include_bytes!("../resources/plain_cube.obj");
//     // let sphere_vertices = include_bytes!("../resources/sus.obj");
//     let sphere_vertices: Vec<u8> = vec![]; // include_bytes!("../resources/uvsphere.obj");

//     let vertex_buffer = load_wavefront(&display, cube_vertices);
//     //let uvsphere = support::load_wavefront(&display, sphere_vertices);
//     //let mut sh = support::load_wavefront(&display, cube_vertices);


//     let program = program!(&display,
//         140 => {
//             vertex: "
//                 #version 140

//                 uniform mat4 persp_matrix;
//                 uniform mat4 view_matrix;

//                 in vec3 position;
//                 in vec3 normal;
//                 out vec3 v_position;
//                 out vec3 v_normal;

//                 void main() {
//                     v_position = position;
//                     v_normal = normal;
//                     gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
//                 }
//             ",

//             fragment: "
//                 #version 140

//                 in vec3 v_normal;
//                 out vec4 f_color;

//                 const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

//                 void main() {
//                     float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
//                     vec3 color = (0.3 + 0.7 * lum) * vec3(0.1, 0.1, 0.1); // vec3(1.0, 1.0, 1.0);
//                     f_color = vec4(color, 1.0);
//                 }
//             ",
//         },
//     ).unwrap();

//     let mut camera = support::camera::CameraState::new(0.1);

//     // support::start_loop(|| {
//     //     camera.update();

//     //     let params = glium::DrawParameters {
//     //         depth: glium::Depth {
//     //             test: glium::DepthTest::IfLess,
//     //             write: true,
//     //             .. Default::default()
//     //         },
//     //         // polygon_mode: glium::PolygonMode::Line,
//     //         .. Default::default()
//     //     };

//     //     let mut target = display.draw();
//     //     target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
//     //     target.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
//     //                 &uniform! { model: model, view: view, perspective: perspective, u_light: light },
//     //                 &params).unwrap();

//     // }


//    //let mut transition_iter = ShapeTransitionIterator::new(&vertex_buffer, &uvsphere, 8);
// //    let everything: Ve = transition_iter.collect();

//     // the main loop
//     support::start_loop(|| {
//         camera.update();

//         // building the uniforms
//         let uniforms = uniform! {
//             persp_matrix: camera.get_perspective(),
//             view_matrix: camera.get_view(),
//         };

//         // draw parameters
//         let params = glium::DrawParameters {
//             depth: glium::Depth {
//                 test: glium::DepthTest::IfLess,
//                 write: true,
//                 .. Default::default()
//             },
//             // polygon_mode: glium::PolygonMode::Point,
//             polygon_mode: glium::PolygonMode::Line,
//             .. Default::default()
//         };

//         // drawing a frame
//         let mut target = display.draw();
//         target.clear_color_and_depth((200.0 / 255.0,
//                                       197.0 / 255.0,
//                                       200.0 / 255.0, 0.0), 1.0);
//         target.draw(&vertex_buffer,
//                     &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
//                     &program, &uniforms, &params).unwrap();

//         // target.draw(&uvsphere,
//         //             &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
//         //             &program, &uniforms, &params).unwrap();

//         // target.draw(&sh,
//         //             &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
//         //             &program, &uniforms, &params).unwrap();


//         target.finish().unwrap();

//         let mut action = support::Action::Continue;

//         // polling and handling the events received by the window
//         events_loop.poll_events(|event| {

//             // let input = match *event {
//             //     glutin::WindowEvent::KeyboardInput { input, .. } => input,
//             //     _ => return,
//             // };
//             // let pressed = input.state == glutin::ElementState::Pressed;
//             // let key = match input.virtual_keycode {
//             //     Some(key) => key,
//             //     None => return,
//             // };

//             match event {
//                 glutin::Event::WindowEvent { event, .. } => match event {
//                     glutin::WindowEvent::Closed => action = support::Action::Stop,
//                     glutin::WindowEvent::KeyboardInput { input, ..} if input.virtual_keycode == Some(glutin::VirtualKeyCode::Space) => {
//                         //sh = glium::vertex::VertexBuffer::new(&display,
//                         // &((transition_iter.next()).unwrap())).unwrap();
//                     }
//                     ev => camera.process_input(&ev),
//                 },
//                 _ => (),
//             }
//         });

//         action
//     });

//     // let derefed = &*shape;
//     // // println!("derefed: {:#?}",
//     // for v in shape.read().unwrap() {
//     //     println!("v: {:?}", v);
//     // }

//     // println!("vb/bindings: {:?}", vertex_buffer.get_bindings());

//     // // println!("{:?}", vertex_buffer.into_vertices_source());

//     // for v in vertex_buffer.read().unwrap() {
//     //     println!("v: {:?}", v);
//     // }


//     // //    println!("{:#?}", vertex_buffer.read());
//     // for v1 in vertex_buffer.read().unwrap().iter() {

//     //     let mut min_dist = std::f32::MAX;

//     //     for v2 in uvsphere.read().unwrap().iter() {

//     //         let d = dist(v1, v2);

//     //         if d < min_dist {
//     //             min_dist = d
//     //         }
//     //     }
//     //     println!("dist: {}", min_dist);

//     // }
}
