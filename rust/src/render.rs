use crate::world::World;

pub fn draw(world: &World) {
    println!("Wind      : {}", world.wind);
    println!("Swing     : {:.2}", world.swing_height);
    println!("Memory    : {}", world.memory);
    println!("Observer  : {}", world.observer);
}
