use bevy::prelude::*;
use bevy::ui::Val::Px;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score_one: 0, score_two: 0 })
        .add_startup_system(startup.system())
        .add_system(paddle_movement_system.system())
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
}

fn startup(mut commands: Commands,
           mut materials: ResMut<Assets<ColorMaterial>>,
           asset_server: Res<AssetServer>) {
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
        .insert(Paddle { speed: 500.0, paddle_number:0 })
        .insert(Collider::Paddle);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz((bounds.x/2 as f32)-45 as f32, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 120.0)),
            ..Default::default()
        })
        .insert(Paddle { speed: 500.0, paddle_number:1 })
        .insert(Collider::Paddle);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
    // left
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Solid);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Solid);
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
                top: 1,
                left: 1,
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
        let mut direction = 0.0;
        if x.0.paddle_number == 1 {
            if keyboard_input.pressed(KeyCode::Down) {
                direction -= 1.0;
            }

            if keyboard_input.pressed(KeyCode::Up) {
                direction += 1.0;
            }
        }
        else {
            if keyboard_input.pressed(KeyCode::LControl) {
                direction -= 1.0;
            }

            if keyboard_input.pressed(KeyCode::LShift) {
                direction += 1.0;
            }
        }

        let translation = &mut x.1.translation;
        translation.y += time.delta_seconds() * direction * x.0.speed;
        translation.y = translation.y.min(240.0).max(-240.0);
    }
}
