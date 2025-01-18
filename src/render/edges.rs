use glam::Vec3;
use glam::Vec4Swizzles;
use sdl2::pixels::Color;

use crate::bsp::Edge;

use crate::bsp::Vertex;

use camera::Camera;

use sdl2::render::WindowCanvas;

use super::camera;
use super::ndc_to_screen;

pub fn render_edges(
    canvas: &mut sdl2::render::WindowCanvas,
    camera: &Camera,
    vertices: &[Vertex],
    edges: &[Edge],
    screen_width: u32,
    screen_height: u32,
) {
    let view_proj = camera.projection_matrix() * camera.view_matrix();

    for edge in edges {
        let v1_world = vertices[edge.start_vertex as usize].coordinates;
        let v2_world = vertices[edge.end_vertex as usize].coordinates;

        let v1_clip = view_proj * v1_world.extend(1.0);
        let v2_clip = view_proj * v2_world.extend(1.0);

        if v1_clip.w <= 0.0 || v2_clip.w <= 0.0 {
            continue; // Skip edges behind the camera
        }

        let v1_ndc = v1_clip.xyz() / v1_clip.w;
        let v2_ndc = v2_clip.xyz() / v2_clip.w;

        if let Some((v1_clipped, v2_clipped)) = clip_edge_to_screen(v1_ndc, v2_ndc) {
            let v1_screen = ndc_to_screen(v1_clipped, screen_width, screen_height);
            let v2_screen = ndc_to_screen(v2_clipped, screen_width, screen_height);

            canvas.set_draw_color(sdl2::pixels::Color::BLACK);
            canvas
                .draw_line(v1_screen, v2_screen)
                .expect("Failed to draw edge");
        }
    }

    canvas.present();
}

fn clip_edge_to_screen(v1_ndc: Vec3, v2_ndc: Vec3) -> Option<(Vec3, Vec3)> {
    let v1 = v1_ndc;
    let v2 = v2_ndc;

    // Define NDC bounds
    let min_ndc = Vec3::new(-1.0, -1.0, -1.0);
    let max_ndc = Vec3::new(1.0, 1.0, 1.0);

    // Clip the edge to the NDC bounds
    let mut t_min: f32 = 0.0;
    let mut t_max: f32 = 1.0;

    for i in 0..3 {
        let delta = v2[i] - v1[i];

        if delta == 0.0 {
            // Parallel to this axis
            if v1[i] < min_ndc[i] || v1[i] > max_ndc[i] {
                return None; // Outside bounds
            }
        } else {
            // Calculate intersections with the planes
            let t1 = (min_ndc[i] - v1[i]) / delta;
            let t2 = (max_ndc[i] - v1[i]) / delta;

            // Order t1 and t2
            let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

            t_min = t_min.max(t_near);
            t_max = t_max.min(t_far);

            if t_min > t_max {
                return None; // Fully outside
            }
        }
    }

    // Compute the clipped coordinates
    let v1_clipped = v1 + t_min * (v2 - v1);
    let v2_clipped = v1 + t_max * (v2 - v1);

    Some((v1_clipped, v2_clipped))
}
