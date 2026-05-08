use std::time::SystemTime;

mod rsm;
const IDX_START: usize = std::mem::size_of::<DbBlock>() / 2;

pub struct DbBlock { // database block layout
    pub type_: u8, // block type
    pub flags: u8, // flags
    pub spare: u16,
    pub right_ptr: u16, // right pointer
    pub last_idx: u16, // last used index
    pub last_free:u16, // last free lw in block
    pub global: rsm::var_u // end block header
}

pub struct Gbd {
    pub block: u32,
    pub next: Option<Box<Gbd>>,
    pub mem: Option<Box<DbBlock>>,
    pub dirty: Option<Box<Gbd>>,
    pub last_accessed: SystemTime

}