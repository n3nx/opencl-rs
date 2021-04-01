/*
 * types.rs - OpenCL header ABI bindings. See CREDITS.md for full details.
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

#![allow(non_camel_case_types)]
use libc::{c_char, c_void, intptr_t, size_t};

pub type cl_platform_id = *mut c_void;
pub type cl_device_id = *mut c_void;
pub type cl_context = *mut c_void;
pub type cl_command_queue = *mut c_void;
pub type cl_mem = *mut c_void;
pub type cl_program = *mut c_void;
pub type cl_kernel = *mut c_void;
pub type cl_event = *mut c_void;
pub type cl_sampler = *mut c_void;

// Primitive Data Types
pub type cl_char = i8;
pub type cl_uchar = u8;
pub type cl_short = i16;
pub type cl_ushort = u16;
pub type cl_int = i32;
pub type cl_uint = u32;
pub type cl_long = i64;
pub type cl_ulong = u64;
pub type cl_half = u16;
pub type cl_float = f32;
pub type cl_double = f64;

// BEGIN Header Types
pub type cl_bool = cl_uint;
pub type cl_bitfield = cl_ulong;
pub type cl_properties = cl_ulong;
pub type cl_device_type = cl_bitfield;
pub type cl_platform_info = cl_uint;
pub type cl_device_info = cl_uint;
pub type cl_device_fp_config = cl_bitfield;
pub type cl_device_mem_cache_type = cl_uint;
pub type cl_device_local_mem_type = cl_uint;
pub type cl_device_exec_capabilities = cl_bitfield;
// #[cfg(feature = "cl_version_2_0")]
pub type cl_device_svm_capabilities = cl_bitfield;
pub type cl_command_queue_properties = cl_bitfield;
// #[cfg(feature = "cl_version_1_2")]
pub type cl_device_partition_property = intptr_t;
// #[cfg(feature = "cl_version_1_2")]
pub type cl_device_affinity_domain = cl_bitfield;

pub type cl_context_properties = intptr_t;
pub type cl_context_info = cl_uint;
// #[cfg(feature = "cl_version_2_0")]
pub type cl_queue_properties = cl_properties;
pub type cl_command_queue_info = cl_uint;
pub type cl_channel_order = cl_uint;
pub type cl_channel_type = cl_uint;
pub type cl_mem_flags = cl_bitfield;

// #[cfg(feature = "cl_version_2_0")]
pub type cl_svm_mem_flags = cl_bitfield;

pub type cl_mem_object_type = cl_uint;
pub type cl_mem_info = cl_uint;
// #[cfg(feature = "cl_version_1_2")]
pub type cl_mem_migration_flags = cl_bitfield;
pub type cl_image_info = cl_uint;
// #[cfg(feature = "cl_version_1_1")]
pub type cl_buffer_create_type = cl_uint;
pub type cl_addressing_mode = cl_uint;
pub type cl_filter_mode = cl_uint;
pub type cl_sampler_info = cl_uint;
pub type cl_map_flags = cl_bitfield;
// #[cfg(feature = "cl_version_2_0")]
pub type cl_pipe_properties = intptr_t;
// #[cfg(feature = "cl_version_2_0")]
pub type cl_pipe_info = cl_uint;
pub type cl_program_info = cl_uint;
pub type cl_program_build_info = cl_uint;
// #[cfg(feature = "cl_version_1_2")]
pub type cl_program_binary_type = cl_uint;
pub type cl_build_status = cl_int;
pub type cl_kernel_info = cl_uint;
// #ifdef CL_VERSION_1_2
pub type cl_kernel_arg_info = cl_uint;
pub type cl_kernel_arg_address_qualifier = cl_uint;
pub type cl_kernel_arg_access_qualifier = cl_uint;
pub type cl_kernel_arg_type_qualifier = cl_bitfield;
// #endif
pub type cl_kernel_work_group_info = cl_uint;
// #ifdef CL_VERSION_2_1
pub type cl_kernel_sub_group_info = cl_uint;
// #endif
pub type cl_event_info = cl_uint;
pub type cl_command_type = cl_uint;
pub type cl_profiling_info = cl_uint;
// #ifdef CL_VERSION_2_0
pub type cl_sampler_properties = cl_properties;
pub type cl_kernel_exec_info = cl_uint;
// #endif
// #ifdef CL_VERSION_3_0
pub type cl_device_atomic_capabilities = cl_bitfield;
pub type cl_device_device_enqueue_capabilities = cl_bitfield;
pub type cl_khronos_vendor_id = cl_uint;
pub type cl_mem_properties = cl_properties;
pub type cl_version = cl_uint;
// #endif

#[repr(C)]
pub struct cl_image_format {
    pub image_channel_order: cl_channel_order,
    pub image_channel_data_type: cl_channel_type,
}

#[repr(C)]
pub struct cl_image_desc {
    pub image_type: cl_mem_object_type,
    pub image_width: size_t,
    pub image_height: size_t,
    pub image_depth: size_t,
    pub image_array_size: size_t,
    pub image_row_pitch: size_t,
    pub image_slice_pitch: size_t,
    pub num_mip_levels: cl_uint,
    pub mem_samples: cl_uint,
    #[cfg(feature = "cl_version_2_0")]
    // #ifdef CL_VERSION_2_0
    pub mem_object: cl_mem,
    // #endif
    pub buffer: cl_mem,
}

// #ifdef CL_VERSION_1_1
#[repr(C)]
pub struct cl_buffer_region {
    pub origin: size_t,
    pub size: size_t,
}
// #endif

// #ifdef CL_VERSION_3_0
pub const CL_NAME_VERSION_MAX_NAME_SIZE: usize = 64;
// typedef struct _cl_name_version {
//     cl_version              version;
//     char                    name[CL_NAME_VERSION_MAX_NAME_SIZE];
// } cl_name_version;
#[repr(C)]
pub struct cl_name_version {
    pub version: cl_version,
    pub name: [c_char; CL_NAME_VERSION_MAX_NAME_SIZE],
}
// #endif
