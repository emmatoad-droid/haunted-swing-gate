use crate::world::World;

pub fn update(world: &mut World) {
    world.swing_height += world.wind as f32 * 0.15;
    world.swing_height *= 0.92;
    if world.swing_height.abs() < 0.01 {
        world.swing_height = 0.0;
    }
}
