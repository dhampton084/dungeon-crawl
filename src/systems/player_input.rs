use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn get_player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::S => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|(entity, pos)| {
                let destination = *pos + delta;
                commands.push(((), WantsToMove {
                    destination: destination,
                    entity: *entity,
                }));

            });
        }
        
        *turn_state = TurnState::PlayerTurn;
    }
}
