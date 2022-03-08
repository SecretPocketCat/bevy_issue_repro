use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct ScaledSprite;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("icon.png"),
        transform: Transform {
            translation: Vec3::splat(0.),
            scale: Vec3::ONE,
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("icon.png"),
        transform: Transform {
            translation: Vec3::new(50., 50., 1.),
            scale: Vec3::ONE,
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::GREEN,
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn()
        .insert(Transform {
            scale: Vec3::new(1., 1., 0.),
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .with_children(|b| {
            b.spawn_bundle(SpriteBundle {
                texture: assets.load("icon.png"),
                transform: Transform {
                    translation: Vec3::new(100., 100., 2.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::BLUE,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
