//START: boilerplate
use crate::prelude::*;

//START: query
#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    //END: query
    //END: boilerplate

    //START: target
    let targets : Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target) )
        .collect();
    //END: target

    //START: damage
    targets.iter().for_each(|(message, target)| {
        let is_player = ecs
            .entry_ref(*target)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*target);
            }
            //END: target
        }
        commands.remove(*message);
    });
    //END: damage
}