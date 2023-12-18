//START: boilerplate
use crate::prelude::*;

//START: query
#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    //END: query
    //END: boilerplate

    //START: target
    let target : Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target) )
        .collect();
    //END: target

    //START: damage
    target.iter().for_each(|(message, target)| {
        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*target);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    });
    //END: damage
}