use std::ptr::NonNull;

mod database;
mod compile;

#[macro_export]
macro_rules! RSM_STRING {
    ($x:expr) => {
        stringify!($x)
    };
}
pub const OFF: i8 = -1;
pub const RSM_MAGIC: u32 = 4155766917; // Seems unique: not sure where/why this magic number is used yet
pub const MAX_DATABASE_BLKS: u32 = 2147483647; // 2 ** 31 - 1 unsigned
pub const MAX_BLOCK_SIZE: u32 = 256; // max block size in KiB
pub const VERSION_MAJOR: u8 = 0; // Major version number
pub const VERSION_MINOR: u8 = 1; // Minor version number
pub const VERSION_PATCH: u8 = 1; // Patch version number
pub const VERSION_PRE: u8 = 0; // Pre-release number
pub const VERSION_TEST: u8 = 0; // Test version number
pub const MBYTE: u32 = 1048576; // 1024 * 1024
pub const MAX_JOBS: u32 = 1024; // max number of jobs
pub const DAMEONS: u8 = 16; // jobs per daemon
pub const MIN_DAEMONS: u8 = 2;
pub const MAX_DAEMONS: u8 = 16;
pub const MAX_GLOBAL_BUFFERS: u32 = 131072; // Maximum global buffers in MiB
pub const MAX_ROUTINE_BUFFERS: u32 = 4095; // Maximum routine buffers in MiB


#[cfg(target_os = "macos")]
pub const PRVGRP: u8 = 80; // admin for mac

#[cfg(target_os = "linux")]
pub const PRVGRP: u8 = 0; // admin for linux

pub const DEFAULT_PREC:  u16 = 18;      // default number of decimal places
pub const MAX_PREC:      u16 = 128;     // max number of decimal places
pub const MAX_NUM_BYTES: u16 = 256;     // max size of a number
pub const MAX_STR_LEN:   u32 = 65534;   // max size of a string (65535 VAR/NODE_UNDEFINED)
pub const MAX_KEY_SIZE:  u16 = 255;     // max size of a key
pub const MAX_SUB_LEN:   u16 = 127;     // max size of a subscript
pub const MAX_NUM_SUBS:  u16 = 63;      // max number of subscripts
pub const MAX_NUM_ARGS:  u16 = 127 - 1; // max number of arguments
pub const MAX_NUM_TAGS:  u16 = 256;     // max number of tags/labels
pub const MAX_NUM_VARS:  u16 = 255;     // max number of routine variables

// dbver (comes from makefile in original implmentation)
// uses -D flag in make file, could maybe use cfg()
// #[cfg(feature = "dbver1")]
// pub const VAR_LEN: u8 = 8; 
// #[cfg(not(feature = "dbver1"))]
// pub const VAR_LEN: u8 = 32;

pub const MAX_ECODE: u16 = 1024; // max size for $ECODE
pub const SECDAY: u32 = 86400; // seconds in a day
pub const YRADJ: u32 = 47117; // days from Jan 1, 1841 to 1970

pub const MAX_VOL: u8 = 1; // max number of vols
pub const UCIS: u8 = 64; // always 64...for some reason


// KEYCMP
pub const KEQUAL: i8 = 0;
pub const K2_LESSER: i8 = -1;
pub const K2_GREATER: i8 = 1;

pub const MAX_DO_FRAMES: u8  = 127;                    // maximum permitted do_frame (0 - 126)
pub const STM1_FRAME: u8     = MAX_DO_FRAMES;          // where $STACK(-1) data goes

pub const UNLIMITED: i32     = -1;                     // unlimited timeout for sequential IO

pub const MIN_SEQ_IO: u8     = 0;                      // minimum sequential IO channel
pub const MAX_SEQ_IO: u8     = 64;                     // maximum sequential IO channel
pub const MAX_SEQ_NAME: u16  = 256;                    // max file name size
pub const MAX_SEQ_OUT: u8    = 6;                      // max output terminator size
pub const MAX_DKEY_LEN: u8   = 16;                     // max $KEY seq stored

// SQ_Chan->type values
pub const SQ_FREE: u8        = 0;                      // free
pub const SQ_FILE: u8        = 1;                      // disk file
pub const SQ_SOCK: u8        = 2;                      // socket
pub const SQ_PIPE: u8        = 3;                      // named pipe
pub const SQ_TERM: u8        = 4;                      // terminal/character device

// Write control
pub const SQ_LF: i8          = -1;                     // WRITE !
pub const SQ_FF: i8          = -2;                     // WRITE #

// SQ_USE flags - OR'd together, fits in u16
pub const SQ_USE_ECHO: u16          = 1;               // turn echo on
pub const SQ_USE_NOECHO: u16        = 2;               // turn echo off
pub const SQ_USE_ESCAPE: u16        = 4;               // turn escape on
pub const SQ_USE_NOESCAPE: u16      = 8;               // turn escape off
pub const SQ_USE_TYPEAHEAD: u16     = 16;              // turn type-ahead on
pub const SQ_USE_NOTYPEAHEAD: u16   = 32;              // turn type-ahead off
pub const SQ_USE_DISCON: u16        = 128;             // disconnect client from socket
pub const SQ_USE_DELNONE: u16       = 256;             // no delete function
pub const SQ_USE_DEL8: u16          = 512;             // use backspace as delete
pub const SQ_USE_DEL127: u16        = 1024;            // use delete as delete
pub const SQ_USE_DELBOTH: u16       = 2048;            // use both as delete
pub const SQ_CONTROLC: u16          = 4096;            // enable <Control-C> trapping
pub const SQ_NOCONTROLC: u16        = 8192;            // no <Control-C> trap, ignore it
pub const SQ_CONTROLT: u16          = 16384;           // enable <Control-T> status
pub const SQ_NOCONTROLT: u16        = 32768;           // disable <Control-T> status

pub const MIN_GBD: u16       = 64;                     // minimum number of GBDs

pub const GBD_HASH: u16      = 1024;                   // hash size for global buffers
pub const NUM_DIRTY: u16     = 1024;                   // max queued dirty chains
pub const NUM_GARB: u16      = 8192;                   // max queued garbage blocks
pub const RBD_HASH: u16      = 1023;                   // hash size for routine names
pub const GBD_FREE: u16      = GBD_HASH;               // head of GBD free list

pub const AVROUSIZE: u16     = 3072;                   // average compiled routine size
pub const MAXROUSIZE: u16    = 65534;                  // max compiled routine size (= MAX_STR_LEN)
pub const MAXROULINE: u16    = 65534;                  // max routine lines (= MAX_STR_LEN)



// TODO: rewrite in rust
// #if RSM_DBVER == 1
// #   define DB_VER   1                                                           // database version
// #   define COMP_VER 7                                                           // compiler version
// #else
// #   define DB_VER   2                                                           // database version
// #   define COMP_VER 8                                                           // compiler version
// #endif

pub const GL_JOURNAL: u8 = 1; // journal global flag
pub const GL_TOP_DEFINED: u8 = 2; // top node of global

pub const LOCKTAB_SIZE: u16 = 32768; // 32 KiB per job
pub const UCI_IS_LOCALVAR: u16 = 255; // for struct mvar
pub const VAR_UNDEFINED: u32 = (MAX_STR_LEN + 1); // undefined variable (also NODE_UNDEFINED)
pub const ROOT_UNDEFINED: u32 = VAR_UNDEFINED; // undefined root of variable


pub const MAX_ASTK: u16 = 1024; // max depth of addstk
pub const MAX_SSTK: u16 = (MBYTE * 2); // max string stack at 2 MiB
pub const MAX_ISTK: u16 = 65536; // max indirect stack

pub const MAX_HISTORY: u8 = 128; //max size of history recall buffer

// do frame types (negatives are error codes)
pub const TYPE_RUN: u8 = 1; // normal rsm startup
pub const TYPE_JOB: u8 = 2; // got jobbed [0] only
pub const TYPE_DO: u8 = 3; // DO
pub const TYPE_EXTRINSIC: u8 = 4; // Extrinsic
pub const TYPE_XECUTE: u8 = 5; // eXecute

pub const DO_FLAG_TEST: u8 = 1; // $TEST value (0/1)
pub const DO_FLAG_ATT: u8 = 2; // sym attach done
pub const DO_FLAG_FOR: u8 = 4; // called from a FOR (infor)
pub const DO_FLAG_ERROR: u8 = 8; // error frame

pub const SIG_HUP: u32 = 1; // SIGHUP (ERRZ66)
pub const SIG_CC: u32 = (1U << 2); // <Control-C> (SIGINT - ERRZ51)
pub const SIG_QUIT: u32 = (1U << 3); // SIGQUIT (HALT)
pub const SIG_TERM: u32 = (1U << 15); // SIGTERM (HALT)
pub const SIG_STOP: u32 = (1U << 17); // SIGSTOP (HALT)
pub const SIG_WS: u32 = (1U << 28); // Window size changes (SIGWINCH - ignore)
pub const SIG_CT: u32 = (1U << 29); // <Control-T> signal (SIGINFO)
pub const SIG_U1: u32 = (1U << 30); // User signal 1 (ERRZ67)
pub const SIG_U2: u32 = (1U << 31); // User signal 2 (ERRZ68)

pub const VOL_FILENAME_MAX: u16 = 255; // max chars in stored filename
pub const JNL_FILENAME_MAX: u16 = 226; // max chars in journal filename

pub const HISTORIC_EOK: u8 = 1; // E number syntax OK
pub const HISTORIC_OFFOK: u8 = 2; // GO/DO with offset OK
pub const HISTORIC_DNOK: u8 = 4; // $NEXT OK

pub const MAX_MAP_SIZE: u16 = (MAX_DATABASE_BLKS / 8 + std::mem::size_of(LabelBlock)) / 1024 + 1;

/* 
semaphore defines
semaphores are setup with a value equal to systab->maxjob
a read takes one semaphore unit
a write takes systab->maxjob units
a SEM_ATOMIC only takes a SEM_WRITE and also sets the atomic flag
*/ 
pub const SEM_SYS: u8 = 0; // System table Semaphore
pub const SEM_LOCK: u8 = 1; // Lock table Semaphore
pub const SEM_GLOBAL: u8 = 2; // Global database module
pub const SEM_ROU: u8 = 3; // Routine buffer
pub const SEM_WD: u8 = 4; // Write daemons
pub const SEM_ATOMIC: u8 = 5; // Atomic operations
pub const SEM_MAX: u8 = 6; // total number of these

// semaphore defines
pub const SEM_READ: i16 = -1;  // read lock

// these 3 functions are implemented with a raw pointer in the original implementation, but I #don't want to do that
// defining these as functions that can be called at runtime

// write lock
pub fn sem_write(systab: &SysTab) -> i32 {
    (-(systab.maxjob as i32))
}
// from read to write
pub fn sem_r_wr(systab: &SysTab) -> i32 {
    (-(systab.maxjob - 1) as i32)
}
// from write to read
pub fn sem_wr_to_r(systab: &SysTab) -> i32 {
    (systab.maxjob - 1) as i32
}

pub const MAX_TRANTAB: u8 = 64;

// #[repr(C, packed)]
// pub struct systab {
//     pub address: *mut std::ffi::c_void,
//     // pub jobtab: *mut
// }


pub struct JobTab {
    pub pid: i32, // OS PID (0 if unused)
    pub cur_do: i32, // current do frame address
    pub commands: u32, // commands executed
    pub grefs: u32, // global references
    pub last_block_flags: u32, // journal etc of last DB block
    pub error_frame: i16, // frame error happened in
    pub etrap_at: i16, // where $ET was invoked
    pub trap: u32, // outstanding traps
    pub attention: i32, // do something
    pub async_error: i16, // async errors
    pub user: i16, // user number
    pub priv_: i16, // privs this job
    pub precision: i16, // decimal precision
    pub io: u8, // current io index
    pub test: u8, // current $TEST
    pub uci: u8, // current UCI number
    pub vol: u8, // current volset number
    pub luci: u8, // current lock UCI number
    pub lvol: u8, // current lock volset number
    pub ruci: u8, // current rou UCI number
    pub rvol: u8, // current rou volset number
    pub last_ref: mvar, // $REFERENCE 
    pub start_len: i16, // length start data  
    pub start_dh: [u8; 14], // store start time here
    pub dostk: [DoFrame; MAX_DO_FRAMES + 1], // the do stack (0 - 126 and STM1_FRAME)
    pub seqio: [SQChan; MAX_SEQ_IO],
    pub view: [*mut database::Gbd; MAX_VOL], // defined in rsm-core/database.rs ~ database.h
}

// in reference implementation, author uses a union because the variable can be stored in 3 different ways
// u_int64 var_q
// u_int64 var_qu[VARLEN / 8]
// u_char var_cu[VARLEN]
// for now, this will be implemented more simply, but will potentially change as needed
pub struct VarU {
    pub var_q: [u64; VARLEN] // just storing an array of bytes
}

pub struct MVar {
    pub name: var_u, // variable name
    pub volset: u8, // volset number
    pub uci: u8, // UCI number -> 255 = local variable
    pub slen: u8, // subs length
    pub key: [u8; MAX_KEY_SIZE + 1] // the subs (key) - allow for 0
}

pub struct DoFrame {
    pub routine: &mut [u8], // address of routine (or xecute source)
    pub pc: &mut [u8], // current RSM pc
    pub symbol: &mut [i16], // process space symbol table pointers
    pub newtab: &mut [u8], // process space new table
    pub endlin: &mut [u8], // address of current ENDLIN
    pub rounam: var_u, // routine name
    pub vol: u8, // rou source vol set #
    pub uci: u8, // rou source UCI #
    pub line_num: u16, // current routine line
    pub estack: u8, // current estack offset
    pub type_: u8, // see TYPE_X def
    pub level: u8, // current argumentless do level
    pub flags: u8, // flags for this frame
    pub savasp: u64, // saved asp
    pub savssp: u64, // saved ssp
    pub asp: u64, // entry asp
    pub ssp: u64, // entry ssp
    pub isp: u64 // do frame
}

// **************************************************************
// Generated with Claude, wasn't sure the best way to rewrite this
#[cfg(not(any(target_os = "macos", target_os = "openbsd")))]
pub struct Semun {
    pub val: i32,
    pub buf: *mut libc::semid_ds,
    pub array: *mut u16,
    #[cfg(any(target_os = "linux", target_os = "aix"))]
    pub __buf: *mut libc::seminfo,
}

#[cfg(any(target_os = "macos", target_os = "openbsd"))]
pub struct Semun {
    pub val: i32,
    pub buf: *mut libc::semid_ds,
    pub array: *mut u16,
}
// ******************************************************************
pub struct RsmCString {
    // will likely use String or &str, but will keep this in case I want to use it
    pub len: u16,
    pub buf: [u8; MAX_STR_LEN + 1]
}
pub struct SQChan {
    pub type_: u8, // type of device
    pub options: u8, // type of specific options
    pub mode: u8, // how option is opened
    pub fid: i32, // OS supplied file id
    pub s: ServerTab, // TODO: define 
    pub dx: u16, // $X
    pub dy: u16, // $Y
    pub name: [u8; MAX_SEQ_NAME], // name of what was opened
    pub dkey_len: i16, // $KEY length stored
    pub dkey: [u8; MAX_DKEY_LEN], // stored $KEY (null term)
    pub out_len: u16, // length of output terminator
    pub out_term: [u8; MAX_SEQ_OUT], // the output terminator
    pub in_term: InTerm, // TODO: define routine for namespace
    pub namespace: VarU // define the $IO stuff
} // sequential io

pub struct UciTab {
    pub name: VarU, // UCI name
    pub global: u16 // define the UCI table
}

pub struct MsgData {
    pub gbddata: *mut Gbd, // Gbd pointer
    pub intdata: u16 // or an integer/block number
}

pub struct WdTab { // write daemon table
    pub pid: u32,
    pub doing: u32,
    pub currmsg: MsgData
}


pub struct TranTab {
    pub from_global: VarU, // from global
    pub from_vol: u8, // volumeset
    pub from_uci: u8, // UCI#
    pub to_global: VarU, // to global
    pub to_vol: u8, // volumeset
    pub to_uci: u8 // UCI #
}

pub struct LabelBlock {
    pub magic: u16, // RSM magic number
    pub max_block: u16, // maximum block number
    pub header_bytes: u16, // bytes in label/map
    pub block_size: u16, // bytes per data block
    pub volnam: VarU,
    pub db_ver: u16,
    pub creation_time: u64, // defined if RSM_DBVER=1
    pub journal_available: u8, // jrnl turned on at startup
    pub journal_requested: u8,
    pub clean: u8,
    pub journal_file: [char; JNL_FILENAME_MAX] // journal file name
    pub uci: [UciTab; UCIS]
}

pub struct DBStat {
    // database statistics
    pub dbget: u16,
    pub dbset: u16,
    pub dbkil: u16,
    pub dbdat: u16,
    pub dbord: u16,
    pub dbqry: u16,
    pub lasttry: u16,
    pub lastok: u16,
    pub logrd: u16,
    pub phyrd: u16,
    pub logwt: u16,
    pub phywt: u16,
    pub blkalloc: u16,
    pub blkreorg: u16, 
    pub diskerrors: u16
}

#[repr(C)]
pub struct VolDef {
    pub vollab: *mut LabelBlock, // ptr to volset label block
    pub map: std::ffi::c_void, // start of map area
    pub first_free: std::ffi::c_void, // first word wiht free bits
    pub gbd_hash: [*mut Gbd; GBD_HASH + 1], // Gbd hash table
    pub gbd_head: *mut Gbd, // head of global buffer desc
    pub num_gbd: u16, // number of global buffers
    pub global_buf: std::ffi::c_void, // start of global buffers
    pub zero_block: std::ffi::c_void, // empty block in memory
    pub rbd_hash: [*mut Rbd; RBD_HASH + 1],
    pub rbd_head: *mut Rbd, // head of routine buffer desc
    pub num_rbd: u16, // number of routine buffers
    pub num_of_daemons: u32,
    pub wd_tab: [WdTab; MAX_DAEMONS], // write daemon info table
    

}
pub struct LockTab {
    pub fwd_link: *mut LockTab, // next tab pointer
    pub size: u32, // num bytes
    pub job: u16, // int job, -1 = free
    pub lock_count: u16,
    pub byte_count: u16,
    pub vol: u8, // vol number
    pub uci: u8, // UCI number (255 = local)
    pub name: VarU, // var name
    pub key: [u8; MAX_KEY_SIZE + 1] // key

}

#[repr(C)]
pub struct SysTab{ // system tables
    pub address: *mut std::ffi::c_void,
    pub jobtab: *mut JobTab,
    pub maxjob: u16, // max jobs allowed
    pub sem_id: u32, // Gbd Semaphore ID
    pub historic: u32, // E Notation, tag+off, $NEXT
    pub precision: u32, // decimal precision
    pub max_tt: u32, // max TRANTAB used,
    pub tt: [TranTab; MAX_TRANTAB], // translation tables
    pub start_user: u32, // user who started the environment
    pub lockstart: *mut std::ffi::c_void, // head of lock table
    pub locksize: u32, // num bytes
    pub locktab: *mut LockTab, // head of used locks
    pub lockfree: *mut LockTab, // head of lock free space
    pub addoff: u64, // offset from systab to other buffers
    pub addzie: u64, // other buffer size
    pub vol: [*mut VolDef; MAX_VOL], // array of volume pointers
    pub last_blk_used: [u32; MAX_VOL]

}