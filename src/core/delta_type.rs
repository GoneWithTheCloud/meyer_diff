#[derive(Clone, Copy, Eq, PartialEq,Debug)]
pub enum DeltaType {
    Change,
    Delete,
    Insert,
    Equal,
}
