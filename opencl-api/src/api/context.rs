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
use crate::objects::bitfields::DeviceType;
use crate::objects::enums::{ParamValue, Size};
use crate::objects::functions::status_update;
use crate::objects::structs::ContextInfo;
use crate::objects::traits::GetSetGo;
use crate::objects::types::{APIResult, ContextPtr, DeviceList, Properties};
use crate::objects::wrappers::WrappedMutablePointer;
use crate::{gen_param_value, size_getter};
use libc::c_void;
use opencl_heads::ffi;
use opencl_heads::ffi::clGetContextInfo;
use opencl_heads::types::*;
use std::ptr;

pub fn create_context(
    properties: &Properties,
    devices: DeviceList,
    pfn_notify: Option<extern "C" fn(*const c_char, *const c_void, size_t, *mut c_void)>,
    user_data: WrappedMutablePointer<c_void>,
) -> APIResult<ContextPtr> {
    let fn_name = "clCreateContext";
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
            user_data.unwrap(),
            &mut status_code,
        )
    };
    status_update(
        status_code,
        fn_name,
        ContextPtr::from_ptr(context, fn_name)?,
    )
}

pub fn create_context_from_type(
    properties: &Properties,
    device_type: DeviceType,
    pfn_notify: Option<extern "C" fn(*const c_char, *const c_void, size_t, *mut c_void)>,
    user_data: WrappedMutablePointer<c_void>,
) -> APIResult<ContextPtr> {
    let fn_name = "clCreateContextFromType";
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
            user_data.unwrap(),
            &mut status_code,
        )
    };
    status_update(
        status_code,
        fn_name,
        ContextPtr::from_ptr(context, fn_name)?,
    )
}

pub fn retain_context(context: &ContextPtr) -> APIResult<()> {
    let status_code = unsafe { ffi::clRetainContext(context.unwrap()) };
    status_update(status_code, "clRetainContext", ())
}

pub fn release_context(context: ContextPtr) -> APIResult<()> {
    let status_code = unsafe { ffi::clReleaseContext(context.unwrap()) };
    status_update(status_code, "clReleaseContext", ())
}

pub fn get_context_info(
    context: &ContextPtr,
    param_name: cl_context_info,
) -> APIResult<ParamValue> {
    type C = ContextInfo;
    let context = context.unwrap();
    size_getter!(get_context_info_size, clGetContextInfo);
    match param_name {
        C::REFERENCE_COUNT | C::NUM_DEVICES => {
            let param_value = gen_param_value!(clGetContextInfo, u32, context, param_name);
            Ok(ParamValue::UInt(param_value))
        }
        C::DEVICES | C::PROPERTIES => {
            let size = get_context_info_size(context, param_name)?;
            let param_value = gen_param_value!(clGetContextInfo, isize, context, param_name, size);
            Ok(ParamValue::ArrCPtr(param_value))
        }
        _ => status_update(40404, "clGetContextInfo", ParamValue::default()),
    }
}

pub fn set_context_destructor_callback(
    context: &ContextPtr,
    pfn_notify: extern "C" fn(context: cl_context, user_data: *mut c_void),
    user_data: WrappedMutablePointer<c_void>,
) -> APIResult<()> {
    let status_code = unsafe {
        ffi::clSetContextDestructorCallback(context.unwrap(), pfn_notify, user_data.unwrap())
    };
    status_update(status_code, "clSetContextDestructorCallback", ())
}

/************************/
/* /\ /\ /\ /\ /\ /\ /\ */
/*|__|__|__|__|__|__|__|*/
/*|  |  |  |  |  |  |  |*/
/*|  |  Unit Tests  |  |*/
/*|__|__|__|__|__|__|__|*/
/*|__|__|__|__|__|__|__|*/
/************************/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::device::get_device_ids;
    use crate::api::platform::get_platform_ids;
    use crate::objects::bitfields::DeviceType;
    use crate::objects::structs::ContextProperties;
    use crate::objects::types::{PlatformPtr, WrapMutPtr};

    #[test]
    fn test_create_context() {
        let platform_ids = get_platform_ids().unwrap();
        // Choose the first platform
        // let platform_id = platform_ids[0];
        let platform_id = PlatformPtr::from_ptr(platform_ids[0], "test_fn").unwrap();

        let device_ids =
            get_device_ids(&platform_id, DeviceType::new(DeviceType::DEFAULT).unwrap()).unwrap();
        assert!(0 < device_ids.len());

        let context = create_context(&None, device_ids, None, WrapMutPtr::null());
        let context = context.unwrap();
        eprintln!("CL_CONTEXT_PTR: {:?}", context);
        assert_ne!(context.unwrap(), ptr::null_mut());
        release_context(context).unwrap();
    }
    #[test]
    fn test_create_context_from_type() {
        let platform_ids = get_platform_ids().unwrap();
        // Choose the first platform
        // let platform_id = platform_ids[0];
        let platform_id = PlatformPtr::from_ptr(platform_ids[0], "test_fn").unwrap();
        // let properties = vec![ContextProperties::PLATFORM, platform_id as isize, 0];
        let properties = ContextProperties.platform(&platform_id);
        let default_device = DeviceType::new(DeviceType::DEFAULT).unwrap();
        let context =
            create_context_from_type(&properties, default_device, None, WrapMutPtr::null());
        let context = context.unwrap();
        eprintln!("CL_CONTEXT_PTR: {:?}", context);
        assert_ne!(context.unwrap(), ptr::null_mut());
        release_context(context).unwrap();
    }
    #[test]
    fn test_get_context_info_1() {
        let platform_ids = get_platform_ids().unwrap();
        // Choose the first platform
        // let platform_id = platform_ids[0];
        let platform_id = PlatformPtr::from_ptr(platform_ids[0], "test_fn").unwrap();
        // let properties = vec![ContextProperties::PLATFORM, platform_id as isize, 0];
        let properties = ContextProperties.platform(&platform_id);
        let default_device = DeviceType::new(DeviceType::DEFAULT).unwrap();
        let context = create_context_from_type(
            &properties,
            default_device,
            None,
            WrappedMutablePointer::null(),
        );
        let context = context.unwrap();
        eprintln!("CL_CONTEXT_PTR: {:?}", context);
        assert_ne!(context.unwrap(), ptr::null_mut());
        let device = get_context_info(&context, ContextInfo::DEVICES);
        eprintln!("CL_CONTEXT_DEVICE: {:?}", device);
        assert_ne!(device.unwrap().unwrap_arr_cptr().unwrap()[0], 0);

        let properties = get_context_info(&context, ContextInfo::PROPERTIES);
        eprintln!("CL_CONTEXT_PROPERTIES: {:?}", properties);
        let re_platform_id = properties.unwrap().unwrap_arr_cptr().unwrap()[1];
        assert_eq!(re_platform_id, platform_id.unwrap() as isize);
        release_context(context).unwrap();
    }
    #[test]
    fn test_get_context_info_2() {
        let platform_ids = get_platform_ids().unwrap();
        // Choose the first platform
        // let platform_id = platform_ids[0];
        let platform_id = PlatformPtr::from_ptr(platform_ids[0], "test_fn").unwrap();

        // let properties = vec![ContextProperties::PLATFORM, platform_id as isize, 0];
        let properties = ContextProperties.platform(&platform_id);
        let default_device = DeviceType::new(DeviceType::DEFAULT).unwrap();
        let context = create_context_from_type(
            &properties,
            default_device,
            None,
            WrappedMutablePointer::null(),
        );
        let context = context.unwrap();
        eprintln!("CL_CONTEXT_PTR: {:?}", context);
        assert_ne!(context.unwrap(), ptr::null_mut());
        let device_count = get_context_info(&context, ContextInfo::NUM_DEVICES);
        eprintln!("CL_CONTEXT_DEVICE_COUNT: {:?}", device_count);
        assert_ne!(device_count.unwrap().unwrap_uint().unwrap(), 0);
        let ref_count = get_context_info(&context, ContextInfo::REFERENCE_COUNT);
        eprintln!("CL_CONTEXT_REFERENCE_COUNT: {:?}", ref_count);
        assert_ne!(ref_count.unwrap().unwrap_uint().unwrap(), 0);

        release_context(context).unwrap();
    }
}
