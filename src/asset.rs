use crate::render::Model;
use crate::obj::Obj;

pub struct AssetLoader {
    models: Vec<Model>,
}

impl AssetLoader {
    pub fn new() -> Self {
        let mut asset_loader = AssetLoader {
            models: Vec::new(),
        };
        asset_loader.initialize();
        asset_loader
    }

    fn initialize(&mut self) {
        self.models.clear();
        let assets_dir = "assets";
        for entry in std::fs::read_dir(assets_dir)
            .expect(format!("Failed to read assets directory {:?}", &assets_dir).as_str()) {
            let entry = entry.expect("Failed to read entry in assets directory");
            if entry.path().extension().map_or(false, |ext| ext == "obj") {
                let obj = Obj::read_from_file(
                    entry.path()
                        .to_str().expect("Failed to convert path to str"))
                        .expect("Failed to read OBJ file");
                let model = Model::from(obj);
                self.models.push(model);
            }
        }
    }

    pub fn get_models(&self) -> &Vec<Model> {
        &self.models
    }
}