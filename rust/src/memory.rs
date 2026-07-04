use crate::world::World;

pub fn update(world: &mut World) {
    if world.swing_height.abs() > 1.5 {
        world.memory += 1;
    }
}
