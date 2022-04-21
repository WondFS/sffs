use std::collections::HashMap;

pub enum PageUsedStatus {
    Clean,
    Dirty,
    Busy(u32),
}

// 磁盘中 每个Block中的Page使用情况，0 Clean 1 Dirty/Used
// 内存中 每个Block中的Page使用清康，0 Clean 1 Dirty 2 Used
pub struct BIT {
    
}

impl BIT {

}

