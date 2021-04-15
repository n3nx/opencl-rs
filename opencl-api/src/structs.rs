/*
 * structs.rs - Constants defined structs for easier access.header
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

#![allow(non_upper_case_globals, dead_code)]
use crate::errors::ValidationError;
use crate::gen_add_trait;
use crate::helpers::{BitfieldResult, GetSetGo, Properties, QueueProperties};
// use libc::c_void;
use opencl_heads::consts::*;
use opencl_heads::types::*;

#[non_exhaustive]
pub struct StatusCode;
impl StatusCode {
    // Error Codes
    pub const SUCCESS: cl_int = CL_SUCCESS;
    pub const DEVICE_NOT_FOUND: cl_int = CL_DEVICE_NOT_FOUND;
    pub const DEVICE_NOT_AVAILABLE: cl_int = CL_DEVICE_NOT_AVAILABLE;
    pub const COMPILER_NOT_AVAILABLE: cl_int = CL_COMPILER_NOT_AVAILABLE;
    pub const MEM_OBJECT_ALLOCATION_FAILURE: cl_int = CL_MEM_OBJECT_ALLOCATION_FAILURE;
    pub const OUT_OF_RESOURCES: cl_int = CL_OUT_OF_RESOURCES;
    pub const OUT_OF_HOST_MEMORY: cl_int = CL_OUT_OF_HOST_MEMORY;
    pub const PROFILING_INFO_NOT_AVAILABLE: cl_int = CL_PROFILING_INFO_NOT_AVAILABLE;
    pub const MEM_COPY_OVERLAP: cl_int = CL_MEM_COPY_OVERLAP;
    pub const IMAGE_FORMAT_MISMATCH: cl_int = CL_IMAGE_FORMAT_MISMATCH;
    pub const IMAGE_FORMAT_NOT_SUPPORTED: cl_int = CL_IMAGE_FORMAT_NOT_SUPPORTED;
    pub const BUILD_PROGRAM_FAILURE: cl_int = CL_BUILD_PROGRAM_FAILURE;
    pub const MAP_FAILURE: cl_int = CL_MAP_FAILURE;
    // #ifdef CL_VERSION_1_1
    pub const MISALIGNED_SUB_BUFFER_OFFSET: cl_int = CL_MISALIGNED_SUB_BUFFER_OFFSET;
    pub const EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST: cl_int =
        CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST;
    // #endif
    // #ifdef CL_VERSION_1_2
    pub const COMPILE_PROGRAM_FAILURE: cl_int = CL_COMPILE_PROGRAM_FAILURE;
    pub const LINKER_NOT_AVAILABLE: cl_int = CL_LINKER_NOT_AVAILABLE;
    pub const LINK_PROGRAM_FAILURE: cl_int = CL_LINK_PROGRAM_FAILURE;
    pub const DEVICE_PARTITION_FAILED: cl_int = CL_DEVICE_PARTITION_FAILED;
    pub const KERNEL_ARG_INFO_NOT_AVAILABLE: cl_int = CL_KERNEL_ARG_INFO_NOT_AVAILABLE;
    // #endif
    pub const INVALID_VALUE: cl_int = CL_INVALID_VALUE;
    pub const INVALID_DEVICE_TYPE: cl_int = CL_INVALID_DEVICE_TYPE;
    pub const INVALID_PLATFORM: cl_int = CL_INVALID_PLATFORM;
    pub const INVALID_DEVICE: cl_int = CL_INVALID_DEVICE;
    pub const INVALID_CONTEXT: cl_int = CL_INVALID_CONTEXT;
    pub const INVALID_QUEUE_PROPERTIES: cl_int = CL_INVALID_QUEUE_PROPERTIES;
    pub const INVALID_COMMAND_QUEUE: cl_int = CL_INVALID_COMMAND_QUEUE;
    pub const INVALID_HOST_PTR: cl_int = CL_INVALID_HOST_PTR;
    pub const INVALID_MEM_OBJECT: cl_int = CL_INVALID_MEM_OBJECT;
    pub const INVALID_IMAGE_FORMAT_DESCRIPTOR: cl_int = CL_INVALID_IMAGE_FORMAT_DESCRIPTOR;
    pub const INVALID_IMAGE_SIZE: cl_int = CL_INVALID_IMAGE_SIZE;
    pub const INVALID_SAMPLER: cl_int = CL_INVALID_SAMPLER;
    pub const INVALID_BINARY: cl_int = CL_INVALID_BINARY;
    pub const INVALID_BUILD_OPTIONS: cl_int = CL_INVALID_BUILD_OPTIONS;
    pub const INVALID_PROGRAM: cl_int = CL_INVALID_PROGRAM;
    pub const INVALID_PROGRAM_EXECUTABLE: cl_int = CL_INVALID_PROGRAM_EXECUTABLE;
    pub const INVALID_KERNEL_NAME: cl_int = CL_INVALID_KERNEL_NAME;
    pub const INVALID_KERNEL_DEFINITION: cl_int = CL_INVALID_KERNEL_DEFINITION;
    pub const INVALID_KERNEL: cl_int = CL_INVALID_KERNEL;
    pub const INVALID_ARG_INDEX: cl_int = CL_INVALID_ARG_INDEX;
    pub const INVALID_ARG_VALUE: cl_int = CL_INVALID_ARG_VALUE;
    pub const INVALID_ARG_SIZE: cl_int = CL_INVALID_ARG_SIZE;
    pub const INVALID_KERNEL_ARGS: cl_int = CL_INVALID_KERNEL_ARGS;
    pub const INVALID_WORK_DIMENSION: cl_int = CL_INVALID_WORK_DIMENSION;
    pub const INVALID_WORK_GROUP_SIZE: cl_int = CL_INVALID_WORK_GROUP_SIZE;
    pub const INVALID_WORK_ITEM_SIZE: cl_int = CL_INVALID_WORK_ITEM_SIZE;
    pub const INVALID_GLOBAL_OFFSET: cl_int = CL_INVALID_GLOBAL_OFFSET;
    pub const INVALID_EVENT_WAIT_LIST: cl_int = CL_INVALID_EVENT_WAIT_LIST;
    pub const INVALID_EVENT: cl_int = CL_INVALID_EVENT;
    pub const INVALID_OPERATION: cl_int = CL_INVALID_OPERATION;
    pub const INVALID_GL_OBJECT: cl_int = CL_INVALID_GL_OBJECT;
    pub const INVALID_BUFFER_SIZE: cl_int = CL_INVALID_BUFFER_SIZE;
    pub const INVALID_MIP_LEVEL: cl_int = CL_INVALID_MIP_LEVEL;
    pub const INVALID_GLOBAL_WORK_SIZE: cl_int = CL_INVALID_GLOBAL_WORK_SIZE;
    // cl_version_1_1;
    pub const INVALID_PROPERTY: cl_int = CL_INVALID_PROPERTY;
    // cl_version_1_2;
    pub const INVALID_IMAGE_DESCRIPTOR: cl_int = CL_INVALID_IMAGE_DESCRIPTOR;
    pub const INVALID_COMPILER_OPTIONS: cl_int = CL_INVALID_COMPILER_OPTIONS;
    pub const INVALID_LINKER_OPTIONS: cl_int = CL_INVALID_LINKER_OPTIONS;
    pub const INVALID_DEVICE_PARTITION_COUNT: cl_int = CL_INVALID_DEVICE_PARTITION_COUNT;
    // cl_version_2_0;
    pub const INVALID_PIPE_SIZE: cl_int = CL_INVALID_PIPE_SIZE;
    pub const INVALID_DEVICE_QUEUE: cl_int = CL_INVALID_DEVICE_QUEUE;
    // cl_version_2_2;
    pub const INVALID_SPEC_ID: cl_int = CL_INVALID_SPEC_ID;
    pub const MAX_SIZE_RESTRICTION_EXCEEDED: cl_int = CL_MAX_SIZE_RESTRICTION_EXCEEDED;
}

/******************************************************************
 *
 *                      Bitfields
 *
 *
*/

#[non_exhaustive]
pub struct DeviceType(cl_device_type);
impl DeviceType {
    /* cl_device_type - cl_bitfield */
    pub const DEFAULT: cl_device_type = CL_DEVICE_TYPE_DEFAULT;
    pub const CPU: cl_device_type = CL_DEVICE_TYPE_CPU;
    pub const GPU: cl_device_type = CL_DEVICE_TYPE_GPU;
    pub const ACCELERATOR: cl_device_type = CL_DEVICE_TYPE_ACCELERATOR;
    // 1.2;
    pub const CUSTOM: cl_device_type = CL_DEVICE_TYPE_CUSTOM;
    pub const ALL: cl_device_type = CL_DEVICE_TYPE_ALL;
}

gen_add_trait!(DeviceType);

impl GetSetGo for DeviceType {
    fn new(value: cl_device_type) -> BitfieldResult<Self> {
        type T = DeviceType;
        let fn_name = "DeviceType";
        match value {
            T::DEFAULT | T::CPU | T::GPU | T::ACCELERATOR | T::CUSTOM | T::ALL => {
                Ok(DeviceType(value))
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_type {
        self.0
    }
    fn set(&mut self, value: cl_device_type) -> BitfieldResult<()> {
        type T = DeviceType;
        let fn_name = "DeviceType";
        match value {
            T::DEFAULT | T::CPU | T::GPU | T::ACCELERATOR | T::CUSTOM | T::ALL => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
pub struct DeviceFPConfig(cl_device_fp_config);
impl DeviceFPConfig {
    /* cl_device_fp_config - cl_bitfield */
    pub const DENORM: cl_device_fp_config = CL_FP_DENORM;
    pub const INF_NAN: cl_device_fp_config = CL_FP_INF_NAN;
    pub const ROUND_TO_NEAREST: cl_device_fp_config = CL_FP_ROUND_TO_NEAREST;
    pub const ROUND_TO_ZERO: cl_device_fp_config = CL_FP_ROUND_TO_ZERO;
    pub const ROUND_TO_INF: cl_device_fp_config = CL_FP_ROUND_TO_INF;
    pub const FMA: cl_device_fp_config = CL_FP_FMA;
    // #ifdef CL_VERSION_1_1;
    pub const SOFT_FLOAT: cl_device_fp_config = CL_FP_SOFT_FLOAT;
    // #endif;
    // #ifdef CL_VERSION_1_2;
    pub const CORRECTLY_ROUNDED_DIVIDE_SQRT: cl_device_fp_config =
        CL_FP_CORRECTLY_ROUNDED_DIVIDE_SQRT;
    // #endif;
}

gen_add_trait!(DeviceFPConfig);

impl GetSetGo for DeviceFPConfig {
    fn new(value: cl_device_fp_config) -> BitfieldResult<Self> {
        type T = DeviceFPConfig;
        let fn_name = "DeviceFPConfig";
        match value {
            T::DENORM
            | T::INF_NAN
            | T::ROUND_TO_NEAREST
            | T::ROUND_TO_ZERO
            | T::ROUND_TO_INF
            | T::FMA
            | T::SOFT_FLOAT
            | T::CORRECTLY_ROUNDED_DIVIDE_SQRT => Ok(DeviceFPConfig(value)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_fp_config {
        self.0
    }
    fn set(&mut self, value: cl_device_fp_config) -> BitfieldResult<()> {
        type T = DeviceFPConfig;
        let fn_name = "DeviceFPConfig";
        match value {
            T::DENORM
            | T::INF_NAN
            | T::ROUND_TO_NEAREST
            | T::ROUND_TO_ZERO
            | T::ROUND_TO_INF
            | T::FMA
            | T::SOFT_FLOAT
            | T::CORRECTLY_ROUNDED_DIVIDE_SQRT => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
pub struct DeviceExecCapabilities(cl_device_exec_capabilities);
impl DeviceExecCapabilities {
    /* cl_device_exec_capabilities - cl_bitfield */
    pub const KERNEL: cl_device_exec_capabilities = CL_EXEC_KERNEL;
    pub const NATIVE_KERNEL: cl_device_exec_capabilities = CL_EXEC_NATIVE_KERNEL;
}

gen_add_trait!(DeviceExecCapabilities);

impl GetSetGo for DeviceExecCapabilities {
    fn new(value: cl_device_exec_capabilities) -> BitfieldResult<Self> {
        type T = DeviceExecCapabilities;
        let fn_name = "DeviceExecCapabilities";
        match value {
            T::KERNEL | T::NATIVE_KERNEL => Ok(DeviceExecCapabilities(value)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_exec_capabilities {
        self.0
    }
    fn set(&mut self, value: cl_device_exec_capabilities) -> BitfieldResult<()> {
        type T = DeviceExecCapabilities;
        let fn_name = "DeviceExecCapabilities";
        match value {
            T::KERNEL | T::NATIVE_KERNEL => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
pub struct CommandQueueProperties(cl_command_queue_properties);
impl CommandQueueProperties {
    /* cl_command_queue_properties - cl_bitfield */
    pub const OUT_OF_ORDER_EXEC_MODE_ENABLE: cl_command_queue_properties =
        CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE;
    pub const PROFILING_ENABLE: cl_command_queue_properties = CL_QUEUE_PROFILING_ENABLE;
    // #ifdef CL_VERSION_2_0;
    pub const ON_DEVICE: cl_command_queue_properties = CL_QUEUE_ON_DEVICE;
    pub const ON_DEVICE_DEFAULT: cl_command_queue_properties = CL_QUEUE_ON_DEVICE_DEFAULT;
    // #endif;
}

gen_add_trait!(CommandQueueProperties);

impl GetSetGo for CommandQueueProperties {
    fn new(value: cl_command_queue_properties) -> BitfieldResult<Self> {
        type T = CommandQueueProperties;
        let fn_name = "CommandQueueProperties";
        match value {
            T::OUT_OF_ORDER_EXEC_MODE_ENABLE
            | T::PROFILING_ENABLE
            | T::ON_DEVICE
            | T::ON_DEVICE_DEFAULT => Ok(CommandQueueProperties(value)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_command_queue_properties {
        self.0
    }
    fn set(&mut self, value: cl_command_queue_properties) -> BitfieldResult<()> {
        type T = CommandQueueProperties;
        let fn_name = "CommandQueueProperties";
        match value {
            T::OUT_OF_ORDER_EXEC_MODE_ENABLE
            | T::PROFILING_ENABLE
            | T::ON_DEVICE
            | T::ON_DEVICE_DEFAULT => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
pub struct DeviceAffinityDomain(cl_device_affinity_domain);
impl DeviceAffinityDomain {
    // #ifdef CL_VERSION_1_2;
    /* cl_device_affinity_domain - cl_bitfield*/
    pub const NUMA: cl_device_affinity_domain = CL_DEVICE_AFFINITY_DOMAIN_NUMA;
    pub const L4_CACHE: cl_device_affinity_domain = CL_DEVICE_AFFINITY_DOMAIN_L4_CACHE;
    pub const L3_CACHE: cl_device_affinity_domain = CL_DEVICE_AFFINITY_DOMAIN_L3_CACHE;
    pub const L2_CACHE: cl_device_affinity_domain = CL_DEVICE_AFFINITY_DOMAIN_L2_CACHE;
    pub const L1_CACHE: cl_device_affinity_domain = CL_DEVICE_AFFINITY_DOMAIN_L1_CACHE;
    pub const NEXT_PARTITIONABLE: cl_device_affinity_domain =
        CL_DEVICE_AFFINITY_DOMAIN_NEXT_PARTITIONABLE;
    // #endif;
}

gen_add_trait!(DeviceAffinityDomain);

impl GetSetGo for DeviceAffinityDomain {
    fn new(value: cl_device_affinity_domain) -> BitfieldResult<Self> {
        type T = DeviceAffinityDomain;
        let fn_name = "DeviceAffinityDomain";
        match value {
            T::NUMA
            | T::L1_CACHE
            | T::L2_CACHE
            | T::L3_CACHE
            | T::L4_CACHE
            | T::NEXT_PARTITIONABLE => Ok(DeviceAffinityDomain(value)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_affinity_domain {
        self.0
    }
    fn set(&mut self, value: cl_device_affinity_domain) -> BitfieldResult<()> {
        type T = DeviceAffinityDomain;
        let fn_name = "DeviceAffinityDomain";
        match value {
            T::NUMA
            | T::L1_CACHE
            | T::L2_CACHE
            | T::L3_CACHE
            | T::L4_CACHE
            | T::NEXT_PARTITIONABLE => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
pub struct DeviceSVMCapabilities(cl_device_svm_capabilities);
impl DeviceSVMCapabilities {
    // #ifdef CL_VERSION_2_0;
    /* cl_device_svm_capabilities - cl_bitfield */
    pub const COARSE_GRAIN_BUFFER: cl_device_svm_capabilities = CL_DEVICE_SVM_COARSE_GRAIN_BUFFER;
    pub const FINE_GRAIN_BUFFER: cl_device_svm_capabilities = CL_DEVICE_SVM_FINE_GRAIN_BUFFER;
    pub const FINE_GRAIN_SYSTEM: cl_device_svm_capabilities = CL_DEVICE_SVM_FINE_GRAIN_SYSTEM;
    pub const ATOMICS: cl_device_svm_capabilities = CL_DEVICE_SVM_ATOMICS;
    // #endif;
}

gen_add_trait!(DeviceSVMCapabilities);

impl GetSetGo for DeviceSVMCapabilities {
    fn new(value: cl_device_svm_capabilities) -> BitfieldResult<Self> {
        type T = DeviceSVMCapabilities;
        let fn_name = "DeviceSVMCapabilities";
        match value {
            T::COARSE_GRAIN_BUFFER | T::FINE_GRAIN_BUFFER | T::FINE_GRAIN_SYSTEM | T::ATOMICS => {
                Ok(DeviceSVMCapabilities(value))
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_svm_capabilities {
        self.0
    }
    fn set(&mut self, value: cl_device_svm_capabilities) -> BitfieldResult<()> {
        type T = DeviceSVMCapabilities;
        let fn_name = "DeviceSVMCapabilities";
        match value {
            T::COARSE_GRAIN_BUFFER | T::FINE_GRAIN_BUFFER | T::FINE_GRAIN_SYSTEM | T::ATOMICS => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
pub struct MemFlags(cl_mem_flags);
impl MemFlags {
    /* cl_mem_flags and cl_svm_mem_flags - cl_bitfield */
    pub const READ_WRITE: cl_mem_flags = CL_MEM_READ_WRITE;
    pub const WRITE_ONLY: cl_mem_flags = CL_MEM_WRITE_ONLY;
    pub const READ_ONLY: cl_mem_flags = CL_MEM_READ_ONLY;
    pub const USE_HOST_PTR: cl_mem_flags = CL_MEM_USE_HOST_PTR;
    pub const ALLOC_HOST_PTR: cl_mem_flags = CL_MEM_ALLOC_HOST_PTR;
    pub const COPY_HOST_PTR: cl_mem_flags = CL_MEM_COPY_HOST_PTR;
    /* reserved: cl_bitfield = 1 << 6: cl_bitfield = */
    // #ifdef CL_VERSION_1_2;
    pub const HOST_WRITE_ONLY: cl_mem_flags = CL_MEM_HOST_WRITE_ONLY;
    pub const HOST_READ_ONLY: cl_mem_flags = CL_MEM_HOST_READ_ONLY;
    pub const HOST_NO_ACCESS: cl_mem_flags = CL_MEM_HOST_NO_ACCESS;
    // #endif;
    // #ifdef CL_VERSION_2_0;
    pub const KERNEL_READ_AND_WRITE: cl_mem_flags = CL_MEM_KERNEL_READ_AND_WRITE;
    pub const SVM_FINE_GRAIN_BUFFER: cl_mem_flags = CL_MEM_SVM_FINE_GRAIN_BUFFER; /* used by cl_svm_mem_flags only */
    pub const SVM_ATOMICS: cl_mem_flags = CL_MEM_SVM_ATOMICS; /* used by cl_svm_mem_flags only */
    // #endif;
}

gen_add_trait!(MemFlags);

impl GetSetGo for MemFlags {
    fn new(value: cl_mem_flags) -> BitfieldResult<Self> {
        type T = MemFlags;
        let fn_name = "MemFlags";
        match value {
            T::READ_WRITE
            | T::WRITE_ONLY
            | T::READ_ONLY
            | T::USE_HOST_PTR
            | T::ALLOC_HOST_PTR
            | T::COPY_HOST_PTR
            | T::HOST_WRITE_ONLY
            | T::HOST_READ_ONLY
            | T::HOST_NO_ACCESS
            | T::KERNEL_READ_AND_WRITE
            | T::SVM_FINE_GRAIN_BUFFER
            | T::SVM_ATOMICS => Ok(MemFlags(value)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_mem_flags {
        self.0
    }
    fn set(&mut self, value: cl_mem_flags) -> BitfieldResult<()> {
        type T = MemFlags;
        let fn_name = "MemFlags";
        match value {
            T::READ_WRITE
            | T::WRITE_ONLY
            | T::READ_ONLY
            | T::USE_HOST_PTR
            | T::ALLOC_HOST_PTR
            | T::COPY_HOST_PTR
            | T::HOST_WRITE_ONLY
            | T::HOST_READ_ONLY
            | T::HOST_NO_ACCESS
            | T::KERNEL_READ_AND_WRITE
            | T::SVM_FINE_GRAIN_BUFFER
            | T::SVM_ATOMICS => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

// #ifdef CL_VERSION_1_2;
/* cl_mem_migration_flags - cl_bitfield */
#[non_exhaustive]
pub struct MemMigrationFlags(cl_mem_migration_flags);
impl MemMigrationFlags {
    pub const OBJECT_HOST: cl_mem_migration_flags = CL_MIGRATE_MEM_OBJECT_HOST;
    pub const OBJECT_CONTENT_UNDEFINED: cl_mem_migration_flags =
        CL_MIGRATE_MEM_OBJECT_CONTENT_UNDEFINED;
}
// #endif;

gen_add_trait!(MemMigrationFlags);

impl GetSetGo for MemMigrationFlags {
    fn new(value: cl_mem_migration_flags) -> BitfieldResult<Self> {
        type T = MemMigrationFlags;
        let fn_name = "MemMigrationFlags";
        match value {
            T::OBJECT_CONTENT_UNDEFINED | T::OBJECT_HOST => Ok(MemMigrationFlags(value)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_mem_migration_flags {
        self.0
    }
    fn set(&mut self, value: cl_mem_migration_flags) -> BitfieldResult<()> {
        type T = MemMigrationFlags;
        let fn_name = "MemMigrationFlags";
        match value {
            T::OBJECT_CONTENT_UNDEFINED | T::OBJECT_HOST => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
pub struct MapFlags(cl_map_flags);
impl MapFlags {
    /* cl_map_flags - cl_bitfield */
    pub const READ: cl_map_flags = CL_MAP_READ;
    pub const WRITE: cl_map_flags = CL_MAP_WRITE;
    // #ifdef CL_VERSION_1_2;
    pub const WRITE_INVALIDATE_REGION: cl_map_flags = CL_MAP_WRITE_INVALIDATE_REGION;
    // #endif;
}

gen_add_trait!(MapFlags);

impl GetSetGo for MapFlags {
    fn new(value: cl_map_flags) -> BitfieldResult<Self> {
        type T = MapFlags;
        let fn_name = "MapFlags";
        match value {
            T::READ | T::WRITE | T::WRITE_INVALIDATE_REGION => Ok(MapFlags(value)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_map_flags {
        self.0
    }
    fn set(&mut self, value: cl_map_flags) -> BitfieldResult<()> {
        type T = MapFlags;
        let fn_name = "MapFlags";
        match value {
            T::READ | T::WRITE | T::WRITE_INVALIDATE_REGION => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
pub struct KernelArgTypeQualifier(cl_kernel_arg_type_qualifier);
impl KernelArgTypeQualifier {
    // #ifdef CL_VERSION_1_2;
    /* cl_kernel_arg_type_qualifier - cl_bitfield */
    pub const NONE: cl_kernel_arg_type_qualifier = CL_KERNEL_ARG_TYPE_NONE;
    pub const CONST: cl_kernel_arg_type_qualifier = CL_KERNEL_ARG_TYPE_CONST;
    pub const RESTRICT: cl_kernel_arg_type_qualifier = CL_KERNEL_ARG_TYPE_RESTRICT;
    pub const VOLATILE: cl_kernel_arg_type_qualifier = CL_KERNEL_ARG_TYPE_VOLATILE;
    // #ifdef CL_VERSION_2_0;
    pub const PIPE: cl_kernel_arg_type_qualifier = CL_KERNEL_ARG_TYPE_PIPE;
    // #endif;
    // #endif;
}

gen_add_trait!(KernelArgTypeQualifier);

impl GetSetGo for KernelArgTypeQualifier {
    fn new(value: cl_kernel_arg_type_qualifier) -> BitfieldResult<Self> {
        type T = KernelArgTypeQualifier;
        let fn_name = "KernelArgTypeQualifier";
        match value {
            T::NONE | T::CONST | T::RESTRICT | T::VOLATILE | T::PIPE => {
                Ok(KernelArgTypeQualifier(value))
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_kernel_arg_type_qualifier {
        self.0
    }
    fn set(&mut self, value: cl_kernel_arg_type_qualifier) -> BitfieldResult<()> {
        type T = KernelArgTypeQualifier;
        let fn_name = "KernelArgTypeQualifier";
        match value {
            T::NONE | T::CONST | T::RESTRICT | T::VOLATILE | T::PIPE => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
pub struct DeviceAtomicCapabilities(cl_device_atomic_capabilities);
impl DeviceAtomicCapabilities {
    /* cl_device_atomic_capabilities - cl_bitfield */
    // #ifdef CL_VERSION_3_0;
    pub const ORDER_RELAXED: cl_device_atomic_capabilities = CL_DEVICE_ATOMIC_ORDER_RELAXED;
    pub const ORDER_ACQ_REL: cl_device_atomic_capabilities = CL_DEVICE_ATOMIC_ORDER_ACQ_REL;
    pub const ORDER_SEQ_CST: cl_device_atomic_capabilities = CL_DEVICE_ATOMIC_ORDER_SEQ_CST;
    pub const SCOPE_WORK_ITEM: cl_device_atomic_capabilities = CL_DEVICE_ATOMIC_SCOPE_WORK_ITEM;
    pub const SCOPE_WORK_GROUP: cl_device_atomic_capabilities = CL_DEVICE_ATOMIC_SCOPE_WORK_GROUP;
    pub const SCOPE_DEVICE: cl_device_atomic_capabilities = CL_DEVICE_ATOMIC_SCOPE_DEVICE;
    pub const SCOPE_ALL_DEVICES: cl_device_atomic_capabilities = CL_DEVICE_ATOMIC_SCOPE_ALL_DEVICES;
    // #endif;
}

gen_add_trait!(DeviceAtomicCapabilities);

impl GetSetGo for DeviceAtomicCapabilities {
    fn new(value: cl_device_atomic_capabilities) -> BitfieldResult<Self> {
        type T = DeviceAtomicCapabilities;
        let fn_name = "DeviceAtomicCapabilities";
        match value {
            T::ORDER_RELAXED
            | T::ORDER_ACQ_REL
            | T::ORDER_SEQ_CST
            | T::SCOPE_WORK_ITEM
            | T::SCOPE_WORK_GROUP
            | T::SCOPE_DEVICE
            | T::SCOPE_ALL_DEVICES => Ok(DeviceAtomicCapabilities(value)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_atomic_capabilities {
        self.0
    }
    fn set(&mut self, value: cl_device_atomic_capabilities) -> BitfieldResult<()> {
        type T = DeviceAtomicCapabilities;
        let fn_name = "DeviceAtomicCapabilities";
        match value {
            T::ORDER_RELAXED
            | T::ORDER_ACQ_REL
            | T::ORDER_SEQ_CST
            | T::SCOPE_WORK_ITEM
            | T::SCOPE_WORK_GROUP
            | T::SCOPE_DEVICE
            | T::SCOPE_ALL_DEVICES => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
pub struct DeviceDeviceEnqueueCapabilities(cl_device_device_enqueue_capabilities);
impl DeviceDeviceEnqueueCapabilities {
    /* cl_device_device_enqueue_capabilities - cl_bitfield */
    // #ifdef CL_VERSION_3_0;
    pub const SUPPORTED: cl_device_device_enqueue_capabilities = CL_DEVICE_QUEUE_SUPPORTED;
    pub const REPLACEABLE_DEFAULT: cl_device_device_enqueue_capabilities =
        CL_DEVICE_QUEUE_REPLACEABLE_DEFAULT;
    // #endif;
}

gen_add_trait!(DeviceDeviceEnqueueCapabilities);

impl GetSetGo for DeviceDeviceEnqueueCapabilities {
    fn new(value: cl_device_device_enqueue_capabilities) -> BitfieldResult<Self> {
        type T = DeviceDeviceEnqueueCapabilities;
        let fn_name = "DeviceDeviceEnqueueCapabilities";
        match value {
            T::SUPPORTED | T::REPLACEABLE_DEFAULT => Ok(DeviceDeviceEnqueueCapabilities(value)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_device_enqueue_capabilities {
        self.0
    }
    fn set(&mut self, value: cl_device_device_enqueue_capabilities) -> BitfieldResult<()> {
        type T = DeviceDeviceEnqueueCapabilities;
        let fn_name = "DeviceDeviceEnqueueCapabilities";
        match value {
            T::SUPPORTED | T::REPLACEABLE_DEFAULT => {
                self.0 = value;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

/*************************************************************
 *
 *                     API Parameters
 *
 * */

#[non_exhaustive]
pub struct PlatformInfo;
impl PlatformInfo {
    /* cl_platform_info - cl_uint */
    pub const PROFILE: cl_platform_info = CL_PLATFORM_PROFILE;
    pub const VERSION: cl_platform_info = CL_PLATFORM_VERSION;
    pub const NAME: cl_platform_info = CL_PLATFORM_NAME;
    pub const VENDOR: cl_platform_info = CL_PLATFORM_VENDOR;
    pub const EXTENSIONS: cl_platform_info = CL_PLATFORM_EXTENSIONS;
    // 2.1
    pub const HOST_TIMER_RESOLUTION: cl_platform_info = CL_PLATFORM_HOST_TIMER_RESOLUTION;
    // 3.0
    pub const NUMERIC_VERSION: cl_platform_info = CL_PLATFORM_NUMERIC_VERSION;
    // 3.0
    pub const EXTENSIONS_WITH_VERSION: cl_platform_info = CL_PLATFORM_EXTENSIONS_WITH_VERSION;
}

#[non_exhaustive]
pub struct DeviceInfo;
impl DeviceInfo {
    /* cl_device_info - cl_uint */
    pub const TYPE: cl_device_info = CL_DEVICE_TYPE;
    pub const VENDOR_ID: cl_device_info = CL_DEVICE_VENDOR_ID;
    pub const MAX_COMPUTE_UNITS: cl_device_info = CL_DEVICE_MAX_COMPUTE_UNITS;
    pub const MAX_WORK_ITEM_DIMENSIONS: cl_device_info = CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS;
    pub const MAX_WORK_GROUP_SIZE: cl_device_info = CL_DEVICE_MAX_WORK_GROUP_SIZE;
    pub const MAX_WORK_ITEM_SIZES: cl_device_info = CL_DEVICE_MAX_WORK_ITEM_SIZES;
    pub const PREFERRED_VECTOR_WIDTH_CHAR: cl_device_info = CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR;
    pub const PREFERRED_VECTOR_WIDTH_SHORT: cl_device_info = CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT;
    pub const PREFERRED_VECTOR_WIDTH_INT: cl_device_info = CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT;
    pub const PREFERRED_VECTOR_WIDTH_LONG: cl_device_info = CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG;
    pub const PREFERRED_VECTOR_WIDTH_FLOAT: cl_device_info = CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT;
    pub const PREFERRED_VECTOR_WIDTH_DOUBLE: cl_device_info =
        CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE;
    pub const MAX_CLOCK_FREQUENCY: cl_device_info = CL_DEVICE_MAX_CLOCK_FREQUENCY;
    pub const ADDRESS_BITS: cl_device_info = CL_DEVICE_ADDRESS_BITS;
    pub const MAX_READ_IMAGE_ARGS: cl_device_info = CL_DEVICE_MAX_READ_IMAGE_ARGS;
    pub const MAX_WRITE_IMAGE_ARGS: cl_device_info = CL_DEVICE_MAX_WRITE_IMAGE_ARGS;
    pub const MAX_MEM_ALLOC_SIZE: cl_device_info = CL_DEVICE_MAX_MEM_ALLOC_SIZE;
    pub const IMAGE2D_MAX_WIDTH: cl_device_info = CL_DEVICE_IMAGE2D_MAX_WIDTH;
    pub const IMAGE2D_MAX_HEIGHT: cl_device_info = CL_DEVICE_IMAGE2D_MAX_HEIGHT;
    pub const IMAGE3D_MAX_WIDTH: cl_device_info = CL_DEVICE_IMAGE3D_MAX_WIDTH;
    pub const IMAGE3D_MAX_HEIGHT: cl_device_info = CL_DEVICE_IMAGE3D_MAX_HEIGHT;
    pub const IMAGE3D_MAX_DEPTH: cl_device_info = CL_DEVICE_IMAGE3D_MAX_DEPTH;
    pub const IMAGE_SUPPORT: cl_device_info = CL_DEVICE_IMAGE_SUPPORT;
    pub const MAX_PARAMETER_SIZE: cl_device_info = CL_DEVICE_MAX_PARAMETER_SIZE;
    pub const MAX_SAMPLERS: cl_device_info = CL_DEVICE_MAX_SAMPLERS;
    pub const MEM_BASE_ADDR_ALIGN: cl_device_info = CL_DEVICE_MEM_BASE_ADDR_ALIGN;
    pub const MIN_DATA_TYPE_ALIGN_SIZE: cl_device_info = CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE;
    pub const SINGLE_FP_CONFIG: cl_device_info = CL_DEVICE_SINGLE_FP_CONFIG;
    pub const GLOBAL_MEM_CACHE_TYPE: cl_device_info = CL_DEVICE_GLOBAL_MEM_CACHE_TYPE;
    pub const GLOBAL_MEM_CACHELINE_SIZE: cl_device_info = CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE;
    pub const GLOBAL_MEM_CACHE_SIZE: cl_device_info = CL_DEVICE_GLOBAL_MEM_CACHE_SIZE;
    pub const GLOBAL_MEM_SIZE: cl_device_info = CL_DEVICE_GLOBAL_MEM_SIZE;
    pub const MAX_CONSTANT_BUFFER_SIZE: cl_device_info = CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE;
    pub const MAX_CONSTANT_ARGS: cl_device_info = CL_DEVICE_MAX_CONSTANT_ARGS;
    pub const LOCAL_MEM_TYPE: cl_device_info = CL_DEVICE_LOCAL_MEM_TYPE;
    pub const LOCAL_MEM_SIZE: cl_device_info = CL_DEVICE_LOCAL_MEM_SIZE;
    pub const ERROR_CORRECTION_SUPPORT: cl_device_info = CL_DEVICE_ERROR_CORRECTION_SUPPORT;
    pub const PROFILING_TIMER_RESOLUTION: cl_device_info = CL_DEVICE_PROFILING_TIMER_RESOLUTION;
    pub const ENDIAN_LITTLE: cl_device_info = CL_DEVICE_ENDIAN_LITTLE;
    pub const AVAILABLE: cl_device_info = CL_DEVICE_AVAILABLE;
    pub const COMPILER_AVAILABLE: cl_device_info = CL_DEVICE_COMPILER_AVAILABLE;
    pub const EXECUTION_CAPABILITIES: cl_device_info = CL_DEVICE_EXECUTION_CAPABILITIES;
    pub const QUEUE_PROPERTIES: cl_device_info = CL_DEVICE_QUEUE_PROPERTIES; /* deprecated */
    // #ifdef CL_VERSION_2_0;
    pub const QUEUE_ON_HOST_PROPERTIES: cl_device_info = CL_DEVICE_QUEUE_ON_HOST_PROPERTIES;
    // #endif;
    pub const NAME: cl_device_info = CL_DEVICE_NAME;
    pub const VENDOR: cl_device_info = CL_DEVICE_VENDOR;
    pub const DRIVER_VERSION: cl_device_info = CL_DRIVER_VERSION;
    pub const PROFILE: cl_device_info = CL_DEVICE_PROFILE;
    pub const VERSION: cl_device_info = CL_DEVICE_VERSION;
    pub const EXTENSIONS: cl_device_info = CL_DEVICE_EXTENSIONS;
    pub const PLATFORM: cl_device_info = CL_DEVICE_PLATFORM;
    // #ifdef CL_VERSION_1_2;
    pub const DOUBLE_FP_CONFIG: cl_device_info = CL_DEVICE_DOUBLE_FP_CONFIG;
    // #endif;
    /* 0x1033 reserved for CL_DEVICE_HALF_FP_CONFIG which is already defined in "cl_ext.h" */
    // #ifdef CL_VERSION_1_1;
    pub const PREFERRED_VECTOR_WIDTH_HALF: cl_device_info = CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF;
    pub const HOST_UNIFIED_MEMORY: cl_device_info = CL_DEVICE_HOST_UNIFIED_MEMORY; /* deprecated */
    pub const NATIVE_VECTOR_WIDTH_CHAR: cl_device_info = CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR;
    pub const NATIVE_VECTOR_WIDTH_SHORT: cl_device_info = CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT;
    pub const NATIVE_VECTOR_WIDTH_INT: cl_device_info = CL_DEVICE_NATIVE_VECTOR_WIDTH_INT;
    pub const NATIVE_VECTOR_WIDTH_LONG: cl_device_info = CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG;
    pub const NATIVE_VECTOR_WIDTH_FLOAT: cl_device_info = CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT;
    pub const NATIVE_VECTOR_WIDTH_DOUBLE: cl_device_info = CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE;
    pub const NATIVE_VECTOR_WIDTH_HALF: cl_device_info = CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF;
    pub const OPENCL_C_VERSION: cl_device_info = CL_DEVICE_OPENCL_C_VERSION;
    // #endif;
    // #ifdef CL_VERSION_1_2;
    pub const LINKER_AVAILABLE: cl_device_info = CL_DEVICE_LINKER_AVAILABLE;
    pub const BUILT_IN_KERNELS: cl_device_info = CL_DEVICE_BUILT_IN_KERNELS;
    pub const IMAGE_MAX_BUFFER_SIZE: cl_device_info = CL_DEVICE_IMAGE_MAX_BUFFER_SIZE;
    pub const IMAGE_MAX_ARRAY_SIZE: cl_device_info = CL_DEVICE_IMAGE_MAX_ARRAY_SIZE;
    pub const PARENT_DEVICE: cl_device_info = CL_DEVICE_PARENT_DEVICE;
    pub const PARTITION_MAX_SUB_DEVICES: cl_device_info = CL_DEVICE_PARTITION_MAX_SUB_DEVICES;
    pub const PARTITION_PROPERTIES: cl_device_info = CL_DEVICE_PARTITION_PROPERTIES;
    pub const PARTITION_AFFINITY_DOMAIN: cl_device_info = CL_DEVICE_PARTITION_AFFINITY_DOMAIN;
    pub const PARTITION_TYPE: cl_device_info = CL_DEVICE_PARTITION_TYPE;
    pub const REFERENCE_COUNT: cl_device_info = CL_DEVICE_REFERENCE_COUNT;
    pub const PREFERRED_INTEROP_USER_SYNC: cl_device_info = CL_DEVICE_PREFERRED_INTEROP_USER_SYNC;
    pub const PRINTF_BUFFER_SIZE: cl_device_info = CL_DEVICE_PRINTF_BUFFER_SIZE;
    // #endif;
    // #ifdef CL_VERSION_2_0;
    pub const IMAGE_PITCH_ALIGNMENT: cl_device_info = CL_DEVICE_IMAGE_PITCH_ALIGNMENT;
    pub const IMAGE_BASE_ADDRESS_ALIGNMENT: cl_device_info = CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT;
    pub const MAX_READ_WRITE_IMAGE_ARGS: cl_device_info = CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS;
    pub const MAX_GLOBAL_VARIABLE_SIZE: cl_device_info = CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE;
    pub const QUEUE_ON_DEVICE_PROPERTIES: cl_device_info = CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES;
    pub const QUEUE_ON_DEVICE_PREFERRED_SIZE: cl_device_info =
        CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE;
    pub const QUEUE_ON_DEVICE_MAX_SIZE: cl_device_info = CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE;
    pub const MAX_ON_DEVICE_QUEUES: cl_device_info = CL_DEVICE_MAX_ON_DEVICE_QUEUES;
    pub const MAX_ON_DEVICE_EVENTS: cl_device_info = CL_DEVICE_MAX_ON_DEVICE_EVENTS;
    pub const SVM_CAPABILITIES: cl_device_info = CL_DEVICE_SVM_CAPABILITIES;
    pub const GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE: cl_device_info =
        CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE;
    pub const MAX_PIPE_ARGS: cl_device_info = CL_DEVICE_MAX_PIPE_ARGS;
    pub const PIPE_MAX_ACTIVE_RESERVATIONS: cl_device_info = CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS;
    pub const PIPE_MAX_PACKET_SIZE: cl_device_info = CL_DEVICE_PIPE_MAX_PACKET_SIZE;
    pub const PREFERRED_PLATFORM_ATOMIC_ALIGNMENT: cl_device_info =
        CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT;
    pub const PREFERRED_GLOBAL_ATOMIC_ALIGNMENT: cl_device_info =
        CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT;
    pub const PREFERRED_LOCAL_ATOMIC_ALIGNMENT: cl_device_info =
        CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT;
    // #endif;
    // #ifdef CL_VERSION_2_1;
    pub const IL_VERSION: cl_device_info = CL_DEVICE_IL_VERSION;
    pub const MAX_NUM_SUB_GROUPS: cl_device_info = CL_DEVICE_MAX_NUM_SUB_GROUPS;
    pub const SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS: cl_device_info =
        CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS;
    // #endif;
    // #if;CL_VERSION_3_0;
    pub const NUMERIC_VERSION: cl_device_info = CL_DEVICE_NUMERIC_VERSION;
    pub const EXTENSIONS_WITH_VERSION: cl_device_info = CL_DEVICE_EXTENSIONS_WITH_VERSION;
    pub const ILS_WITH_VERSION: cl_device_info = CL_DEVICE_ILS_WITH_VERSION;
    pub const BUILT_IN_KERNELS_WITH_VERSION: cl_device_info =
        CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION;
    pub const ATOMIC_MEMORY_CAPABILITIES: cl_device_info = CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES;
    pub const ATOMIC_FENCE_CAPABILITIES: cl_device_info = CL_DEVICE_ATOMIC_FENCE_CAPABILITIES;
    pub const NON_UNIFORM_WORK_GROUP_SUPPORT: cl_device_info =
        CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT;
    pub const OPENCL_C_ALL_VERSIONS: cl_device_info = CL_DEVICE_OPENCL_C_ALL_VERSIONS;
    pub const PREFERRED_WORK_GROUP_SIZE_MULTIPLE: cl_device_info =
        CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE;
    pub const WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT: cl_device_info =
        CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT;
    pub const GENERIC_ADDRESS_SPACE_SUPPORT: cl_device_info =
        CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT;
    /* 0x106A to 0x106E - Reserved for upcoming KHR extension */
    pub const OPENCL_C_FEATURES: cl_device_info = CL_DEVICE_OPENCL_C_FEATURES;
    pub const DEVICE_ENQUEUE_CAPABILITIES: cl_device_info = CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES;
    pub const PIPE_SUPPORT: cl_device_info = CL_DEVICE_PIPE_SUPPORT;
    pub const LATEST_CONFORMANCE_VERSION_PASSED: cl_device_info =
        CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED;
    // #endif
}

#[non_exhaustive]
pub struct DeviceMemCacheType;
impl DeviceMemCacheType {
    /* cl_device_mem_cache_type - cl_uint */
    pub const NONE: cl_device_mem_cache_type = CL_NONE;
    pub const READ_ONLY_CACHE: cl_device_mem_cache_type = CL_READ_ONLY_CACHE;
    pub const READ_WRITE_CACHE: cl_device_mem_cache_type = CL_READ_WRITE_CACHE;
}

#[non_exhaustive]
pub struct DeviceLocalMemType;
impl DeviceLocalMemType {
    /* cl_device_local_mem_type - cl_uint */
    pub const LOCAL: cl_device_local_mem_type = CL_LOCAL;
    pub const GLOBAL: cl_device_local_mem_type = CL_GLOBAL;
}

#[non_exhaustive]
pub struct ContextInfo;
impl ContextInfo {
    /* cl_context_info - cl_uint */
    pub const REFERENCE_COUNT: cl_context_info = CL_CONTEXT_REFERENCE_COUNT;
    pub const DEVICES: cl_context_info = CL_CONTEXT_DEVICES;
    pub const PROPERTIES: cl_context_info = CL_CONTEXT_PROPERTIES;
    // #ifdef CL_VERSION_1_1;
    pub const NUM_DEVICES: cl_context_info = CL_CONTEXT_NUM_DEVICES;
    // #endif;
}

#[non_exhaustive]
pub struct ContextProperties;
impl ContextProperties {
    /* cl_context_properties - cl_uint */
    const PLATFORM: cl_context_properties = CL_CONTEXT_PLATFORM;
    // #ifdef CL_VERSION_1_2;
    const INTEROP_USER_SYNC: cl_context_properties = CL_CONTEXT_INTEROP_USER_SYNC;
    // #endif;
    pub fn platform(&self, platform_id: cl_platform_id) -> Properties {
        Some(vec![Self::PLATFORM, platform_id as isize, 0])
    }
    pub fn interop_user_sync(&self, value: cl_bool) -> Properties {
        match value {
            0 | 1 => Some(vec![Self::INTEROP_USER_SYNC, value as isize, 0]),
            _ => None,
        }
    }
}

#[non_exhaustive]
pub struct DevicePartitionProperty;
impl DevicePartitionProperty {
    // #ifdef CL_VERSION_1_2;
    /* cl_device_partition_property - cl_uint */
    const EQUALLY: cl_device_partition_property = CL_DEVICE_PARTITION_EQUALLY;
    const BY_COUNTS: cl_device_partition_property = CL_DEVICE_PARTITION_BY_COUNTS;
    const BY_COUNTS_LIST_END: cl_device_partition_property = CL_DEVICE_PARTITION_BY_COUNTS_LIST_END;
    const BY_AFFINITY_DOMAIN: cl_device_partition_property = CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN;
    // #endif;
    pub fn equally(&self, compute_units: cl_uint) -> Properties {
        Some(vec![Self::EQUALLY, compute_units as isize, 0])
    }
    pub fn by_counts(&self, left_cu: cl_uint, right_cu: cl_uint) -> Properties {
        Some(vec![
            Self::BY_COUNTS,
            left_cu as isize,
            right_cu as isize,
            Self::BY_COUNTS_LIST_END,
            0,
        ])
    }
    pub fn by_affinity_domain(&self, domain: DeviceAffinityDomain) -> Properties {
        Some(vec![Self::BY_AFFINITY_DOMAIN, domain.get() as isize, 0])
    }
}

#[non_exhaustive]
pub struct CommandQueueInfo;
impl CommandQueueInfo {
    /* cl_command_queue_info - cl_uint */
    pub const CONTEXT: cl_command_queue_info = CL_QUEUE_CONTEXT;
    pub const DEVICE: cl_command_queue_info = CL_QUEUE_DEVICE;
    pub const REFERENCE_COUNT: cl_command_queue_info = CL_QUEUE_REFERENCE_COUNT;
    pub const PROPERTIES: cl_command_queue_info = CL_QUEUE_PROPERTIES;
    // #ifdef CL_VERSION_2_0;
    const SIZE: cl_command_queue_info = CL_QUEUE_SIZE;
    // #endif;
    // #ifdef CL_VERSION_2_1;
    pub const DEVICE_DEFAULT: cl_command_queue_info = CL_QUEUE_DEVICE_DEFAULT;
    // #endif;
    // #ifdef CL_VERSION_3_0;
    const PROPERTIES_ARRAY: cl_command_queue_info = CL_QUEUE_PROPERTIES_ARRAY;
    // #endif;
    pub fn properties(&self, queue_prop: CommandQueueProperties) -> QueueProperties {
        Some(vec![
            Self::PROPERTIES_ARRAY as cl_properties,
            queue_prop.get() as cl_properties,
            0,
        ])
    }
    pub fn size(&self, size: cl_uint) -> QueueProperties {
        Some(vec![Self::SIZE as cl_properties, size as cl_properties, 0])
    }
}

#[non_exhaustive]
pub struct ChannelOrder;
impl ChannelOrder {
    /* cl_channel_order - cl_uint */
    pub const R: cl_channel_order = CL_R;
    pub const A: cl_channel_order = CL_A;
    pub const RG: cl_channel_order = CL_RG;
    pub const RA: cl_channel_order = CL_RA;
    pub const RGB: cl_channel_order = CL_RGB;
    pub const RGBA: cl_channel_order = CL_RGBA;
    pub const BGRA: cl_channel_order = CL_BGRA;
    pub const ARGB: cl_channel_order = CL_ARGB;
    pub const INTENSITY: cl_channel_order = CL_INTENSITY;
    pub const LUMINANCE: cl_channel_order = CL_LUMINANCE;
    // #ifdef CL_VERSION_1_1;
    pub const Rx: cl_channel_order = CL_Rx;
    pub const RGx: cl_channel_order = CL_RGx;
    pub const RGBx: cl_channel_order = CL_RGBx;
    // #endif;
    // #ifdef CL_VERSION_1_2;
    pub const DEPTH: cl_channel_order = CL_DEPTH;
    pub const DEPTH_STENCIL: cl_channel_order = CL_DEPTH_STENCIL;
    // #endif;
    // #ifdef CL_VERSION_2_0;
    pub const SRGB: cl_channel_order = CL_sRGB;
    pub const SRGBx: cl_channel_order = CL_sRGBx;
    pub const SRGBA: cl_channel_order = CL_sRGBA;
    pub const SBGRA: cl_channel_order = CL_sBGRA;
    pub const ABGR: cl_channel_order = CL_ABGR;
    // #endif
}

#[non_exhaustive]
pub struct ChannelType;
impl ChannelType {
    /* cl_channel_type - cl_uint */
    pub const SNORM_INT8: cl_channel_type = CL_SNORM_INT8;
    pub const SNORM_INT16: cl_channel_type = CL_SNORM_INT16;
    pub const UNORM_INT8: cl_channel_type = CL_UNORM_INT8;
    pub const UNORM_INT16: cl_channel_type = CL_UNORM_INT16;
    pub const UNORM_SHORT_565: cl_channel_type = CL_UNORM_SHORT_565;
    pub const UNORM_SHORT_555: cl_channel_type = CL_UNORM_SHORT_555;
    pub const UNORM_INT_101010: cl_channel_type = CL_UNORM_INT_101010;
    pub const SIGNED_INT8: cl_channel_type = CL_SIGNED_INT8;
    pub const SIGNED_INT16: cl_channel_type = CL_SIGNED_INT16;
    pub const SIGNED_INT32: cl_channel_type = CL_SIGNED_INT32;
    pub const UNSIGNED_INT8: cl_channel_type = CL_UNSIGNED_INT8;
    pub const UNSIGNED_INT16: cl_channel_type = CL_UNSIGNED_INT16;
    pub const UNSIGNED_INT32: cl_channel_type = CL_UNSIGNED_INT32;
    pub const HALF_FLOAT: cl_channel_type = CL_HALF_FLOAT;
    pub const FLOAT: cl_channel_type = CL_FLOAT;
    // #ifdef CL_VERSION_1_2;
    pub const UNORM_INT24: cl_channel_type = CL_UNORM_INT24;
    // #endif;
    // #ifdef CL_VERSION_2_1;
    pub const UNORM_INT_101010_2: cl_channel_type = CL_UNORM_INT_101010_2;
    // #endif;
}

#[non_exhaustive]
pub struct MemObjectType;
impl MemObjectType {
    /* cl_mem_object_type - cl_uint */
    pub const BUFFER: cl_mem_object_type = CL_MEM_OBJECT_BUFFER;
    pub const IMAGE2D: cl_mem_object_type = CL_MEM_OBJECT_IMAGE2D;
    pub const IMAGE3D: cl_mem_object_type = CL_MEM_OBJECT_IMAGE3D;
    // #ifdef CL_VERSION_1_2;
    pub const IMAGE2D_ARRAY: cl_mem_object_type = CL_MEM_OBJECT_IMAGE2D_ARRAY;
    pub const IMAGE1D: cl_mem_object_type = CL_MEM_OBJECT_IMAGE1D;
    pub const IMAGE1D_ARRAY: cl_mem_object_type = CL_MEM_OBJECT_IMAGE1D_ARRAY;
    pub const IMAGE1D_BUFFER: cl_mem_object_type = CL_MEM_OBJECT_IMAGE1D_BUFFER;
    // #endif;
    // #ifdef CL_VERSION_2_0;
    pub const PIPE: cl_mem_object_type = CL_MEM_OBJECT_PIPE;
    // #endif;
}

#[non_exhaustive]
pub struct MemInfo;
impl MemInfo {
    /* cl_mem_info - cl_uint */
    pub const TYPE: cl_mem_info = CL_MEM_TYPE;
    pub const FLAGS: cl_mem_info = CL_MEM_FLAGS;
    pub const SIZE: cl_mem_info = CL_MEM_SIZE;
    pub const HOST_PTR: cl_mem_info = CL_MEM_HOST_PTR;
    pub const MAP_COUNT: cl_mem_info = CL_MEM_MAP_COUNT;
    pub const REFERENCE_COUNT: cl_mem_info = CL_MEM_REFERENCE_COUNT;
    pub const CONTEXT: cl_mem_info = CL_MEM_CONTEXT;
    // #ifdef CL_VERSION_1_1;
    pub const ASSOCIATED_MEMOBJECT: cl_mem_info = CL_MEM_ASSOCIATED_MEMOBJECT;
    pub const OFFSET: cl_mem_info = CL_MEM_OFFSET;
    // #endif;
    // #ifdef CL_VERSION_2_0;
    pub const USES_SVM_POINTER: cl_mem_info = CL_MEM_USES_SVM_POINTER;
    // #endif;
    // #ifdef CL_VERSION_3_0;
    pub const PROPERTIES: cl_mem_info = CL_MEM_PROPERTIES;
    // #endif;
}

#[non_exhaustive]
pub struct ImageInfo;
impl ImageInfo {
    /* cl_image_info - cl_uint */
    pub const FORMAT: cl_image_info = CL_IMAGE_FORMAT;
    pub const ELEMENT_SIZE: cl_image_info = CL_IMAGE_ELEMENT_SIZE;
    pub const ROW_PITCH: cl_image_info = CL_IMAGE_ROW_PITCH;
    pub const SLICE_PITCH: cl_image_info = CL_IMAGE_SLICE_PITCH;
    pub const WIDTH: cl_image_info = CL_IMAGE_WIDTH;
    pub const HEIGHT: cl_image_info = CL_IMAGE_HEIGHT;
    pub const DEPTH: cl_image_info = CL_IMAGE_DEPTH;
    // #ifdef CL_VERSION_1_2;
    pub const ARRAY_SIZE: cl_image_info = CL_IMAGE_ARRAY_SIZE;
    pub const BUFFER: cl_image_info = CL_IMAGE_BUFFER;
    pub const NUM_MIP_LEVELS: cl_image_info = CL_IMAGE_NUM_MIP_LEVELS;
    pub const NUM_SAMPLES: cl_image_info = CL_IMAGE_NUM_SAMPLES;
    // #endif;
}

#[non_exhaustive]
pub struct PipeInfo;
impl PipeInfo {
    /* cl_pipe_info - cl_uint */
    // #ifdef CL_VERSION_2_0;
    pub const PACKET_SIZE: cl_pipe_info = CL_PIPE_PACKET_SIZE;
    pub const MAX_PACKETS: cl_pipe_info = CL_PIPE_MAX_PACKETS;
    // #endif;
    // #ifdef CL_VERSION_3_0;
    pub const PROPERTIES: cl_pipe_info = CL_PIPE_PROPERTIES;
    // #endif;
}

#[non_exhaustive]
pub struct AddressingMode;
impl AddressingMode {
    /* cl_addressing_mode - cl_uint */
    pub const NONE: cl_addressing_mode = CL_ADDRESS_NONE;
    pub const CLAMP_TO_EDGE: cl_addressing_mode = CL_ADDRESS_CLAMP_TO_EDGE;
    pub const CLAMP: cl_addressing_mode = CL_ADDRESS_CLAMP;
    pub const REPEAT: cl_addressing_mode = CL_ADDRESS_REPEAT;
    // #ifdef CL_VERSION_1_1;
    pub const MIRRORED_REPEAT: cl_addressing_mode = CL_ADDRESS_MIRRORED_REPEAT;
    // #endif;
}

#[non_exhaustive]
pub struct FilterMode;
impl FilterMode {
    /* cl_filter_mode - cl_uint */
    pub const NEAREST: cl_filter_mode = CL_FILTER_NEAREST;
    pub const LINEAR: cl_filter_mode = CL_FILTER_LINEAR;
}

#[non_exhaustive]
pub struct SamplerInfo;
impl SamplerInfo {
    /* cl_sampler_info - cl_uint */
    pub const REFERENCE_COUNT: cl_sampler_info = CL_SAMPLER_REFERENCE_COUNT;
    pub const CONTEXT: cl_sampler_info = CL_SAMPLER_CONTEXT;
    pub const NORMALIZED_COORDS: cl_sampler_info = CL_SAMPLER_NORMALIZED_COORDS;
    pub const ADDRESSING_MODE: cl_sampler_info = CL_SAMPLER_ADDRESSING_MODE;
    pub const FILTER_MODE: cl_sampler_info = CL_SAMPLER_FILTER_MODE;
    // #ifdef CL_VERSION_2_0;
    /* These enumerants are for the cl_khr_mipmap_image extension.;
    They have since been added to cl_ext.h with an appropriate;
    KHR suffix, but are left here for backwards compatibility. */
    pub const MIP_FILTER_MODE: cl_sampler_info = CL_SAMPLER_MIP_FILTER_MODE;
    pub const LOD_MIN: cl_sampler_info = CL_SAMPLER_LOD_MIN;
    pub const LOD_MAX: cl_sampler_info = CL_SAMPLER_LOD_MAX;
    // #endif;
    // #ifdef CL_VERSION_3_0;
    pub const PROPERTIES: cl_sampler_info = CL_SAMPLER_PROPERTIES;
    // #endif;
}

#[non_exhaustive]
pub struct ProgramInfo;
impl ProgramInfo {
    /* cl_program_info - cl_uint */
    pub const REFERENCE_COUNT: cl_program_info = CL_PROGRAM_REFERENCE_COUNT;
    pub const CONTEXT: cl_program_info = CL_PROGRAM_CONTEXT;
    pub const NUM_DEVICES: cl_program_info = CL_PROGRAM_NUM_DEVICES;
    pub const DEVICES: cl_program_info = CL_PROGRAM_DEVICES;
    pub const SOURCE: cl_program_info = CL_PROGRAM_SOURCE;
    pub const BINARY_SIZES: cl_program_info = CL_PROGRAM_BINARY_SIZES;
    pub const BINARIES: cl_program_info = CL_PROGRAM_BINARIES;
    // #ifdef CL_VERSION_1_2;
    pub const NUM_KERNELS: cl_program_info = CL_PROGRAM_NUM_KERNELS;
    pub const KERNEL_NAMES: cl_program_info = CL_PROGRAM_KERNEL_NAMES;
    // #endif;
    // #ifdef CL_VERSION_2_1;
    pub const IL: cl_program_info = CL_PROGRAM_IL;
    // #endif;
    // #ifdef CL_VERSION_2_2;
    pub const SCOPE_GLOBAL_CTORS_PRESENT: cl_program_info = CL_PROGRAM_SCOPE_GLOBAL_CTORS_PRESENT;
    pub const SCOPE_GLOBAL_DTORS_PRESENT: cl_program_info = CL_PROGRAM_SCOPE_GLOBAL_DTORS_PRESENT;
    // #endif;
}

#[non_exhaustive]
pub struct ProgramBuildInfo;
impl ProgramBuildInfo {
    /* cl_program_build_info - cl_uint */
    pub const BUILD_STATUS: cl_program_build_info = CL_PROGRAM_BUILD_STATUS;
    pub const BUILD_OPTIONS: cl_program_build_info = CL_PROGRAM_BUILD_OPTIONS;
    pub const BUILD_LOG: cl_program_build_info = CL_PROGRAM_BUILD_LOG;
    // #ifdef CL_VERSION_1_2;
    pub const BINARY_TYPE: cl_program_build_info = CL_PROGRAM_BINARY_TYPE;
    // #endif;
    // #ifdef CL_VERSION_2_0;
    pub const BUILD_GLOBAL_VARIABLE_TOTAL_SIZE: cl_program_build_info =
        CL_PROGRAM_BUILD_GLOBAL_VARIABLE_TOTAL_SIZE;
    // #endif;
}

#[non_exhaustive]
pub struct ProgramBinaryType;
impl ProgramBinaryType {
    // #ifdef CL_VERSION_1_2;
    /* cl_program_binary_type - cl_uint */
    pub const NONE: cl_program_binary_type = CL_PROGRAM_BINARY_TYPE_NONE;
    pub const COMPILED_OBJECT: cl_program_binary_type = CL_PROGRAM_BINARY_TYPE_COMPILED_OBJECT;
    pub const LIBRARY: cl_program_binary_type = CL_PROGRAM_BINARY_TYPE_LIBRARY;
    pub const EXECUTABLE: cl_program_binary_type = CL_PROGRAM_BINARY_TYPE_EXECUTABLE;
    // #endif;
}

#[non_exhaustive]
pub struct BuildStatus;
impl BuildStatus {
    /* cl_build_status - cl_int */
    pub const SUCCESS: cl_build_status = CL_BUILD_SUCCESS;
    pub const NONE: cl_build_status = CL_BUILD_NONE;
    pub const ERROR: cl_build_status = CL_BUILD_ERROR;
    pub const IN_PROGRESS: cl_build_status = CL_BUILD_IN_PROGRESS;
}

#[non_exhaustive]
pub struct KernelInfo;
impl KernelInfo {
    /* cl_kernel_info - cl_uint */
    pub const FUNCTION_NAME: cl_kernel_info = CL_KERNEL_FUNCTION_NAME;
    pub const NUM_ARGS: cl_kernel_info = CL_KERNEL_NUM_ARGS;
    pub const REFERENCE_COUNT: cl_kernel_info = CL_KERNEL_REFERENCE_COUNT;
    pub const CONTEXT: cl_kernel_info = CL_KERNEL_CONTEXT;
    pub const PROGRAM: cl_kernel_info = CL_KERNEL_PROGRAM;
    // #ifdef CL_VERSION_1_2;
    pub const ATTRIBUTES: cl_kernel_info = CL_KERNEL_ATTRIBUTES;
    // #endif;
}

#[non_exhaustive]
pub struct KernelArgInfo;
impl KernelArgInfo {
    // #ifdef CL_VERSION_1_2;
    /* cl_kernel_arg_info - cl_uint */
    pub const ADDRESS_QUALIFIER: cl_kernel_arg_info = CL_KERNEL_ARG_ADDRESS_QUALIFIER;
    pub const ACCESS_QUALIFIER: cl_kernel_arg_info = CL_KERNEL_ARG_ACCESS_QUALIFIER;
    pub const TYPE_NAME: cl_kernel_arg_info = CL_KERNEL_ARG_TYPE_NAME;
    pub const TYPE_QUALIFIER: cl_kernel_arg_info = CL_KERNEL_ARG_TYPE_QUALIFIER;
    pub const NAME: cl_kernel_arg_info = CL_KERNEL_ARG_NAME;
    // #endif;
}

#[non_exhaustive]
pub struct KernelArgAddressQualifier;
impl KernelArgAddressQualifier {
    // #ifdef CL_VERSION_1_2;
    /* cl_kernel_arg_address_qualifier - cl_uint */
    pub const GLOBAL: cl_kernel_arg_address_qualifier = CL_KERNEL_ARG_ADDRESS_GLOBAL;
    pub const LOCAL: cl_kernel_arg_address_qualifier = CL_KERNEL_ARG_ADDRESS_LOCAL;
    pub const CONSTANT: cl_kernel_arg_address_qualifier = CL_KERNEL_ARG_ADDRESS_CONSTANT;
    pub const PRIVATE: cl_kernel_arg_address_qualifier = CL_KERNEL_ARG_ADDRESS_PRIVATE;
    // #endif;
}

#[non_exhaustive]
pub struct KernelArgAccessQualifier;
impl KernelArgAccessQualifier {
    // #ifdef CL_VERSION_1_2;
    /* cl_kernel_arg_access_qualifier */
    pub const READ_ONLY: cl_kernel_arg_access_qualifier = CL_KERNEL_ARG_ACCESS_READ_ONLY;
    pub const WRITE_ONLY: cl_kernel_arg_access_qualifier = CL_KERNEL_ARG_ACCESS_WRITE_ONLY;
    pub const READ_WRITE: cl_kernel_arg_access_qualifier = CL_KERNEL_ARG_ACCESS_READ_WRITE;
    pub const NONE: cl_kernel_arg_access_qualifier = CL_KERNEL_ARG_ACCESS_NONE;
    // #endif;
}

#[non_exhaustive]
pub struct KernelWorkGroupInfo;
impl KernelWorkGroupInfo {
    /* cl_kernel_work_group_info - cl_uint */
    pub const WORK_GROUP_SIZE: cl_kernel_work_group_info = CL_KERNEL_WORK_GROUP_SIZE;
    pub const COMPILE_WORK_GROUP_SIZE: cl_kernel_work_group_info =
        CL_KERNEL_COMPILE_WORK_GROUP_SIZE;
    pub const LOCAL_MEM_SIZE: cl_kernel_work_group_info = CL_KERNEL_LOCAL_MEM_SIZE;
    pub const PREFERRED_WORK_GROUP_SIZE_MULTIPLE: cl_kernel_work_group_info =
        CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE;
    pub const PRIVATE_MEM_SIZE: cl_kernel_work_group_info = CL_KERNEL_PRIVATE_MEM_SIZE;
    // #ifdef CL_VERSION_1_2;
    pub const GLOBAL_WORK_SIZE: cl_kernel_work_group_info = CL_KERNEL_GLOBAL_WORK_SIZE;
    // #endif;
}

#[non_exhaustive]
pub struct KernelSubGroupInfo;
impl KernelSubGroupInfo {
    // #ifdef CL_VERSION_2_1;
    /* cl_kernel_sub_group_info - cl_uint */
    pub const MAX_SUB_GROUP_SIZE_FOR_NDRANGE: cl_kernel_sub_group_info =
        CL_KERNEL_MAX_SUB_GROUP_SIZE_FOR_NDRANGE;
    pub const SUB_GROUP_COUNT_FOR_NDRANGE: cl_kernel_sub_group_info =
        CL_KERNEL_SUB_GROUP_COUNT_FOR_NDRANGE;
    pub const LOCAL_SIZE_FOR_SUB_GROUP_COUNT: cl_kernel_sub_group_info =
        CL_KERNEL_LOCAL_SIZE_FOR_SUB_GROUP_COUNT;
    pub const MAX_NUM_SUB_GROUPS: cl_kernel_sub_group_info = CL_KERNEL_MAX_NUM_SUB_GROUPS;
    pub const COMPILE_NUM_SUB_GROUPS: cl_kernel_sub_group_info = CL_KERNEL_COMPILE_NUM_SUB_GROUPS;
    // #endif;
}

#[non_exhaustive]
pub struct KernelExecInfo;
impl KernelExecInfo {
    // #ifdef CL_VERSION_2_0;
    /* cl_kernel_exec_info - cl_uint */
    pub const EXEC_INFO_SVM_PTRS: cl_kernel_exec_info = CL_KERNEL_EXEC_INFO_SVM_PTRS;
    pub const EXEC_INFO_SVM_FINE_GRAIN_SYSTEM: cl_kernel_exec_info =
        CL_KERNEL_EXEC_INFO_SVM_FINE_GRAIN_SYSTEM;
    // #endif;
}

#[non_exhaustive]
pub struct EventInfo;
impl EventInfo {
    /* cl_event_info - cl_uint */
    pub const COMMAND_QUEUE: cl_event_info = CL_EVENT_COMMAND_QUEUE;
    pub const COMMAND_TYPE: cl_event_info = CL_EVENT_COMMAND_TYPE;
    pub const REFERENCE_COUNT: cl_event_info = CL_EVENT_REFERENCE_COUNT;
    pub const COMMAND_EXECUTION_STATUS: cl_event_info = CL_EVENT_COMMAND_EXECUTION_STATUS;
    // #ifdef CL_VERSION_1_1;
    pub const CONTEXT: cl_event_info = CL_EVENT_CONTEXT;
    // #endif;
}

#[non_exhaustive]
pub struct CommandType;
impl CommandType {
    /* cl_command_type - cl_uint */
    pub const NDRANGE_KERNEL: cl_command_type = CL_COMMAND_NDRANGE_KERNEL;
    pub const TASK: cl_command_type = CL_COMMAND_TASK;
    pub const NATIVE_KERNEL: cl_command_type = CL_COMMAND_NATIVE_KERNEL;
    pub const READ_BUFFER: cl_command_type = CL_COMMAND_READ_BUFFER;
    pub const WRITE_BUFFER: cl_command_type = CL_COMMAND_WRITE_BUFFER;
    pub const COPY_BUFFER: cl_command_type = CL_COMMAND_COPY_BUFFER;
    pub const READ_IMAGE: cl_command_type = CL_COMMAND_READ_IMAGE;
    pub const WRITE_IMAGE: cl_command_type = CL_COMMAND_WRITE_IMAGE;
    pub const COPY_IMAGE: cl_command_type = CL_COMMAND_COPY_IMAGE;
    pub const COPY_IMAGE_TO_BUFFER: cl_command_type = CL_COMMAND_COPY_IMAGE_TO_BUFFER;
    pub const COPY_BUFFER_TO_IMAGE: cl_command_type = CL_COMMAND_COPY_BUFFER_TO_IMAGE;
    pub const MAP_BUFFER: cl_command_type = CL_COMMAND_MAP_BUFFER;
    pub const MAP_IMAGE: cl_command_type = CL_COMMAND_MAP_IMAGE;
    pub const UNMAP_MEM_OBJECT: cl_command_type = CL_COMMAND_UNMAP_MEM_OBJECT;
    pub const MARKER: cl_command_type = CL_COMMAND_MARKER;
    pub const ACQUIRE_GL_OBJECTS: cl_command_type = CL_COMMAND_ACQUIRE_GL_OBJECTS;
    pub const RELEASE_GL_OBJECTS: cl_command_type = CL_COMMAND_RELEASE_GL_OBJECTS;
    // #ifdef CL_VERSION_1_1;
    pub const READ_BUFFER_RECT: cl_command_type = CL_COMMAND_READ_BUFFER_RECT;
    pub const WRITE_BUFFER_RECT: cl_command_type = CL_COMMAND_WRITE_BUFFER_RECT;
    pub const COPY_BUFFER_RECT: cl_command_type = CL_COMMAND_COPY_BUFFER_RECT;
    pub const USER: cl_command_type = CL_COMMAND_USER;
    // #endif;
    // #ifdef CL_VERSION_1_2;
    pub const BARRIER: cl_command_type = CL_COMMAND_BARRIER;
    pub const MIGRATE_MEM_OBJECTS: cl_command_type = CL_COMMAND_MIGRATE_MEM_OBJECTS;
    pub const FILL_BUFFER: cl_command_type = CL_COMMAND_FILL_BUFFER;
    pub const FILL_IMAGE: cl_command_type = CL_COMMAND_FILL_IMAGE;
    // #endif;
    // #ifdef CL_VERSION_2_0;
    pub const SVM_FREE: cl_command_type = CL_COMMAND_SVM_FREE;
    pub const SVM_MEMCPY: cl_command_type = CL_COMMAND_SVM_MEMCPY;
    pub const SVM_MEMFILL: cl_command_type = CL_COMMAND_SVM_MEMFILL;
    pub const SVM_MAP: cl_command_type = CL_COMMAND_SVM_MAP;
    pub const SVM_UNMAP: cl_command_type = CL_COMMAND_SVM_UNMAP;
    // #endif;
    // #ifdef CL_VERSION_3_0;
    pub const SVM_MIGRATE_MEM: cl_command_type = CL_COMMAND_SVM_MIGRATE_MEM;
    // #endif;
}

#[non_exhaustive]
pub struct CommandExecutionStatus;
impl CommandExecutionStatus {
    /* command execution status */
    pub const COMPLETE: cl_uint = CL_COMPLETE;
    pub const RUNNING: cl_uint = CL_RUNNING;
    pub const SUBMITTED: cl_uint = CL_SUBMITTED;
    pub const QUEUED: cl_uint = CL_QUEUED;
}

#[non_exhaustive]
pub struct BufferCreateType;
impl BufferCreateType {
    /* cl_buffer_create_type */
    // #ifdef CL_VERSION_1_1;
    pub const REGION: cl_buffer_create_type = CL_BUFFER_CREATE_TYPE_REGION;
    // #endif;
}

#[non_exhaustive]
pub struct ProfilingInfo;
impl ProfilingInfo {
    /* cl_profiling_info - cl_uint */
    pub const QUEUED: cl_profiling_info = CL_PROFILING_COMMAND_QUEUED;
    pub const SUBMIT: cl_profiling_info = CL_PROFILING_COMMAND_SUBMIT;
    pub const START: cl_profiling_info = CL_PROFILING_COMMAND_START;
    pub const END: cl_profiling_info = CL_PROFILING_COMMAND_END;
    // #ifdef CL_VERSION_2_0;
    pub const COMPLETE: cl_profiling_info = CL_PROFILING_COMMAND_COMPLETE;
    // #endif;
}

#[non_exhaustive]
pub struct KhronosVendorId;
impl KhronosVendorId {
    /* cl_khronos_vendor_id */
    pub const CODEPLAY: cl_khronos_vendor_id = CL_KHRONOS_VENDOR_ID_CODEPLAY;
}

#[non_exhaustive]
pub struct Version;
impl Version {
    // #ifdef CL_VERSION_3_0
    /* cl_version */
    pub const MAJOR_BITS: cl_version = CL_VERSION_MAJOR_BITS;
    pub const MINOR_BITS: cl_version = CL_VERSION_MINOR_BITS;
    pub const PATCH_BITS: cl_version = CL_VERSION_PATCH_BITS;
    pub const MAJOR_MASK: cl_bitfield = CL_VERSION_MAJOR_MASK;
    pub const MINOR_MASK: cl_bitfield = CL_VERSION_MINOR_MASK;
    pub const PATCH_MASK: cl_bitfield = CL_VERSION_PATCH_MASK;
}

// pub const MAJOR(version): cl_bitfield = (version) >> (CL_VERSION_MINOR_BITS + CL_VERSION_PATCH_BITS);
// pub const MINOR(version): cl_bitfield = ((version) >> CL_VERSION_PATCH_BITS) & CL_VERSION_MINOR_MASK;
// pub const PATCH(version): cl_bitfield = (version) & CL_VERSION_PATCH_MASK;
// pub const CL_MAKE_VERSION(major, minor, patch): cl_bitfield = (((major) & CL_VERSION_MAJOR_MASK)<< (CL_VERSION_MINOR_BITS + CL_VERSION_PATCH_BITS)) | (((minor) & CL_VERSION_MINOR_MASK) << CL_VERSION_PATCH_BITS) | ((patch) & CL_VERSION_PATCH_MASK);
// #endif
// #ifdef CL_VERSION_3_0
/* cl_version */
fn major(version: cl_ulong) -> cl_bitfield {
    (version) >> (CL_VERSION_MINOR_BITS + CL_VERSION_PATCH_BITS)
}

fn minor(version: cl_ulong) -> cl_bitfield {
    ((version) >> CL_VERSION_PATCH_BITS) & CL_VERSION_MINOR_MASK
}

fn patch(version: cl_ulong) -> cl_bitfield {
    (version) & CL_VERSION_PATCH_MASK
}

fn make_version(major: cl_ulong, minor: cl_ulong, patch: cl_ulong) -> cl_bitfield {
    (((major) & (minor) & CL_VERSION_MINOR_MASK) << CL_VERSION_PATCH_BITS)
        | ((patch) & CL_VERSION_PATCH_MASK)
}
