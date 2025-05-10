use bevy::prelude::*;
mod core_plugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, core_plugin::CorePlugin))
        .run();
}
