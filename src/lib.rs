use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use bevy_atmosphere::prelude::*;
//use bevy_console::{ConsoleConfiguration, ConsolePlugin};
use bevy_spectator::{Spectator, SpectatorPlugin};
use winit::window::Icon;

mod chunk;
mod client;
mod lobby;
mod server;
use client::ClientPlugin;
use lobby::LobbyPlugin;
use server::ServerPlugin;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

#[cfg(target_os = "android")]
#[bevy_main]
fn main() {
    entry_point()
}

#[no_mangle]
pub fn entry_point() {
    App::new()
        /* Necessary for atmosphere plugin
        .insert_resource(Msaa::Sample4)
        .insert_resource(AtmosphereModel::default()) // Default Atmosphere material, we can edit it to simulate another planet
        .insert_resource(CycleTimer(Timer::new(
            bevy::utils::Duration::from_millis(50), // Update our atmosphere every 50ms (in a real game, this would be much slower, but for the sake of an example we use a faster update)
            TimerMode::Repeating,
        )))
        */
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "κύβος".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
            //ConsolePlugin,
            //AtmospherePlugin,
            SpectatorPlugin,
            LobbyPlugin,
            ServerPlugin,
            ClientPlugin,
        ))
        .add_state::<AppState>()
        //.insert_resource(ConsoleConfiguration { ..default() })
        .add_systems(Startup, setup)
        .add_systems(Startup, create_ground_plane)
        //.add_systems(Update, change_nishita)
        //.add_systems(Update, daylight_cycle)
        .add_systems(Update, update_compass)
        //.add_systems(Startup, set_window_icon)
        .run();
}

fn setup(mut commands: Commands) {
    // Our Sun
    commands.spawn((
        DirectionalLightBundle {
            ..Default::default()
        },
        Sun, // Marks the light as Sun
    ));

    commands.spawn((
        Camera3dBundle::default(),
        AtmosphereCamera::default(),
        Spectator,
    ));
}

// Marker for updating the position of the light, not needed unless we have multiple lights
#[derive(Component)]
struct Sun;

// Timer for updating the daylight cycle (updating the atmosphere every frame is slow, so it's better to do incremental changes)
#[derive(Resource)]
struct CycleTimer(Timer);

// We can edit the Atmosphere resource and it will be updated automatically
fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let t = time.elapsed_seconds_wrapped() / (2.0 * 140.);
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * 100000.0;
        }
    }
}

fn change_nishita(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key1) {
        info!("Changed to Atmosphere Preset 1 (Sunset)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            sun_position: Vec3::new(0., 0., -1.),
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key2) {
        info!("Changed to Atmosphere Preset 2 (Noir Sunset)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            sun_position: Vec3::new(0., 0., -1.),
            rayleigh_coefficient: Vec3::new(1e-5, 1e-5, 1e-5),
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key3) {
        info!("Changed to Atmosphere Preset 3 (Magenta)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            rayleigh_coefficient: Vec3::new(2e-5, 1e-5, 2e-5),
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key4) {
        info!("Changed to Atmosphere Preset 4 (Strong Mie)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            mie_coefficient: 5e-5,
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key5) {
        info!("Changed to Atmosphere Preset 5 (Larger Scale)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            rayleigh_scale_height: 16e3,
            mie_scale_height: 2.4e3,
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key6) {
        info!("Changed to Atmosphere Preset 6 (Weak Intensity)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            sun_intensity: 11.0,
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key7) {
        info!("Changed to Atmosphere Preset 7 (Half Radius)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            ray_origin: Vec3::new(0., 6372e3 / 2., 0.),
            planet_radius: 6371e3 / 2.,
            atmosphere_radius: 6471e3 / 2.,
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key8) {
        info!("Changed to Atmosphere Preset 8 (Sideways World)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            ray_origin: Vec3::new(6372e3, 0., 0.),
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key9) {
        info!("Changed to Atmosphere Preset 9 (Inverted Mie Direction)");
        commands.insert_resource(AtmosphereModel::new(Nishita {
            mie_direction: -0.758,
            ..default()
        }));
    } else if keys.just_pressed(KeyCode::Key0) {
        info!("Reset Atmosphere to Default");
        commands.remove_resource::<AtmosphereModel>();
    }
}

fn create_ground_plane(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1000.0,
        ..default()
    }));

    let mut transform = Transform::from_translation(Vec3::new(0.0, -1.0, 0.0));
    transform.scale = Vec3::new(1000.0, 1.0, 1000.0);

    commands.spawn(PbrBundle {
        mesh,
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform,
        ..Default::default()
    });
}

fn update_compass(/*mut commands: Commands,*/ mut query: Query<&mut Transform, With<Spectator>>,) {
    if let Some(transform) = query.single_mut().into() {
        // Convert transform to a rotation around the y axis in degrees
        let quat = transform.rotation;
        //let yaw = (2.0 * (quat.w * quat.z + quat.x * quat.y)).atan2(1.0 - 2.0 * (quat.y.powi(2) + quat.z.powi(2)));
        let yaw = quat.y.atan2(quat.w) * 2.0;
        let yaw = yaw * 180.0 / std::f32::consts::PI;

        let pitch = (2.0 * (quat.w * quat.x - quat.y * quat.z)).asin();
        let pitch = pitch * 180.0 / std::f32::consts::PI;

        info!("yaw {yaw}, pitch {pitch}");
    }
}

pub fn set_window_icon(
    main_window: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let Some(primary) = windows.get_window(main_window.single()) else {
        return;
    };

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/kubos.ico")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}
