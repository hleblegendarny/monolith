#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{
    prelude::*, window::WindowResolution};
fn main () {
    let mut app = App::new();
    //plugins
    app.add_plugins(DefaultPlugins.set(WindowPlugin
        {
            primary_window: Some(Window
            {
                resolution: WindowResolution::new(1280.,720.),
                ..default()
            }),
            ..default()
        }));
    //systems
    app.add_systems(Startup, render);
    //start
    app.run();
}
fn render (mut commands: Commands) {
    #[derive(Component)]
    struct Camera2d;
    //inner box
    #[derive(Component)]
    struct MessageBox;
    //outter box
    #[derive(Component)]
    struct ChatBox;
    #[derive(Component)]
    //Buttons enum
    enum Button {
        Red,
        Green,
        Yellow,
        Blue
    }
    //spawns Camera
    commands.spawn((Camera2dBundle{
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::hsla(0., 0., 0., 1.)), 
            ..default()
        },
        ..default()
    },Camera2d));
}