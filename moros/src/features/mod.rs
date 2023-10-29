pub trait Feature<'a> {
    fn new() -> Self;
    fn on_create_move(&self, cmd: u64) {}
    fn on_paint_traverse(&self) {}
}
