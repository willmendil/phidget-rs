// phidget-rs/src/errors.rs
//
// Copyright (c) 2023, Frank Pagliughi
//
// This file is part of the 'phidget-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use phidget_sys as ffi;
use std::{
    ffi::CStr,
    fmt,
    os::raw::{c_char, c_int},
    ptr,
};

/////////////////////////////////////////////////////////////////////////////

/// Return Codes from the phidgets22 library.
/// These are all the integer success/failure codes returned by the calls
/// to the phidget22 library. A zero indicates success, whereas any other
/// value indicates failure. These are unsigned, so all errors are >0.
/// This type is a Rust std::error::Error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
#[allow(missing_docs)]
pub enum ReturnCode {
    Ok = 0,
    Perm = 1,
    NoEnt = 2,
    Timeout = 3,
    Interrupted = 4,
    Io = 5,
    NoMemory = 6,
    Access = 7,
    Fault = 8,
    Busy = 9,
    Exist = 10,
    NotDir = 11,
    IsDir = 12,
    Invalid = 13,
    NFile = 14,
    MFile = 15,
    NoSPC = 16,
    FBig = 17,
    ROFS = 18,
    RO = 19,
    Unsupported = 20,
    InvalidArg = 21,
    Again = 22,
    NotEmpty = 26,
    Duplicate = 27,
    Unexpected = 28,
    Eof = 31,
    ConnRef = 35,
    BadPassword = 37,
    NoDev = 40,
    Pipe = 41,
    Resolv = 44,
    NetUnavail = 45,
    ConnReset = 46,
    HostUnreach = 48,
    WrongDevice = 50,
    UnknownVal = 51,
    NotAttached = 52,
    InvalidPacket = 53,
    TooBig = 54,
    BadVersion = 55,
    Closed = 56,
    NotConfigured = 57,
    KeepAlive = 58,
    Failsafe = 59,
    UnknownValHigh = 60,
    UnknownValLow = 61,
}

impl ReturnCode {
    /// Convert the raw return code into a Result, where zero is Ok, and
    /// everything else is an error.
    pub fn result(rc: c_int) -> Result<()> {
        match rc {
            0 => Ok(()),
            _ => Err(ReturnCode::from(rc)),
        }
    }
}

impl std::error::Error for ReturnCode {}

impl fmt::Display for ReturnCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == ReturnCode::Ok {
            write!(f, "OK")
        }
        else {
            let mut descr: *const c_char = ptr::null_mut();
            unsafe {
                if ffi::Phidget_getErrorDescription(*self as c_int, &mut descr) == 0
                    && !descr.is_null()
                {
                    // TODO: Handle conversion error?
                    let msg = CStr::from_ptr(descr).to_string_lossy();
                    write!(f, "{}", msg)
                }
                else {
                    write!(f, "Unknown")
                }
            }
        }
    }
}

impl From<c_int> for ReturnCode {
    /// Converts an unsigned integer into a `ReturnCode` error.
    /// Note that instead of implementing `try_from`, any unknown integer
    /// value is returned as a `ReturnCode::Unexpected` error.
    fn from(val: c_int) -> Self {
        use ReturnCode::*;
        match val {
            0 => Ok,
            1 => Perm,
            2 => NoEnt,
            3 => Timeout,
            4 => Interrupted,
            5 => Io,
            6 => NoMemory,
            7 => Access,
            8 => Fault,
            9 => Busy,
            10 => Exist,
            11 => NotDir,
            12 => IsDir,
            13 => Invalid,
            14 => NFile,
            15 => MFile,
            16 => NoSPC,
            17 => FBig,
            18 => ROFS,
            19 => RO,
            20 => Unsupported,
            21 => InvalidArg,
            22 => Again,
            26 => NotEmpty,
            27 => Duplicate,
            28 => Unexpected,
            31 => Eof,
            35 => ConnRef,
            37 => BadPassword,
            40 => NoDev,
            41 => Pipe,
            44 => Resolv,
            45 => NetUnavail,
            46 => ConnReset,
            48 => HostUnreach,
            50 => WrongDevice,
            51 => UnknownVal,
            52 => NotAttached,
            53 => InvalidPacket,
            54 => TooBig,
            55 => BadVersion,
            56 => Closed,
            57 => NotConfigured,
            58 => KeepAlive,
            59 => Failsafe,
            60 => UnknownValHigh,
            61 => UnknownValLow,
            _ => Unexpected,
        }
    }
}

/// The error type for the crate is a phidget22 return code.
pub type Error = ReturnCode;

/// The default result type for the phidget-rs library
pub type Result<T> = std::result::Result<T, Error>;
