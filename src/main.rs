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
const OFFSET: f32 = 10.0;

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
        .add_systems(Update, (move_ant, show_ant_location, close_on_esc))
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

        horiz += OFFSET;
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

        vert += OFFSET;
    }

    for i in 0..((WIDTH / OFFSET) as usize) {
        for j in 0..((HEIGHT / OFFSET) as usize) {
            let x_pos = LEFT + (i as f32 * OFFSET) + (OFFSET / 2.0);
            let y_pos = BOTTOM + (j as f32 * OFFSET) + (OFFSET / 2.0);
            commands.spawn(Cell { x: x_pos, y: y_pos });
        }
    }

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(5.0, 10.0))),
            material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        },
        Ant {
            movement_speed: 30.0,
            rotation_speed: f32::to_radians(45.0),
        },
    ));
}

fn move_ant(
    mut ant_query: Query<(&mut Ant, &mut Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (ant, mut transform) = ant_query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        movement_factor += 1.0;
    };
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        movement_factor -= 1.0;
    };
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        rotation_factor += 1.0;
    };
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        rotation_factor -= 1.0;
    };

    transform.rotate_z(rotation_factor * ant.rotation_speed * time.delta_seconds());
    let movement_direction = transform.rotation * Vec3::Y;
    let movement_distance = movement_factor * ant.movement_speed * time.delta_seconds();
    let translation_delta = movement_direction * movement_distance;
    transform.translation += translation_delta;
}

fn show_ant_location(
    mut ant_query: Query<&mut Transform, (With<Ant>, Without<Marker>)>,
    cell_query: Query<&Cell>,
    mut marker_query: Query<(Entity, &mut Transform), With<Marker>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ant_transform = ant_query.single_mut();
    for cell in cell_query.iter() {
        let ant_location = ant_transform.translation.xy();

        if cell.contains((ant_location.x, ant_location.y)) {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(OFFSET, OFFSET))),
                    material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
                    transform: Transform::from_xyz(cell.x, cell.y, 1.0),
                    ..default()
                },
                Marker,
            ));
            for (ent_id, transform) in marker_query.iter_mut() {
                if !cell.contains(transform.translation.xy().into()) {
                    commands.entity(ent_id).despawn();
                };
            }
        };
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

#[derive(Component)]
struct Ant {
    movement_speed: f32,
    rotation_speed: f32,
}

#[derive(Component, Default)]
struct Cell {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Marker;

enum Direction {
    N,
    NW,
    W,
    SW,
    S,
    SE,
    E,
    NE,
}

impl Direction {
    fn to_radians(&self) -> f32 {
        match self {
            Self::N => f32::to_radians(90.0),
            Self::NW => f32::to_radians(45.0),
            Self::W => f32::to_radians(0.0),
            Self::SW => f32::to_radians(315.0),
            Self::S => f32::to_radians(270.0),
            Self::SE => f32::to_radians(225.0),
            Self::E => f32::to_radians(180.0),
            Self::NE => f32::to_radians(135.0),
        }
    }
}

impl Cell {
    fn contains(&self, point: (f32, f32)) -> bool {
        let half_offset = OFFSET / 2.0;
        let left_edge = self.x - half_offset;
        let right_edge = self.x + half_offset;
        let bottom_edge = self.y - half_offset;
        let top_edge = self.y + half_offset;

        (point.0 >= left_edge && point.0 < right_edge)
            && (point.1 >= bottom_edge && point.1 < top_edge)
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
        // z-axis runs front to back so we always rotate about it in 2D
        let theta = (y / x).atan();
        transform.rotation = Quat::from_rotation_z(theta);

        Self {
            transform,
            mesh: Rectangle::new(length, stroke),
        }
    }
}
