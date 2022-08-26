use bevy::prelude::*;

fn hello_world_system() {
    println!("Hello, world!");
}

fn main() {
    App::new().add_startup_system(hello_world_system).run();
}
