use bevy::app::{App, Startup};
use bevy::DefaultPlugins;
use bevy::math::{Vec2, Vec3};
use bevy::pbr::{PbrBundle, PointLightBundle, StandardMaterial};
use bevy::prelude::{Assets, Camera3dBundle, ClearColor, Color, Commands, Component, Cuboid, default, Entity, Mesh, PointLight, Query, ResMut, Transform, With};
use bevy::window::PrimaryWindow;

use bevy_flurx_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin
        ))
        .insert_resource(ClearColor(Color::GRAY))
        .add_systems(Startup, (
            spawn_webview,
            spawn_shape
        ))
        .run();
}

#[derive(Component)]
struct WebviewWindow;

fn spawn_webview(
    mut commands: Commands,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.spawn((
        WebviewWindow,
        WryWebViewBundle {
            // uri: Uri::LocalRoot("api".to_string()),
            uri: Uri::Remote("https://bevyengine.org/".to_string()),
            use_devtools: UseDevtools(true),
            is_open_devtools: IsOpenDevtools(true),
            ..default()
        },
        AsChild {
            parent: ParentWindow(primary_window.single()),
            bounds: Bounds {
                position: Vec2::new(100., 100.),
                size: Vec2::new(500., 500.),
                ..default()
            },
            resizable: Resizable(true),
        },
    ));
}

fn spawn_shape(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(3., 3., 3.)),
        material: materials.add(Color::MIDNIGHT_BLUE),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}