use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "warn,bevy_portal=debug".into(),
        level: Level::DEBUG,
    }));

    #[cfg(not(debug_assertions))]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "warn".into(),
        level: Level::WARN,
    }));

    app.run();
}
