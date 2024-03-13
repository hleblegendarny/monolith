use bevy::app::{App, Startup};
fn main() {
    let mut app = App::new();
    app.add_systems(Startup, hello_world);
    app.run();
}
fn hello_world()
{
    println!("[DEBUG]: Hello, World!");
}
