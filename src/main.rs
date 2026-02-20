const CAMSPEED: f32 = 40.;
const COLOR_ADJ: f32 = 1.5;
const CUTTER_SPEED: f32 = 0.3;
const CUBE_SIZE: f32 = 0.3;

use bevy::prelude::*;
#[derive(Component)]
enum Cutter {
    X,
    Z,
}
#[derive(Component)]
struct ArrayCube;
// use rayon::iter::IntoParallelIterator;
// use rayon::iter::ParallelIterator;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .init_resource::<Assets<Mesh>>()
        .add_systems(
            Startup,
            (add_cubes, add_camera, add_cutters, add_instructions),
        )
        .add_systems(Update, (move_camera, move_cutter))
        .run();
}
fn add_instructions(mut commands: Commands) {
    commands.spawn(Text::new("use WASD and RF to move forward/left/back/down and to rise/fall. use arrow keys to control the cutters."));
}
fn move_cutter(
    cutter_query: Query<(&Cutter, &mut Transform), Without<ArrayCube>>,
    input: Res<ButtonInput<KeyCode>>,
    cubes: Query<(&mut Visibility, &Transform), With<ArrayCube>>,
) {
    let mut delta = Vec3::default();
    if input.pressed(KeyCode::ArrowLeft) {
        delta.x += 1.0 * CUTTER_SPEED;
    }
    if input.pressed(KeyCode::ArrowRight) {
        delta.x -= 1.0 * CUTTER_SPEED;
    }
    if input.pressed(KeyCode::ArrowUp) {
        delta.z += 1.0 * CUTTER_SPEED;
    }
    if input.pressed(KeyCode::ArrowDown) {
        delta.z -= 1.0 * CUTTER_SPEED;
    }
    // return early if no input
    if delta == Vec3::default() {
        return;
    }
    let mut bounds = Vec3::default();
    for (dimension, mut transf) in cutter_query {
        match dimension {
            Cutter::X => {
                transf.translation.x = (transf.translation.x + delta.x).clamp(-30.0, 30.0);
                bounds.x = transf.translation.x;
            }
            Cutter::Z => {
                transf.translation.z = (transf.translation.z + delta.z).clamp(-30., 30.);
                bounds.z = transf.translation.z;
            }
        }
    }
    for (mut cube_vis, cube_transform) in cubes {
        if cube_transform.translation.x > bounds.x || cube_transform.translation.z > bounds.z {
            *cube_vis = Visibility::Hidden;
        } else {
            *cube_vis = Visibility::Visible;
        }
    }
}
fn solve_quadratic<T>(a: T, b: T, c: T) -> (f32, Option<(f32, bool)>)
where
    T: Into<f32>,
{
    let (a, b, c) = (a.into(), b.into(), c.into());
    let disc: f32 = b.powi(2) - (4. * a * c);
    if disc == 0.0 {
        // one real solution, which can never be imaginary
        (-b / (2.0 * a), None)
    } else {
        if disc < 0.0 {
            // two imaginary solution
            (
                (-b + disc.abs().sqrt()) / (2.0 * a),
                Some(((-b - disc.abs().sqrt()) / (2.0 * a), true)),
            )
        } else {
            // disc > 0.0
            // two real solutions
            (
                (-b + disc.sqrt()) / (2.0 * a),
                Some(((-b - disc.sqrt()) / (2.0 * a), false)),
            )
        }
    }
}
fn move_camera(
    cam_query: Query<&mut Transform, With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut cam in cam_query {
        let mut moved = false;
        if keys.pressed(KeyCode::KeyD) {
            let x = cam.local_x();
            cam.translation += x * CAMSPEED * time.delta_secs();
            moved = true;
        }
        if keys.pressed(KeyCode::KeyA) {
            let x = cam.local_x();

            cam.translation -= x * CAMSPEED * time.delta_secs();
            moved = true;
        }
        if keys.pressed(KeyCode::KeyW) {
            let z = cam.local_z();
            cam.translation -= z * CAMSPEED * time.delta_secs();
            moved = true;
        }
        if keys.pressed(KeyCode::KeyS) {
            let z = cam.local_z();
            cam.translation += z * CAMSPEED * time.delta_secs();
            moved = true;
        }
        if keys.pressed(KeyCode::KeyR) {
            cam.translation.z += CAMSPEED * time.delta_secs();
            moved = true;
        }
        if keys.pressed(KeyCode::KeyF) {
            cam.translation.z -= CAMSPEED * time.delta_secs();
            moved = true;
        }

        if moved {
            cam.look_at(Vec3::splat(0.), Dir3::Z);
        }
    }
}

fn add_cutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut x_transform = Transform::from_translation(Vec3::from_array([30., 0., 0.]));
    x_transform.rotate_x(std::f32::consts::PI / 2.0);
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(LinearRgba::rgb(1.0, 0.0, 0.)))),
        x_transform,
        Cutter::X,
    ));
    let mut z_transform = Transform::from_translation(Vec3::from_array([0.0, 0., 30.0]));
    z_transform.rotate_z(std::f32::consts::PI / 2.0);
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(LinearRgba::rgb(0.0, 0.0, 1.0)))),
        z_transform,
        Cutter::Z,
    ));
}
fn add_camera(mut commands: Commands) {
    let camera = Camera3d::default();
    commands.spawn((
        camera,
        Transform::from_xyz(100., 100.0, 100.0).looking_at([0., 0., 0.].into(), Dir3::Z),
    ));
}
fn add_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for a in -20..20 {
        for b in -20..20 {
            for c in -20..20 {
                let mut red: f32;
                let green: f32;
                let mut blue: f32;
                (red, green, blue) = match solve_quadratic(a as f32, b as f32, c as f32) {
                    (r1, None) => (r1, 1.0, 1.0),
                    (r1, Some((r2, imag))) => (r1, if imag { 1.0 } else { 0.0 }, r2),
                };
                (red, blue) = ((red * COLOR_ADJ), (blue * COLOR_ADJ));
                commands.spawn((
                    Transform::from_xyz(a as f32, b as f32, c as f32),
                    Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(CUBE_SIZE)))),
                    MeshMaterial3d(
                        materials.add(StandardMaterial::from_color(Color::LinearRgba(
                            LinearRgba::rgb(red, green, blue),
                        ))),
                    ),
                    Visibility::Visible,
                    ArrayCube,
                ));
            }
        }
    }
}
