/*
 * helpers.rs - Helper functions for OpenCL APIs.
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
use crate::structs::StatusCode;
use opencl_heads::types::*;

pub fn status_update<T>(status_code: cl_int, result: T) -> Result<T, Status> {
    if StatusCode::SUCCESS != status_code {
        Err(Status::from(status_code))
    } else {
        Ok(result)
    }
}
