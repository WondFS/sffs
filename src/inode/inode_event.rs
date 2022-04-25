use crate::inode::inode;

pub struct InodeEventGroup {
    pub inode: inode::Inode,
    pub need_delete: bool,
    pub events: Vec<InodeEvent>,
}

impl InodeEventGroup {
    pub fn new() -> InodeEventGroup {
        InodeEventGroup {
            inode: inode::Inode::new(),
            need_delete: false,
            events: vec![],
        }
    }
}

pub enum InodeEvent {
    AddContent(AddContentInodeEvent),
    TruncateContent(TruncateContentInodeEvent),
    ChangeContent(ChangeContentInodeEvent),
    DeleteContent(DeleteContentInodeEvent),
    ModifyStat(ModifyInodeStatInodeEvent),
    None,
}

pub struct AddContentInodeEvent {
    pub index: u32,
    pub offset: u32,
    pub len: u32,
    pub size: u32,
    pub content: Vec<u8>,
}

pub struct TruncateContentInodeEvent {
    pub index: u32,
    pub offset: u32,
    pub len: u32,
    pub size: u32,
    pub o_size: u32,
    pub v_address: u32,
}

pub struct ChangeContentInodeEvent {
    pub index: u32,
    pub offset: u32,
    pub v_address: u32,
}

pub struct DeleteContentInodeEvent {
    pub index: u32,
    pub size: u32,
    pub v_address: u32,
}

pub struct ModifyInodeStatInodeEvent {
    pub file_type: inode::InodeFileType,
    pub ino: u32,
    pub size: u32,
    pub uid: u32,
    pub gid: u16,
    pub n_link: u8,
}