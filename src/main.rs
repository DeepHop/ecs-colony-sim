// ─────────────────────────────────────────────────────────────────────────────
// ⬇ Bevy prelude + plugin dependencies
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy::log::LogPlugin;
use bevy_egui::EguiPlugin;
use plugins::simulation_plugin::SimulationPlugin;
use bevy::diagnostic::{
    FrameTimeDiagnosticsPlugin,
    EntityCountDiagnosticsPlugin,
    LogDiagnosticsPlugin,
};

// ⬇ External plugins, resources, and ECS systems organized into submodules
mod core_plugin;    // General app setup (camera, clear color, resource init, etc.)
mod resources;      // Shared global resources (e.g. TimeState)
mod plugins;        // Game-specific plugins (e.g. SimulationPlugin)
mod systems;        // Game systems organized by domain (e.g. UI, AI, movement)

fn main() {
    App::new()
        .add_plugins((
            // ⬇ Base Bevy plugins with configured window + logging
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280., 720.)
                            .with_scale_factor_override(1.0),
                        title: "ECS Colony Sim".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    filter: "warn,ecs_colony_sim=trace".into(),
                    ..default()
                }),

            // ⬇ Custom plugins and diagnostics
            SimulationPlugin,                             // Handles simulation time progression
            EguiPlugin { enable_multipass_for_primary_context: true },  // GUI integration
            FrameTimeDiagnosticsPlugin::default(),        // FPS tracking
            EntityCountDiagnosticsPlugin::default(),      // Entity count debug
            LogDiagnosticsPlugin::default(),              // Periodic diagnostics logging
            core_plugin::CorePlugin,                      // Core game setup logic
        ))
        .run();
}
