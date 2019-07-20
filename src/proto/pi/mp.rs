//! Multi-processor management protocols.

use crate::proto::Protocol;
use crate::{unsafe_guid, Result, Status};
use bitflags::bitflags;
use core::convert::TryInto;
use core::ffi::c_void;
use core::ptr;
use core::time::Duration;

/// Callback to be called on the AP.
pub type Procedure = extern "win64" fn(*mut c_void);

bitflags! {
    /// Flags indicating if the processor is BSP or AP,
    /// if the processor is enabled or disabled, and if
    /// the processor is healthy.
    #[derive(Default)]
    pub struct StatusFlag: u32 {
        /// Processor is playing the role of BSP.
        const PROCESSOR_AS_BSP_BIT = 0b00000001;
        /// Processor is enabled.
        const PROCESSOR_ENABLED_BIT = 0b00000010;
        /// Processor is healthy.
        const PROCESSOR_HEALTH_STATUS_BIT = 0b00000100;
    }
}

/// Information about number of logical processors on the platform.
#[derive(Default, Debug)]
pub struct ProcessorCount {
    /// Total number of processors (including BSP).
    pub total: usize,
    /// Number of processors (including BSP) that are currently enabled.
    pub enabled: usize,
}

/// Information about processor on the platform.
#[repr(C)]
#[derive(Default, Debug)]
pub struct ProcessorInformation {
    /// Unique processor ID determined by system hardware.
    pub processor_id: u64,
    /// Flags indicating BSP, enabled and healthy status.
    pub status_flag: StatusFlag,
    /// Physical location of the processor.
    pub location: CPUPhysicalLocation,
}

/// Information about physical location of the processor.
#[repr(C)]
#[derive(Default, Debug)]
pub struct CPUPhysicalLocation {
    /// Zero-based physical package number that identifies
    /// the cartridge of the processor.
    pub package: u32,
    /// Zero-based physical core number within package of the processor.
    pub core: u32,
    /// Zero-based logical thread number within core of the processor.
    pub thread: u32,
}

/// Protocol that provides services needed for multi-processor management.
#[repr(C)]
#[unsafe_guid("3fdda605-a76e-4f46-ad29-12f4531b3d08")]
#[derive(Protocol)]
pub struct MPServices {
    get_number_of_processors: extern "win64" fn(
        this: *const MPServices,
        number_of_processors: *mut usize,
        number_of_enabled_processors: *mut usize,
    ) -> Status,
    get_processor_info: extern "win64" fn(
        this: *const MPServices,
        processor_number: usize,
        processor_info_buffer: *mut ProcessorInformation,
    ) -> Status,
    startup_all_aps: extern "win64" fn(
        this: *const MPServices,
        procedure: Procedure,
        single_thread: bool,
        wait_event: *mut c_void,
        timeout_in_micro_seconds: usize,
        procedure_argument: *mut c_void,
        failed_cpu_list: *mut *mut usize,
    ) -> Status,
    startup_this_ap: extern "win64" fn(
        this: *const MPServices,
        procedure: Procedure,
        processor_number: usize,
        wait_event: *mut c_void,
        timeout_in_micro_seconds: usize,
        procedure_argument: *mut c_void,
        finished: *mut bool,
    ) -> Status,
    switch_bsp: extern "win64" fn(
        this: *const MPServices,
        processor_number: usize,
        enable_old_bsp: bool,
    ) -> Status,
    enable_disable_ap: extern "win64" fn(
        this: *const MPServices,
        processor_number: usize,
        enable_ap: bool,
        health_flag: *const u32,
    ) -> Status,
    who_am_i: extern "win64" fn(this: *const MPServices, processor_number: *mut usize) -> Status,
}

impl MPServices {
    /// Retrieves the number of logical processors and the number of enabled logical processors in the system.
    pub fn get_number_of_processors(&self) -> Result<ProcessorCount> {
        let mut total: usize = 0;
        let mut enabled: usize = 0;
        (self.get_number_of_processors)(self, &mut total, &mut enabled)
            .into_with_val(|| ProcessorCount { total, enabled })
    }

    /// Gets detailed information on the requested processor at the instant this call is made.
    pub fn get_processor_info(&self, processor_number: usize) -> Result<ProcessorInformation> {
        let mut pi: ProcessorInformation = Default::default();
        (self.get_processor_info)(self, processor_number, &mut pi).into_with_val(|| pi)
    }

    /// Executes provided function on all APs in blocking mode.
    pub fn startup_all_aps(
        &self,
        single_thread: bool,
        procedure: Procedure,
        procedure_argument: *mut c_void,
        timeout: Option<Duration>,
    ) -> Result {
        let timeout_arg = match timeout {
            Some(timeout) => timeout.as_micros().try_into().unwrap(),
            None => 0,
        };

        (self.startup_all_aps)(
            self,
            procedure,
            single_thread,
            ptr::null_mut(),
            timeout_arg,
            procedure_argument,
            ptr::null_mut(),
        )
        .into()
    }

    /// Executes provided function on a specific AP in blocking mode.
    pub fn startup_this_ap(
        &self,
        processor_number: usize,
        procedure: Procedure,
        procedure_argument: *mut c_void,
        timeout: Option<Duration>,
    ) -> Result {
        let timeout_arg = match timeout {
            Some(timeout) => timeout.as_micros().try_into().unwrap(),
            None => 0,
        };

        (self.startup_this_ap)(
            self,
            procedure,
            processor_number,
            ptr::null_mut(),
            timeout_arg,
            procedure_argument,
            ptr::null_mut(),
        )
        .into()
    }

    /// Switches the requested AP to be the BSP from that point onward.
    pub fn switch_bsp(&self, processor_number: usize, enable_old_bsp: bool) -> Result {
        (self.switch_bsp)(self, processor_number, enable_old_bsp).into()
    }

    /// Enables or disables an AP from this point onward.
    pub fn enable_disable_ap(
        &self,
        processor_number: usize,
        enable_ap: bool,
        health_flag: Option<u32>,
    ) -> Result {
        let health_flag_ptr = match health_flag {
            Some(val) => &val,
            None => ptr::null(),
        };
        (self.enable_disable_ap)(self, processor_number, enable_ap, health_flag_ptr).into()
    }

    /// Gets the handle number of the caller processor.
    pub fn who_am_i(&self) -> Result<usize> {
        let mut processor_number: usize = 0;
        (self.who_am_i)(self, &mut processor_number).into_with_val(|| processor_number)
    }
}