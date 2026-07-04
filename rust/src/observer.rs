use crate::world::World;

pub fn update(world: &mut World) {
    world.observer =
        if world.swing_height.abs() > 2.5 {
            "Someone noticed."
        } else {
            "Only the trees."
        }.into();
}
