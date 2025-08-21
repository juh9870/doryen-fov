use crate::{FovAlgorithm, MapData};

#[derive(Default)]
pub struct FovDummy {}


impl FovDummy {
    pub fn new() -> Self {
        Default::default()
    }
}

impl FovAlgorithm for FovDummy {
    fn compute_fov(
        &mut self,
        map: &mut MapData,
        _x: usize,
        _y: usize,
        _max_radius: usize,
        _light_walls: bool,
    ) {
        map.fov[..].copy_from_bitslice(&map.transparent[..]);
    }
}
