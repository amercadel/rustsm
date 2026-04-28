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

// Note: the following three MUST be a power of 2 as they are masks for &
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