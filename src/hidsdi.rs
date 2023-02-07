use core::ffi::*;

use crate::{errhandlingapi::*, win_types::*};

#[link(name = "hid")]
extern "system" {
  /// MSDN: [HidD_GetHidGuid](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidsdi/nf-hidsdi-hidd_gethidguid)
  fn HidD_GetHidGuid(hid_guid: *mut GUID);

  /// MSDN: [HidD_GetIndexedString](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidsdi/nf-hidsdi-hidd_getindexedstring)
  fn HidD_GetIndexedString(
    hid_device_object: HANDLE, string_index: ULONG, buffer: PVOID,
    buffer_length: ULONG,
  ) -> BOOLEAN;

  /// MSDN: [HidD_GetManufacturerString](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidsdi/nf-hidsdi-hidd_getmanufacturerstring)
  fn HidD_GetManufacturerString(
    hid_device_object: HANDLE, buffer: PVOID, buffer_length: ULONG,
  ) -> BOOLEAN;

  /// MSDN: [HidD_GetPhysicalDescriptor](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidsdi/nf-hidsdi-hidd_getphysicaldescriptor)
  fn HidD_GetPhysicalDescriptor(
    hid_device_object: HANDLE, buffer: PVOID, buffer_length: ULONG,
  ) -> BOOLEAN;

  /// MSDN: [HidD_GetProductString](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidsdi/nf-hidsdi-hidd_getproductstring)
  fn HidD_GetProductString(
    hid_device_object: HANDLE, buffer: PVOID, buffer_length: ULONG,
  ) -> BOOLEAN;

  /// MSDN: [HidD_GetSerialNumberString](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidsdi/nf-hidsdi-hidd_getserialnumberstring)
  fn HidD_GetSerialNumberString(
    hid_device_object: HANDLE, buffer: PVOID, buffer_length: ULONG,
  ) -> BOOLEAN;
}

/// Returns the device interface GUID for HIDClass devices.
#[inline]
pub fn hidd_get_hid_guid() -> GUID {
  let mut guid = GUID::default();
  unsafe { HidD_GetHidGuid(&mut guid) };
  guid
}

/// Gets an indexed string from an open top-level collection.
///
/// * The buffer should be <=4093 bytes.
///
/// On success, the buffer will have been written with the zero-termianted
/// string data.
#[inline]
#[track_caller]
pub fn hidd_get_indexed_string(
  hid_device_object: HANDLE, string_index: ULONG, buf: &mut [u8],
) -> OsResult<()> {
  let buffer_length: ULONG = buf.len().try_into().unwrap();
  let buffer = buf.as_mut_ptr().cast();
  let success = unsafe {
    HidD_GetIndexedString(
      hid_device_object,
      string_index,
      buffer,
      buffer_length,
    )
  };
  if success.into() {
    Ok(())
  } else {
    Err(get_last_error_here())
  }
}

/// Gets the embedded string that identifies the device manufacturer.
///
/// * The buffer should be <=4093 bytes.
///
/// On success, the buffer will have been written with the zero-termianted
/// string data.
#[inline]
#[track_caller]
pub fn hidd_get_manufacturer_string(
  hid_device_object: HANDLE, buf: &mut [u8],
) -> OsResult<()> {
  let buffer_length: ULONG = buf.len().try_into().unwrap();
  let buffer = buf.as_mut_ptr().cast();
  let success = unsafe {
    HidD_GetManufacturerString(hid_device_object, buffer, buffer_length)
  };
  if success.into() {
    Ok(())
  } else {
    Err(get_last_error_here())
  }
}

/// Gets the returns the "Physical Descriptor" of a top-level collection.
///
/// * The buffer should be <=4093 bytes.
///
/// On success, the buffer will have been written with the zero-termianted
/// string data.
#[inline]
#[track_caller]
pub fn hidd_get_physical_descriptor(
  hid_device_object: HANDLE, buf: &mut [u8],
) -> OsResult<()> {
  let buffer_length: ULONG = buf.len().try_into().unwrap();
  let buffer = buf.as_mut_ptr().cast();
  let success = unsafe {
    HidD_GetPhysicalDescriptor(hid_device_object, buffer, buffer_length)
  };
  if success.into() {
    Ok(())
  } else {
    Err(get_last_error_here())
  }
}

/// Gets the embedded string that identifies the manufacturer's product.
///
/// * The buffer should be <=4093 bytes.
///
/// On success, the buffer will have been written with the zero-termianted
/// string data.
#[inline]
#[track_caller]
pub fn hidd_get_product_string(
  hid_device_object: HANDLE, buf: &mut [u8],
) -> OsResult<()> {
  let buffer_length: ULONG = buf.len().try_into().unwrap();
  let buffer = buf.as_mut_ptr().cast();
  let success =
    unsafe { HidD_GetProductString(hid_device_object, buffer, buffer_length) };
  if success.into() {
    Ok(())
  } else {
    Err(get_last_error_here())
  }
}

/// Gets the embedded string that identifies the the serial number of the
/// collection's physical device.
///
/// * The buffer should be <=4093 bytes.
///
/// On success, the buffer will have been written with the zero-termianted
/// string data.
#[inline]
#[track_caller]
pub fn hidd_get_serial_number_string(
  hid_device_object: HANDLE, buf: &mut [u8],
) -> OsResult<()> {
  let buffer_length: ULONG = buf.len().try_into().unwrap();
  let buffer = buf.as_mut_ptr().cast();
  let success = unsafe {
    HidD_GetSerialNumberString(hid_device_object, buffer, buffer_length)
  };
  if success.into() {
    Ok(())
  } else {
    Err(get_last_error_here())
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct GUID {
  pub data1: c_ulong,
  pub data2: c_ushort,
  pub data3: c_ushort,
  pub data4: [c_uchar; 8],
}
