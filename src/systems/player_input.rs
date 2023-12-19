use crate::prelude::*;

//START: header
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]

pub fn player_input(
    //END: header
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            //START: get
            VirtualKeyCode::G => {
                // <callout id="co.inventory.match_g" />
                let (player, player_pos) = players // <callout id="co.inventory.players_destructure" />
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos))) // <callout id="co.inventory.player_find_map" />
                    .unwrap();

                let mut items = <(Entity, &Item, &Point)>::query(); // <callout id="co.inventory.item_query" />
                items
                    .iter(ecs)
                    .filter(|(_entity, _item, &item_pos)| item_pos == player_pos) // <callout id="co.inventory.item_filter" />
                    .for_each(|(entity, _item, _item_pos)| {
                        commands.remove_component::<Point>(*entity); // <callout id="co.inventory.no_point" />
                        commands.add_component(*entity, Carried(player)); // <callout id="co.inventory.add_carry" />
                    });
                Point::new(0, 0)
            }
            //END: get
            //START: numbers
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            //END: numbers
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut did_something = false;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    // did_something = true;

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        };
        *turn_state = TurnState::PlayerTurn;
    }
}

//START: use_item
fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    // <callout id="co.inventory.use_item_fn" />
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap(); // <callout id="co.inventory.find_the_player" />

    let item_entity = <(Entity, &Item, &Carried)>::query() // <callout id="co.inventory.item_numbering" />
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate() // <callout id="co.inventory.enumerate_carried" />
        .filter(|(item_count, (_, _, _))| *item_count == n) // <callout id="co.inventory.enum_filter" />
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity)); // <callout id="co.inventory.enum_fm" />

    if let Some(item_entity) = item_entity {
        // <callout id="co.inventory.if_let_pickup" />
        commands.push((
            (),
            ActivateItem {
                // <callout id="co.inventory.create_activate" />
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }

    Point::zero()
}
//END: use_item
