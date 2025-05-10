use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_egui::EguiContextPass;
use crate::resources::time_state::{TimeState, Season};
use crate::systems::ui::draw_time_state_ui;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::from(Srgba::rgb_u8(25, 25, 25))))
            .insert_resource(TimeState {
                tick: 0,
                day: 1,
                season: Season::Spring,
                year: 1,
                paused: false,
            })
            .add_systems(Startup, setup_camera)
            .add_systems(EguiContextPass, draw_time_state_ui);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: false,
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 2.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0.0, 0.0, 999.0),
        GlobalTransform::default(),
    ));
}
