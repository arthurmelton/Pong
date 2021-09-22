use bevy::prelude::*;
use bevy::ui::Val::Px;
use bevy::sprite::collide_aabb::{Collision, collide};
use rand::seq::SliceRandom;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: 920.0,
            height: 620.0,
            ..Default::default()
        })
        .insert_resource(Scoreboard { score_one: 0, score_two: 0 })
        .add_startup_system(startup.system())
        .add_system(paddle_movement_system.system())
        .add_system(ball_collision_system.system())
        .add_system(ball_movement_system.system())
        .add_system(scoreboard_system.system())
        .run();
}

struct Paddle {
    speed: f32,
    paddle_number:i32,
}

struct Ball {
    velocity: Vec3,
}

struct Scoreboard {
    score_one: i32,
    score_two: i32,
}

enum Collider {
    Solid,
    Paddle,
    Right,
    Left,
}

fn startup(mut commands: Commands,
           mut materials: ResMut<Assets<ColorMaterial>>,
           asset_server: Res<AssetServer>,
            mut windows: ResMut<Windows>) {
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz((-bounds.x/2 as f32)+45 as f32, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 120.0)),
            ..Default::default()
        })
        .insert(Paddle { speed: 300.0, paddle_number:0 })
        .insert(Collider::Paddle);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz((bounds.x/2 as f32)-45 as f32, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 120.0)),
            ..Default::default()
        })
        .insert(Paddle { speed: 300.0, paddle_number:1 })
        .insert(Collider::Paddle);
    use rand::seq::SliceRandom;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Ball {
            velocity: 600.0 * Vec3::new(*vec![0.5, -0.5].choose(&mut rand::thread_rng()).unwrap(), *vec![0.5, -0.5].choose(&mut rand::thread_rng()).unwrap(), 0.0).normalize(),
        });
    // left
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Left);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Right);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Solid);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material,
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Solid);
    let mut window = windows.get_primary_mut().unwrap();
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "0 - 0".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Relative,
            position: Rect {
                top: Px(-5.0),
                left: Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for mut x in query.iter_mut() {
        let mut Direction = 0.0;
        if x.0.paddle_number == 1 {
            if keyboard_input.pressed(KeyCode::Down) {
                Direction -= 1.0;
            }

            if keyboard_input.pressed(KeyCode::Up) {
                Direction += 1.0;
            }
        }
        else {
            if keyboard_input.pressed(KeyCode::LControl) {
                Direction -= 1.0;
            }

            if keyboard_input.pressed(KeyCode::LShift) {
                Direction += 1.0;
            }
        }
        let translation = &mut x.1.translation;
        translation.y += time.delta_seconds() * Direction * x.0.speed;
        translation.y = translation.y.min(240.0).max(-240.0);
    }
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!("{} - {}", scoreboard.score_two, scoreboard.score_one);
}

fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    if let Ok((ball, mut transform)) = ball_query.single_mut() {
        if ball.velocity.y != 0.0 {
            transform.translation += ball.velocity * delta_seconds;
        }
        else {
            transform.translation.y = 0.0;
            transform.translation.x = 0.0;
        }
    }
}

fn ball_collision_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    if let Ok((mut ball, ball_transform, sprite)) = ball_query.single_mut() {
        let ball_size = sprite.size;
        let velocity = &mut ball.velocity;

        if velocity.x == 0.0 {
            velocity.x = *vec![300.0, -300.0].choose(&mut rand::thread_rng()).unwrap();
            velocity.y = *vec![300.0, -300.0].choose(&mut rand::thread_rng()).unwrap();
        }
        
        // check collision with walls
        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = velocity.x > 0.0,
                    Collision::Right => reflect_x = velocity.x < 0.0,
                    Collision::Top => reflect_y = velocity.y < 0.0,
                    Collision::Bottom => reflect_y = velocity.y > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    velocity.x = -velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    velocity.y = -velocity.y;
                }

                if let Collider::Left = *collider {
                    scoreboard.score_one += 1;
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                }
                if let Collider::Right = *collider {
                    scoreboard.score_two += 1;
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                }
            }
        }
    }
}
