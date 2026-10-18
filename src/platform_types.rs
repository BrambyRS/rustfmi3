//! Platform types for FMI 3.0 interface
//! These are the basic data types required by the FMI 3.0 standard
#![allow(dead_code)]

use std::ffi::{c_char, c_void};

/// Enumeration of the different data types that can be set or retrieved in the FMI 3.0 interface.
pub enum FMI3DataType {
    FMI3Boolean,
    FMI3Float32,
    FMI3Float64,
    FMI3Int8,
    FMI3Int16,
    FMI3Int32,
    FMI3Int64,
    FMI3UInt8,
    FMI3UInt16,
    FMI3UInt32,
    FMI3UInt64,
    FMI3Binary,
    FMI3String,
    FMI3Clock,
}

// #region number types
pub type FMI3Boolean = bool;
pub type FMI3Float32 = f32;
pub type FMI3Float64 = f64;
pub type FMI3Int8 = i8;
pub type FMI3Int16 = i16;
pub type FMI3Int32 = i32;
pub type FMI3Int64 = i64;
pub type FMI3UInt8 = u8;
pub type FMI3UInt16 = u16;
pub type FMI3UInt32 = u32;
pub type FMI3UInt64 = u64;
// #endregion number types

// #region text and binary types
pub type FMI3Byte = u8;
pub type FMI3Binary = *const FMI3Byte;
pub type FMI3Char = c_char;
pub type FMI3String = *const FMI3Char;
// #endregion text and binary types

// #region basic types
pub type FMI3Clock = bool;
pub type FMI3FMUState = *mut c_void;
pub type FMI3Instance = *mut c_void;
pub type FMI3InstanceEnvironment = *mut c_void;
pub type FMI3ValueReference = u32;
// #endregion basic types
