/*
 * bitfields.rs - opencl api bitfield objects.
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
use crate::errors::ValidationError;
use crate::gen_add_trait;
use crate::objects::traits::GetSetGo;
use crate::objects::types::BitfieldResult;
use opencl_heads::consts::*;
use opencl_heads::types::*;

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
        const MAX: cl_bitfield = T::DEFAULT + T::CPU + T::GPU + T::ACCELERATOR + T::CUSTOM + T::ALL;
        match value {
            val @ 1..=MAX => Ok(DeviceType(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_type {
        self.0
    }
    fn set(&mut self, value: cl_device_type) -> BitfieldResult<()> {
        type T = DeviceType;
        let fn_name = "DeviceType";
        const MAX: cl_bitfield = T::DEFAULT + T::CPU + T::GPU + T::ACCELERATOR + T::CUSTOM + T::ALL;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
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
        const MAX: cl_bitfield = T::DENORM
            | T::INF_NAN
            | T::ROUND_TO_NEAREST
            | T::ROUND_TO_ZERO
            | T::ROUND_TO_INF
            | T::FMA
            | T::SOFT_FLOAT
            | T::CORRECTLY_ROUNDED_DIVIDE_SQRT;
        match value {
            val @ 1..=MAX => Ok(DeviceFPConfig(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_fp_config {
        self.0
    }
    fn set(&mut self, value: cl_device_fp_config) -> BitfieldResult<()> {
        type T = DeviceFPConfig;
        let fn_name = "DeviceFPConfig";
        const MAX: cl_bitfield = T::DENORM
            | T::INF_NAN
            | T::ROUND_TO_NEAREST
            | T::ROUND_TO_ZERO
            | T::ROUND_TO_INF
            | T::FMA
            | T::SOFT_FLOAT
            | T::CORRECTLY_ROUNDED_DIVIDE_SQRT;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
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
        const MAX: cl_bitfield = T::KERNEL + T::NATIVE_KERNEL;
        match value {
            val @ 1..=MAX => Ok(DeviceExecCapabilities(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_exec_capabilities {
        self.0
    }
    fn set(&mut self, value: cl_device_exec_capabilities) -> BitfieldResult<()> {
        type T = DeviceExecCapabilities;
        let fn_name = "DeviceExecCapabilities";
        const MAX: cl_bitfield = T::KERNEL + T::NATIVE_KERNEL;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
#[derive(Debug)]
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
        const MAX: cl_bitfield = T::OUT_OF_ORDER_EXEC_MODE_ENABLE
            + T::ON_DEVICE
            + T::ON_DEVICE_DEFAULT
            + T::PROFILING_ENABLE;
        match value {
            val @ 1..=MAX => Ok(CommandQueueProperties(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_command_queue_properties {
        self.0
    }
    fn set(&mut self, value: cl_command_queue_properties) -> BitfieldResult<()> {
        type T = CommandQueueProperties;
        let fn_name = "CommandQueueProperties";
        const MAX: cl_bitfield = T::OUT_OF_ORDER_EXEC_MODE_ENABLE
            + T::ON_DEVICE
            + T::ON_DEVICE_DEFAULT
            + T::PROFILING_ENABLE;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
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
        const MAX: cl_bitfield =
            T::NUMA + T::L1_CACHE + T::L2_CACHE + T::L3_CACHE + T::L4_CACHE + T::NEXT_PARTITIONABLE;
        match value {
            val @ 1..=MAX => Ok(DeviceAffinityDomain(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_affinity_domain {
        self.0
    }
    fn set(&mut self, value: cl_device_affinity_domain) -> BitfieldResult<()> {
        type T = DeviceAffinityDomain;
        let fn_name = "DeviceAffinityDomain";
        const MAX: cl_bitfield =
            T::NUMA + T::L1_CACHE + T::L2_CACHE + T::L3_CACHE + T::L4_CACHE + T::NEXT_PARTITIONABLE;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
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
        const MAX: cl_bitfield =
            T::COARSE_GRAIN_BUFFER | T::FINE_GRAIN_BUFFER | T::FINE_GRAIN_SYSTEM + T::ATOMICS;
        match value {
            val @ 1..=MAX => Ok(DeviceSVMCapabilities(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_svm_capabilities {
        self.0
    }
    fn set(&mut self, value: cl_device_svm_capabilities) -> BitfieldResult<()> {
        type T = DeviceSVMCapabilities;
        let fn_name = "DeviceSVMCapabilities";
        const MAX: cl_bitfield =
            T::COARSE_GRAIN_BUFFER | T::FINE_GRAIN_BUFFER | T::FINE_GRAIN_SYSTEM + T::ATOMICS;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}

#[non_exhaustive]
#[derive(Clone)]
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
        const MAX: cl_bitfield = T::READ_WRITE
            + T::WRITE_ONLY
            + T::READ_ONLY
            + T::USE_HOST_PTR
            + T::ALLOC_HOST_PTR
            + T::COPY_HOST_PTR
            + T::HOST_WRITE_ONLY
            + T::HOST_READ_ONLY
            + T::HOST_NO_ACCESS
            + T::KERNEL_READ_AND_WRITE
            + T::SVM_FINE_GRAIN_BUFFER
            + T::SVM_ATOMICS;
        match value {
            val @ 1..=MAX => Ok(MemFlags(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_mem_flags {
        self.0
    }
    fn set(&mut self, value: cl_mem_flags) -> BitfieldResult<()> {
        type T = MemFlags;
        let fn_name = "MemFlags";
        const MAX: cl_bitfield = T::READ_WRITE
            + T::WRITE_ONLY
            + T::READ_ONLY
            + T::USE_HOST_PTR
            + T::ALLOC_HOST_PTR
            + T::COPY_HOST_PTR
            + T::HOST_WRITE_ONLY
            + T::HOST_READ_ONLY
            + T::HOST_NO_ACCESS
            + T::KERNEL_READ_AND_WRITE
            + T::SVM_FINE_GRAIN_BUFFER
            + T::SVM_ATOMICS;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
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
        const MAX: cl_bitfield = T::OBJECT_CONTENT_UNDEFINED + T::OBJECT_HOST;
        match value {
            val @ 1..=MAX => Ok(MemMigrationFlags(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_mem_migration_flags {
        self.0
    }
    fn set(&mut self, value: cl_mem_migration_flags) -> BitfieldResult<()> {
        type T = MemMigrationFlags;
        let fn_name = "MemMigrationFlags";
        const MAX: cl_bitfield = T::OBJECT_CONTENT_UNDEFINED + T::OBJECT_HOST;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
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
        const MAX: cl_bitfield = T::READ + T::WRITE + T::WRITE_INVALIDATE_REGION;
        match value {
            val @ 1..=MAX => Ok(MapFlags(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_map_flags {
        self.0
    }
    fn set(&mut self, value: cl_map_flags) -> BitfieldResult<()> {
        type T = MapFlags;
        let fn_name = "MapFlags";
        const MAX: cl_bitfield = T::READ + T::WRITE + T::WRITE_INVALIDATE_REGION;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
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
        const MAX: cl_bitfield = T::NONE + T::CONST + T::RESTRICT + T::VOLATILE + T::PIPE;
        match value {
            val @ 1..=MAX => Ok(KernelArgTypeQualifier(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_kernel_arg_type_qualifier {
        self.0
    }
    fn set(&mut self, value: cl_kernel_arg_type_qualifier) -> BitfieldResult<()> {
        type T = KernelArgTypeQualifier;
        let fn_name = "KernelArgTypeQualifier";
        const MAX: cl_bitfield = T::NONE + T::CONST + T::RESTRICT + T::VOLATILE + T::PIPE;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
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
        const MAX: cl_bitfield = T::ORDER_RELAXED
            + T::ORDER_ACQ_REL
            + T::ORDER_SEQ_CST
            + T::SCOPE_WORK_ITEM
            + T::SCOPE_WORK_GROUP
            + T::SCOPE_DEVICE
            + T::SCOPE_ALL_DEVICES;
        match value {
            val @ 1..=MAX => Ok(DeviceAtomicCapabilities(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_atomic_capabilities {
        self.0
    }
    fn set(&mut self, value: cl_device_atomic_capabilities) -> BitfieldResult<()> {
        type T = DeviceAtomicCapabilities;
        let fn_name = "DeviceAtomicCapabilities";
        const MAX: cl_bitfield = T::ORDER_RELAXED
            + T::ORDER_ACQ_REL
            + T::ORDER_SEQ_CST
            + T::SCOPE_WORK_ITEM
            + T::SCOPE_WORK_GROUP
            + T::SCOPE_DEVICE
            + T::SCOPE_ALL_DEVICES;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
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
        const MAX: cl_bitfield = T::SUPPORTED + T::REPLACEABLE_DEFAULT;
        match value {
            val @ 1..=MAX => Ok(DeviceDeviceEnqueueCapabilities(val)),
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
    fn get(&self) -> cl_device_device_enqueue_capabilities {
        self.0
    }
    fn set(&mut self, value: cl_device_device_enqueue_capabilities) -> BitfieldResult<()> {
        type T = DeviceDeviceEnqueueCapabilities;
        let fn_name = "DeviceDeviceEnqueueCapabilities";
        const MAX: cl_bitfield = T::SUPPORTED + T::REPLACEABLE_DEFAULT;
        match value {
            val @ 1..=MAX => {
                self.0 = val;
                Ok(())
            }
            _ => Err(ValidationError::InvalidBitfield(fn_name)),
        }
    }
}
