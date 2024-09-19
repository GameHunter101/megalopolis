use std::rc::Rc;

use gamezap::{
    ecs::{components as core_components, material::Material, scene},
    model::Vertex,
};
use nalgebra::Vector3;
use perlin_noise::PerlinNoise;

pub mod components {
    pub mod camera_control_component;
}

pub mod perlin_noise;

#[tokio::main]
async fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    let window = video_subsystem
        .window("Megalopolis", 800, 800)
        .build()
        .unwrap();

    let mut engine = gamezap::GameZap::builder()
        .antialiasing()
        .window_and_renderer(
            sdl_context,
            video_subsystem,
            event_pump,
            window,
            wgpu::Color {
                r: 0.3725,
                g: 0.5843,
                b: 0.9294,
                a: 1.0,
            },
        )
        .build()
        .await;

    let mut scene = scene::Scene::default();

    let concept_manager = scene.get_concept_manager();
    let device = engine.renderer.device.clone();
    let queue = engine.renderer.queue.clone();

    // Camera
    let camera_component = core_components::camera_component::CameraComponent::new_3d(
        concept_manager.clone(),
        (1000, 1000),
        100.0,
        0.01,
        200.0,
    );

    let camera_transform_component = core_components::transform_component::TransformComponent::new(
        concept_manager.clone(),
        Vector3::new(0.0, 10.0, -15.0),
        (algoe::bivector::Bivector::new(0.0, 1.0, 0.0) * -std::f32::consts::FRAC_PI_4 / 3.0)
            .exponentiate(),
        Vector3::new(1.0, 1.0, 1.0),
    );

    let camera_control_component =
        components::camera_control_component::CameraControlComponent::new(0.005);

    let camera_entity = scene.create_entity(
        0,
        true,
        vec![
            Box::new(camera_component),
            Box::new(camera_transform_component),
            Box::new(camera_control_component),
        ],
        None,
    );

    scene.set_active_camera(camera_entity);

    // Terrain
    let terrain_resolution = 100;
    let terrain_size = 20.0;

    let (terrain_vertices, terrain_indices) =
        terrain_mesh_creation(terrain_resolution, terrain_size / terrain_resolution as f32);

    let terrain_mesh_component = core_components::mesh_component::MeshComponent::new(
        concept_manager.clone(),
        terrain_vertices,
        terrain_indices,
    );

    let terrain_transform_component = core_components::transform_component::TransformComponent::new(
        concept_manager.clone(),
        Vector3::new(terrain_size / -2.0, 0.0, terrain_size / -2.0),
        algoe::rotor::Rotor3::default(),
        Vector3::new(1.0, 1.0, 1.0),
    );

    let perlin_size = 5;

    let perlin = PerlinNoise::new(perlin_size, 1, 1.0, 0);

    let terrain_height_map = image::RgbaImage::from_fn(
        terrain_resolution as u32 + 1,
        terrain_resolution as u32 + 1,
        |x, y| {
            let perlin_val = perlin.evaluate(
                x as f32 / (terrain_resolution / perlin_size + 1) as f32,
                y as f32 / (terrain_resolution / perlin_size + 1) as f32,
            );
            let height = ((perlin_val + 1.0) / 2.0 * 255.0) as u8;
            image::Rgba([0, height, 0, 0])
        },
    );

    let terrain_height_texture = Rc::new(
        gamezap::texture::Texture::from_rgba(
            &device,
            &queue,
            &terrain_height_map,
            Some("Terrain height map"),
            true,
            true,
        )
        .unwrap(),
    );

    let terrain_material = Material::new(
        "shaders/terrain_vert.wgsl",
        "shaders/terrain_frag.wgsl",
        vec![terrain_height_texture],
        None,
        true,
        device.clone(),
    );

    let _terrain_entity = scene.create_entity(
        0,
        true,
        vec![
            Box::new(terrain_mesh_component),
            Box::new(terrain_transform_component),
        ],
        Some((vec![terrain_material], 0)),
    );

    engine.create_scene(scene);
    engine.main_loop();
}

/// Creates a 2-dimensional grid.
/// Specify the resolution of the grid (number of vertices),
/// and the size of every quad of the mesh.
/// The final mesh is of size `(resolution * quad_size)^2`
fn terrain_mesh_creation(resolution: usize, quad_size: f32) -> (Vec<Vertex>, Vec<u32>) {
    let vertices = (0..resolution)
        .flat_map(|i| {
            (0..resolution)
                .flat_map(|j| {
                    let i = i as f32;
                    let j = j as f32;
                    let i_scaled = i * quad_size;
                    let j_scaled = j * quad_size;
                    // let resolution = resolution as f32;
                    // let grid_size = resolution * quad_size;
                    vec![
                        Vertex {
                            position: [j_scaled, 0.0, i_scaled],
                            tex_coords: [j, i],
                            normal: [0.0, 1.0, 0.0],
                        },
                        Vertex {
                            position: [j_scaled + quad_size, 0.0, i_scaled],
                            tex_coords: [(j + 1.0), i],
                            normal: [0.0, 1.0, 0.0],
                        },
                        Vertex {
                            position: [j_scaled + quad_size, 0.0, i_scaled + quad_size],
                            tex_coords: [(j + 1.0), (i + 1.0)],
                            normal: [0.0, 1.0, 0.0],
                        },
                        Vertex {
                            position: [j_scaled, 0.0, i_scaled + quad_size],
                            tex_coords: [j, (i + 1.0)],
                            normal: [0.0, 1.0, 0.0],
                        },
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let indices = (0..resolution * resolution)
        .flat_map(|i| {
            let i = 4 * i as u32;
            vec![i, i + 1, i + 2, /* | */ i, i + 2, i + 3]
        })
        .collect();

    (vertices, indices)
}
