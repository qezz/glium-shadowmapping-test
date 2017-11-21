#![feature(slice_patterns)]

#[macro_use]
extern crate glium;
extern crate image;

#[cfg(feature = "genmesh")]
extern crate genmesh;
extern crate obj;

#[cfg(feature = "genmesh")]
use obj::{Obj, SimplePolygon};

extern crate wavefront_obj;
// use wavefront_obj::obj::parse;

use glium::{glutin, Surface};

// use std::io::Cursor;
use std::path::Path;

mod internal;
use internal::{ runtime_readbytes, load_wavefront, load_jpg_texture };
// use internal::skybox::Skybox;
use internal::program::program_from_shader_paths;

mod support;
use support::Action;

extern crate cgmath;
use cgmath::SquareMatrix;

#[allow(non_snake_case)]
fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();


    let mut camera = support::camera::CameraState::new(0.5, 0.04);

    let mut iteration: u32 = 0;

    let shadow_resolution: u32 = 1024;

    let depth_program = program_from_shader_paths(&display,
                                                  "src/shaders/depth_sh.vert",
                                                  "src/shaders/depth_sh.frag");

    let texture = load_jpg_texture(&display, "resources/textures/asphalt.jpg");

    let room_vertices = runtime_readbytes("resources/objects/room_thickwalls.obj");
    let room_vb = load_wavefront(&display, &room_vertices.as_slice());


    let depth_texture =
        glium::texture::DepthTexture2d::empty_with_format(&display,
                                                          glium::texture::DepthFormat::I16,
                                                          glium::texture::MipmapsOption::NoMipmap,
                                                          shadow_resolution, shadow_resolution).unwrap();

     // let mut framebuffer = glium::framebuffer::SimpleFrameBuffer::depth_only(&display, &depth_texture)
     //    .unwrap();

    // let texture_depth_color = glium::texture::Texture2d::empty_with_format(&display, glium::texture::UncompressedFloatFormat::I16I16I16, glium::texture::MipmapsOption::NoMipmap,
    //         shadow_resolution, shadow_resolution).unwrap();

    let mut framebuffer = glium::framebuffer::SimpleFrameBuffer::depth_only(&display, &depth_texture)
        .unwrap();

    // let mut framebuffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &texture_depth_color,
    //                                                                                &depth_texture)
    //     .unwrap();


    let sh_program = program_from_shader_paths(&display,
                                               "src/shaders/sh_shader.vert",
                                               "src/shaders/sh_shader.frag");

    support::start_loop(|| {

        // FIXME: make light to move
        // iteration = (iteration + 1) % 315;
        iteration = 123;

        camera.update();

        let mut target = display.draw();

        let depth_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                //clamp: glium::draw_parameters::DepthClamp::Clamp,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.clear_color_and_depth((200.0 / 255.0,
                                      197.0 / 255.0,
                                      200.0 / 255.0, 0.0), 1.0);
        // target.clear_color(0.0, 0.0, 0.0, 0.0);



        let light = [(iteration as f32 / 10.0).sin() * 14.0,
                     -10.0,
                     (iteration as f32 / 10.0).cos() * 14.0 ];

        let light_trajectory_radius = 10.0;

        let lightInvDir: cgmath::Vector3<f32> = cgmath::vec3(0.5, 2.0, 2.0_f32);
            // cgmath::vec3((iteration as f32 / 10.0).sin() * light_trajectory_radius,
            //              10.0,
            //              (iteration as f32 / 10.0).cos() * light_trajectory_radius);

        let depthProjectionMatrix: cgmath::Matrix4<f32> = cgmath::ortho(-10.0, 10.0, -10.0, 10.0, -10.0, 20.0_f32);

        let depthViewMatrix: cgmath::Matrix4<f32> =
            cgmath::Matrix4::look_at(cgmath::EuclideanSpace::from_vec(lightInvDir),
                                     cgmath::Point3{x: 0.0, y: 0.0, z: 0.0_f32},
                                     cgmath::vec3(0.0, 1.0, 0.0_f32));

        let depthModelMatrix: cgmath::Matrix4<f32> = cgmath::Matrix4::from_value(1.0);

        let depthMVP = depthProjectionMatrix * depthViewMatrix * depthModelMatrix;
        // println!("depthMVP: {:?}", depthMVP);
        // panic!();

        let depth_uniforms = uniform! {
            depthMVP: Into::<[[f32; 4]; 4]>::into(depthMVP)
        };

        // FIXME
        // target.draw(&room_vb,
        //             &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        //             &sh_program,
        //             &sh_uniforms,
        //             &params)
        //     .unwrap();

        framebuffer.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0); // depth should be 1.0
        // framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);
        framebuffer.draw(&room_vb,
                         &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                         &depth_program,
                         &depth_uniforms,
                         &depth_params)
            //&Default::default())
            .unwrap();

        // render room //

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                clamp: glium::draw_parameters::DepthClamp::Clamp,
                .. Default::default()
            },
            // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };


        let projection_matrix: cgmath::Matrix4<f32> = camera.get_perspective().into();
        let view_matrix: cgmath::Matrix4<f32> = camera.get_view().into();
        let model_matrix: cgmath::Matrix4<f32> = cgmath::Matrix4::from_value(1.0);
        let mvp = projection_matrix * view_matrix * model_matrix;

        let bias_matrix: cgmath::Matrix4<f32> = [[ 0.5_f32, 0.0, 0.0, 0.0, ],
                                                 [ 0.0, 0.5, 0.0, 0.0, ],
                                                 [ 0.0, 0.0, 0.5, 0.0, ],
                                                 [ 0.5, 0.5, 0.5, 1.0, ]].into();

        let depth_bias_mvp = bias_matrix * depthMVP;

        // let read_back: Vec<Vec<(u8, u8, u8, u8)>> = depth_texture.read();
        // println!("read_back: {:?}", read_back[0].len());
        // println!("depth tex: {} by {}", depth_texture.get_height().unwrap(), depth_texture.get_width());
        // panic!();


        let sh_uniforms = uniform! {
            MVP: Into::<[[f32; 4]; 4]>::into(mvp),
            DepthBiasMVP:  Into::<[[f32; 4]; 4]>::into(depth_bias_mvp),
            myTextureSampler: &texture,
            shadowMap: depth_texture.sampled()
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
        };

        // depth_texture

        target.draw(&room_vb,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &sh_program,
                    &sh_uniforms,
                    &params)
            .unwrap();


        target.finish().unwrap();

        // panic!();

        let mut action = support::Action::Continue;

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => action = support::Action::Stop,
                    // #\Space to reload shaders
                    glutin::WindowEvent::KeyboardInput { input, ..} if input.virtual_keycode == Some(glutin::VirtualKeyCode::Space) => {
                        // let read_back: Vec<Vec<(u8, u8, u8, u8)>> = depth_texture.read();
                        println!("depth_texture?");
                    }
                    glutin::WindowEvent::KeyboardInput { input, ..} if input.virtual_keycode == Some(glutin::VirtualKeyCode::Escape) => {
                        action = Action::Stop;
                    }
                    ev => camera.process_input(&ev),
                },
                _ => (),
            }
        });

        action
    });


}
