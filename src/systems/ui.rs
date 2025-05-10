use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::resources::time_state::TimeState;

pub fn draw_time_state_ui(mut contexts: EguiContexts, time_state: Res<TimeState>) {
    egui::Window::new("TimeState Debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Tick: {}", time_state.tick));
        ui.label(format!("Day: {}", time_state.day));
        ui.label(format!("Season: {:?}", time_state.season));
        ui.label(format!("Year: {}", time_state.year));
        ui.label(format!("Paused: {}", time_state.paused));
    });
}
