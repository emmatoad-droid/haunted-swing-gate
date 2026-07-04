use rand::Rng;
use crate::world::World;

pub fn update(world: &mut World) {
    let mut rng = rand::thread_rng();
    world.wind = rng.gen_range(-3..=3);
}
