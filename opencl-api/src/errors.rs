/*
 * errors.rs - Error handling for OpenCL APIs.
 *
 * Copyright 2020-2021 Naman Bishnoi
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::enums::Status;
use std::{error, fmt};

// All Error Traits here
pub trait ToLibraryError {
    fn to_error(self) -> OpenCLAPILibraryError;
}

// Main Library Error
#[derive(Debug)]
pub enum OpenCLAPILibraryError {
    StatusError(Status),
    APIError(ValidationError),
    HelperError(HelperError),
}

impl OpenCLAPILibraryError {
    pub fn get_status_error(&self) -> Option<&Status> {
        match self {
            OpenCLAPILibraryError::StatusError(x) => Some(x),
            _ => None,
        }
    }
    pub fn get_api_error(&self) -> Option<&ValidationError> {
        match self {
            OpenCLAPILibraryError::APIError(x) => Some(x),
            _ => None,
        }
    }
    pub fn get_helper_error(&self) -> Option<&HelperError> {
        match self {
            OpenCLAPILibraryError::HelperError(x) => Some(x),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InvalidStatusCode(&'static str),
    InvalidBitfield(&'static str),
    InvalidError,
}

impl ToLibraryError for ValidationError {
    fn to_error(self) -> OpenCLAPILibraryError {
        OpenCLAPILibraryError::APIError(self)
    }
}

impl error::Error for ValidationError {}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStatusCode(fn_name) => write!(
                f,
                "Undefined Error: {} ==> Unconventional status code found in function \'{}\' with resulting output.",
                self, fn_name
            ),
            Self::InvalidBitfield(fn_name) => write!(f, "Undefined Error: {} ==> Unidentified bitfield configuration found in function \'{}\' with resulting output.", self, fn_name),
            _ => write!(f, "Undefined Error: {}", self),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HelperError {
    BytesIntoString,
}

impl ToLibraryError for HelperError {
    fn to_error(self) -> OpenCLAPILibraryError {
        OpenCLAPILibraryError::HelperError(self)
    }
}
