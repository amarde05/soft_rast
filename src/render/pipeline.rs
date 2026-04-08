use glam::{Vec4, Vec4Swizzles};

use crate::{engine::mesh::MeshRegistry, render::{RenderObject, Triangle, camera::Camera}};



pub struct Pipeline {
    
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline { }
    }

    pub fn submit_ros(&self, camera: &Camera, render_objects: &Vec<RenderObject>, mesh_registry: &MeshRegistry) -> Vec<Triangle> {
        let mut buffer_a = self.unroll_triangles(camera, render_objects, mesh_registry);
        let mut buffer_b = Vec::new();

        self.clip(&buffer_a, &mut buffer_b);
        buffer_a.clear();

        self.divide_by_w(&mut buffer_b);

        //println!("Rendering {} triangles", buffer_b.len());

        buffer_b
    }

    fn unroll_triangles(&self, camera: &Camera, render_objects: &Vec<RenderObject>, mesh_registry: &MeshRegistry) -> Vec<Triangle> {
        let mut tris = Vec::new();

        for object in render_objects {
            let mvp = camera.vp() * object.model_matrix;
            let mesh = mesh_registry.get_mesh(object.mesh_id).expect("Invalid Mesh Id");

            for chunk in mesh.indices.chunks(3) {
                let v0 = mvp * mesh.vertices[chunk[0] as usize];
                let v1 = mvp * mesh.vertices[chunk[1] as usize];
                let v2 = mvp * mesh.vertices[chunk[2] as usize];

                let tri = Triangle { verts: [v0, v1, v2] };

                match self.is_backface(&tri) {
                    true => continue,
                    false => tris.push(tri),
                }
            }
        }

        tris
    }

    fn is_backface(&self, triangle: &Triangle) -> bool {
        let normal = triangle.get_normal();
        normal.dot(triangle.verts[0].xyz()) >= 0.
    }

    fn clip(&self, input: &Vec<Triangle>, output: &mut Vec<Triangle>) {
        // Run Sutherland-Hodgman to clip each triangle
        for tri in input {
            let mut output_verts = tri.verts.to_vec();

            for edge in ClipEdge::EDGES {
                let input_verts = output_verts.clone();
                output_verts.clear();

                let input_count = input_verts.len() as i32;

                for i in 0..input_count {
                    let current_point = input_verts[i as usize];
                    let prev_point = input_verts[(i - 1).rem_euclid(input_count) as usize];

                    let intersecting_point = edge.get_intersect_point(prev_point, current_point);

                    if edge.point_inside(current_point) {
                        if !edge.point_inside(prev_point) {
                            output_verts.push(intersecting_point);
                        }
                        output_verts.push(current_point);
                    }
                    else if edge.point_inside(prev_point) {
                        output_verts.push(intersecting_point);
                    }
                }
            }

            // Combine vertices back into triangles, if there are any left
            if output_verts.len() > 0 {
                for i in 1..(output_verts.len() - 1) {
                    let tri = Triangle {
                        verts: [
                            output_verts[0],
                            output_verts[i],
                            output_verts[i + 1]
                        ]
                    };

                    output.push(tri);
                }
            }
        }
    }

    // Perspective divide
    fn divide_by_w(&self, output: &mut Vec<Triangle>) {
        for tri in output {
            for vert in &mut tri.verts {
                vert.x /= vert.w;
                vert.y /= vert.w;
                vert.z /= vert.w;
            }
        }
    }
}

enum ClipEdge {
    Left,
    Right,
    Bottom,
    Top,
    Near,
    Far
}

impl ClipEdge {
    pub const EDGES: [ClipEdge; 6] = [ClipEdge::Left, ClipEdge::Right, ClipEdge::Bottom, ClipEdge::Top, ClipEdge::Near, ClipEdge::Far];

    pub fn point_inside(&self, point: Vec4) -> bool {
        match self {
            ClipEdge::Left => point.x >= -point.w,
            ClipEdge::Right => point.x <= point.w,
            ClipEdge::Bottom => point.y >= -point.w,
            ClipEdge::Top => point.y <= point.w,
            ClipEdge::Near => point.z >= -point.w,
            ClipEdge::Far => point.z <= point.w
        }
    }

    pub fn get_intersect_point(&self, p1: Vec4, p2: Vec4) -> Vec4 {
        let (d1, d2) = match self {
            ClipEdge::Left => (p1.w + p1.x, p2.w + p2.x),
            ClipEdge::Right => (p1.w - p1.x, p2.w - p2.x),
            ClipEdge::Bottom => (p1.w + p1.y, p2.w + p2.y),
            ClipEdge::Top => (p1.w - p1.y, p2.w - p2.y),
            ClipEdge::Near => (p1.w + p1.z, p2.w + p2.z),
            ClipEdge::Far => (p1.w - p1.z, p2.w - p2.z)
        };

        let t = d1 / (d1 - d2);

        p1 + t * (p2 - p1)
    }
}