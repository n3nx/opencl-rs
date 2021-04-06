/*
 * platform.rs - Platform API wrappers.
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
#![allow(dead_code, unused_assignments)]

use crate::enums::Status;
use crate::helpers::*;
use crate::structs::StatusCode;
use opencl_heads::ffi::*;
use std::ptr;

pub type Platforms = Vec<cl_platform_id>;

fn get_platform_count(num_entries: cl_uint) -> Result<cl_uint, Status> {
    let mut platform_count: cl_uint = 0;
    let status_code =
        unsafe { clGetPlatformIDs(num_entries, ptr::null_mut(), &mut platform_count) };
    status_update(status_code, platform_count)
}

pub fn get_platform_ids() -> Result<Platforms, Status> {
    let mut status_code = StatusCode::SUCCESS;
    let platform_count = get_platform_count(0)?;
    if platform_count == 0 {
        Ok(Vec::default())
    } else {
        let vector_length = platform_count as usize;
        let mut all_platforms: Platforms = vec![ptr::null_mut(); vector_length];
        unsafe {
            status_code =
                clGetPlatformIDs(platform_count, all_platforms.as_mut_ptr(), ptr::null_mut());
        };
        status_update(status_code, all_platforms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // #[ignore]
    fn test_get_platform_ids() {
        let id = get_platform_ids().unwrap();
        assert_ne!(id, vec!(ptr::null_mut()));
        assert_ne!(id, vec!());
    }
}
