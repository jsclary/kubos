use bevy::prelude::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}

#[derive(Component)]
pub struct Server {}

fn setup(mut _commands: Commands) {
    //commands.add()
}

fn update(_time: Res<Time>) {}
