use crate::engine::mesh::Mesh;


pub fn load_mesh(file_name: &str) -> Mesh {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);

    let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).unwrap();

    let m = &models[0].mesh;
    m.into()
}