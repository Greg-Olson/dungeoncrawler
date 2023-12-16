use crate::prelude::*;

#[system]
#[read_component(Point)] // <callout id="co.ecs.collision_detect.components" />
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {// <callout id="co.ecs.collision_detect.commands" />
//END: queries
//START: get_player_pos
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query()
        .filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);
//END: get_player_pos
//START: enemy_iter
    let mut enemies = <(Entity, &Point)>::query()
        .filter(component::<Enemy>());
    enemies
        .iter(ecs)
        .filter(|(_,pos)| **pos == player_pos)// <callout id="co.ecs.collisions.filter" />
        .for_each(|(entity, _)| {// <callout id="co.ecs.collisions.foreach" />
            commands.remove(*entity);// <callout id="co.ecs.collisions.command" />
        }
    );
//END: enemy_iter
}