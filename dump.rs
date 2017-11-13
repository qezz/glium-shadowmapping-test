    // let test_tex_cube = glium::vertex::VertexBuffer::new(&display, &[
    //     Vertex { position: [1.000000, -1.000000, -1.000000], normal: [0.0000, -1.0000, 0.0000],  tex_coords: [0.0, 1.0] },
    //     Vertex { position: [1.000000, -1.000000, 1.000000], normal: [0.0000, -1.0000, 0.0000],   tex_coords: [0.0, 1.0] },
    //     Vertex { position: [-1.000000, -1.000000, 1.000000], normal: [0.0000, -1.0000, 0.0000],  tex_coords: [0.0, 1.0] },
    //     Vertex { position: [-1.000000, -1.000000, -1.000000], normal: [0.0000, -1.0000, 0.0000], tex_coords: [0.0, 1.0] },
    //     Vertex { position: [1.000000, 1.000000, -1.000000], normal: [ 0.0000, 1.0000, 0.0000],   tex_coords: [0.0, 1.0] },
    //     Vertex { position: [1.000000, 1.000000, 1.000001], normal: [ 1.0000, -0.0000, 0.0000],   tex_coords: [0.0, 1.0] },
    //     Vertex { position: [-1.000000, 1.000000, 1.000000], normal: [0.0000, -0.0000, 1.0000],   tex_coords: [0.0, 1.0] },
    //     Vertex { position: [-1.000000, 1.000000, -1.000000], normal: [0.0000, 0.0000, -1.0000],   tex_coords: [0.0, 1.0] },
    // ]).unwrap();

    // let test_tex_cube = glium::vertex::VertexBuffer::new(&display, &[
    //     Vertex { position: [0.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    //     Vertex { position: [0.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    //     Vertex { position: [1.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    //     Vertex { position: [1.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    //     Vertex { position: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    //     Vertex { position: [0.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    //     Vertex { position: [0.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    //     Vertex { position: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    //     Vertex { position: [1.0, 1.0, 2.0], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    // ]).unwrap();

    // let test_teapot = support::load_wavefront(&display, include_bytes!("../resources/teapot.obj"));

    // let image = image::load(Cursor::new(&include_bytes!("../resources/wall-diffuse.jpg")[..]),
    //                         image::JPEG).unwrap().to_rgba();
    // let image_dimensions = image.dimensions();
    // let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    // let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    // let image = image::load(Cursor::new(&include_bytes!("../resources/wall-normal.png")[..]),
    //                         image::PNG).unwrap().to_rgba();
    // let image_dimensions = image.dimensions();
    // let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    // let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();






        // target.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
        //             &uniform!{ model: model, view: camera.get_view(),
        //                        // cameraPosition: camera.get_view(),
        //                        perspective: camera.get_perspective(),
        //                        u_light: light,
        //                        light: light,
        //                        diffuse_tex: &diffuse_texture, normal_tex: &normal_map,
        //                        light_position: light,
        //                        light_intensities: [0.0, 0.2, 0.0f32],
        //                        light_attenuation: 1.0f32,
        //                        light_ambientCoefficient: 0.2f32
        //             },
        //             &params).unwrap();
        // target.draw(&plain, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
        //             &uniform!{ model: model, view: camera.get_view(),
        //                        // cameraPosition: camera.get_view(),
        //                        perspective: camera.get_perspective(),
        //                        u_light: light,
        //                        light: light,
        //                        diffuse_tex: &diffuse_texture, normal_tex: &normal_map,
        //                        light_position: light,
        //                        light_intensities: [0.9, 0.9, 0.9f32],
        //                        light_attenuation: 0.0f32,
        //                        light_ambientCoefficient: 0.2f32
        //             },
//             &params).unwrap();




        // target.draw(&test_teapot, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
        //             &uniform!{ model: model, view: camera.get_view(),
        //                        // cameraPosition: camera.get_view(),
        //                        perspective: camera.get_perspective(),
        //                        u_light: light,
        //                        light: light,
        //                        diffuse_tex: &cube_texture, normal_tex: &normal_map,
        //                        light_position: light,
        //                        light_intensities: [0.9, 0.9, 0.9f32],
        //                        light_attenuation: 0.0f32,
        //                        light_ambientCoefficient: 0.2f32
        //             },
        //             &params).unwrap();



    //println!("include_bytes!(\"../resources/cube.obj\"): {}", *include_bytes!("../resources/cube.obj"));
    // let shape = support::load_wavefront(&display, include_bytes!("../resources/plane.obj"));

    // let plain = support::load_wavefront(&display, include_bytes!("../resources/grid.obj"));

    // let test_tex_cube = support::load_wavefront(&display, include_bytes!("../resources/tex_cube_minimal.obj"));

    // let sss = piston_parse("../resources/tex_cube_minimal.obj");
    // println!(": {:#?}", sss);

    // let test_tex_cube = Obj::<SimplePolygon>::load(&Path::new("../resources/tex_cube_minimal.obj")).unwrap();

    // println!("load: {:#?}", test_tex_cube.position);

    // let pss = [
    //     [ -1.0,-1.0,-1.0,],
    //     [ -1.0,-1.0, 1.0,],
    //     [ -1.0, 1.0, 1.0,],
    //     [ 1.0, 1.0,-1.0,],
    //     [ -1.0,-1.0,-1.0,],
    //     [ -1.0, 1.0,-1.0,],
    //     [ 1.0,-1.0, 1.0,],
    //     [ -1.0,-1.0,-1.0,],
    //     [ 1.0,-1.0,-1.0,],
    //     [ 1.0, 1.0,-1.0,],
    //     [ 1.0,-1.0,-1.0,],
    //     [ -1.0,-1.0,-1.0,],
    //     [ -1.0,-1.0,-1.0,],
    //     [ -1.0, 1.0, 1.0,],
    //     [ -1.0, 1.0,-1.0,],
    //     [ 1.0,-1.0, 1.0,],
    //     [ -1.0,-1.0, 1.0,],
    //     [ -1.0,-1.0,-1.0,],
    //     [ -1.0, 1.0, 1.0,],
    //     [ -1.0,-1.0, 1.0,],
    //     [ 1.0,-1.0, 1.0,],
    //     [ 1.0, 1.0, 1.0,],
    //     [ 1.0,-1.0,-1.0,],
    //     [ 1.0, 1.0,-1.0,],
    //     [ 1.0,-1.0,-1.0,],
    //     [ 1.0, 1.0, 1.0,],
    //     [ 1.0,-1.0, 1.0,],
    //     [ 1.0, 1.0, 1.0,],
    //     [ 1.0, 1.0,-1.0,],
    //     [ -1.0, 1.0,-1.0,],
    //     [ 1.0, 1.0, 1.0,],
    //     [ -1.0, 1.0,-1.0,],
    //     [ -1.0, 1.0, 1.0,],
    //     [ 1.0, 1.0, 1.0,],
    //     [ -1.0, 1.0, 1.0,],
    //     [ 1.0,-1.0, 1.0f32],
    // ];

    // let mut vb = Vec::new();
    // for v in pss.iter() { // test_tex_cube.position {
    //     vb.push(Vertex { position: *v,  normal: [0.0, 0.0, 1.0], tex_coords: [0.5, 0.50] });
    // }
    // let test_tex_cube = glium::vertex::VertexBuffer::new(&display, &vb).unwrap();

    // println!("vb: {:#?}", vb);

    // unreachable!();

