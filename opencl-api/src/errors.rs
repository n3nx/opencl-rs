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

#[derive(Debug)]
pub enum Error {
    StatusError(Status),
    APIError(ValidationError),
    UtilError(UtilError),
}

impl Error {
    pub fn get_status_error(&self) -> Option<&Status> {
        match self {
            Error::StatusError(x) => Some(x),
            _ => None,
        }
    }
    pub fn get_api_error(&self) -> Option<&ValidationError> {
        match self {
            Error::APIError(x) => Some(x),
            _ => None,
        }
    }
    pub fn get_util_error(&self) -> Option<&UtilError> {
        match self {
            Error::UtilError(x) => Some(x),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UndefinedError {
    InvalidStatusCode,
    InvalidError,
}

impl error::Error for UndefinedError {}
impl fmt::Display for UndefinedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UndefinedError::InvalidStatusCode => write!(
                f,
                "Undefined Error: {} -> Unconventional status code found in resulting output.",
                self
            ),
            _ => write!(f, "Undefined Error: {}", self),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {}

#[derive(Debug, PartialEq)]
pub enum UtilError {
    // BytesIntoString(std::string::FromUtf8Error),
    BytesIntoString,
}
