use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::First, remove);

    for stage in [
        CoreStage::First,
        CoreStage::PreUpdate,
        CoreStage::Update,
        CoreStage::PostUpdate,
        CoreStage::Last,
    ]
    .iter()
    {
        app.add_system_to_stage(stage.clone(), log_removed(stage.clone()));
    }

    app.run();
}

#[derive(Component)]
struct ToBeRemoved;

fn setup(mut commands: Commands) {
    commands.spawn().insert(ToBeRemoved);
}

fn remove(mut commands: Commands, q: Query<Entity, With<ToBeRemoved>>) {
    for e in q.iter() {
        commands.entity(e).remove::<ToBeRemoved>();
    }
}

fn log_removed(
    stage: CoreStage,
) -> impl Fn(Query<(), With<ToBeRemoved>>, RemovedComponents<ToBeRemoved>) {
    move |query, removed| {
        let count = removed.iter().len();
        let query_count = query.iter().count();

        if count > 0 || query_count > 0 {
            info!("Removed: {count}; Query count: {query_count} in {stage:?}");
        }
    }
}
