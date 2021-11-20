#![allow(non_upper_case_globals, dead_code)]
use crate::objects::bitfields::CommandQueueProperties;
use crate::objects::functions::bool_to_clbool;
use crate::objects::structs::{
    AddressingMode, CommandQueueInfo, DeviceInfo, FilterMode, SamplerInfo,
};
use crate::objects::traits::GetSetGo;
use crate::objects::types::{LongProperties, PlatformPtr, Properties};
use opencl_heads::consts::*;
use opencl_heads::types::*;

#[non_exhaustive]
pub struct SamplerProperties;
impl SamplerProperties {
    pub fn gen(
        &self,
        normalized_coords: Option<bool>,
        addressing_mode: Option<AddressingMode>,
        filter_mode: Option<FilterMode>,
    ) -> LongProperties {
        let mut array: Vec<cl_properties> = Vec::with_capacity(7);
        match normalized_coords {
            Some(x) => {
                array.push(SamplerInfo::NORMALIZED_COORDS as cl_properties);
                array.push(bool_to_clbool(x) as cl_properties)
            }
            None => (),
        };
        match addressing_mode {
            Some(x) => {
                array.push(SamplerInfo::ADDRESSING_MODE as cl_properties);
                array.push(x.get() as cl_properties)
            }
            None => (),
        };
        match filter_mode {
            Some(x) => {
                array.push(SamplerInfo::FILTER_MODE as cl_properties);
                array.push(x.get() as cl_properties)
            }
            None => (),
        };
        array.push(0);
        Some(array)
    }
}

#[non_exhaustive]
pub struct QueueProperties;
impl QueueProperties {
    pub fn gen(
        &self,
        queue_properties: Option<CommandQueueProperties>,
        queue_size: Option<cl_uint>,
    ) -> LongProperties {
        let mut array: Vec<cl_properties> = Vec::with_capacity(5);
        match queue_properties {
            Some(x) => {
                array.push(CommandQueueInfo::PROPERTIES as cl_properties);
                array.push(x.get() as cl_properties);
                match queue_size {
                    Some(y) if y < DeviceInfo::QUEUE_ON_DEVICE_MAX_SIZE => {
                        // if x.get() == CommandQueueProperties::ON_DEVICE
                        // || x.get() == CommandQueueProperties::ON_DEVICE_DEFAULT
                        // {
                        array.push(CommandQueueInfo::SIZE as cl_properties);
                        array.push(y as cl_properties)
                        // } else {
                        // unimplemented!()
                        // }
                    }
                    Some(_) => unimplemented!(),
                    None => (),
                };
            }
            None => (),
        };
        array.push(0);
        Some(array)
    }
}

#[non_exhaustive]
pub struct ContextProperties;
impl ContextProperties {
    /* cl_context_properties - cl_uint */
    const PLATFORM: cl_context_properties = CL_CONTEXT_PLATFORM;
    // #ifdef CL_VERSION_1_2;
    const INTEROP_USER_SYNC: cl_context_properties = CL_CONTEXT_INTEROP_USER_SYNC;
    // #endif;
    // TODO: FIX property generators!
    // pub fn new(props: cl_addressing_mode) -> PropertyResult<Self> {
    //     type T = AddressingMode;
    //     let fn_name = "AddressingMode";
    //     match props {
    //         T::NONE | T::CLAMP | T::CLAMP_TO_EDGE | T::MIRRORED_REPEAT | T::REPEAT => {
    //             Ok(AddressingMode(props))
    //         }
    //         _ => Err(ValidationError::InvalidProperty(fn_name)),
    //     }
    // }
    // pub fn get(&self) -> cl_addressing_mode {
    //     self.0
    // }

    // pub fn platform(&self, platform_id: &PlatformPtr) -> Properties {
    //     let intptr_platform_id = platform_id.unwrap() as intptr_t;
    //     match intptr_platform_id {
    //         0 => None,
    //         _ => Some(vec![Self::PLATFORM, intptr_platform_id, 0]),
    //     }
    // }
    // pub fn interop_user_sync(&self, value: cl_bool) -> Properties {
    //     match value {
    //         0 | 1 => Some(vec![Self::INTEROP_USER_SYNC, value as isize, 0]),
    //         _ => None,
    //     }
    // }
    pub fn gen(
        &self,
        platform: Option<&PlatformPtr>,
        interop_user_sync: Option<bool>,
    ) -> Properties {
        let mut array: Vec<intptr_t> = Vec::with_capacity(5);
        match interop_user_sync {
            Some(x) => {
                array.push(Self::INTEROP_USER_SYNC as intptr_t);
                array.push(bool_to_clbool(x) as intptr_t)
            }
            None => (),
        };
        match platform {
            Some(x) => {
                array.push(Self::PLATFORM as intptr_t);
                array.push(x.unwrap() as intptr_t)
            }
            None => (),
        };
        array.push(0);
        Some(array)
    }
}
