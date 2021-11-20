/*
 * sampler.rs - Sampler object API wrappers (Part of OpenCL Runtime Layer).
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
//!
//! A sampler object describes how to sample an image when the image is read in the kernel.
//! The built-in functions to read from an image in a kernel take a sampler as an argument.
//! The sampler arguments to the image read function can be sampler objects created using
//! OpenCL functions and passed as argument values to the kernel or can be samplers declared
//! inside a kernel.
//!
use crate::objects::enums::{ParamValue, Size};
use crate::objects::functions::status_update;
use crate::objects::structs::{SamplerInfo, StatusCode};
use crate::objects::types::{APIResult, ContextPtr, SamplerPtr};
use crate::{gen_param_value, size_getter};
use libc::c_void;
use opencl_heads::ffi;
use opencl_heads::ffi::clGetSamplerInfo;
use opencl_heads::types::*;
use std::ptr;

//TODO: Generalize the AddressingMode and FilterMode structs for create_sampler
//Deprecated since OpenCL 2.0
pub fn create_sampler(
    context: &ContextPtr,
    normalized_coords: cl_bool,
    addressing_mode: cl_addressing_mode,
    filter_mode: cl_filter_mode,
) -> APIResult<SamplerPtr> {
    let fn_name = "clCreateSampler";
    let mut status_code = StatusCode::INVALID_VALUE;
    let sampler_ptr = unsafe {
        ffi::clCreateSampler(
            context.unwrap(),
            normalized_coords,
            addressing_mode,
            filter_mode,
            &mut status_code,
        )
    };
    status_update(
        status_code,
        fn_name,
        SamplerPtr::from_ptr(sampler_ptr, fn_name)?,
    )
}

/// * context must be a valid OpenCL context.
/// * sampler_properties specifies a list of sampler property names and their
/// corresponding values. Each sampler property name is immediately followed
/// by the corresponding desired value. The list is terminated with 0. The
/// list of supported properties is described in the Sampler Properties table.
/// If a supported property and its value is not specified in sampler_properties,
/// its default value will be used. sampler_properties can be NULL in which
/// case the default values for supported sampler properties will be used.
pub fn create_sampler_with_properties(
    context: &ContextPtr,
    sampler_properties: Option<[cl_ulong; 5]>,
) -> APIResult<SamplerPtr> {
    let fn_name = "clCreateSamplerWithProperties";
    let mut status_code = StatusCode::INVALID_VALUE;
    let sampler_properties = match sampler_properties {
        Some(x) => x.as_ptr(),
        None => ptr::null(),
    };
    let sampler_ptr = unsafe {
        ffi::clCreateSamplerWithProperties(context.unwrap(), sampler_properties, &mut status_code)
    };
    status_update(
        status_code,
        fn_name,
        SamplerPtr::from_ptr(sampler_ptr, fn_name)?,
    )
}

pub fn retain_sampler(sampler: &SamplerPtr) -> APIResult<()> {
    let fn_name = "clRetainSampler";
    let status_code = unsafe { ffi::clRetainSampler(sampler.unwrap()) };
    status_update(status_code, fn_name, ())
}
pub fn release_sampler(sampler: &SamplerPtr) -> APIResult<()> {
    let fn_name = "clReleaseSampler";
    let status_code = unsafe { ffi::clReleaseSampler(sampler.unwrap()) };
    status_update(status_code, fn_name, ())
}

pub fn get_sampler_info(
    sampler: &SamplerPtr,
    param_name: cl_sampler_info,
) -> APIResult<ParamValue> {
    type S = SamplerInfo;
    let fn_name = "clGetSamplerInfo";
    let sampler = sampler.unwrap();
    size_getter!(get_sampler_info_size, clGetSamplerInfo);
    match param_name {
        S::ADDRESSING_MODE | S::FILTER_MODE | S::NORMALIZED_COORDS | S::REFERENCE_COUNT => {
            let param_value = gen_param_value!(clGetSamplerInfo, u32, sampler, param_name);
            Ok(ParamValue::UInt(param_value))
        }
        S::CONTEXT => {
            let param_value = gen_param_value!(clGetSamplerInfo, isize, sampler, param_name);
            Ok(ParamValue::CPtr(param_value))
        }
        // Missing before 3.0
        S::PROPERTIES => {
            let size = get_sampler_info_size(sampler, param_name)?;
            let param_value = gen_param_value!(clGetSamplerInfo, isize, sampler, param_name, size);
            Ok(ParamValue::ArrCPtr(param_value))
        }
        _ => status_update(40404, fn_name, ParamValue::default()),
    }
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
    use crate::api::context::{create_context, release_context};
    use crate::api::device::get_device_ids;
    use crate::api::platform::get_platform_ids;
    use crate::objects::bitfields::DeviceType;
    use crate::objects::structs::{AddressingMode, FilterMode};
    use crate::objects::traits::GetSetGo;
    use crate::objects::types::{PlatformPtr, WrapMutPtr};

    #[test]
    fn test_create_sampler() {
        let platform_ids = get_platform_ids().unwrap();
        // Choose the first platform
        // let platform_id = platform_ids[0];
        let platform_id = PlatformPtr::from_ptr(platform_ids[0], "test_fn").unwrap();

        let device_ids =
            get_device_ids(&platform_id, DeviceType::new(DeviceType::DEFAULT).unwrap()).unwrap();
        assert!(0 < device_ids.len());

        let context = create_context(&None, device_ids, None, WrapMutPtr::null());
        let context = context.unwrap();

        // TODO: Generalize the property generator for every API
        let sampler_properties = Some([
            SamplerInfo::ADDRESSING_MODE as cl_ulong,
            AddressingMode::REPEAT as cl_ulong,
            SamplerInfo::FILTER_MODE as cl_ulong,
            FilterMode::LINEAR as cl_ulong,
            0,
        ]);
        let sampler = create_sampler_with_properties(&context, sampler_properties);
        let sampler = sampler.unwrap();
        // eprintln!("CL_CONTEXT_PTR: {:?}", context);
        assert_ne!(sampler.unwrap(), ptr::null_mut());
        release_sampler(&sampler).unwrap();
        release_context(context).unwrap();
    }

    #[test]
    fn test_get_sampler_info() {
        let platform_ids = get_platform_ids().unwrap();
        // Choose the first platform
        // let platform_id = platform_ids[0];
        let platform_id = PlatformPtr::from_ptr(platform_ids[0], "test_fn").unwrap();

        let device_ids =
            get_device_ids(&platform_id, DeviceType::new(DeviceType::DEFAULT).unwrap()).unwrap();
        assert!(0 < device_ids.len());

        let context = create_context(&None, device_ids, None, WrapMutPtr::null());
        let context = context.unwrap();

        // TODO: Generalize the property generator for every API
        let sampler_properties = Some([
            SamplerInfo::ADDRESSING_MODE as cl_ulong,
            AddressingMode::REPEAT as cl_ulong,
            SamplerInfo::FILTER_MODE as cl_ulong,
            FilterMode::LINEAR as cl_ulong,
            0,
        ]);
        let sampler = create_sampler_with_properties(&context, sampler_properties);
        let sampler = sampler.unwrap();
        // Context information
        let sampler_context = get_sampler_info(&sampler, SamplerInfo::CONTEXT);
        eprintln!("{:?}", sampler_context);
        match sampler_context {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
        // Reference count
        let sampler_reference_count = get_sampler_info(&sampler, SamplerInfo::REFERENCE_COUNT);
        eprintln!("{:?}", sampler_reference_count);
        match sampler_reference_count {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
        // Addressing mode
        let sampler_addressing_mode = get_sampler_info(&sampler, SamplerInfo::ADDRESSING_MODE);
        eprintln!("{:?}", sampler_addressing_mode);
        match sampler_addressing_mode {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
        // Filter mode
        let sampler_filter_mode = get_sampler_info(&sampler, SamplerInfo::FILTER_MODE);
        eprintln!("{:?}", sampler_filter_mode);
        match sampler_filter_mode {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
        release_sampler(&sampler).unwrap();
        release_context(context).unwrap();
    }
}
