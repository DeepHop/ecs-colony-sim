use bevy::prelude::*;
use crate::resources::time_state::TimeState;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_time_state);
    }
}

fn tick_time_state(mut time_state: ResMut<TimeState>) {
    if !time_state.paused {
        time_state.tick += 1;
    }
}
