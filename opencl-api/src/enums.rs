/*
 * enums.rs - Enumerators containing OpenCL Constants.
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
use opencl_heads::*;

// Status Codes
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    Success = CL_SUCCESS,
    DeviceNotFound = CL_DEVICE_NOT_FOUND,
    DeviceNotAvailable = CL_DEVICE_NOT_AVAILABLE,
    CompilerNotAvailable = CL_COMPILER_NOT_AVAILABLE,
    MemObjectAllocationFailure = CL_MEM_OBJECT_ALLOCATION_FAILURE,
    OutOfResources = CL_OUT_OF_RESOURCES,
    OutOfHostMemory = CL_OUT_OF_HOST_MEMORY,
    ProfilingInfoNotAvailable = CL_PROFILING_INFO_NOT_AVAILABLE,
    MemCopyOverlap = CL_MEM_COPY_OVERLAP,
    ImageFormatMismatch = CL_IMAGE_FORMAT_MISMATCH,
    ImageFormatNotSupported = CL_IMAGE_FORMAT_NOT_SUPPORTED,
    BuildProgramFailure = CL_BUILD_PROGRAM_FAILURE,
    MapFailure = CL_MAP_FAILURE,
    MisalignedSubBufferOffset = CL_MISALIGNED_SUB_BUFFER_OFFSET,
    ExecStatusErrorForEventsInWaitList = CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST,
    CompileProgramFailure = CL_COMPILE_PROGRAM_FAILURE,
    LinkerNotAvailable = CL_LINKER_NOT_AVAILABLE,
    LinkProgramFailure = CL_LINK_PROGRAM_FAILURE,
    DevicePartitionFailed = CL_DEVICE_PARTITION_FAILED,
    KernelArgInfoNotAvailable = CL_KERNEL_ARG_INFO_NOT_AVAILABLE,
    InvalidValue = CL_INVALID_VALUE,
    InvalidDeviceType = CL_INVALID_DEVICE_TYPE,
    InvalidPlatform = CL_INVALID_PLATFORM,
    InvalidDevice = CL_INVALID_DEVICE,
    InvalidContext = CL_INVALID_CONTEXT,
    InvalidQueueProperties = CL_INVALID_QUEUE_PROPERTIES,
    InvalidCommandQueue = CL_INVALID_COMMAND_QUEUE,
    InvalidHostPtr = CL_INVALID_HOST_PTR,
    InvalidMemObject = CL_INVALID_MEM_OBJECT,
    InvalidImageFormatDescriptor = CL_INVALID_IMAGE_FORMAT_DESCRIPTOR,
    InvalidImageSize = CL_INVALID_IMAGE_SIZE,
    InvalidSampler = CL_INVALID_SAMPLER,
    InvalidBinary = CL_INVALID_BINARY,
    InvalidBuildOptions = CL_INVALID_BUILD_OPTIONS,
    InvalidProgram = CL_INVALID_PROGRAM,
    InvalidProgramExecutable = CL_INVALID_PROGRAM_EXECUTABLE,
    InvalidKernelName = CL_INVALID_KERNEL_NAME,
    InvalidKernelDefinition = CL_INVALID_KERNEL_DEFINITION,
    InvalidKernel = CL_INVALID_KERNEL,
    InvalidArgIndex = CL_INVALID_ARG_INDEX,
    InvalidArgValue = CL_INVALID_ARG_VALUE,
    InvalidArgSize = CL_INVALID_ARG_SIZE,
    InvalidKernelArgs = CL_INVALID_KERNEL_ARGS,
    InvalidWorkDimension = CL_INVALID_WORK_DIMENSION,
    InvalidWorkGroupSize = CL_INVALID_WORK_GROUP_SIZE,
    InvalidWorkItemSize = CL_INVALID_WORK_ITEM_SIZE,
    InvalidGlobalOffset = CL_INVALID_GLOBAL_OFFSET,
    InvalidEventWaitList = CL_INVALID_EVENT_WAIT_LIST,
    InvalidEvent = CL_INVALID_EVENT,
    InvalidOperation = CL_INVALID_OPERATION,
    InvalidGLObject = CL_INVALID_GL_OBJECT,
    InvalidBufferSize = CL_INVALID_BUFFER_SIZE,
    InvalidMIPLevel = CL_INVALID_MIP_LEVEL,
    InvalidGlobalWorkSize = CL_INVALID_GLOBAL_WORK_SIZE,
    InvalidProperty = CL_INVALID_PROPERTY,
    InvalidImageDescriptor = CL_INVALID_IMAGE_DESCRIPTOR,
    InvalidCompilerOptions = CL_INVALID_COMPILER_OPTIONS,
    InvalidLinkerOptions = CL_INVALID_LINKER_OPTIONS,
    InvalidDevicePartitionCount = CL_INVALID_DEVICE_PARTITION_COUNT,
    InvalidPipeSize = CL_INVALID_PIPE_SIZE,
    InvalidDeviceQueue = CL_INVALID_DEVICE_QUEUE,
    InvalidDeviceSpecId = CL_INVALID_SPEC_ID,
    MaxSizeRestrictionExceeded = CL_MAX_SIZE_RESTRICTION_EXCEEDED,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlatformInfo {
    Profile = CL_PLATFORM_PROFILE,
    Version = CL_PLATFORM_VERSION,
    Name = CL_PLATFORM_NAME,
    Vendor = CL_PLATFORM_VENDOR,
    Extensions = CL_PLATFORM_EXTENSIONS,
    // 2.1
    HostTimerResolution = CL_PLATFORM_HOST_TIMER_RESOLUTION,
    // 3.0
    NumericVersion = CL_PLATFORM_NUMERIC_VERSION,
    // 3.0
    ExtensionsWithVersion = CL_PLATFORM_EXTENSIONS_WITH_VERSION,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceType {
    DEFAULT = CL_DEVICE_TYPE_DEFAULT,
    CPU = CL_DEVICE_TYPE_CPU,
    GPU = CL_DEVICE_TYPE_GPU,
    ACCELERATOR = CL_DEVICE_TYPE_ACCELERATOR,
    CUSTOM = CL_DEVICE_TYPE_CUSTOM,
    ALL = CL_DEVICE_TYPE_ALL,
}

/* cl_device_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceInfo {
    Type = CL_DEVICE_TYPE,
    VendorId = CL_DEVICE_VENDOR_ID,
    MaxComputeUnits = CL_DEVICE_MAX_COMPUTE_UNITS,
    MaxWorkItemDimensions = CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS,
    MaxWorkGroupSize = CL_DEVICE_MAX_WORK_GROUP_SIZE,
    MaxWorkItemSizes = CL_DEVICE_MAX_WORK_ITEM_SIZES,
    PreferredVectorWidthChar = CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR,
    PreferredVectorWidthShort = CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT,
    PreferredVectorWidthInt = CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT,
    PreferredVectorWidthLong = CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG,
    PreferredVectorWidthFloat = CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT,
    PreferredVectorWidthDouble = CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE,
    MaxClockFrequency = CL_DEVICE_MAX_CLOCK_FREQUENCY,
    AddressBits = CL_DEVICE_ADDRESS_BITS,
    MaxReadImageArgs = CL_DEVICE_MAX_READ_IMAGE_ARGS,
    MaxWriteImageArgs = CL_DEVICE_MAX_WRITE_IMAGE_ARGS,
    MaxMemAllocSize = CL_DEVICE_MAX_MEM_ALLOC_SIZE,
    Image2DMaxWidth = CL_DEVICE_IMAGE2D_MAX_WIDTH,
    Image2DMaxHeight = CL_DEVICE_IMAGE2D_MAX_HEIGHT,
    Image3DMaxWidth = CL_DEVICE_IMAGE3D_MAX_WIDTH,
    Image3DMaxHeight = CL_DEVICE_IMAGE3D_MAX_HEIGHT,
    Image3DMaxDepth = CL_DEVICE_IMAGE3D_MAX_DEPTH,
    ImageSupport = CL_DEVICE_IMAGE_SUPPORT,
    MaxParameterSize = CL_DEVICE_MAX_PARAMETER_SIZE,
    MaxSamplers = CL_DEVICE_MAX_SAMPLERS,
    MemBaseAddrAlign = CL_DEVICE_MEM_BASE_ADDR_ALIGN,
    MinDataTypeAlignSize = CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE,
    SingleFPConfig = CL_DEVICE_SINGLE_FP_CONFIG,
    GlobalMemCacheType = CL_DEVICE_GLOBAL_MEM_CACHE_TYPE,
    GlobalMemCachelineSize = CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE,
    GlobalMemCacheSize = CL_DEVICE_GLOBAL_MEM_CACHE_SIZE,
    GlobalMemSize = CL_DEVICE_GLOBAL_MEM_SIZE,
    MaxConstantBufferSize = CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE,
    MaxConstantArgs = CL_DEVICE_MAX_CONSTANT_ARGS,
    LocalMemType = CL_DEVICE_LOCAL_MEM_TYPE,
    LocalMemSize = CL_DEVICE_LOCAL_MEM_SIZE,
    ErrorCorrectionSupport = CL_DEVICE_ERROR_CORRECTION_SUPPORT,
    ProfilingTimerResolution = CL_DEVICE_PROFILING_TIMER_RESOLUTION,
    EndianLittle = CL_DEVICE_ENDIAN_LITTLE,
    Available = CL_DEVICE_AVAILABLE,
    CompilerAvailable = CL_DEVICE_COMPILER_AVAILABLE,
    ExecutionCapabilities = CL_DEVICE_EXECUTION_CAPABILITIES,
    // QueueProperties = CL_DEVICE_QUEUE_PROPERTIES, /* deprecated */
    // #ifdef CL_VERSION_2_0;
    QueueOnHostProperties = CL_DEVICE_QUEUE_ON_HOST_PROPERTIES,
    // #endif;
    Name = CL_DEVICE_NAME,
    Vendor = CL_DEVICE_VENDOR,
    DriverVersion = CL_DRIVER_VERSION,
    Profile = CL_DEVICE_PROFILE,
    Version = CL_DEVICE_VERSION,
    Extensions = CL_DEVICE_EXTENSIONS,
    Platform = CL_DEVICE_PLATFORM,
    // #ifdef CL_VERSION_1_2;
    DoubleFPConfig = CL_DEVICE_DOUBLE_FP_CONFIG,
    // #endif;
    /* 0x1033 reserved for CL_DEVICE_HALF_FP_CONFIG which is already defined in "cl_ext.h" */
    // #ifdef CL_VERSION_1_1;
    PreferredVectorWidthHalf = CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF,
    HostUnifiedMemory = CL_DEVICE_HOST_UNIFIED_MEMORY, /* deprecated */
    NativeVectorWidthChar = CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR,
    NativeVectorWidthShort = CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT,
    NativeVectorWidthInt = CL_DEVICE_NATIVE_VECTOR_WIDTH_INT,
    NativeVectorWidthLong = CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG,
    NativeVectorWidthFloat = CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT,
    NativeVectorWidthDouble = CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE,
    NativeVectorWidthHalf = CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF,
    OpenCLCVersion = CL_DEVICE_OPENCL_C_VERSION,
    // #endif;
    // #ifdef CL_VERSION_1_2;
    LinkerAvailable = CL_DEVICE_LINKER_AVAILABLE,
    BuiltinKernels = CL_DEVICE_BUILT_IN_KERNELS,
    ImageMaxBufferSize = CL_DEVICE_IMAGE_MAX_BUFFER_SIZE,
    ImageMaxArraySize = CL_DEVICE_IMAGE_MAX_ARRAY_SIZE,
    ParentDevice = CL_DEVICE_PARENT_DEVICE,
    PartitionMaxSubDevices = CL_DEVICE_PARTITION_MAX_SUB_DEVICES,
    PartitionProperties = CL_DEVICE_PARTITION_PROPERTIES,
    PartitionAffinityDomain = CL_DEVICE_PARTITION_AFFINITY_DOMAIN,
    PartitionType = CL_DEVICE_PARTITION_TYPE,
    ReferenceCount = CL_DEVICE_REFERENCE_COUNT,
    PreferredInteropUserSync = CL_DEVICE_PREFERRED_INTEROP_USER_SYNC,
    PrintfBufferSize = CL_DEVICE_PRINTF_BUFFER_SIZE,
    // #endif;
    // #ifdef CL_VERSION_2_0;
    ImagePitchAlignment = CL_DEVICE_IMAGE_PITCH_ALIGNMENT,
    ImageBaseAddressAlignment = CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT,
    MaxReadWriteImageArgs = CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS,
    MaxGlobalVariableSize = CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE,
    QueueOnDeviceProperties = CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES,
    QueueOnDevicePreferredSize = CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE,
    QueueOnDeviceMaxSize = CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE,
    MaxOnDeviceQueues = CL_DEVICE_MAX_ON_DEVICE_QUEUES,
    MaxOnDeviceEvents = CL_DEVICE_MAX_ON_DEVICE_EVENTS,
    SvmCapabilities = CL_DEVICE_SVM_CAPABILITIES,
    GlobalVariablePreferredTotalSize = CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE,
    MaxPipeArgs = CL_DEVICE_MAX_PIPE_ARGS,
    PipeMaxActiveReservations = CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS,
    PipeMaxPacketSize = CL_DEVICE_PIPE_MAX_PACKET_SIZE,
    PreferredPlatformAtomicAlignment = CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT,
    PreferredGlobalAtomicAlignment = CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT,
    PreferredLocalAtomicAlignment = CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT,
    // #endif;
    // #ifdef CL_VERSION_2_1;
    IlVersion = CL_DEVICE_IL_VERSION,
    MaxNumSubGroups = CL_DEVICE_MAX_NUM_SUB_GROUPS,
    SubGroupIndependentForwardProgress = CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS,
    // #endif;
    // #ifdef CL_VERSION_3_0;
    NumericVersion = CL_DEVICE_NUMERIC_VERSION,
    ExtensionsWithVersion = CL_DEVICE_EXTENSIONS_WITH_VERSION,
    IlsWithVersion = CL_DEVICE_ILS_WITH_VERSION,
    BuiltInKernelsWithVersion = CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION,
    AtomicMemoryCapabilities = CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES,
    AtomicFenceCapabilities = CL_DEVICE_ATOMIC_FENCE_CAPABILITIES,
    NonUniformWorkGroupSupport = CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT,
    OpenCLCAllVersions = CL_DEVICE_OPENCL_C_ALL_VERSIONS,
    PreferredWorkGroupSizeMultiple = CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE,
    WorkGroupCollectiveFunctionsSupport = CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT,
    GenericAddressSpaceSupport = CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT,
    /* 0x106A to 0x106E - Reserved for upcoming KHR extension */
    OpenCLCFeatures = CL_DEVICE_OPENCL_C_FEATURES,
    DeviceEnqueueCapabilities = CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES,
    PipeSupport = CL_DEVICE_PIPE_SUPPORT,
    LatestConformanceVersionPassed = CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED,
    // #endif
}

/* cl_device_fp_config - bitfield */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceFPConfig {
    Denorm = CL_FP_DENORM,
    Infinite = CL_FP_INF_NAN,
    RoundToNearest = CL_FP_ROUND_TO_NEAREST,
    RoundToZero = CL_FP_ROUND_TO_ZERO,
    RoundToInfinite = CL_FP_ROUND_TO_INF,
    Fma = CL_FP_FMA,
    // #ifdef CL_VERSION_1_1;
    SoftFloat = CL_FP_SOFT_FLOAT,
    // #endif;
    // #ifdef CL_VERSION_1_2;
    CorrectlyRoundedDivideSqrt = CL_FP_CORRECTLY_ROUNDED_DIVIDE_SQRT,
    // #endif;
}

/* cl_device_mem_cache_type */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceMemCacheType {
    None = CL_NONE,
    ReadOnlyCache = CL_READ_ONLY_CACHE,
    ReadWriteCache = CL_READ_WRITE_CACHE,
}

/* cl_device_local_mem_type */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceLocalMemType {
    Local = CL_LOCAL,
    Global = CL_GLOBAL,
}

/* cl_device_exec_capabilities - bitfield */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceExecCapabilities {
    ExecKernel = CL_EXEC_KERNEL,
    ExecNativeKernel = CL_EXEC_NATIVE_KERNEL,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandQueueProperties {
    OutOfOrderExecModeEnable = CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE,
    ProfilingEnable = CL_QUEUE_PROFILING_ENABLE,
    // #ifdef CL_VERSION_2_0
    OnDevice = CL_QUEUE_ON_DEVICE,
    OnDeviceDefault = CL_QUEUE_ON_DEVICE_DEFAULT,
    // #endif
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContextInfo {
    ReferenceCount = CL_CONTEXT_REFERENCE_COUNT,
    Devices = CL_CONTEXT_DEVICES,
    Properties = CL_CONTEXT_PROPERTIES,
    // #ifdef CL_VERSION_1_1
    NumDevices = CL_CONTEXT_NUM_DEVICES,
    // #endif
}

/* cl_context_properties */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContextProperties {
    Platform = CL_CONTEXT_PLATFORM,
    // #ifdef CL_VERSION_1_2;
    InteropUserSync = CL_CONTEXT_INTEROP_USER_SYNC,
    // #endif;
}

// #ifdef CL_VERSION_1_2;
/* cl_device_partition_property */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DevicePartitionProperty {
    Equally = CL_DEVICE_PARTITION_EQUALLY,
    ByCounts = CL_DEVICE_PARTITION_BY_COUNTS,
    ByCountsListEnd = CL_DEVICE_PARTITION_BY_COUNTS_LIST_END,
    ByAffinityDomain = CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN,
}
// #endif;

// #ifdef CL_VERSION_1_2;
/* cl_device_affinity_domain */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceAffinityDomain {
    Numa = CL_DEVICE_AFFINITY_DOMAIN_NUMA,
    L4Cache = CL_DEVICE_AFFINITY_DOMAIN_L4_CACHE,
    L3Cache = CL_DEVICE_AFFINITY_DOMAIN_L3_CACHE,
    L2Cache = CL_DEVICE_AFFINITY_DOMAIN_L2_CACHE,
    L1Cache = CL_DEVICE_AFFINITY_DOMAIN_L1_CACHE,
    NextPartitionable = CL_DEVICE_AFFINITY_DOMAIN_NEXT_PARTITIONABLE,
}
// #endif;

// #ifdef CL_VERSION_2_0;
/* cl_device_svm_capabilities */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceSVMCapabilities {
    CoarseGrainBuffer = CL_DEVICE_SVM_COARSE_GRAIN_BUFFER,
    FineGrainBuffer = CL_DEVICE_SVM_FINE_GRAIN_BUFFER,
    FineGrainSystem = CL_DEVICE_SVM_FINE_GRAIN_SYSTEM,
    Atomics = CL_DEVICE_SVM_ATOMICS,
}
// #endif;

/* cl_command_queue_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandQueueInfo {
    Context = CL_QUEUE_CONTEXT,
    Device = CL_QUEUE_DEVICE,
    ReferenceCount = CL_QUEUE_REFERENCE_COUNT,
    Properties = CL_QUEUE_PROPERTIES,
    // #ifdef CL_VERSION_2_0;
    Size = CL_QUEUE_SIZE,
    // #endif;
    // #ifdef CL_VERSION_2_1;
    DeviceDefault = CL_QUEUE_DEVICE_DEFAULT,
    // #endif;
    // #ifdef CL_VERSION_3_0;
    PropertiesArray = CL_QUEUE_PROPERTIES_ARRAY,
    // #endif;
}

/* cl_mem_flags and cl_svm_mem_flags - bitfield */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemFlags {
    ReadWrite = CL_MEM_READ_WRITE,
    ReadOnly = CL_MEM_READ_ONLY,
    WriteOnly = CL_MEM_WRITE_ONLY,
    UseHostPtr = CL_MEM_USE_HOST_PTR,
    AllocHostPtr = CL_MEM_ALLOC_HOST_PTR,
    CopyHostPtr = CL_MEM_COPY_HOST_PTR,
    /* reserved: cl_bitfield = 1 << 6: cl_bitfield = */
    // #ifdef CL_VERSION_1_2;
    HostReadOnly = CL_MEM_HOST_READ_ONLY,
    HostWriteOnly = CL_MEM_HOST_WRITE_ONLY,
    NoAccess = CL_MEM_HOST_NO_ACCESS,
    // #endif;
    // #ifdef CL_VERSION_2_0;
    KernelReadAndWrite = CL_MEM_KERNEL_READ_AND_WRITE,
    // #endif;
}

// #ifdef CL_VERSION_2_0;
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVMMemFlags {
    MemFlags,
    FineGrainBuffer = CL_MEM_SVM_FINE_GRAIN_BUFFER, /* used by cl_svm_mem_flags only */
    Atomics = CL_MEM_SVM_ATOMICS,                   /* used by cl_svm_mem_flags only */
}
// #endif;

// #ifdef CL_VERSION_1_2;
/* cl_mem_migration_flags - bitfield */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemMigrationFlags {
    ObjectHost = CL_MIGRATE_MEM_OBJECT_HOST,
    ObjectContentUndefined = CL_MIGRATE_MEM_OBJECT_CONTENT_UNDEFINED,
}
// #endif;

/* cl_channel_order */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChannelOrder {
    R = CL_R,
    A = CL_A,
    Rg = CL_RG,
    Ra = CL_RA,
    Rgb = CL_RGB,
    Rgba = CL_RGBA,
    Bgra = CL_BGRA,
    Argb = CL_ARGB,
    Intensity = CL_INTENSITY,
    Luminance = CL_LUMINANCE,
    // #ifdef CL_VERSION_1_1;
    Rx = CL_Rx,
    RGx = CL_RGx,
    RGBx = CL_RGBx,
    // #endif;
    // #ifdef CL_VERSION_1_2;
    Depth = CL_DEPTH,
    DepthStencil = CL_DEPTH_STENCIL,
    // #endif;
    // #ifdef CL_VERSION_2_0;
    Srgb = CL_sRGB,
    Srgbx = CL_sRGBx,
    Srgba = CL_sRGBA,
    Sbgra = CL_sBGRA,
    Abgr = CL_ABGR,
    // #endif
}

/* cl_channel_type */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChannelType {
    SNormInt8 = CL_SNORM_INT8,
    SNormInt16 = CL_SNORM_INT16,
    UNormInt8 = CL_UNORM_INT8,
    UNormInt16 = CL_UNORM_INT16,
    UNormShort565 = CL_UNORM_SHORT_565,
    UNormShort555 = CL_UNORM_SHORT_555,
    UNormInt101010 = CL_UNORM_INT_101010,
    SignedInt8 = CL_SIGNED_INT8,
    SignedInt16 = CL_SIGNED_INT16,
    SignedInt32 = CL_SIGNED_INT32,
    UnsignedInt8 = CL_UNSIGNED_INT8,
    UnsignedInt16 = CL_UNSIGNED_INT16,
    UnsignedInt32 = CL_UNSIGNED_INT32,
    HalfFloat = CL_HALF_FLOAT,
    Float = CL_FLOAT,
    // #ifdef CL_VERSION_1_2;
    UNormInt24 = CL_UNORM_INT24,
    // #endif;
    // #ifdef CL_VERSION_2_1;
    UNormInt101010_2 = CL_UNORM_INT_101010_2,
    // #endif;
}

/* cl_mem_object_type */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemObjectType {
    Buffer = CL_MEM_OBJECT_BUFFER,
    Image2D = CL_MEM_OBJECT_IMAGE2D,
    Image3D = CL_MEM_OBJECT_IMAGE3D,
    // #ifdef CL_VERSION_1_2;
    Image2DArray = CL_MEM_OBJECT_IMAGE2D_ARRAY,
    Image1D = CL_MEM_OBJECT_IMAGE1D,
    Image1DArray = CL_MEM_OBJECT_IMAGE1D_ARRAY,
    Image1DBuffer = CL_MEM_OBJECT_IMAGE1D_BUFFER,
    // #endif;
    // #ifdef CL_VERSION_2_0;
    ObjectPipe = CL_MEM_OBJECT_PIPE, // #endif
}

/* cl_mem_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemInfo {
    Type = CL_MEM_TYPE,
    Flags = CL_MEM_FLAGS,
    Size = CL_MEM_SIZE,
    HostPtr = CL_MEM_HOST_PTR,
    MapCount = CL_MEM_MAP_COUNT,
    ReferenceCount = CL_MEM_REFERENCE_COUNT,
    Context = CL_MEM_CONTEXT,
    // #ifdef CL_VERSION_1_1;
    AssociatedMemObject = CL_MEM_ASSOCIATED_MEMOBJECT,
    Offset = CL_MEM_OFFSET,
    // #endif;
    // #ifdef CL_VERSION_2_0;
    UsesSVMPointer = CL_MEM_USES_SVM_POINTER,
    // #endif;
    // #ifdef CL_VERSION_3_0;
    Properties = CL_MEM_PROPERTIES,
    // #endif;
}

/* cl_image_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ImageInfo {
    Format = CL_IMAGE_FORMAT,
    ElementSize = CL_IMAGE_ELEMENT_SIZE,
    RowPitch = CL_IMAGE_ROW_PITCH,
    SlicePitch = CL_IMAGE_SLICE_PITCH,
    Width = CL_IMAGE_WIDTH,
    Height = CL_IMAGE_HEIGHT,
    Depth = CL_IMAGE_DEPTH,
    // #ifdef CL_VERSION_1_2;
    ArraySize = CL_IMAGE_ARRAY_SIZE,
    Buffer = CL_IMAGE_BUFFER,
    NumMipLevels = CL_IMAGE_NUM_MIP_LEVELS,
    NumSamples = CL_IMAGE_NUM_SAMPLES,
    // #endif;
}

/* cl_pipe_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PipeInfo {
    // #ifdef CL_VERSION_2_0;
    PacketSize = CL_PIPE_PACKET_SIZE,
    MaxPackets = CL_PIPE_MAX_PACKETS,
    // #endif;
    // #ifdef CL_VERSION_3_0;
    Properties = CL_PIPE_PROPERTIES,
    // #endif;
}

/* cl_addressing_mode */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AddressingMode {
    None = CL_ADDRESS_NONE,
    ClampToEdge = CL_ADDRESS_CLAMP_TO_EDGE,
    Clamp = CL_ADDRESS_CLAMP,
    Repeat = CL_ADDRESS_REPEAT,
    // #ifdef CL_VERSION_1_1;
    MirroredRepeat = CL_ADDRESS_MIRRORED_REPEAT,
    // #endif;
}

/* cl_filter_mode */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FilterMode {
    Nearest = CL_FILTER_NEAREST,
    Linear = CL_FILTER_LINEAR,
}

/* cl_sampler_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SamplerInfo {
    ReferenceCount = CL_SAMPLER_REFERENCE_COUNT,
    Context = CL_SAMPLER_CONTEXT,
    NormalizedCoords = CL_SAMPLER_NORMALIZED_COORDS,
    AddressingMode = CL_SAMPLER_ADDRESSING_MODE,
    FilterMode = CL_SAMPLER_FILTER_MODE,
    // #ifdef CL_VERSION_2_0;
    /* These enumerants are for the cl_khr_mipmap_image extension.;
    They have since been added to cl_ext.h with an appropriate;
    KHR suffix, but are left here for backwards compatibility. */
    MipFilterMode = CL_SAMPLER_MIP_FILTER_MODE,
    LodMin = CL_SAMPLER_LOD_MIN,
    LodMax = CL_SAMPLER_LOD_MAX,
    // #endif;
    // #ifdef CL_VERSION_3_0;
    Properties = CL_SAMPLER_PROPERTIES,
    // #endif;
}

/* cl_map_flags - bitfield */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapFlags {
    Read = CL_MAP_READ,
    Write = CL_MAP_WRITE,
    // #ifdef CL_VERSION_1_2;
    WriteInvalidateRegion = CL_MAP_WRITE_INVALIDATE_REGION, // #endif;
}

/* cl_program_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProgramInfo {
    ReferenceCount = CL_PROGRAM_REFERENCE_COUNT,
    Context = CL_PROGRAM_CONTEXT,
    NumDevices = CL_PROGRAM_NUM_DEVICES,
    Devices = CL_PROGRAM_DEVICES,
    Source = CL_PROGRAM_SOURCE,
    BinarySizes = CL_PROGRAM_BINARY_SIZES,
    Binaries = CL_PROGRAM_BINARIES,
    // #ifdef CL_VERSION_1_2;
    NumKernels = CL_PROGRAM_NUM_KERNELS,
    KernelNames = CL_PROGRAM_KERNEL_NAMES,
    // #endif;
    // #ifdef CL_VERSION_2_1;
    Il = CL_PROGRAM_IL,
    // #endif;
    // #ifdef CL_VERSION_2_2;
    ScopeGlobalCtorsPresent = CL_PROGRAM_SCOPE_GLOBAL_CTORS_PRESENT,
    ScopeGlobalDtorsPresent = CL_PROGRAM_SCOPE_GLOBAL_DTORS_PRESENT, // #endif;
}

/* cl_program_build_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProgramBuildInfo {
    Status = CL_PROGRAM_BUILD_STATUS,
    Options = CL_PROGRAM_BUILD_OPTIONS,
    Log = CL_PROGRAM_BUILD_LOG,
    // #ifdef CL_VERSION_1_2;
    BinaryType = CL_PROGRAM_BINARY_TYPE,
    // #endif;
    // #ifdef CL_VERSION_2_0;
    BuildGlobalVariableTotalSize = CL_PROGRAM_BUILD_GLOBAL_VARIABLE_TOTAL_SIZE, // #endif;
}

// #ifdef CL_VERSION_1_2;
/* cl_program_binary_type */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProgramBinaryType {
    None = CL_PROGRAM_BINARY_TYPE_NONE,
    CompiledObject = CL_PROGRAM_BINARY_TYPE_COMPILED_OBJECT,
    Library = CL_PROGRAM_BINARY_TYPE_LIBRARY,
    Executable = CL_PROGRAM_BINARY_TYPE_EXECUTABLE,
}
// #endif;

/* cl_build_status */
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildStatus {
    Success = CL_BUILD_SUCCESS,
    None = CL_BUILD_NONE,
    Error = CL_BUILD_ERROR,
    BuildInProgress = CL_BUILD_IN_PROGRESS,
}

/* cl_kernel_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KernelInfo {
    FunctionName = CL_KERNEL_FUNCTION_NAME,
    NumArgs = CL_KERNEL_NUM_ARGS,
    ReferenceCount = CL_KERNEL_REFERENCE_COUNT,
    Context = CL_KERNEL_CONTEXT,
    Program = CL_KERNEL_PROGRAM,
    // #ifdef CL_VERSION_1_2;
    Attributes = CL_KERNEL_ATTRIBUTES,
    // #endif;
}

// #ifdef CL_VERSION_1_2;
/* cl_kernel_arg_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KernelArgInfo {
    AddressQualifier = CL_KERNEL_ARG_ADDRESS_QUALIFIER,
    AccessQualifier = CL_KERNEL_ARG_ACCESS_QUALIFIER,
    TypeName = CL_KERNEL_ARG_TYPE_NAME,
    TypeQualifier = CL_KERNEL_ARG_TYPE_QUALIFIER,
    Name = CL_KERNEL_ARG_NAME,
}
// #endif;

// #ifdef CL_VERSION_1_2;
/* cl_kernel_arg_address_qualifier */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KernelArgAddressQualifier {
    Global = CL_KERNEL_ARG_ADDRESS_GLOBAL,
    Local = CL_KERNEL_ARG_ADDRESS_LOCAL,
    Constant = CL_KERNEL_ARG_ADDRESS_CONSTANT,
    Private = CL_KERNEL_ARG_ADDRESS_PRIVATE,
}
// #endif;

// #ifdef CL_VERSION_1_2;
/* cl_kernel_arg_access_qualifier */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KernelArgAccessQualifier {
    ReadOnly = CL_KERNEL_ARG_ACCESS_READ_ONLY,
    WriteOnly = CL_KERNEL_ARG_ACCESS_WRITE_ONLY,
    ReadWrite = CL_KERNEL_ARG_ACCESS_READ_WRITE,
    None = CL_KERNEL_ARG_ACCESS_NONE,
}
// #endif;

// #ifdef CL_VERSION_1_2;
/* cl_kernel_arg_type_qualifier */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KernelArgTypeQualifier {
    None = CL_KERNEL_ARG_TYPE_NONE,
    Const = CL_KERNEL_ARG_TYPE_CONST,
    Restrict = CL_KERNEL_ARG_TYPE_RESTRICT,
    Volatile = CL_KERNEL_ARG_TYPE_VOLATILE,
    // #ifdef CL_VERSION_2_0;
    Pipe = CL_KERNEL_ARG_TYPE_PIPE,
    // #endif;
}
// #endif;

/* cl_kernel_work_group_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KernelWorkGroupInfo {
    WorkGroupSize = CL_KERNEL_WORK_GROUP_SIZE,
    CompileWorkGroupSize = CL_KERNEL_COMPILE_WORK_GROUP_SIZE,
    LocalMemSize = CL_KERNEL_LOCAL_MEM_SIZE,
    PreferredWorkGroupSizeMultiple = CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE,
    PrivateMemSize = CL_KERNEL_PRIVATE_MEM_SIZE,
    // #ifdef CL_VERSION_1_2;
    GlobalWorkSize = CL_KERNEL_GLOBAL_WORK_SIZE,
    // #endif;
}

// #ifdef CL_VERSION_2_1;
/* cl_kernel_sub_group_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KernelSubGroupInfo {
    MaxSubGroupSizeForNDRange = CL_KERNEL_MAX_SUB_GROUP_SIZE_FOR_NDRANGE,
    SubGroupCountForNDRange = CL_KERNEL_SUB_GROUP_COUNT_FOR_NDRANGE,
    LocalSizeForSubGroupCount = CL_KERNEL_LOCAL_SIZE_FOR_SUB_GROUP_COUNT,
    MaxNumSubGroups = CL_KERNEL_MAX_NUM_SUB_GROUPS,
    CompileNumSubGroups = CL_KERNEL_COMPILE_NUM_SUB_GROUPS,
}
// #endif;

// #ifdef CL_VERSION_2_0;
/* cl_kernel_exec_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KernelExecInfo {
    SVMPointers = CL_KERNEL_EXEC_INFO_SVM_PTRS,
    SVMFineGrainSystem = CL_KERNEL_EXEC_INFO_SVM_FINE_GRAIN_SYSTEM,
}
// #endif;

/* cl_event_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventInfo {
    CommandQueue = CL_EVENT_COMMAND_QUEUE,
    CommandType = CL_EVENT_COMMAND_TYPE,
    ReferenceCount = CL_EVENT_REFERENCE_COUNT,
    CommandExecutionStatus = CL_EVENT_COMMAND_EXECUTION_STATUS,
    // #ifdef CL_VERSION_1_1;
    Context = CL_EVENT_CONTEXT,
    // #endif;
}

/* cl_command_type */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandType {
    NDRangeKernel = CL_COMMAND_NDRANGE_KERNEL,
    Task = CL_COMMAND_TASK,
    NativeKernel = CL_COMMAND_NATIVE_KERNEL,
    ReadBuffer = CL_COMMAND_READ_BUFFER,
    WriteBuffer = CL_COMMAND_WRITE_BUFFER,
    CopyBuffer = CL_COMMAND_COPY_BUFFER,
    ReadImage = CL_COMMAND_READ_IMAGE,
    WriteImage = CL_COMMAND_WRITE_IMAGE,
    CopyImage = CL_COMMAND_COPY_IMAGE,
    CopyImageToBuffer = CL_COMMAND_COPY_IMAGE_TO_BUFFER,
    CopyBufferToImage = CL_COMMAND_COPY_BUFFER_TO_IMAGE,
    MapBuffer = CL_COMMAND_MAP_BUFFER,
    MapImage = CL_COMMAND_MAP_IMAGE,
    UnmapMemObject = CL_COMMAND_UNMAP_MEM_OBJECT,
    Marker = CL_COMMAND_MARKER,
    AcquireGLObjects = CL_COMMAND_ACQUIRE_GL_OBJECTS,
    ReleaseGLObjects = CL_COMMAND_RELEASE_GL_OBJECTS,
    // #ifdef CL_VERSION_1_1;
    ReadBufferRect = CL_COMMAND_READ_BUFFER_RECT,
    WriteBufferRect = CL_COMMAND_WRITE_BUFFER_RECT,
    CopyBufferRect = CL_COMMAND_COPY_BUFFER_RECT,
    User = CL_COMMAND_USER,
    // #endif;
    // #ifdef CL_VERSION_1_2;
    Barrier = CL_COMMAND_BARRIER,
    MigrateMemObjects = CL_COMMAND_MIGRATE_MEM_OBJECTS,
    FillBuffer = CL_COMMAND_FILL_BUFFER,
    FillImage = CL_COMMAND_FILL_IMAGE,
    // #endif;
    // #ifdef CL_VERSION_2_0;
    SVMFree = CL_COMMAND_SVM_FREE,
    SVMMemcpy = CL_COMMAND_SVM_MEMCPY,
    SVMMemfill = CL_COMMAND_SVM_MEMFILL,
    SVMMap = CL_COMMAND_SVM_MAP,
    SVMUnmap = CL_COMMAND_SVM_UNMAP,
    // #endif;
    // #ifdef CL_VERSION_3_0;
    SVMMigrateMem = CL_COMMAND_SVM_MIGRATE_MEM,
    // #endif;
}

/// command execution status
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandExecutionStatus {
    Complete = CL_COMPLETE,
    Running = CL_RUNNING,
    Submitted = CL_SUBMITTED,
    Queued = CL_QUEUED,
}

// #ifdef CL_VERSION_1_1;
/* cl_buffer_create_type */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferCreateType {
    Region = CL_BUFFER_CREATE_TYPE_REGION,
}
// #endif;

/* cl_profiling_info */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProfilingInfo {
    Queued = CL_PROFILING_COMMAND_QUEUED,
    Submit = CL_PROFILING_COMMAND_SUBMIT,
    Start = CL_PROFILING_COMMAND_START,
    End = CL_PROFILING_COMMAND_END,
    // #ifdef CL_VERSION_2_0;
    Complete = CL_PROFILING_COMMAND_COMPLETE,
    // #endif;
}

// #ifdef CL_VERSION_3_0;
/* cl_device_atomic_capabilities - bitfield */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceAtomicCapabilities {
    OrderRelaxed = CL_DEVICE_ATOMIC_ORDER_RELAXED,
    OrderAcqRel = CL_DEVICE_ATOMIC_ORDER_ACQ_REL,
    OrderSeqCST = CL_DEVICE_ATOMIC_ORDER_SEQ_CST,
    ScopeWorkItem = CL_DEVICE_ATOMIC_SCOPE_WORK_ITEM,
    ScopeWorkGroup = CL_DEVICE_ATOMIC_SCOPE_WORK_GROUP,
    ScopeDevice = CL_DEVICE_ATOMIC_SCOPE_DEVICE,
    ScopeAllDevices = CL_DEVICE_ATOMIC_SCOPE_ALL_DEVICES,
}
// #endif;

// #ifdef CL_VERSION_3_0;
/* cl_device_device_enqueue_capabilities - bitfield */
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceDeviceEnqueueCapabilities {
    Supported = CL_DEVICE_QUEUE_SUPPORTED,
    ReplaceableDefault = CL_DEVICE_QUEUE_REPLACEABLE_DEFAULT,
}
// #endif;

/* cl_khronos_vendor_id */
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KhronosVendorId {
    Codeplay = CL_KHRONOS_VENDOR_ID_CODEPLAY,
}

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
