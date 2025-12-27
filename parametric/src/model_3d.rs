use crate::feature::Feature;
use crate::param::ParamSet;
use crate::sketch::{Sketch, SketchId};
use model::Model;
use std::collections::HashMap;

pub struct Model3D {
    pub param_set: ParamSet,
    pub sketches: HashMap<SketchId, Sketch>,
    pub features: Vec<Feature>,
}

impl Model3D {
    // pub fn insert_param(&mut self, param: Param) -> ParamId {
    //     let param_id: ParamId = ParamId::generate();
    //
    //     self.params.insert(param_id, param);
    //
    //     param_id
    // }

    pub fn insert_sketch(&mut self, sketch: Sketch) -> SketchId {
        let sketch_id: SketchId = SketchId::generate();

        self.sketches.insert(sketch_id, sketch);

        sketch_id
    }

    pub fn insert_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }
}

impl Model for Model3D {
    fn logical_type_id(&self) -> Option<&'static str> {
        todo!()
    }
}
