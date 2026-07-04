mod world;
mod swing;
mod memory;
mod observer;
mod weather;
mod render;
mod sprite;

use world::World;

fn main() {
    println!("🌳 Haunted Swing");
    let mut world = World::new();
    for day in 1..=30 {
        println!("\nDay {}", day);
        world.step();
        render::draw(&world);
    }
}
