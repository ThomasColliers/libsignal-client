//
// Copyright 2020-2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use std::convert::TryFrom;
use std::fmt;

use device_transfer::Error as DeviceTransferError;
use libsignal_protocol::*;
use signal_crypto::Error as SignalCryptoError;

/// The top-level error type (opaquely) returned to C clients when something goes wrong.
#[derive(Debug)]
pub enum SignalFfiError {
    Signal(SignalProtocolError),
    DeviceTransfer(DeviceTransferError),
    SignalCrypto(SignalCryptoError),
    InsufficientOutputSize(usize, usize),
    NullPointer,
    InvalidUtf8String,
    UnexpectedPanic(std::boxed::Box<dyn std::any::Any + std::marker::Send>),
    InvalidType,
}

impl fmt::Display for SignalFfiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SignalFfiError::Signal(s) => write!(f, "{}", s),
            SignalFfiError::DeviceTransfer(c) => {
                write!(f, "Device transfer operation failed: {}", c)
            }
            SignalFfiError::SignalCrypto(c) => {
                write!(f, "Cryptographic operation failed: {}", c)
            }
            SignalFfiError::NullPointer => write!(f, "null pointer"),
            SignalFfiError::InvalidType => write!(f, "invalid type"),
            SignalFfiError::InvalidUtf8String => write!(f, "invalid UTF8 string"),
            SignalFfiError::InsufficientOutputSize(n, h) => {
                write!(f, "needed {} elements only {} provided", n, h)
            }

            SignalFfiError::UnexpectedPanic(e) => match e.downcast_ref::<&'static str>() {
                Some(s) => write!(f, "unexpected panic: {}", s),
                None => write!(f, "unknown unexpected panic"),
            },
        }
    }
}

impl From<SignalProtocolError> for SignalFfiError {
    fn from(e: SignalProtocolError) -> SignalFfiError {
        SignalFfiError::Signal(e)
    }
}

impl From<DeviceTransferError> for SignalFfiError {
    fn from(e: DeviceTransferError) -> SignalFfiError {
        SignalFfiError::DeviceTransfer(e)
    }
}

impl From<SignalCryptoError> for SignalFfiError {
    fn from(e: SignalCryptoError) -> SignalFfiError {
        SignalFfiError::SignalCrypto(e)
    }
}

pub type SignalFfiResult<T> = Result<T, SignalFfiError>;

/// Represents an error returned by a callback, following the C conventions that 0 means "success".
#[derive(Debug)]
pub struct CallbackError {
    value: std::num::NonZeroI32,
}

impl CallbackError {
    /// Returns `None` if `value` is zero; otherwise, wraps the value in `Self`.
    pub fn check(value: i32) -> Option<Self> {
        let value = std::num::NonZeroI32::try_from(value).ok()?;
        Some(Self { value })
    }
}

impl fmt::Display for CallbackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error code {}", self.value)
    }
}

impl std::error::Error for CallbackError {}