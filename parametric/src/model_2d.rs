use crate::param::ParamSet;
use crate::sketch::Sketch;
use model::Model;

pub struct Model2D {
    pub param_set: ParamSet,
    pub sketch: Sketch,
}

impl Model2D {
    // pub fn insert_param(&mut self, param: Param) -> ParamId {
    //     let param_id: ParamId = ParamId::generate();
    //
    //     self.params.insert(param_id, param);
    //
    //     param_id
    // }

    pub fn set_sketch(&mut self, sketch: Sketch) {
        self.sketch = sketch;
    }
}

impl Model for Model2D {
    fn logical_type_id(&self) -> Option<&'static str> {
        todo!()
    }
}
