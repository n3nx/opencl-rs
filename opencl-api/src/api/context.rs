/*
 * context.rs - Context API wrappers (Part of OpenCL Platform Layer).
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
use crate::enums::{ParamValue, Size};
use crate::helpers::{status_update, APIResult, GetSetGo, Properties};
use crate::structs::{ContextInfo, DeviceType};
use crate::{gen_param_value, size_getter};
use libc::c_void;
use opencl_heads::ffi;
use opencl_heads::ffi::clGetContextInfo;
use opencl_heads::types::*;
use std::ptr;
use std::vec;

pub fn create_context(
    properties: &Properties,
    devices: Vec<cl_device_id>,
    pfn_notify: Option<extern "C" fn(*const c_char, *const c_void, size_t, *mut c_void)>,
    user_data: *mut c_void,
) -> APIResult<cl_context> {
    let mut status_code = 0;
    let properties = match properties {
        Some(x) => x.as_ptr(),
        None => ptr::null(),
    };
    let context = unsafe {
        ffi::clCreateContext(
            properties,
            devices.len() as cl_uint,
            devices.as_ptr(),
            pfn_notify,
            user_data,
            &mut status_code,
        )
    };
    status_update(status_code, "clCreateContext", context)
}

pub fn create_context_from_type(
    properties: &Properties,
    device_type: DeviceType,
    pfn_notify: Option<extern "C" fn(*const c_char, *const c_void, size_t, *mut c_void)>,
    user_data: *mut c_void,
) -> APIResult<cl_context> {
    let mut status_code = 0;
    let properties = match properties {
        Some(x) => x.as_ptr(),
        None => ptr::null(),
    };
    let context = unsafe {
        ffi::clCreateContextFromType(
            properties,
            device_type.get(),
            pfn_notify,
            user_data,
            &mut status_code,
        )
    };
    status_update(status_code, "clCreateContextFromType", context)
}

pub fn retain_context(context: cl_context) -> APIResult<()> {
    let status_code = unsafe { ffi::clRetainContext(context) };
    status_update(status_code, "clRetainContext", ())
}

pub fn release_context(context: cl_context) -> APIResult<()> {
    let status_code = unsafe { ffi::clReleaseContext(context) };
    status_update(status_code, "clReleaseContext", ())
}

pub fn get_context_info(context: cl_context, param_name: cl_context_info) -> APIResult<ParamValue> {
    type C = ContextInfo;
    size_getter!(get_context_info_size, clGetContextInfo);
    match param_name {
        C::REFERENCE_COUNT | C::NUM_DEVICES => {
            let param_value = gen_param_value!(clGetContextInfo, u32, context, param_name);
            Ok(ParamValue::UInt(param_value))
        }
        C::DEVICES | C::PROPERTIES => {
            let size = get_context_info_size(context, param_name)?;
            let filler = 0;
            let param_value =
                gen_param_value!(clGetContextInfo, isize, context, param_name, size, filler);
            Ok(ParamValue::ArrCPtr(param_value))
        }
        _ => status_update(40404, "clGetContextInfo", ParamValue::default()),
    }
}

pub fn set_context_destructor_callback(
    context: cl_context,
    pfn_notify: extern "C" fn(context: cl_context, user_data: *mut c_void),
    user_data: *mut c_void,
) -> APIResult<()> {
    let status_code =
        unsafe { ffi::clSetContextDestructorCallback(context, pfn_notify, user_data) };
    status_update(status_code, "clSetContextDestructorCallback", ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::device::get_device_ids;
    use crate::api::platform::get_platform_ids;
    use crate::structs::{ContextProperties, DeviceType};

    #[test]
    fn test_create_context() {
        let platform_ids = get_platform_ids().unwrap();
        // Choose the first platform
        let platform_id = platform_ids[0];

        let device_ids =
            get_device_ids(platform_id, DeviceType::new(DeviceType::DEFAULT).unwrap()).unwrap();
        assert!(0 < device_ids.len());

        let context = create_context(&None, device_ids, None, ptr::null_mut());
        let context = context.unwrap();
        eprintln!("CL_CONTEXT_PTR: {:?}", context);
        assert_ne!(context, ptr::null_mut());
        release_context(context).unwrap();
    }
    #[test]
    fn test_create_context_from_type() {
        let platform_ids = get_platform_ids().unwrap();
        // Choose the first platform
        let platform_id = platform_ids[0];
        // let properties = vec![ContextProperties::PLATFORM, platform_id as isize, 0];
        let properties = ContextProperties.platform(platform_id);
        let default_device = DeviceType::new(DeviceType::DEFAULT).unwrap();
        let context = create_context_from_type(&properties, default_device, None, ptr::null_mut());
        let context = context.unwrap();
        eprintln!("CL_CONTEXT_PTR: {:?}", context);
        assert_ne!(context, ptr::null_mut());
        release_context(context).unwrap();
    }
    #[test]
    fn test_get_context_info_1() {
        let platform_ids = get_platform_ids().unwrap();
        // Choose the first platform
        let platform_id = platform_ids[0];

        // let properties = vec![ContextProperties::PLATFORM, platform_id as isize, 0];
        let properties = ContextProperties.platform(platform_id);
        let default_device = DeviceType::new(DeviceType::DEFAULT).unwrap();
        let context = create_context_from_type(&properties, default_device, None, ptr::null_mut());
        let context = context.unwrap();
        eprintln!("CL_CONTEXT_PTR: {:?}", context);
        assert_ne!(context, ptr::null_mut());
        let device = get_context_info(context, ContextInfo::DEVICES);
        eprintln!("CL_CONTEXT_DEVICE: {:?}", device);
        assert_ne!(device.unwrap().unwrap_arr_cptr().unwrap()[0], 0);

        let properties = get_context_info(context, ContextInfo::PROPERTIES);
        eprintln!("CL_CONTEXT_PROPERTIES: {:?}", properties);
        let re_platform_id = properties.unwrap().unwrap_arr_cptr().unwrap()[1];
        assert_eq!(re_platform_id, platform_id as isize);
        release_context(context).unwrap();
    }
    #[test]
    fn test_get_context_info_2() {
        let platform_ids = get_platform_ids().unwrap();
        // Choose the first platform
        let platform_id = platform_ids[0];

        // let properties = vec![ContextProperties::PLATFORM, platform_id as isize, 0];
        let properties = ContextProperties.platform(platform_id);
        let default_device = DeviceType::new(DeviceType::DEFAULT).unwrap();
        let context = create_context_from_type(&properties, default_device, None, ptr::null_mut());
        let context = context.unwrap();
        eprintln!("CL_CONTEXT_PTR: {:?}", context);
        assert_ne!(context, ptr::null_mut());
        let device_count = get_context_info(context, ContextInfo::NUM_DEVICES);
        eprintln!("CL_CONTEXT_DEVICE_COUNT: {:?}", device_count);
        assert_ne!(device_count.unwrap().unwrap_uint().unwrap(), 0);
        let ref_count = get_context_info(context, ContextInfo::REFERENCE_COUNT);
        eprintln!("CL_CONTEXT_REFERENCE_COUNT: {:?}", ref_count);
        assert_ne!(ref_count.unwrap().unwrap_uint().unwrap(), 0);

        release_context(context).unwrap();
    }
}
