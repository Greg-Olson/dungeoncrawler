//START: boilerplate
use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(
    #[resource] map: &Map,
    ecs: &SubWorld,
    commands: &mut CommandBuffer
) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();
    //END: boilerplate
    //START: findplayer
    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);
    //END: findplayer

    // START: dijkstra
    let search_targets = vec![player_idx];// <callout id="co.gauntlet.searchvec" />
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,// <callout id="co.gauntlet.dims" />
        SCREEN_HEIGHT,
        &search_targets,
        map,// <callout id="co.gauntlet.mapref" />
        1024.0// <callout id="co.gauntlet.maxiters" />
    );
    //END: dijkstra

    // START: chase
    movers.iter(ecs).for_each(| (entity, pos, _) | {// <callout id="co.gauntlet.eachmover" />
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(
            &dijkstra_map, idx, map
        )
        {// <callout id="co.gauntlet.findlowest" />
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);// <callout id="co.gauntlet.distance_to_player" />
            let destination = if distance > 1.2 {// <callout id="co.gauntlet.ifgr12" />
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };
    //END: chase

    //START: repeat
            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(target, _, _)| {
                    if ecs.entry_ref(*target).unwrap().get_component::<Player>()
                        .is_ok() {
                            commands
                                .push(((), WantsToAttack{ 
                                    attacker: *entity,
                                    target: *target
                                }));
                    }
                    attacked = true;
                });

            if !attacked {
                commands
                    .push(((), WantsToMove{ entity: *entity, destination }));
            }
        }
    });//END: repeat
}
