use crate::cheat::Cheat;

pub trait Feature<'a> {
    fn new(cheat: &'a Cheat) -> Self;
    fn on_create_move(&self, cmd: u64) {}
    fn on_paint_traverse(&self) {}
}

pub struct WallHack<'a> {
    cheat: &'a Cheat,
}

impl<'a> Feature<'a> for WallHack<'a> {
    fn new(cheat: &'a Cheat) -> Self {
        Self { cheat }
    }
}

pub struct AimBot<'a> {
    cheat: &'a Cheat,
}

impl<'a> Feature<'a> for AimBot<'a> {
    fn new(cheat: &'a Cheat) -> Self {
        Self { cheat }
    }
}
