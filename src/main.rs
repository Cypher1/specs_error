use specs::prelude::*;
use specs::Component;
use component_group::ComponentGroup;

#[derive(Component, Clone, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub dx: i32,
    pub dy: i32,
}

#[derive(Component, Clone, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(ComponentGroup)]
struct PhysicsItem {
    velocity: Velocity,
    position: Position,
}

fn main() -> Result<(), std::io::Error> {
    let mut world = World::new();
    world.register::<Velocity>();
    world.register::<Position>();
    world.insert((Velocity {dx: 1, dy: 0}, Position {x: 0, y: 0}));

    Ok(())
}
