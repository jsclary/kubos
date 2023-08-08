use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use winit::window::Icon;

#[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
use bevy_console::{ConsoleConfiguration, ConsolePlugin};

mod chunk;
mod client;
mod lobby;
mod server;
mod skybox;
use client::ClientPlugin;
use lobby::LobbyPlugin;
use server::ServerPlugin;
use skybox::SkyboxPlugin;

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
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(PlayerLastPitch(0.0))
        .insert_resource(PlayerLastYaw(0.0))
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
            SkyboxPlugin,
            LobbyPlugin,
            ServerPlugin,
            ClientPlugin,
            ScreenDiagnosticsPlugin::default(), // TODO: Use one of our fonts?
            ScreenFrameDiagnosticsPlugin,
            EguiPlugin,
        ))
        .add_state::<AppState>()
        .add_systems(Startup, (create_ground_plane, set_window_icon));

    // TODO: This ensures everything builds properly with bevy_egui. It can be removed after
    // https://github.com/mvlabat/bevy_egui/issues/198 or once we've added our own egui code.
    #[cfg(any(target_os = "android", target_os = "ios", target_arch = "wasm32"))]
    app.add_systems(Update, ui_example_system);

    // TODO: This condition can be removed if https://github.com/mvlabat/bevy_egui/issues/198 is resolved.
    #[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
    app.insert_resource(Msaa::Sample4)
        .add_plugins(ConsolePlugin)
        .insert_resource(ConsoleConfiguration { ..default() });

    app.run();
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

    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    transform.scale = Vec3::new(1000.0, 1.0, 1000.0);

    commands.spawn(PbrBundle {
        mesh,
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform,
        ..Default::default()
    });
}

#[derive(Resource)]
struct PlayerLastPitch(f32);
#[derive(Resource)]
struct PlayerLastYaw(f32);

#[cfg(feature = "spectator")]
fn update_compass(
    /*mut commands: Commands,*/ mut query: Query<&mut Transform, With<Spectator>>,
    mut last_pitch: ResMut<PlayerLastPitch>,
    mut last_yaw: ResMut<PlayerLastYaw>,
) {
    if let Some(transform) = query.single_mut().into() {
        // Convert transform to a rotation around the y axis in degrees
        let quat = transform.rotation;
        //let yaw = (2.0 * (quat.w * quat.z + quat.x * quat.y)).atan2(1.0 - 2.0 * (quat.y.powi(2) + quat.z.powi(2)));
        let yaw = quat.y.atan2(quat.w) * 2.0;
        let yaw = yaw * 180.0 / std::f32::consts::PI;

        let pitch = (2.0 * (quat.w * quat.x - quat.y * quat.z)).asin();
        let pitch = pitch * 180.0 / std::f32::consts::PI;

        if (last_pitch.0 - pitch).abs() > 0.01 || (last_yaw.0 - yaw).abs() > 0.01 {
            info!("yaw {yaw}, pitch {pitch}");
        }
        last_pitch.0 = pitch;
        last_yaw.0 = yaw;
    }
}

pub fn set_window_icon(
    main_window: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    // TODO: This should work on X Windows, too, but it's unclear if cfg!(unix) would be a sufficient qualifier
    // given that it is possible to run bevy under SDL2. Android complains about it, though.
    if cfg!(windows) {
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
}

// Test code for Egui to determine if things will build properly on different targets.
fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
