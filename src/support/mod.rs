#![allow(dead_code)]

// extern crate genmesh;
extern crate obj;

use std::thread;
use std::time::{Duration, Instant};

// use glium;
// use glium::Display;
// use glium::vertex::VertexBuffer;

pub mod camera;
pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F) where F: FnMut() -> Action {
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            // if you have a game, update the state here
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}


// use ::internal::Vertex;

///// Returns a vertex buffer that should be rendered as `TrianglesList`.
// pub fn load_wavefront(display: &Display, data: &[u8]) -> VertexBuffer<Vertex> {



//     let mut data = ::std::io::BufReader::new(data);
//     let data = obj::Obj::load(&mut data);

//     let mut vertex_data = Vec::new();

//     for object in data.object_iter() {
//         for shape in object.group_iter().flat_map(|g| g.indices().iter()) {
//             //println!("{:#?}", shape);
//             match shape {
//                 &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
//                     for v in [v1, v2, v3].iter() {
//                         let position = data.position()[v.0];
//                         let texture = v.1.map(|index| data.texture()[index]);
//                         let normal = v.2.map(|index| data.normal()[index]);

//                         let texture = texture.unwrap_or([0.0, 0.0]);
//                         let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

//                         vertex_data.push(Vertex {
//                             position: position,
//                             normal: normal,
//                             tex_coords: texture,
//                         })
//                     }
//                 },
//                 _ => unimplemented!()
//             }
//         }
//     }

//     println!("{:#?}", vertex_data);

//     glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap() // .into_vertex_buffer_any()
// }
