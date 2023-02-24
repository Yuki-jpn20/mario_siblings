use bevy::{
    prelude::*,
    time::FixedTimestep,
    sprite::collide_aabb::{collide, Collision},
};

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.050))
                .with_system(move_mario)
                .with_system(check_for_collisions.after(move_mario))
                .with_system(gravity.after(check_for_collisions)),
        )
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .run();
}

const BOTTOM_WALL: f32 = -300.;
const FLOOR1: f32 = -100.;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const MARIO_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct Mario{
    floor: Entity
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    
    let back = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::NONE,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(800.0, 800.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Floor)
        .insert(Collider)
        .id();

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, BOTTOM_WALL, 0.0),
                scale: Vec3::new(800.0, 30.0, 30.0),
                ..default()
            },
            ..default()
        })
        .insert(Floor)
        .insert(Collider);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-300.0, FLOOR1, 0.0),
                scale: Vec3::new(200.0, 30.0, 30.0),
                ..default()
            },
            ..default()
        })
        .insert(Floor)
        .insert(Collider);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: MARIO_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -250.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 30.0),
                ..default()
            },
            ..default()
        })
        .insert(Mario{floor: back})
        .insert(Velocity(Vec3::new(0.0, -1.0, 0.0)))
        .insert(Collider);
}

fn move_mario(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, & Mario), With<Mario>>,
) {
    let (mut velocity, mut transform, mario) = query.single_mut();
    let mut xdirection = 0.0;
    let mut ydirection = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        xdirection -= 10.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        xdirection += 10.0;
    }

    if keyboard_input.pressed(KeyCode::Up) && mario.floor.id() != 1 {
        velocity.y = 30.0;
        ydirection += 10.0;
    }

    let new_xposition = transform.translation.x + xdirection;
    let new_yposition = transform.translation.y + ydirection;

    transform.translation.x = new_xposition;
    transform.translation.y = new_yposition;
}

fn gravity(
    mut query: Query<(&mut Velocity, &mut Transform, &Mario), With<Mario>>,
) {
    let (mut velocity, mut transform, mario) = query.single_mut();

    if mario.floor.id() == 1 && velocity.y > -15.0 {
        let gravity = -2.0;
        velocity.y = velocity.y + gravity;
    }
    transform.translation.y = transform.translation.y + velocity.y;
}

fn check_for_collisions(
    mut mario_query: Query<(&mut Velocity, &Transform, &mut Mario), With<Mario>>,
    collider_query: Query<(Entity, &Transform, Option<&Floor>), With<Collider>>,
) {
    let (mut mario_velocity, mario_transform, mut mario) = mario_query.single_mut();

    for (collider_entity, collider_transform, maybe_floor) in &collider_query {
        let collision = collide(
            mario_transform.translation,
            mario_transform.scale.truncate(),
            collider_transform.translation,
            collider_transform.scale.truncate(),
        );
        if let Some(collision) = collision {

            if maybe_floor.is_some() {
                match collision {
                    Collision::Left => {}
                    Collision::Right => {}
                    Collision::Top => {
                        mario_velocity.y = 0.0;
                        mario.floor = collider_entity;
                    }
                    Collision::Bottom => {
                        mario_velocity.y = -2.0;
                    }
                    Collision::Inside => {
                        mario.floor = collider_entity;
                    }
                }
            }
        }
    }
}