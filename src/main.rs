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
use glium::texture::depth_texture2d::DepthTexture2d;
use glium::texture::{ DepthFormat, MipmapsOption };
use glium::draw_parameters::{ DrawParameters, BackfaceCullingMode };
use glium::framebuffer::{ DepthAttachment, ToDepthAttachment };

// use std::io::Cursor;

mod internal;
use internal::{ Shaders, Reload,
                runtime_readbytes, load_wavefront, load_jpg_texture };
use internal::skybox::Skybox;
use internal::program::program_from_shader_paths;

mod support;
use support::Action;

use std::iter;

extern crate cgmath;
use cgmath::SquareMatrix;
use cgmath::prelude::EuclideanSpace;

#[allow(non_snake_case)]
fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let cube_vertices = runtime_readbytes("../resources/objects/room_thickwalls.obj");
    let vertex_buffer = load_wavefront(&display, &cube_vertices.as_slice());

    let plain_vertices = runtime_readbytes("../resources/objects/plain.obj");
    let plain_vb = load_wavefront(&display, &plain_vertices.as_slice());

    let mut shaders = Shaders::new("shaders/shadows.vert", "shaders/shadows.frag");
    let mut program =
        glium::Program::from_source(&display, &shaders.vert, &shaders.frag, None)
        .unwrap();

    //    let diffuse_texture = load_png_texture(&display, "../resources/textures/cube.png");
    let diffuse_texture = load_jpg_texture(&display, "../resources/textures/earth.jpg");
    let asphalt_texture = load_jpg_texture(&display, "../resources/textures/asphalt.jpg");

    let sky = Skybox::new(500.0, &display);

    let material_color: [f32; 4] = [0.9, 0.4, 0.4, 1.0];
    let reflect_factor: f32 = 0.9;

    let mut camera = support::camera::CameraState::new(0.1, 0.04);

    let mut iteration: u32 = 0;

    let model = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    let plain_model = [
        [20.0, 0.0, 0.0, 0.0],
        [0.0, 20.0, 0.0, 0.0],
        [0.0, 0.0, 20.0, 0.0],
        [0.0, -2.0, 0.0, 1.0f32]
    ];

    let another_surface_model = [
        [10.0, 0.0, 0.0, 0.0],
        [0.0, 10.0, 0.0, 0.0],
        [0.0, 0.0, 10.0, 0.0],
        [2.0, 10.0, 10.0, 1.0f32]
    ];

    // let : cgmath::Matrix4<f32> =
    //     cgmath::Matrix4::look_at(cgmath::EuclideanSpace::from_vec(lightInvDir),
    //                              cgmath::Point3{x: 0.0, y: 0.0, z: 0.0_f32},
    //                              cgmath::vec3(0.0,1.0,0.0_f32));


    // depth calculations?

    // let ProjectionMatrix = camera.get_perspective();
    // let ViewMatrix = camera.get_view();
    // let ModelMatrix = glm::mat4(1.0);
    // let MVP = ProjectionMatrix * ViewMatrix * ModelMatrix;

//     let depthBiasMVP = biasMatrix*depthMVP;

//     let mvp_uniforms = uniform! {
//         MatrixID: &MVP,
//         ModelMatrixID: &ModelMatrix,
//         ViewMatrixID: &ViewMatrix,
//         DepthBiasID: &depthBiasMVP,
    // //        lightInvDirID:
    //     };


    // FROM 16th TUTORIAL
    // see https://github.com/opengl-tutorials/ogl/blob/master/tutorial16_shadowmaps/tutorial16_SimpleVersion.cpp#L86

    // let vertex_array = vec![];

    // Create and compile our GLSL program from the shaders
    // let mut depthShaders = Shaders::new("shaders/DepthRTT.vertexshader",
    //                                     "shaders/DepthRTT.fragmentshader");
    let depthProgram = program_from_shader_paths(&display, "shaders/shadowShader.vert",
                                                 "shaders/shadowShader.frag");

    // Get a handle for our "MVP" uniform
    let depthMatrix = depthProgram.get_uniform("depthMVP");

    // Load the texture
    // ...?

    // Read our .obj file
    // ... and probably skip several steps till VBO
    let room_thickwalls_vb = runtime_readbytes("../resources/objects/room_thickwalls.obj");

    // The framebuffer, which regroups 0, 1, or more textures, and 0 or 1 depth buffer.
    // let framebuffer = glium::framebuffer::SimpleFrameBuffer::new(&display, &asphalt_texture); // FIXME: change texture

    // Tex
    let _depthTexture = DepthTexture2d::empty_with_format(&display,
                                                         DepthFormat::I16,
                                                         MipmapsOption::NoMipmap,
                                                         1024, 1024)
        .expect("cannot create depthTexture");

    // let framebuffer = glium::framebuffer::SimpleFrameBuffer::new(&display, &depthTexture);
    let depthrenderbuffer = glium::framebuffer::DepthRenderBuffer::new(&display,
                                                                       DepthFormat::I16,
                                                                       1024, 1024)
        .expect("Cannot create depth framebuffer");

    // Test framebuffer
    let texture1 = glium::texture::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::F32F32F32F32,
        glium::texture::MipmapsOption::NoMipmap,
        1024, 1024
    ).unwrap();
    let mut framebuffer_clear =
        glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &texture1,
                                                                 &depthrenderbuffer)
        .expect("Cannot create (depth) framebuffer");

    // Create and compile our GLSL program from the shaders let
    // let program_shaders =
    //     Shaders::new("shaders/ShadowMapping_SimpleVersion.vertexshader",
    //                  "shaders/ShadowMapping_SimpleVersion.fragmentshader");

    // let mut program_2 =
    //     glium::Program::from_source(&display, &program_shaders.vert, &program_shaders.frag, None)
    //     .unwrap();

    // // Get a handle for our "myTextureSampler" uniform
    // let texture_uniform = program.get_uniform("myTextureSampler").unwrap().clone();

    // // Get a handle for our "MVP" uniform
    // let matrix = program.get_uniform("MVP").unwrap().clone();
    // let depthBias = program.get_uniform("DepthBiasMVP").unwrap().clone();
    // let shadowMap = program.get_uniform("shadowMap").unwrap().clone();

    // END, next inside loop

    // let test_shaders =
    //     Shaders::new("shaders/test2.vert",
    //                  "shaders/test2.frag");
    // let test_shaders =
    //     Shaders::new("shaders/shadows.vert",
    //                  "shaders/shadows.frag");

    // let mut test_program =
    //     glium::Program::from_source(&display, &test_shaders.vert, &test_shaders.frag, None)
    //     .unwrap();

    let test_program = program_from_shader_paths(&display, "shaders/shadows.vert",
                                                 "shaders/shadows.frag");

    let test_uniforms = uniform! {
        model: model,
        view: camera.get_view(),
        perspective: camera.get_perspective(),
        u_light: [(iteration as f32 / 10.0).sin() * 3.0,
                     2.0,
                     (iteration as f32 / 10.0).cos() * 3.0 ],
        diffuse_tex: &diffuse_texture,
        // normal_tex: &normal_map

        ReflectFactor: reflect_factor,
        MaterialColor: material_color,
        cameraPosition: camera.position, // camera_position,

        persp_matrix: camera.get_perspective(),
        view_matrix: camera.get_view(),
    };

    // empty color attachment to put the data
    let test_texture = glium::Texture2d::empty_with_format(&display,
                                                    glium::texture::UncompressedFloatFormat::U8U8U8U8,
                                                    glium::texture::MipmapsOption::NoMipmap,
                                                    1280, 1280).unwrap();



    let depth_data = iter::repeat(iter::repeat(0.0f32).take(1280).collect::<Vec<_>>())
        .take(1280).collect::<Vec<_>>();

    // let test_texture_2 =
    //     glium::texture::depth_texture2d::DepthTexture2d::empty_with_format(&display,
    //                                                                        glium::texture::DepthFormat::I24,
    //                                                                        // glium::texture::UncompressedFloatFormat::U8U8U8U8,
    //                                                                        glium::texture::MipmapsOption::NoMipmap,
    //                                                                        1280, 1280).unwrap();

    // let test_texture_2 =
    //     glium::texture::depth_texture2d::DepthTexture2d::new(&display,
    //                                                          depth_data,
    //                                                          glium::texture::MipmapsOption::NoMipmap);

    let test_depth = match glium::texture::DepthTexture2d::new(&display, depth_data) {
        Err(_) => return,
        Ok(t) => t
    };

    // drawing with the `IfLess` depth test
    let mut framebuffer =
        glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
            &display, &test_texture, &test_depth
        ).unwrap();

    // let test_params = glium::DrawParameters {
    //     depth: glium::Depth {
    //         test: glium::DepthTest::IfLess,
    //         .. Default::default()
    //     },
    //     .. Default::default()
    // };

    // framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);
    // framebuffer.draw(&vertex_buffer, // &plain_vb,
    //                  &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
    //                  &test_program,
    //                  &test_uniforms, // &glium::uniforms::EmptyUniforms,
    //                  &test_params)
    //     .unwrap();

    let read_back: Vec<Vec<(u8, u8, u8, u8)>> = test_texture.read();

    // assert_eq!(read_back[0][0], (255, 255, 255, 255));
    // assert_eq!(read_back[127][127], (0, 0, 0, 255));

    // println!("{:?}", read_back);
    // panic!();

    // display.assert_no_error(None);

    let mut sh_program = program_from_shader_paths(&display, "shaders/sh_shader.vert", "shaders/sh_shader.frag");

    support::start_loop(|| {

        iteration = (iteration + 1) % 315;

        camera.update();


        // shadows from test

        // /shadows from test

        // shadows

        let params_2 = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess, // glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },

            // add Culling?
            backface_culling: BackfaceCullingMode::CullClockwise, // FIXME? try to use CullCounterClockwise

            // stencil: glium::draw_parameters::Stencil {
            //     test_clockwise: glium::draw_parameters::StencilTest::AlwaysPass,
            //     .. Default::default()
            // },
            // polygon_mode: glium::PolygonMode::Line,
            // scissor: Some(glium::Rect { bottom: 0, left: 100, width: 100, height: 200 }),
            .. Default::default()
        };

        // let shadow_sampler_uniforms = uniform! {
        //     //target_texture: &asphalt_texture,
        //     // mvpMatrix
        //     // MVP: [[0.8593976, 0.0, -0.5113079, 0.0],
        //     //             [0.0, 1.0, 0.0, 0.0], [-0.5113079, 0.0, -0.8593976, 0.0],
        //     //             [-2.5927825, -6.699996, 14.820048, 1.0]], // fixed view for now?
        //     // DepthBiasMVP: biasMatrix, // depthBias,
        //     MVP: [[0.73082864, 0.0, -0.68256104, 0.0],
        //           [0.0, 1.0, 0.0, 0.0],
        //           [-0.68256104, 0.0, -0.73082864, 0.0],
        //           [-0.76694846, -6.0999966, 8.010147, 1.0]],
        //     shadowMap: &asphalt_texture // &texture1
        // };

        // framebuffer.clear_color_and_depth((200.0 / 255.0,
        //                                    197.0 / 255.0,
        //                                    200.0 / 255.0, 0.0), 1.0);
        framebuffer.clear_color_and_depth((0.4, 0.4, 0.4, 0.0), 1.0);
        // framebuffer.draw(&vertex_buffer,
        //                  &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        //                  &depthProgram,
        //                  &shadow_sampler_uniforms,
        //                  &params_2)
        //     .unwrap();


        // /shadows

        // sky

        let sky_model = model.clone();

        let  framebuffer1 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        sky.cubemap.main_level().image(glium::texture::CubeLayer::PositiveX)).unwrap();
        let  framebuffer2 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        sky.cubemap.main_level().image(glium::texture::CubeLayer::NegativeX)).unwrap();
        let  framebuffer3 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        sky.cubemap.main_level().image(glium::texture::CubeLayer::PositiveY)).unwrap();
        let  framebuffer4 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        sky.cubemap.main_level().image(glium::texture::CubeLayer::NegativeY)).unwrap();
        let  framebuffer5 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        sky.cubemap.main_level().image(glium::texture::CubeLayer::PositiveZ)).unwrap();
        let  framebuffer6 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        sky.cubemap.main_level().image(glium::texture::CubeLayer::NegativeZ)).unwrap();

        sky.textures[0].as_surface().blit_whole_color_to(&framebuffer1, &sky.dest_rect,
                        glium::uniforms::MagnifySamplerFilter::Nearest);
        sky.textures[1].as_surface().blit_whole_color_to(&framebuffer2, &sky.dest_rect,
                        glium::uniforms::MagnifySamplerFilter::Nearest);
        sky.textures[2].as_surface().blit_whole_color_to(&framebuffer3, &sky.dest_rect,
                        glium::uniforms::MagnifySamplerFilter::Nearest);
        sky.textures[3].as_surface().blit_whole_color_to(&framebuffer4, &sky.dest_rect,
                        glium::uniforms::MagnifySamplerFilter::Nearest);
        sky.textures[4].as_surface().blit_whole_color_to(&framebuffer5, &sky.dest_rect,
                        glium::uniforms::MagnifySamplerFilter::Nearest);
        sky.textures[5].as_surface().blit_whole_color_to(&framebuffer6, &sky.dest_rect,
                                                         glium::uniforms::MagnifySamplerFilter::Nearest);

        // for (id, typ) in (glium::texture::CubeLayer).iter().enumerate() {

        // }

        let skybox_uniforms = uniform! {
             model: sky_model,
             view: camera.get_view(),
             perspective: camera.get_perspective(),
             cubetex: sky.cubemap.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        // /sky

        let light = [(iteration as f32 / 10.0).sin() * 14.0,
                     -10.0,
                     (iteration as f32 / 10.0).cos() * 14.0 ];



        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess, // glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            stencil: glium::draw_parameters::Stencil {
                test_clockwise: glium::draw_parameters::StencilTest::AlwaysPass,
                .. Default::default()
            },
            // polygon_mode: glium::PolygonMode::Line,
            // scissor: Some(glium::Rect { bottom: 0, left: 100, width: 100, height: 200 }),
            .. Default::default()
        };




        // Target

        let uniforms = uniform! {
            model: model,
            view: camera.get_view(),
            perspective: camera.get_perspective(),
            u_light: light,
            diffuse_tex: &diffuse_texture,
            // normal_tex: &normal_map

            ReflectFactor: reflect_factor,
            MaterialColor: material_color,
            cameraPosition: camera.position, // camera_position,

            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
        };

        let plain_uniforms = uniform! {
            model: plain_model,
            view: camera.get_view(),
            perspective: camera.get_perspective(),
            u_light: light,
            diffuse_tex: &test_texture, // &texture1, // &asphalt_texture,
            // normal_tex: &normal_map

            ReflectFactor: reflect_factor,
            MaterialColor: material_color,
            cameraPosition: camera.position, // camera_position,

            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
        };

        // let another_surface_uniforms = uniform! {
        //     model: another_surface_model,
        //     view: camera.get_view(),
        //     perspective: camera.get_perspective(),
        //     u_light: light,
        //     diffuse_tex: &texture1, // &asphalt_texture,
        //     // normal_tex: &normal_map

        //     ReflectFactor: reflect_factor,
        //     MaterialColor: material_color,
        //     cameraPosition: camera.position, // camera_position,

        //     persp_matrix: camera.get_perspective(),
        //     view_matrix: camera.get_view(),
        // };



        // let perspective_matrix: cgmath::Matrix4<f32> = cgmath::perspective(cgmath::Deg(45.0), 1.333, 0.0001, 100.0);

        // // let view_eye: cgmath::Point3<f32> = cgmath::Point3::new(0.1, 0.1, 1.0);
        // let view_eye: cgmath::Point3<f32> = camera.get_view().iter().take(3).collect().into();
        // let view_center: cgmath::Point3<f32> = cgmath::Point3::new(0.0, 0.0, 0.0);
        // let view_up: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 1.0, 0.0);

        // let light_view_matrix: cgmath::Matrix4<f32> =
        //     cgmath::Matrix4::look_at(view_eye, view_center, view_up);

        let radius = 10.0;

        let lightInvDir: cgmath::Vector3<f32> = // cgmath::vec3(0.5, 2.0, 2.0_f32);
            cgmath::vec3((iteration as f32 / 10.0).sin() * radius,
                     10.0,
                     (iteration as f32 / 10.0).cos() * radius);
        let depthProjectionMatrix: cgmath::Matrix4<f32> = cgmath::ortho(-10.0, 10.0, -10.0, 10.0, -10.0, 20.0_f32);
        // println!("{:?}", depthProjectionMatrix);
        let depthViewMatrix: cgmath::Matrix4<f32> =
            cgmath::Matrix4::look_at(cgmath::EuclideanSpace::from_vec(lightInvDir),
                                     cgmath::Point3{x: 0.0, y: 0.0, z: 0.0_f32},
                                     cgmath::vec3(0.0,1.0,0.0_f32));
        let depthModelMatrix: cgmath::Matrix4<f32> = cgmath::Matrix4::from_value(1.0);
        assert!(depthModelMatrix.is_diagonal());

        let biasMatrix: cgmath::Matrix4<f32> = [[ 0.5_f32, 0.0, 0.0, 0.0, ],
                                                [ 0.0, 0.5, 0.0, 0.0, ],
                                                [ 0.0, 0.0, 0.5, 0.0, ],
                                                [ 0.5, 0.5, 0.5, 1.0, ]].into();

        let depthMVP = depthProjectionMatrix * depthViewMatrix * depthModelMatrix;

        let depthBiasMVP = biasMatrix * depthMVP;


        let sh_uniforms = uniform! {
            //depthMVP: Into::<[[f32; 4]; 4]>::into(depthMVP),
            //DepthBiasMVP: Into::<[[f32; 4]; 4]>::into(depthBiasMVP),
            depthMVP: Into::<[[f32; 4]; 4]>::into(depthBiasMVP),
            model: model,
            view: camera.get_view(), // Into::<[[f32; 4]; 4]>::into(light_view_matrix),
            perspective: camera.get_perspective(), // Into::<[[f32; 4]; 4]>::into(perspective_matrix),
            u_light: light,
            diffuse_tex: &diffuse_texture,
            // normal_tex: &normal_map

            ReflectFactor: reflect_factor,
            MaterialColor: material_color,
            cameraPosition: camera.position, // camera_position,

            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
            // cubetex: sky.cubemap.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),

            myTextureSampler: &asphalt_texture,
            shadowMap: &test_depth, // &test_texture_2, // &test_texture,
        };

        let mut target = display.draw();


        //

        // target.draw(asphalt_texture, plain_vb, program,
        target.clear_color_and_depth((200.0 / 255.0,
                                      197.0 / 255.0,
                                      200.0 / 255.0, 0.0), 1.0);

        // target.draw(&sky.vb, &sky.indices, &sky.program,
        //             &skybox_uniforms, &params)
        //     .unwrap();


        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program,
                    &uniforms,
                    &params)
            .unwrap();

        // draw surface texture
        framebuffer.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &sh_program,
                    &sh_uniforms,
                    &params)
            .unwrap();

        target.draw(&plain_vb,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program,
                    &plain_uniforms,
                    &params)
            .unwrap();
        // target.draw(&plain_vb,
        //             &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        //             &program,
        //             &another_surface_uniforms,
        //             &params)
        //     .unwrap();

        target.finish().unwrap();

        let mut action = support::Action::Continue;

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => action = support::Action::Stop,
                    // #\Space to reload shaders
                    glutin::WindowEvent::KeyboardInput { input, ..} if input.virtual_keycode == Some(glutin::VirtualKeyCode::Space) => {
                        shaders.reload();
                        program = glium::Program::from_source(
                            &display,&shaders.vert, &shaders.frag, None)
                            .unwrap();
                        println!("camera");
                        println!("persp: {:?}", camera.get_perspective());
                        println!("view: {:?}", camera.get_view());
                        println!("pos: {:?}", camera.position);
                        let read_back: Vec<Vec<(u8, u8, u8, u8)>> = test_texture.read();
                        println!("test_texture: {:?}", read_back.get(0..1).unwrap());
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
