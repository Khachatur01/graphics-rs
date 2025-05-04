use core::entity::Entity;

pub mod click_draw_tool;
pub mod move_draw_tool;

pub trait AddEntity {
    fn add_entity(&mut self, entity: Entity);
}
