use shipyard::*;
use crate::components::*;
use crate::geometry::*;
use crate::renderer::SceneRenderer;

pub fn init_world(stage_area:Area,renderer:SceneRenderer) -> World {
    let world = World::default();

    world.add_unique(StageArea(stage_area));
    world.add_unique(Motion{ last_pos: None, current_pos: None} );
    world.add_unique(Controller::Waiting);
    world.add_unique(Tick::default());
    world.add_unique_non_send_sync(renderer);

    world
}