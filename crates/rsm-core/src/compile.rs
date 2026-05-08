mod rsm;

pub struct Rbd { // routine buffer descriptor
    pub fwd_link: *mut Rbd, // forward link
    pub chunk_size: u16, // bytes in chunk
    pub attached: u16, // num processes attached
    pub last_access: SystemTime, // last used (seconds since 1970)
    pub rnam: VarU, // routine name
    pub uci: u8,
    pub vol: u8,
    pub rou_size: u16, // rou.len of routine node
    // what follows is the routine from disk (up to MAX_STR_LEN bytes + a NULL)
    pub comp_ver: u16, // compiler version
    pub comp_user: u16, // compiled by user number
    pub comp_date: u32, // date compiled (M form)
    pub comp_time: u32, // time compiled (M form)
    pub tag_tbl: u16, // offset to tag table
    pub num_tags: u16, // number of tags in table
    pub var_tbl: u16, // offset to var table
    pub num_vars: u16, // num vars in table
    pub code: u16, // offset to compiled code
    pub code_size: u16 // bytes of code

}