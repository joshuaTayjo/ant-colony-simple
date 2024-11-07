use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 640.0;
const TOP: f32 = HEIGHT / 2.0;
const BOTTOM: f32 = -(HEIGHT / 2.0);
const RIGHT: f32 = WIDTH / 2.0;
const LEFT: f32 = -(WIDTH / 2.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut horiz = LEFT;

    while horiz < RIGHT {
        let line = Line::from_pts((horiz, TOP), (horiz, BOTTOM), 1.0);
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(line.mesh)),
            material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
            transform: line.transform,
            ..default()
        });

        horiz += 10.0;
    }

    let mut vert = BOTTOM;

    while vert < TOP {
        let line = Line::from_pts((LEFT, vert), (RIGHT, vert), 1.0);
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(line.mesh)),
            material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
            transform: line.transform,
            ..default()
        });

        vert += 10.0;
    }
}

// Simple function to close window when esc is pressed
fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window_ent, _focus) in &focused_windows {
        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window_ent).despawn();
        }
    }
}

#[derive(Component, Default)]
struct Line {
    transform: Transform,
    mesh: Rectangle,
}

impl Line {
    fn from_pts(start: (f32, f32), end: (f32, f32), stroke: f32) -> Self {
        // x and y component distances from the start to end point.
        let x = end.0 - start.0;
        let y = end.1 - start.1;
        // length via distance formula
        let length = ((x * x) + (y * y)).sqrt();
        //transform accounting for the transform positiong being relative to the center of the object in bevy
        let mut transform = Transform::from_xyz(start.0 + (x / 2.0), start.1 + (y / 2.0), 0.0);

        // angle of line based on offset fom x-axis
        let theta = (y / x).atan();
        transform.rotation = Quat::from_rotation_z(theta);

        Self {
            transform,
            mesh: Rectangle::new(length, stroke),
        }
    }
}
