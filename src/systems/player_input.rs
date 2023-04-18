use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
) {
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());

    if let Some(key) = key {
        let delta: Point = match key {
            VirtualKeyCode::Left=> Point::new(-1, 0),
            VirtualKeyCode::Right=> Point::new(1, 0),
            VirtualKeyCode::Up=> Point::new(0,-1),
            VirtualKeyCode::Down=> Point::new(0, 1),
            _=> Point::new(0, 0)
        };

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let (player_entity, dest) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut did_something = false;
        
        if delta.x != 0 || delta.y != 0 {

            let mut hit_something = false;
            did_something = true;
            
            enemies
                .iter(ecs)
                .filter(|(_, pos)| {
                    **pos == dest
                })
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push(((), WantsToAttack{attcker: player_entity, victim: *entity}));
                });

            if ! hit_something {
                commands.push(((), WantsToMove{entity: player_entity, destination: dest }));    
            }

        }

        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current +1);
            }
        }

        *turn_state =TurnState::PlayerTurn;
    }
}