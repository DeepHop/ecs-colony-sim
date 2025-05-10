use bevy::prelude::Resource;

#[derive(Resource)]
pub struct TimeState {
    pub tick: u64,
    pub day: u32,
    pub season: Season,
    pub year: u32,
    pub paused: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}
