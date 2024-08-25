//! `PciIo` protocol.
use core::ffi::c_void;

use crate::{Error, Result, Status, StatusExt};
use uefi_macros::unsafe_protocol;
use uefi_raw::protocol::pci_io::{PciIoProtoWidth, PciIoProtocol};

/// The PciIo protocol.
#[derive(Debug)]
#[repr(transparent)]
#[unsafe_protocol(PciIoProtocol::GUID)]
pub struct PciIo(PciIoProtocol);

impl PciIo {
	/// SHUT UP
	pub fn pci_read(&mut self, width: u32, offset: u32, count: usize, buf: &mut [u8]) -> Result {
		let w = match width {
			1 => PciIoProtoWidth::UINT8,
			2 => PciIoProtoWidth::UINT16,
			4 => PciIoProtoWidth::UINT32,
			_ => return Result::Err(Error::new(Status::INVALID_PARAMETER, ())),
		};

		unsafe {
			(self.0.pci.read)(&mut self.0, w, offset, count, buf.as_mut_ptr() as *mut c_void).to_result()
		}
	}
	/// SHUT UP
	pub fn pci_write(&mut self, width: u32, offset: u32, count: usize, buf: *mut c_void) -> Result {
		let w = match width {
			1 => PciIoProtoWidth::UINT8,
			2 => PciIoProtoWidth::UINT16,
			4 => PciIoProtoWidth::UINT32,
			_ => return Result::Err(Error::new(Status::INVALID_PARAMETER, ())),
		};

		unsafe {
			(self.0.pci.write)(&mut self.0, w, offset, count, buf).to_result()
		}
	}

	/// SHUT UP 2
	pub fn get_location(&mut self) -> Result<(usize, usize, usize, usize)> {
		let mut segment: usize = 0;
		let mut bus: usize = 0;
		let mut device: usize = 0;
		let mut function: usize = 0;

		unsafe {
			(self.0.get_location)(&mut self.0, &mut segment, &mut bus, &mut device, &mut function).to_result_with_val(||
				(segment, bus, device, function)
			)
		}
	}
}
