use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    let default_plugins_with_settings = DefaultPlugins
        .set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        })
        .set(if cfg!(debug_assertions) {
            LogPlugin {
                filter: "warn,bevy_portal=debug".into(),
                level: Level::DEBUG,
            }
        } else {
            LogPlugin {
                filter: "warn".into(),
                level: Level::WARN,
            }
        });

    app.add_plugins(default_plugins_with_settings).run();
}
