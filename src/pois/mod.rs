use crate::flatbuffer::Vec2;
use crate::disease::MixingStrategy;

/// A Spatial Area where agents spend time and mix
pub struct Container<M> where M: MixingStrategy {
    pos: Vec2,
    inhabitants: Vec<u32>,
    mixing_strategy: M
}

pub fn create_containers<M>() -> Vec<Container<M>> where M: MixingStrategy{
    // let positions = agent_households.iter().filter_map(|idx| {
    //     household_positions.get(idx as usize)
    // }).copied().collect();

    return Vec::new();
}
