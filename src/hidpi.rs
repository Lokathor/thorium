#![allow(non_camel_case_types)]

//! Human Interface Device (HID) Parsing Interface.

use core::{
  ffi::c_void,
  mem::{size_of, MaybeUninit},
};

use crate::{win_types::*, winuser::RawInputDevicePreparsedData};

#[link(name = "hid")]
extern "system" {
  /// MSDN: [HidP_GetCaps](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getcaps)
  fn HidP_GetCaps(
    preparsed_data: *const HIDP_PREPARSED_DATA, capabilities: *mut HidpCaps,
  ) -> HidpStatus;

  /// MSDN: [HidP_GetButtonCaps](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getbuttoncaps)
  fn HidP_GetButtonCaps(
    report_type: HidpReportType, button_caps: *mut HidpButtonCaps,
    button_caps_length: *mut USHORT,
    preparsed_data: *const HIDP_PREPARSED_DATA,
  ) -> HidpStatus;

  /// MSDN: [HidP_GetValueCaps](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getvaluecaps)
  fn HidP_GetValueCaps(
    report_type: HidpReportType, value_caps: *mut HidpValueCaps,
    value_caps_length: *mut USHORT, preparsed_data: *const HIDP_PREPARSED_DATA,
  ) -> HidpStatus;

  /// MSDN: [HidP_MaxUsageListLength](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_maxusagelistlength)
  fn HidP_MaxUsageListLength(
    report_type: HidpReportType, usage_page: HidUsagePage,
    preparsed_data: *const HIDP_PREPARSED_DATA,
  ) -> ULONG;

  /// MSDN: [HidP_GetUsages](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getusages)
  fn HidP_GetUsages(
    report_type: HidpReportType, usage_page: HidUsagePage,
    link_collection: USHORT, usage_list: *mut USAGE, usage_length: *mut ULONG,
    preparsed_data: *const HIDP_PREPARSED_DATA, report: *const u8,
    report_length: ULONG,
  ) -> HidpStatus;

  /// MSDN: [HidP_GetUsageValue](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getusagevalue)
  fn HidP_GetUsageValue(
    report_type: HidpReportType, usage_page: HidUsagePage,
    link_collection: USHORT, usage: USAGE, usage_value: *mut ULONG,
    preparsed_data: *const HIDP_PREPARSED_DATA, report: *const u8,
    report_length: ULONG,
  ) -> HidpStatus;

  /// MSDN: [HidP_GetScaledUsageValue](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getscaledusagevalue)
  fn HidP_GetScaledUsageValue(
    report_type: HidpReportType, usage_page: HidUsagePage,
    link_collection: USHORT, usage: USAGE, usage_value: *mut ULONG,
    preparsed_data: *const HIDP_PREPARSED_DATA, report: *const u8,
    report_length: ULONG,
  ) -> HidpStatus;

  /// MSDN: [HidP_GetUsageValueArray](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getusagevaluearray)
  fn HidP_GetUsageValueArray(
    report_type: HidpReportType, usage_page: HidUsagePage,
    link_collection: USHORT, usage: USAGE, usage_value: *mut u8,
    usage_value_byte_length: USHORT,
    preparsed_data: *const HIDP_PREPARSED_DATA, report: *const u8,
    report_length: ULONG,
  ) -> HidpStatus;
}

#[allow(missing_docs)]
pub type HidpResult<T> = Result<T, HidpStatus>;

/// Reads the Preparsed Data to determine the general capabilities of the
/// device.
///
/// Particularly, output tells you how big of buffers you will need to provide
/// to other hidp functions to get info about the buttons and values of a
/// device.
#[inline]
pub fn hidp_get_caps(
  preparsed_data: &RawInputDevicePreparsedData,
) -> HidpResult<HidpCaps> {
  let mut capabilities = HidpCaps::default();
  let status = unsafe {
    HidP_GetCaps(preparsed_data.as_preparsed_data_ptr(), &mut capabilities)
  };
  if status == HidpStatus::SUCCESS {
    Ok(capabilities)
  } else {
    Err(status)
  }
}

/// Reads the Preparsed Data to get info about the buttons for a given report
/// type.
///
/// * Buttons are binary controls, they're each either "off" or "on".
/// * Each button *should* be mapped to a separate "usage" from
///   `HidUsagePage::BUTTONS` (usage page 0x09). Some useage numbers might not
///   be used, either they map to non-button controls or they map to no control
///   at all.
/// * The required buffer size is based on the `number_*_button_caps` field for
///   the given report type (input, output, or feature).
///
/// On success, the returned slice is the starting portion of the input buffer
/// that is now initialized with data.
///
/// ```no_run
/// # use thorium::hidpi::{hidp_get_caps, hidp_get_button_caps, HidpReportType, HidpButtonCaps};
/// # let preparsed_data = todo!();
/// let caps = hidp_get_caps(preparsed_data).unwrap();
/// let num_input_buttons = usize::from(caps.number_input_button_caps);
/// let mut buf: Vec<HidpButtonCaps> = Vec::with_capacity(num_input_buttons);
/// let init_buf = hidp_get_button_caps(
///   HidpReportType::INPUT,
///   buf.spare_capacity_mut(),
///   preparsed_data,
/// ).unwrap();
/// ```
#[inline]
pub fn hidp_get_button_caps<'b>(
  report_type: HidpReportType, buf: &'b mut [MaybeUninit<HidpButtonCaps>],
  preparsed_data: &RawInputDevicePreparsedData,
) -> HidpResult<&'b [HidpButtonCaps]> {
  let mut button_caps_length: USHORT = buf.len().try_into().unwrap();
  let button_caps = buf.as_mut_ptr().cast::<HidpButtonCaps>();
  let preparsed_data = preparsed_data.as_preparsed_data_ptr();
  let status = unsafe {
    HidP_GetButtonCaps(
      report_type,
      button_caps,
      &mut button_caps_length,
      preparsed_data,
    )
  };
  if status == HidpStatus::SUCCESS {
    let len = usize::from(button_caps_length);
    let keep: &[MaybeUninit<HidpButtonCaps>] = &buf[..len];
    let out: &[HidpButtonCaps] =
      unsafe { core::slice::from_raw_parts(keep.as_ptr().cast(), keep.len()) };
    Ok(out)
  } else {
    Err(status)
  }
}

/// Reads the Preparsed Data to get info about the "values" for a given report
/// type.
///
/// * Values are ranged controls, such as an axis. Each control has a specific
///   range.
/// * Each value will *usually* be from `HidUsagePage::GENERIC_DESKTOP` (usage
///   page 0x01), generally an axis or "pov hat" value  (usages 0x30 to 0x39,
///   see [HID Usages And Descriptions][pdf]). A given usage can appear more
///   than once within the full report. When this happened on my own hardware
///   all the instances of a given usage were identical anyway.
/// * The required buffer size is based on the `number_*_value_caps` field for
///   the given report type (input, output, or feature).
///
/// [pdf]: https://usb.org/sites/default/files/hut1_4.pdf
///
/// On success, the returned slice is the starting portion of the input buffer
/// that is now initialized with data.
///
/// ```no_run
/// # use thorium::hidpi::{hidp_get_caps, hidp_get_value_caps, HidpReportType, HidpValueCaps};
/// # let preparsed_data = todo!();
/// let caps = hidp_get_caps(preparsed_data).unwrap();
/// let num_input_buttons = usize::from(caps.number_input_button_caps);
/// let mut buf: Vec<HidpValueCaps> = Vec::with_capacity(num_input_buttons);
/// let init_buf = hidp_get_value_caps(
///   HidpReportType::INPUT,
///   buf.spare_capacity_mut(),
///   preparsed_data,
/// ).unwrap();
/// ```
#[inline]
pub fn hidp_get_value_caps<'b>(
  report_type: HidpReportType, buf: &'b mut [MaybeUninit<HidpValueCaps>],
  preparsed_data: &RawInputDevicePreparsedData,
) -> HidpResult<&'b [HidpValueCaps]> {
  let mut value_caps_length: USHORT = buf.len().try_into().unwrap();
  let value_caps = buf.as_mut_ptr().cast::<HidpValueCaps>();
  let preparsed_data = preparsed_data.as_preparsed_data_ptr();
  let status = unsafe {
    HidP_GetValueCaps(
      report_type,
      value_caps,
      &mut value_caps_length,
      preparsed_data,
    )
  };
  if status == HidpStatus::SUCCESS {
    let len = usize::from(value_caps_length);
    let keep: &[MaybeUninit<HidpValueCaps>] = &buf[..len];
    let out: &[HidpValueCaps] =
      unsafe { core::slice::from_raw_parts(keep.as_ptr().cast(), keep.len()) };
    Ok(out)
  } else {
    Err(status)
  }
}

/// Returns the maximum buffer size required to get all info from
/// [hidp_get_buttons].
///
/// If there's an error of some sort this will return 0.
#[inline]
pub fn hidp_max_button_list_length(
  report_type: HidpReportType, usage_page: HidUsagePage,
  preparsed_data: &RawInputDevicePreparsedData,
) -> usize {
  let ret = unsafe {
    HidP_MaxUsageListLength(
      report_type,
      usage_page,
      preparsed_data.as_preparsed_data_ptr(),
    )
  };
  ret.try_into().unwrap()
}

/// Gets all button usage information from an HID report.
///
/// On success, returns the initial portion of the buffer which has been written
/// with the usage values of all buttons that are currently "on". Any button
/// usages not listed are naturally "off".
///
/// * If `link_collection` is non-zero this will only return buttons in the
///   specified link collection. Otherwise all button info will be returned.
/// * Use [hidp_max_button_list_length] to get the maximum required buffer size,
///   otherwise the buffer might be too small to hold all the results.
///
/// See MSDN: [HidP_GetUsages](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getusages)
#[inline]
pub fn hidp_get_buttons<'b>(
  report_type: HidpReportType, usage_page: HidUsagePage,
  link_collection: USHORT, buf: &'b mut [USAGE],
  preparsed_data: &RawInputDevicePreparsedData, report: &[u8],
) -> HidpResult<&'b [USAGE]> {
  let mut usage_length: ULONG = buf.len().try_into().unwrap();
  let usage_list = buf.as_mut_ptr();
  let report_length: ULONG = report.len().try_into().unwrap();
  let status = unsafe {
    HidP_GetUsages(
      report_type,
      usage_page,
      link_collection,
      usage_list,
      &mut usage_length,
      preparsed_data.as_preparsed_data_ptr(),
      report.as_ptr(),
      report_length,
    )
  };
  if status == HidpStatus::SUCCESS {
    let new_buf_len: usize = usage_length.try_into().unwrap();
    Ok(&buf[..new_buf_len])
  } else {
    Err(status)
  }
}

/// Gets the raw value for a single usage from an HID report (eg: one raw
/// axis value).
///
/// * This function is intended for when `caps.report_count` of the
///   [HidpValueCaps] is 1. Otherwise, only the first value will be extracted
///   properly. In that case you should use [hidp_get_usage_value_array] to get
///   the full data.
///
/// See MSDN:
/// [HidP_GetUsageValue](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getusagevalue)
#[inline]
pub fn hidp_get_usage_value(
  report_type: HidpReportType, usage_page: HidUsagePage,
  link_collection: USHORT, usage: USAGE,
  preparsed_data: &RawInputDevicePreparsedData, report: &[u8],
) -> HidpResult<ULONG> {
  let mut usage_value: ULONG = 0;
  let report_length: ULONG = report.len().try_into().unwrap();
  let status = unsafe {
    HidP_GetUsageValue(
      report_type,
      usage_page,
      link_collection,
      usage,
      &mut usage_value,
      preparsed_data.as_preparsed_data_ptr(),
      report.as_ptr(),
      report_length,
    )
  };
  if status == HidpStatus::SUCCESS {
    Ok(usage_value)
  } else {
    Err(status)
  }
}

/// Gets the scaled value for a single usage from an HID report (eg: one scaled
/// axis value).
///
/// * This function is intended for when `caps.report_count` of the
///   [HidpValueCaps] is 1. Otherwise, only the first value will be extracted
///   properly. In that case you should use [hidp_get_usage_value_array] to get
///   the full data, and then you'll have to scale it yourself.
///
/// See MSDN:
/// [HidP_GetScaledUsageValue](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getscaledusagevalue)
#[inline]
pub fn hidp_get_scaled_usage_value(
  report_type: HidpReportType, usage_page: HidUsagePage,
  link_collection: USHORT, usage: USAGE,
  preparsed_data: &RawInputDevicePreparsedData, report: &[u8],
) -> HidpResult<ULONG> {
  let mut usage_value: ULONG = 0;
  let report_length: ULONG = report.len().try_into().unwrap();
  let status = unsafe {
    HidP_GetScaledUsageValue(
      report_type,
      usage_page,
      link_collection,
      usage,
      &mut usage_value,
      preparsed_data.as_preparsed_data_ptr(),
      report.as_ptr(),
      report_length,
    )
  };
  if status == HidpStatus::SUCCESS {
    Ok(usage_value)
  } else {
    Err(status)
  }
}

/// Get an array of usage values from a multi-count HID report.
///
/// * This function is intended for when `caps.report_count` of the
///   [HidpValueCaps] is greater than 1. If the report count is only 1 then you
///   should use [hidp_get_usage_value] or [hidp_get_scaled_usage_value]
///   instead.
/// * The required buffer size is the `caps.bit_size * caps.report_count`,
///   rounded up to the nearest byte, of the associated [HidpValueCaps].
/// * The buffer is written in little-endian order. I think the individual
///   values are two bytes each?
#[inline]
pub fn hidp_get_usage_value_array(
  report_type: HidpReportType, usage_page: HidUsagePage,
  link_collection: USHORT, usage: USAGE, usage_value: &mut [u8],
  preparsed_data: &RawInputDevicePreparsedData, report: &[u8],
) -> HidpResult<()> {
  let report_length: ULONG = report.len().try_into().unwrap();
  let usage_value_byte_length: USHORT = usage_value.len().try_into().unwrap();
  let status = unsafe {
    HidP_GetUsageValueArray(
      report_type,
      usage_page,
      link_collection,
      usage,
      usage_value.as_mut_ptr(),
      usage_value_byte_length,
      preparsed_data.as_preparsed_data_ptr(),
      report.as_ptr(),
      report_length,
    )
  };
  if status == HidpStatus::SUCCESS {
    Ok(())
  } else {
    Err(status)
  }
}

impl RawInputDevicePreparsedData {
  fn as_preparsed_data_ptr(&self) -> *const HIDP_PREPARSED_DATA {
    self.0.as_ptr().cast::<c_void>()
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HidpReportType(u32);
impl HidpReportType {
  pub const INPUT: Self = Self(0);
  pub const OUTPUT: Self = Self(1);
  pub const FEATURE: Self = Self(2);
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct HidUsagePage(pub USAGE);
impl HidUsagePage {
  pub const GENERIC_DESKTOP: Self = Self(0x01);
  pub const BUTTONS: Self = Self(0x09);
}
impl core::fmt::Debug for HidUsagePage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      Self::GENERIC_DESKTOP => write!(f, "HidUsagePage::GENERIC_DESKTOP"),
      Self::BUTTONS => write!(f, "HidUsagePage::BUTTONS"),
      other => write!(f, "HidUsagePage({:02X})", other.0),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HidpStatus(NTSTATUS);
impl HidpStatus {
  pub const SUCCESS: Self = Self(1_114_112);
  pub const BAD_LOG_PHY_VALUES: Self = Self(-1_072_627_706);
  pub const INVALID_REPORT_LENGTH: Self = Self(-1_072_627_709);
  pub const INVALID_REPORT_TYPE: Self = Self(-1_072_627_710);
  pub const BUFFER_TOO_SMALL: Self = Self(-1_072_627_705);
  pub const INCOMPATIBLE_REPORT_ID: Self = Self(-1_072_627_702);
  pub const INVALID_PREPARSED_DATA: Self = Self(-1_072_627_711);
  pub const USAGE_NOT_FOUND: Self = Self(-1_072_627_708);
  pub const VALUE_OUT_OF_RANGE: Self = Self(-1_072_627_707);
}
impl core::fmt::Debug for HidpStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      Self::SUCCESS => write!(f, "HidpStatus::SUCCESS"),
      Self::INVALID_REPORT_LENGTH => {
        write!(f, "HidpStatus::INVALID_REPORT_LENGTH")
      }
      Self::BAD_LOG_PHY_VALUES => {
        write!(f, "HidpStatus::BAD_LOG_PHY_VALUES")
      }
      Self::VALUE_OUT_OF_RANGE => {
        write!(f, "HidpStatus::VALUE_OUT_OF_RANGE")
      }
      Self::INVALID_REPORT_TYPE => {
        write!(f, "HidpStatus::INVALID_REPORT_TYPE")
      }
      Self::BUFFER_TOO_SMALL => {
        write!(f, "HidpStatus::BUFFER_TOO_SMALL")
      }
      Self::INCOMPATIBLE_REPORT_ID => {
        write!(f, "HidpStatus::INCOMPATIBLE_REPORT_ID")
      }
      Self::INVALID_PREPARSED_DATA => {
        write!(f, "HidpStatus::INVALID_PREPARSED_DATA")
      }
      Self::USAGE_NOT_FOUND => {
        write!(f, "HidpStatus::USAGE_NOT_FOUND")
      }
      Self(other) => write!(f, "HidpStatus({other})"),
    }
  }
}

type HIDP_PREPARSED_DATA = c_void;

pub type USAGE = USHORT;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct HidpCaps {
  pub usage: USAGE,
  pub usage_page: HidUsagePage,
  pub input_report_byte_length: USHORT,
  pub output_report_byte_length: USHORT,
  pub feature_report_byte_length: USHORT,
  pub reserved: [USHORT; 17],
  pub number_link_collection_nodes: USHORT,
  pub number_input_button_caps: USHORT,
  pub number_input_value_caps: USHORT,
  pub number_input_data_indices: USHORT,
  pub number_output_button_caps: USHORT,
  pub number_output_value_caps: USHORT,
  pub number_output_data_indices: USHORT,
  pub number_feature_button_caps: USHORT,
  pub number_feature_value_caps: USHORT,
  pub number_feature_data_indices: USHORT,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CapsRange {
  pub usage_min: USAGE,
  pub usage_max: USAGE,
  pub string_min: USHORT,
  pub string_max: USHORT,
  pub designator_min: USHORT,
  pub designator_max: USHORT,
  pub data_index_min: USHORT,
  pub data_index_max: USHORT,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CapsNotRange {
  pub usage: USAGE,
  pub reserved1: USAGE,
  pub string_index: USHORT,
  pub reserved2: USHORT,
  pub designator_index: USHORT,
  pub reserved3: USHORT,
  pub data_index: USHORT,
  pub reserved4: USHORT,
}
impl core::fmt::Debug for CapsNotRange {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut x = f.debug_struct("CapsNotRange");
    x.field("usage", &self.usage);
    x.field("string_index", &self.string_index);
    x.field("designator_index", &self.designator_index);
    x.field("data_index", &self.data_index);
    x.finish()
  }
}
#[derive(Clone, Copy)]
#[repr(C)]
pub union CapsRangeNotRange {
  pub range: CapsRange,
  pub not_range: CapsNotRange,
}
impl CapsRangeNotRange {
  #[inline]
  #[must_use]
  pub fn range(&self) -> &CapsRange {
    assert_eq!(size_of::<CapsRange>(), size_of::<CapsNotRange>());
    unsafe { &self.range }
  }
  #[inline]
  #[must_use]
  pub fn not_range(&self) -> &CapsNotRange {
    assert_eq!(size_of::<CapsRange>(), size_of::<CapsNotRange>());
    unsafe { &self.not_range }
  }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HidpButtonCaps {
  pub usage_page: HidUsagePage,
  pub report_id: UCHAR,
  pub is_alias: BOOLEAN,
  pub bit_field: USHORT,
  pub link_collection: USHORT,
  pub link_usage: USAGE,
  pub link_usage_page: USAGE,
  pub is_range: BOOLEAN,
  pub is_string_range: BOOLEAN,
  pub is_designator_range: BOOLEAN,
  pub is_absolute: BOOLEAN,
  pub report_count: USHORT,
  pub reserved2: USHORT,
  pub reserved: [ULONG; 9],
  pub u: CapsRangeNotRange,
}
impl core::fmt::Debug for HidpButtonCaps {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut x = f.debug_struct("HidpButtonCaps");
    x.field("usage_page", &self.usage_page);
    x.field("report_id", &self.report_id);
    x.field("is_alias", &self.is_alias);
    x.field("bit_field", &self.bit_field);
    x.field("link_collection", &self.link_collection);
    x.field("link_usage", &self.link_usage);
    x.field("link_usage_page", &self.link_usage_page);
    x.field("is_range", &self.is_range);
    x.field("is_string_range", &self.is_string_range);
    x.field("is_designator_range", &self.is_designator_range);
    x.field("is_absolute", &self.is_absolute);
    x.field("report_count", &self.report_count);
    if self.is_range.into() {
      x.field("range", &self.u.range());
    } else {
      x.field("not_range", &self.u.not_range());
    }
    x.finish()
  }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HidpValueCaps {
  pub usage_page: HidUsagePage,
  pub report_id: UCHAR,
  pub is_alias: BOOLEAN,
  pub bit_field: USHORT,
  pub link_collection: USHORT,
  pub link_usage: USAGE,
  pub link_usage_page: USAGE,
  pub is_range: BOOLEAN,
  pub is_string_range: BOOLEAN,
  pub is_designator_range: BOOLEAN,
  pub is_absolute: BOOLEAN,
  pub has_null: BOOLEAN,
  pub reserved: UCHAR,
  pub bit_size: USHORT,
  pub report_count: USHORT,
  pub reserved2: [USHORT; 5],
  pub units_exp: ULONG,
  pub units: ULONG,
  pub logical_min: LONG,
  pub logical_max: LONG,
  pub physical_min: LONG,
  pub physical_max: LONG,
  pub u: CapsRangeNotRange,
}
impl core::fmt::Debug for HidpValueCaps {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut x = f.debug_struct("HidpValueCaps");
    x.field("usage_page", &self.usage_page);
    x.field("report_id", &self.report_id);
    x.field("is_alias", &self.is_alias);
    x.field("bit_field", &self.bit_field);
    x.field("link_collection", &self.link_collection);
    x.field("link_usage", &self.link_usage);
    x.field("link_usage_page", &self.link_usage_page);
    x.field("is_range", &self.is_range);
    x.field("is_string_range", &self.is_string_range);
    x.field("is_designator_range", &self.is_designator_range);
    x.field("is_absolute", &self.is_absolute);
    x.field("has_null", &self.has_null);
    x.field("bit_size", &self.bit_size);
    x.field("report_count", &self.report_count);
    x.field("units_exp", &self.units_exp);
    x.field("units", &self.units);
    x.field("logical_min", &self.logical_min);
    x.field("logical_max", &self.logical_max);
    x.field("physical_min", &self.physical_min);
    x.field("physical_max", &self.physical_max);
    if self.is_range.into() {
      x.field("range", &self.u.range());
    } else {
      x.field("not_range", &self.u.not_range());
    }
    x.finish()
  }
}
