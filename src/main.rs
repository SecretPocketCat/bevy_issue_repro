use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::Last, hack_z)
        .run();
}

#[derive(Component)]
struct UiZIndex(f32);

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(UiCameraBundle::default());

    cmd.spawn_bundle(ImageBundle {
        color: Color::RED.into(),
        style: Style {
            size: Size::new(Val::Px(100.), Val::Px(100.)),
            margin: Rect::all(Val::Auto),
            ..default()
        },
        ..default()
    })
    .insert(UiZIndex(2.))
    .with_children(|b| {
        b.spawn_bundle(ImageBundle {
            color: Color::GREEN.into(),
            style: Style {
                size: Size::new(Val::Px(100.), Val::Px(100.)),
                position: Rect {
                    left: Val::Px(25.),
                    bottom: Val::Px(25.),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|b| {
            b.spawn_bundle(ImageBundle {
                color: Color::BLUE.into(),
                style: Style {
                    size: Size::new(Val::Px(100.), Val::Px(100.)),
                    position: Rect {
                        left: Val::Px(25.),
                        bottom: Val::Px(25.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .insert(UiZIndex(3.))
            .with_children(|b| {
                b.spawn_bundle(ImageBundle {
                    color: Color::CRIMSON.into(),
                    style: Style {
                        size: Size::new(Val::Px(100.), Val::Px(100.)),
                        position: Rect {
                            left: Val::Px(25.),
                            bottom: Val::Px(25.),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                });
            });
        });
    });
}

fn hack_z(mut q: Query<(&UiZIndex, &mut GlobalTransform)>) {
    for (z, mut t) in q.iter_mut() {
        t.translation.z = z.0;
    }
}
