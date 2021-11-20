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

use crate::objects::enums::Status;
use thiserror::Error;

// All Error Traits here
pub trait ToLibraryError {
    fn to_error(self) -> OpenCLAPILibraryError;
}

// Main Library Error
#[derive(Error, Debug)]
pub enum OpenCLAPILibraryError {
    #[error("status code error: {0:?}")]
    StatusError(Status),
    #[error("api error: `{0}`")]
    APIError(ValidationError),
    #[error("helper error: `{0}`")]
    HelperError(HelperError),
}

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("invalid status code {code} at function `{func}`")]
    InvalidStatusCode { code: i32, func: &'static str },
    #[error("invalid bitfield configuration at function `{0}`")]
    InvalidBitfield(&'static str),
    // #[error("undefined error")]
    // InvalidError,
}

impl ToLibraryError for ValidationError {
    fn to_error(self) -> OpenCLAPILibraryError {
        OpenCLAPILibraryError::APIError(self)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum HelperError {
    #[error("bytes into string")]
    BytesIntoString,
    #[error("null pointer exception at function `{0}`")]
    NullPointerException(&'static str),
}

impl ToLibraryError for HelperError {
    fn to_error(self) -> OpenCLAPILibraryError {
        OpenCLAPILibraryError::HelperError(self)
    }
}
