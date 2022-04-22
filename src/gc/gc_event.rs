pub struct GCEventGroup {
    pub events: Vec<GCEvent>,
}

pub enum GCEvent {
    Erase(EraseGCEvent),
    Move(MoveGCEvent),
    None,
}

// 以Block为单位
pub struct EraseGCEvent {
    pub index: u32,
    pub block_no: u32,
}

// 以Page为单位
pub struct MoveGCEvent {
    pub index: u32,
    pub o_address: u32,
    pub d_address: u32,
}