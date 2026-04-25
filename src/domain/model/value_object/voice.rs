pub enum SoundType {
    Piano,
    Cymbal,
}

pub struct Voice {
    pub freq: f32,
    pub time: f32,
    pub sound_type: SoundType,
}
