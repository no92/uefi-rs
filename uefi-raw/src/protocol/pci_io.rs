use core::ffi::c_void;

use crate::{guid, Guid, Status};

newtype_enum! {
    pub enum PciIoProtoWidth: u64 => {
        UINT8 = 0,
        UINT16 = 1,
        UINT32 = 2,
        UINT64 = 3,
        FIFO_UINT8 = 4,
        FIFO_UINT16 = 5,
        FIFO_UINT32 = 6,
        FIFO_UINT64 = 7,
        FILL_UINT8 = 8,
        FILL_UINT16 = 9,
        FILL_UINT32 = 10,
        FILL_UINT64 = 11,
        WIDTH_MAX = 12,
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PciIoProtoAccess {
	pub read: unsafe extern "efiapi" fn(
		this: *mut Self,
		width: PciIoProtoWidth,
		bar_index: u8,
		offset: u64,
		count: usize,
		buffer: *mut c_void,
	) -> Status,
	pub write: unsafe extern "efiapi" fn(
		this: *mut Self,
		width: PciIoProtoWidth,
		bar_index: u8,
		offset: u64,
		count: usize,
		buffer: *mut c_void,
	) -> Status,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PciIoProtoConfigAccess {
	pub read: unsafe extern "efiapi" fn(
		this: *mut PciIoProtocol,
		width: PciIoProtoWidth,
		offset: u32,
		count: usize,
		buffer: *mut c_void,
	) -> Status,
	pub write: unsafe extern "efiapi" fn(
		this: *mut PciIoProtocol,
		width: PciIoProtoWidth,
		offset: u32,
		count: usize,
		buffer: *mut c_void,
	) -> Status,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PciIoProtocol {
    pub poll_mem: unsafe extern "efiapi" fn(
		this: *mut Self,
		width: PciIoProtoWidth,
		bar_index: u8,
		offset: u64,
		mask: u64,
		value: u64,
		delay: u64,
		result: *mut u64,
	) -> Status,
    pub poll_io: unsafe extern "efiapi" fn(
		this: *mut Self,
		width: PciIoProtoWidth,
		bar_index: u8,
		offset: u64,
		mask: u64,
		value: u64,
		delay: u64,
		result: *mut u64,
	) -> Status,
	pub mem: PciIoProtoAccess,
	pub io: PciIoProtoAccess,
	pub pci: PciIoProtoConfigAccess,
	pub copy_mem: unsafe extern "efiapi" fn(
		this: *mut Self,
		width: PciIoProtoWidth,
		dest_bar_index: u8,
		dest_offset: u64,
		src_bar_index: u8,
		src_offset: u64,
		count: usize,
	) -> Status,
	map: *mut c_void,
	pub unmap: unsafe extern "efiapi" fn(
		this: *mut Self,
		mapping: *mut c_void,
	) -> Status,
	allocate_buffer: *mut c_void,
	pub free_buffer: unsafe extern "efiapi" fn(
		this: *mut Self,
		pages: usize,
		host_address: *mut c_void,
	) -> Status,
	pub flush: unsafe extern "efiapi" fn(
		this: *mut Self,
	) -> Status,
	pub get_location: unsafe extern "efiapi" fn(
		this: *mut Self,
		segment_number: *mut usize,
		bus_number: *mut usize,
		device_number: *mut usize,
		function_number: *mut usize,
	) -> Status,
	attributes: *mut c_void,
	get_bar_attributes: *mut c_void,
	set_bar_attributes: *mut c_void,
	pub rom_size: u64,
	pub rom_image: *mut c_void,
}

impl PciIoProtocol {
    pub const GUID: Guid = guid!("4cf5b200-68b8-4ca5-9eec-b23e3f50029a");
}
