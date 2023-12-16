use crate::prelude::*;

mod map_render;
mod player_input;
mod entity_render;
mod collisions;

pub fn build_scheduler() -> Schedule {
    //START: collision_schedule
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
    //END: collision_schedule
}