/*
 * macros.rs - helper macros for opencl api objects.
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

#![allow(dead_code)]

#[macro_export]
macro_rules! get_count {
    // 0 parameters
    ($fn: ident) => {{
        let mut count = cl_uint::default();
        let status_code = unsafe { $fn(0, ptr::null_mut(), &mut count) };
        status_update(status_code, stringify!($fn), count)?
    }};
    //2 parameters
    ($fn: ident, $param_1: ident, $param_2: ident) => {{
        let mut count = cl_uint::default();
        let status_code = unsafe { $fn($param_1, $param_2, 0, ptr::null_mut(), &mut count) };
        status_update(status_code, stringify!($fn), count)?
    }};
    //3 parameters
    ($fn: ident, $param_1: ident, $param_2: ident, $param_3: ident) => {{
        let mut count = cl_uint::default();
        let status_code =
            unsafe { $fn($param_1, $param_2, $param_3, 0, ptr::null_mut(), &mut count) };
        status_update(status_code, stringify!($fn), count)?
    }};
}

#[macro_export]
macro_rules! size_getter {
    ($name:ident, $fn:ident) => {
        let $name = |ret_dat_ptr: *mut libc::c_void, param_name: cl_uint| -> APIResult<size_t> {
            let mut size: size_t = 0;
            let status_code = unsafe {
                $fn(
                    ret_dat_ptr,
                    param_name,
                    0,
                    ptr::null_mut(),
                    &mut size as *mut size_t,
                )
            };
            status_update(status_code, stringify!($fn), size)
        };
    };
}

#[macro_export]
macro_rules! gen_param_value {
    // return vector object
    ($fn:ident, $typ:tt, $ptr: ident, $param_name: ident, $size: ident) => {{
        if $size == 0 {
            Vec::default()
        } else {
            let arr_len = $size / Size::$typ.get();
            let mut param_value: Vec<$typ> = std::vec::from_elem($typ::default(), arr_len);
            let status_code = unsafe {
                $fn(
                    $ptr,
                    $param_name,
                    $size,
                    param_value.as_mut_ptr() as *mut c_void,
                    ptr::null_mut(),
                )
            };
            status_update(status_code, stringify!($fn), param_value)?
        }
    }};

    // return single object
    ($fn:ident, $typ:tt, $ptr: ident, $param_name: ident) => {{
        let size = Size::$typ.get();
        let mut param_value = $typ::default();
        let status_code = unsafe {
            $fn(
                $ptr,
                $param_name,
                size,
                crate::objects::functions::to_mut_ptr(&mut param_value) as *mut c_void,
                ptr::null_mut(),
            )
        };
        status_update(status_code, stringify!($fn), param_value)?
    }};
}

#[macro_export]
macro_rules! gen_object_list {
    // 0 parameters
    ($fn:ident, $typ:tt, $count: ident) => {{
        let arr_len = $count as usize;
        let mut all_objects: $typ = std::vec::from_elem(ptr::null_mut(), arr_len);
        let status_code = unsafe { $fn($count, all_objects.as_mut_ptr(), ptr::null_mut()) };
        status_update(status_code, stringify!($fn), all_objects)
    }};

    // 2 parameters
    ($fn:ident, $typ:tt, $count: ident, $param_1: ident, $param_2: ident) => {{
        let arr_len = $count as usize;
        let mut all_objects: $typ = std::vec::from_elem(ptr::null_mut(), arr_len);
        let status_code = unsafe {
            $fn(
                $param_1,
                $param_2,
                $count,
                all_objects.as_mut_ptr(),
                ptr::null_mut(),
            )
        };
        status_update(status_code, stringify!($fn), all_objects)
    }};

    // 3 parameters (Non Null)
    ($fn:ident, $typ:tt, $count: ident, $param_1: ident, $param_2: ident, $param_3: ident) => {{
        let arr_len = $count as usize;
        let mut all_objects: Vec<$typ> = std::vec::from_elem($typ::default(), arr_len);
        let status_code = unsafe {
            $fn(
                $param_1,
                $param_2,
                $param_3,
                $count,
                all_objects.as_mut_ptr() as *mut $typ,
                ptr::null_mut(),
            )
        };
        status_update(status_code, stringify!($fn), all_objects)
    }};
}

#[macro_export]
macro_rules! gen_add_trait {
    ($name:ident) => {
        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                Self(self.0 | other.0)
            }
        }
    };
}
