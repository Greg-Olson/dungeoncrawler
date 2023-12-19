//START: use
use crate::prelude::*;
use serde::Deserialize;// <callout id="co.loot.deserialize" />
use ron::de::from_reader;// <callout id="co.loot.ronreader" />
use std::fs::File;// <callout id="co.loot.stdfile" />
use std::collections::HashSet;
use legion::systems::CommandBuffer;
//END: use

//START: ddo
#[derive(Clone, Deserialize, Debug)]// <callout id="co.loot.derive_serde" />
pub struct Template {// <callout id="co.loot.template" />
    pub entity_type : EntityType,// <callout id="co.loot.etype_enum" />
    pub levels : HashSet<usize>,// <callout id="co.loot.hashset" />
    pub frequency : i32,
    pub name : String,
    pub glyph : char,
    pub provides : Option<Vec<(String, i32)>>,// <callout id="co.loot.provides" />
    pub hp : Option<i32>
}

#[derive(Clone, Deserialize, Debug, PartialEq)]// <callout id="co.loot.derive_enum" />
pub enum EntityType {
    Enemy, Item
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {// <callout id="co.loot.templates" />
    pub entities : Vec<Template>,
}
//END: ddo

//START: load
impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron")// <callout id="co.loot.file_open" />
            .expect("Failed opening file");
        from_reader(file).expect("Unable to load templates")// <callout id="co.loot.reader_expect" />
    }
    //END: load

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point]
    ) {
        //START: available
        let mut available_entities = Vec::new();// <callout id="co.loot.make_available" />
        self.entities
            .iter()// <callout id="co.loot.entities_iter" />
            .filter(|e| e.levels.contains(&level))// <callout id="co.loot.entities_filter" />
            .for_each(|t| {
                for _ in 0 .. t.frequency {// <callout id="co.loot.frequency" />
                    available_entities.push(t);
                }
            }
        );
        //END: available

        //START: call_spawn
        let mut commands = CommandBuffer::new(ecs);// <callout id="co.loot.new_cmd_buffer" />
        spawn_points.iter().for_each(|pt| {// <callout id="co.loot.iter_pts" />
            if let Some(entity) = rng.random_slice_entry(&available_entities) {// <callout id="co.loot.rse" />
                self.spawn_entity(pt, entity, &mut commands);// <callout id="co.loot.spawn_entity_call" />
            }
        });
        commands.flush(ecs);
    }
    //END: call_spawn

    //START: spawn
    fn spawn_entity(
        &self,
        pt: &Point,
        template: &Template,
        commands: &mut CommandBuffer
    ) {
        //START: common
        let entity = commands.push((// <callout id="co.loot.push" />
            pt.clone(),// <callout id="co.loot.ptclone" />
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph)// <callout id="co.loot.glyph" />
            },
            Name(template.name.clone())// <callout id="co.loot.name" />
        ));
        //END: common
        //START: matcher
        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item{}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy{});
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer{});
                commands.add_component(entity, Health{
                    current: template.hp.unwrap(),
                    max: template.hp.unwrap()
                });
            }
        }
        //END: matcher
        //START: effects
        if let Some(effects) = &template.provides {
            effects.iter().for_each(|(provides, n)| {
                match provides.as_str() {
                    "Healing" => commands.add_component(entity,
                        ProvidesHealing{ amount: *n}),
                    "MagicMap" => commands.add_component(entity,
                        ProvidesDungeonMap{}),
                    _ => {
                        println!("Warning: we don't know how to provide {}"
                            , provides);
                    }
                }
            });
        }
    }
    //END: effects
    //END: spawn
}