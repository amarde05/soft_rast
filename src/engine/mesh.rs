use glam::Vec4;


pub struct MeshRegistry {
    registry: Vec<Mesh>
}

impl MeshRegistry {
    pub fn new() -> Self {
        MeshRegistry { registry: Vec::new() }
    }

    pub fn register_mesh(&mut self, mesh: Mesh) -> usize {
        self.registry.push(mesh);
        self.registry.len() - 1
    }

    pub fn get_mesh(&self, id: usize) -> Option<&Mesh> {
        match id >= self.registry.len() {
            true => None,
            false => Some(&self.registry[id])
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vec4>,
    pub indices: Vec<u32>
}

impl From<&tobj::Mesh> for Mesh {
    fn from(m: &tobj::Mesh) -> Self {
        let mut vertices = Vec::new();

        for v in 0..m.positions.len() / 3 {
            vertices.push(Vec4::new(
                m.positions[3 * v],
                m.positions[3 * v + 1],
                m.positions[3 * v + 2],
                1.0
            ));
        }

        Self {
            vertices,
            indices: m.indices.clone()
        }
    }
}