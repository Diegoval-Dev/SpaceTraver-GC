use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{look_at, perspective, Mat4, Vec3};
use std::f32::consts::PI;
use std::time::Duration;
use image::io::Reader as ImageReader;

mod camera;
mod color;
mod fragment;
mod framebuffer;
mod obj;
mod shaders;
mod triangle;
mod vertex;

use camera::Camera;
use framebuffer::Framebuffer;
use obj::Obj;
use shaders::{
    earth_shader, fragment_shader, jupiter_shader, mars_shader, moon_shader, rocky_planet_shader,
    saturn_rings_shader, saturn_shader, sun_shader, venus_shader, vertex_shader,spaceship_shader
};
use triangle::triangle;
use vertex::Vertex;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
}
fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    // Transformación para rotación y escala
    let transform_matrix = Mat4::new(
        scale,
        0.0,
        0.0,
        translation.x,
        0.0,
        scale,
        0.0,
        translation.y,
        0.0,
        0.0,
        scale,
        translation.z,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    // Crear matriz de rotación
    let rotation_x = Mat4::new_rotation(Vec3::new(rotation.x, 0.0, 0.0));
    let rotation_y = Mat4::new_rotation(Vec3::new(0.0, rotation.y, 0.0));
    let rotation_z = Mat4::new_rotation(Vec3::new(0.0, 0.0, rotation.z));
    let rotation_matrix = rotation_z * rotation_y * rotation_x;

    transform_matrix * rotation_matrix
}
fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}
fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}
fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0,
        0.0,
        0.0,
        width / 2.0,
        0.0,
        -height / 2.0,
        0.0,
        height / 2.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = sun_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn render_rocky_planet(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = rocky_planet_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn render_venus(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterización y procesamiento de fragmentos
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Shader específico de Venus
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            // Aplicar el shader específico para Venus
            let shaded_color = venus_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn render_earth(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Ensamblado de Triángulos
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterización y procesamiento de fragmentos
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Aplicar `earth_shader` en cada fragmento
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = earth_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn render_mars(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = mars_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn render_jupiter(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = jupiter_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn render_moon(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = moon_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn render_saturn(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = saturn_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn render_saturn_rings(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = saturn_rings_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn render_spaceship(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            // Usar un shader específico para la nave espacial
            let shaded_color = spaceship_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);
    let mut time = 0;
    let mut frame_count = 0;
    let mut fps = 0;
    let mut last_time = std::time::Instant::now();
    let window_title = format!("Sistema Solar - FPS: {}", fps);
    const DISTANCE_SCALE: f32 = 2.0; 
    

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        window_title.as_str(),
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x000000);

    // Configuración del Sol
    let translation_sun = Vec3::new(0.0, 0.0, 0.0);
    let scale_sun = 1.5f32;
    let rotation_sun = Vec3::new(0.0, 0.0, 0.0);
    let obj_sun = Obj::load("assets/models/sun.obj").expect("Failed to load sun");
    let vertex_array_sun = obj_sun.get_vertex_array();

    // Cámara inicial
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut time = 0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 1;
        frame_count += 1;

        // Calcular FPS cada segundo
        let current_time = std::time::Instant::now();
        let elapsed = current_time.duration_since(last_time).as_secs_f32();

        if elapsed >= 1.0 {
            fps = frame_count;
            frame_count = 0;
            last_time = current_time;
        }
        let window_title = format!("Sistema Solar  - FPS: {}", fps * 4);
        window.set_title(&window_title);

        // Procesar entrada de la cámara
        handle_input(&window, &mut camera);

        // Actualizar la matriz de vista de la cámara
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);

        framebuffer.clear();

        // Renderizar el Sol
        let model_matrix_sun = create_model_matrix(translation_sun, scale_sun, rotation_sun);
        let uniforms_sun = Uniforms {
            model_matrix: model_matrix_sun,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time,
        };
        render(&mut framebuffer, &uniforms_sun, &vertex_array_sun);

        //MERCURIO
        // Configuración de Mercurio
        let scale_mercury = 0.4f32;
        let orbit_speed_mercury = 0.02;
        let rotation_speed_mercury = 0.05;
        let orbit_radius_mercury = 10.0 * DISTANCE_SCALE;

        // Cargar el modelo de Mercurio
        let obj_mercury = Obj::load("assets/models/planet.obj").expect("Failed to load planet");
        let vertex_array_mercury = obj_mercury.get_vertex_array();

        // Cálculo de la órbita y rotación de Mercurio
        let mercury_x = orbit_radius_mercury * (time as f32 * orbit_speed_mercury).cos() ;
        let mercury_y = 0.0;
        let mercury_z = orbit_radius_mercury * (time as f32 * orbit_speed_mercury).sin();
        let translation_mercury = Vec3::new(mercury_x, mercury_y, mercury_z);
        let rotation_mercury = Vec3::new(0.0, time as f32 * rotation_speed_mercury, 0.0);

        // Crear la matriz de modelo para Mercurio
        let model_matrix_mercury =
            create_model_matrix(translation_mercury, scale_mercury, rotation_mercury);

        // Crear los uniforms para Mercurio
        let uniforms_mercury = Uniforms {
            model_matrix: model_matrix_mercury,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time,
        };

        // Renderizar Mercurio
        render_rocky_planet(&mut framebuffer, &uniforms_mercury, &vertex_array_mercury);

        //VENUS
        let scale_venus = 0.55f32;
        let orbit_speed_venus = 0.015;
        let rotation_speed_venus = 0.03;
        let orbit_radius_venus = 18.0 * DISTANCE_SCALE;
        let obj_venus = Obj::load("assets/models/planet.obj").expect("Failed to load planet");
        let vertex_array_venus = obj_venus.get_vertex_array();

        let venus_x = orbit_radius_venus * (time as f32 * orbit_speed_venus).cos();
        let venus_y = 0.0;
        let venus_z = orbit_radius_venus * (time as f32 * orbit_speed_venus).sin();
        let translation_venus = Vec3::new(venus_x, venus_y, venus_z);

        let rotation_venus = Vec3::new(0.0, time as f32 * rotation_speed_venus, 0.0);
        let model_matrix_venus =
            create_model_matrix(translation_venus, scale_venus, rotation_venus);
        let uniforms_venus = Uniforms {
            model_matrix: model_matrix_venus,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time,
        };
        render_venus(&mut framebuffer, &uniforms_venus, &vertex_array_venus);

        //TIERRA Y LUNA
        // Configuración de la Tierra
        let scale_earth = 0.6f32;
        let orbit_speed_earth = 0.01;
        let rotation_speed_earth = 0.02;
        let orbit_radius_earth = 25.0 * DISTANCE_SCALE;

        // Cargar el modelo de la Tierra
        let obj_earth = Obj::load("assets/models/planet.obj").expect("Failed to load planet");
        let vertex_array_earth = obj_earth.get_vertex_array();

        // Configuración de la Luna
        let scale_moon = 0.15f32;
        let orbit_speed_moon = 0.03;
        let rotation_speed_moon = 0.05;
        let orbit_radius_moon = 2.5;

        let obj_moon = Obj::load("assets/models/planet.obj").expect("Failed to load moon");
        let vertex_array_moon = obj_moon.get_vertex_array();
        let earth_x = orbit_radius_earth * (time as f32 * orbit_speed_earth).cos();
        let earth_y = 0.0;
        let earth_z = orbit_radius_earth * (time as f32 * orbit_speed_earth).sin();
        let translation_earth = Vec3::new(earth_x, earth_y, earth_z);

        let rotation_earth = Vec3::new(0.0, time as f32 * rotation_speed_earth, 0.0);
        let moon_x =
            translation_earth.x + orbit_radius_moon * (time as f32 * orbit_speed_moon).cos();
        let moon_y = translation_earth.y + 2.0;
        let moon_z =
            translation_earth.z + orbit_radius_moon * (time as f32 * orbit_speed_moon).sin();
        let translation_moon = Vec3::new(moon_x, moon_y, moon_z);

        // Rotación de la Luna sobre su eje
        let rotation_moon = Vec3::new(0.0, time as f32 * rotation_speed_moon, 0.0);

        // Renderizar la Tierra
        let model_matrix_earth =
            create_model_matrix(translation_earth, scale_earth, rotation_earth);
        let uniforms_earth = Uniforms {
            model_matrix: model_matrix_earth,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time,
        };
        render_earth(&mut framebuffer, &uniforms_earth, &vertex_array_earth);

        // Renderizar la Luna
        let model_matrix_moon = create_model_matrix(translation_moon, scale_moon, rotation_moon);
        let uniforms_moon = Uniforms {
            model_matrix: model_matrix_moon,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time,
        };
        render_moon(&mut framebuffer, &uniforms_moon, &vertex_array_moon);

        //MARTE
        // Configuración de Marte
        let scale_mars = 0.5f32;
        let orbit_speed_mars = 0.008; // Velocidad de traslación de Marte
        let rotation_speed_mars = 0.03; // Velocidad de rotación de Marte
        let orbit_radius_mars = 35.0 * DISTANCE_SCALE; // Distancia de Marte al Sol

        // Cargar el modelo de Marte
        let obj_mars = Obj::load("assets/models/planet.obj").expect("Failed to load planet");
        let vertex_array_mars = obj_mars.get_vertex_array();
        // Calcular la posición de Marte en su órbita alrededor del Sol
        let mars_x = orbit_radius_mars * (time as f32 * orbit_speed_mars).cos();
        let mars_y = 0.0;
        let mars_z = orbit_radius_mars * (time as f32 * orbit_speed_mars).sin();
        let translation_mars = Vec3::new(mars_x, mars_y, mars_z);

        // Rotación de Marte sobre su eje
        let rotation_mars = Vec3::new(0.0, time as f32 * rotation_speed_mars, 0.0);

        // Renderizar Marte
        let model_matrix_mars = create_model_matrix(translation_mars, scale_mars, rotation_mars);
        let uniforms_mars = Uniforms {
            model_matrix: model_matrix_mars,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time,
        };
        render_mars(&mut framebuffer, &uniforms_mars, &vertex_array_mars);

        //JUPITER
        // Configuración de Júpiter
        let scale_jupiter = 1.2f32; // Tamaño más grande por ser un gigante gaseoso
        let orbit_speed_jupiter = 0.005; // Velocidad de traslación de Júpiter
        let rotation_speed_jupiter = 0.02; // Velocidad de rotación de Júpiter
        let orbit_radius_jupiter = 50.0 * DISTANCE_SCALE; // Distancia de Júpiter al Sol

        // Cargar el modelo de Júpiter
        let obj_jupiter = Obj::load("assets/models/planet.obj").expect("Failed to load planet");
        let vertex_array_jupiter = obj_jupiter.get_vertex_array();
        // Calcular la posición de Júpiter en su órbita alrededor del Sol
        let jupiter_x = orbit_radius_jupiter * (time as f32 * orbit_speed_jupiter).cos();
        let jupiter_y = 0.0;
        let jupiter_z = orbit_radius_jupiter * (time as f32 * orbit_speed_jupiter).sin();
        let translation_jupiter = Vec3::new(jupiter_x, jupiter_y, jupiter_z);

        // Rotación de Júpiter sobre su eje
        let rotation_jupiter = Vec3::new(0.0, time as f32 * rotation_speed_jupiter, 0.0);

        // Renderizar Júpiter
        let model_matrix_jupiter =
            create_model_matrix(translation_jupiter, scale_jupiter, rotation_jupiter);
        let uniforms_jupiter = Uniforms {
            model_matrix: model_matrix_jupiter,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time,
        };
        render_jupiter(&mut framebuffer, &uniforms_jupiter, &vertex_array_jupiter);

        //SATURNO
        // Configuración de Saturno
        let scale_saturn = 1.0f32; // Tamaño de Saturno
        let orbit_speed_saturn = 0.003; // Velocidad de traslación de Saturno
        let rotation_speed_saturn = 0.015; // Velocidad de rotación de Saturno
        let orbit_radius_saturn = 70.0 * DISTANCE_SCALE; // Distancia de Saturno al Sol

        // Cargar el modelo de Saturno
        let obj_saturn = Obj::load("assets/models/planet.obj").expect("Failed to load planet");
        let vertex_array_saturn = obj_saturn.get_vertex_array();

        // Configuración de los Anillos de Saturno
        let scale_rings = 2.5f32; // Tamaño más grande para los anillos
        let ring_rotation_speed = 0.01; // Velocidad de rotación de los anillos (ligeramente distinta)

        // Calcular la posición de Saturno en su órbita alrededor del Sol
        let saturn_x = orbit_radius_saturn * (time as f32 * orbit_speed_saturn).cos();
        let saturn_y = 0.0;
        let saturn_z = orbit_radius_saturn * (time as f32 * orbit_speed_saturn).sin();
        let translation_saturn = Vec3::new(saturn_x, saturn_y, saturn_z);

        // Rotación de Saturno sobre su eje
        let rotation_saturn = Vec3::new(0.0, time as f32 * rotation_speed_saturn, 0.0);

        // Renderizar Saturno
        let model_matrix_saturn =
            create_model_matrix(translation_saturn, scale_saturn, rotation_saturn);
        let uniforms_saturn = Uniforms {
            model_matrix: model_matrix_saturn,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time,
        };
        render_saturn(&mut framebuffer, &uniforms_saturn, &vertex_array_saturn);

        // Rotación de los Anillos de Saturno
        let ring_rotation = Vec3::new(0.0, time as f32 * ring_rotation_speed, 0.0);

        // Renderizar los Anillos de Saturno con su propio shader
        let model_matrix_rings =
            create_model_matrix(translation_saturn, scale_rings, ring_rotation);
        let uniforms_rings = Uniforms {
            model_matrix: model_matrix_rings,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time,
        };
        render_saturn_rings(&mut framebuffer, &uniforms_rings, &vertex_array_saturn);

        // Configuración de la Nave Espacial
        let scale_ship = 0.009f32;
        let rotation_ship = Vec3::new(0.0, 0.0, 0.0);
        let obj_ship =
            Obj::load("assets/models/naveEspacial1.obj").expect("Failed to load spaceship");
        let vertex_array_ship = obj_ship.get_vertex_array();

        // Posicionar la nave espacial frente a la cámara
        let ship_offset = 0.5; // Distancia de la nave a la cámara
        let ship_position = camera.eye + camera.forward() * ship_offset;
        let translation_ship = Vec3::new(ship_position.x, ship_position.y, ship_position.z);

        // Renderizar la Nave Espacial
        let model_matrix_ship = create_model_matrix(translation_ship, scale_ship, rotation_ship);
        let uniforms_ship = Uniforms {
            model_matrix: model_matrix_ship,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time,
        };

        render_spaceship(&mut framebuffer, &uniforms_ship, &vertex_array_ship);

        //AQUI TERMINA

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI / 50.0;
    let zoom_speed = 0.1;

    //  camera orbit controls
    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, rotation_speed);
    }

    // Camera movement controls
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    // Camera zoom controls
    if window.is_key_down(Key::Up) {
        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.zoom(-zoom_speed);
    }
}
