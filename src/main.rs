use bevy::prelude::MonitorSelection::Primary;
use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod game;
pub mod pieces;

use game::GamePlugin;
use pieces::PiecesPlugin;

fn main() {
    // Important to keep the correct order

    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess in Rust".into(),
                position: WindowPosition::Centered(Primary),
                resolution: (900.0, 900.0).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }),))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(PiecesPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, set_up_bevy)
        .add_systems(Update, update)
        .run();
}

fn set_up_bevy(mut commands: Commands) {
    commands.spawn((
        Camera2d { ..default() },
        Transform::from_xyz(450.0, 450.0, 999.0),
    ));

    commands.spawn((
        Text::new("test \ntest"),
        TextFont {
            font_size: 11.0,
            ..Default::default()
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(110.0),
            height: Val::Px(80.0),

            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
    ));
}

fn update() {}
