#![allow(non_camel_case_types)]

//! Human Interface Device (HID) Public Interface.

use core::{
  ffi::c_void,
  mem::{size_of, MaybeUninit},
};

use crate::{
  errhandlingapi::{get_last_error_here, OsResult},
  win_types::*,
  winuser::RawInputDevicePreparsedData,
};

#[link(name = "hid")]
extern "system" {
  /// MSDN: [HidP_GetCaps](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getcaps)
  fn HidP_GetCaps(
    preparsed_data: *mut HIDP_PREPARSED_DATA, capabilities: *mut HIDP_CAPS,
  ) -> NTSTATUS;

  /// MSDN: [HidP_GetButtonCaps](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getbuttoncaps)
  fn HidP_GetButtonCaps(
    report_type: HIDP_REPORT_TYPE, button_caps: *mut HIDP_BUTTON_CAPS,
    button_caps_length: *mut USHORT, preparsed_data: *mut HIDP_PREPARSED_DATA,
  ) -> NTSTATUS;

  /// MSDN: [HidP_GetValueCaps](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getvaluecaps)
  fn HidP_GetValueCaps(
    report_type: HIDP_REPORT_TYPE, value_caps: *mut HIDP_VALUE_CAPS,
    value_caps_length: *mut USHORT, preparsed_data: *mut HIDP_PREPARSED_DATA,
  ) -> NTSTATUS;

  /// MSDN: [HidP_GetUsages](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_getusages)
  fn HidP_GetUsages(
    report_type: HIDP_REPORT_TYPE, usage_page: USAGE, link_collection: USHORT,
    usage_list: *mut USAGE, usage_length: *mut ULONG,
    preparsed_data: *mut HIDP_PREPARSED_DATA, report: *mut CHAR,
    report_length: ULONG,
  ) -> NTSTATUS;

  /// MSDN: [HidP_MaxUsageListLength](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidpi/nf-hidpi-hidp_maxusagelistlength)
  fn HidP_MaxUsageListLength(
    report_type: HIDP_REPORT_TYPE, usage_page: USAGE,
    preparsed_data: *mut HIDP_PREPARSED_DATA,
  ) -> ULONG;
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HIDP_REPORT_TYPE(u32);
impl HIDP_REPORT_TYPE {
  pub const INPUT: Self = Self(0);
  pub const OUTPUT: Self = Self(1);
  pub const FEATURE: Self = Self(2);
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HIDP_STATUS(NTSTATUS);
impl HIDP_STATUS {
  pub const SUCCESS: Self = Self(1_114_112);
  pub const INVALID_REPORT_LENGTH: Self = Self(-1_072_627_709);
  pub const INVALID_REPORT_TYPE: Self = Self(-1_072_627_710);
  pub const BUFFER_TOO_SMALL: Self = Self(-1_072_627_705);
  pub const INCOMPATIBLE_REPORT_ID: Self = Self(-1_072_627_702);
}
impl core::fmt::Debug for HIDP_STATUS {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      Self::SUCCESS => write!(f, "HIDP_STATUS::SUCCESS"),
      Self::INVALID_REPORT_LENGTH => {
        write!(f, "HIDP_STATUS::INVALID_REPORT_LENGTH")
      }
      Self::INVALID_REPORT_TYPE => {
        write!(f, "HIDP_STATUS::INVALID_REPORT_TYPE")
      }
      Self::BUFFER_TOO_SMALL => {
        write!(f, "HIDP_STATUS::BUFFER_TOO_SMALL")
      }
      Self::INCOMPATIBLE_REPORT_ID => {
        write!(f, "HIDP_STATUS::INCOMPATIBLE_REPORT_ID")
      }
      Self(other) => write!(f, "HIDP_STATUS({other})"),
    }
  }
}

pub type HIDP_PREPARSED_DATA = c_void;
pub type USAGE = USHORT;

const HIDP_STATUS_SUCCESS: NTSTATUS = 1_114_112i32;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct HIDP_CAPS {
  pub usage: USAGE,
  pub usage_page: USAGE,
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
#[derive(Debug, Clone, Copy)]
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
pub struct HIDP_BUTTON_CAPS {
  pub usage_page: USAGE,
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
impl core::fmt::Debug for HIDP_BUTTON_CAPS {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut x = f.debug_struct("HIDP_BUTTON_CAPS");
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
    if self.is_range != 0 {
      x.field("range", &self.u.range());
    } else {
      x.field("not_range", &self.u.not_range());
    }
    x.finish()
  }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HIDP_VALUE_CAPS {
  pub usage_page: USAGE,
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
impl core::fmt::Debug for HIDP_VALUE_CAPS {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut x = f.debug_struct("HIDP_VALUE_CAPS");
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
    x.field("reserved", &self.reserved);
    x.field("bit_size", &self.bit_size);
    x.field("report_count", &self.report_count);
    x.field("reserved2", &self.reserved2);
    x.field("units_exp", &self.units_exp);
    x.field("units", &self.units);
    x.field("logical_min", &self.logical_min);
    x.field("logical_max", &self.logical_max);
    x.field("physical_min", &self.physical_min);
    x.field("physical_max", &self.physical_max);
    if self.is_range != 0 {
      x.field("range", &self.u.range());
    } else {
      x.field("not_range", &self.u.not_range());
    }
    x.finish()
  }
}

impl RawInputDevicePreparsedData {
  #[inline]
  pub fn get_caps(&self) -> OsResult<HIDP_CAPS> {
    //
    let mut caps = HIDP_CAPS::default();
    let ret = unsafe { HidP_GetCaps(self.0.as_ptr() as _, &mut caps) };
    if ret == HIDP_STATUS_SUCCESS {
      Ok(caps)
    } else {
      Err(get_last_error_here())
    }
  }

  #[inline]
  pub fn get_button_caps(
    &self, report_type: HIDP_REPORT_TYPE,
    button_caps: &mut [MaybeUninit<HIDP_BUTTON_CAPS>],
  ) -> OsResult<u16> {
    let mut button_caps_length: USHORT = button_caps.len().try_into().unwrap();
    let ret = unsafe {
      HidP_GetButtonCaps(
        report_type,
        button_caps.as_mut_ptr().cast(),
        &mut button_caps_length,
        self.0.as_ptr() as _,
      )
    };
    if ret == HIDP_STATUS_SUCCESS {
      Ok(button_caps_length)
    } else {
      Err(get_last_error_here())
    }
  }

  #[inline]
  pub fn get_value_caps(
    &self, report_type: HIDP_REPORT_TYPE,
    value_caps: &mut [MaybeUninit<HIDP_VALUE_CAPS>],
  ) -> OsResult<u16> {
    let mut value_caps_length: USHORT = value_caps.len().try_into().unwrap();
    let ret = unsafe {
      HidP_GetValueCaps(
        report_type,
        value_caps.as_mut_ptr().cast(),
        &mut value_caps_length,
        self.0.as_ptr() as _,
      )
    };
    if ret == HIDP_STATUS_SUCCESS {
      Ok(value_caps_length)
    } else {
      Err(get_last_error_here())
    }
  }

  #[inline]
  pub fn get_max_usage_list_length(
    &self, report_type: HIDP_REPORT_TYPE,
  ) -> usize {
    let ret =
      unsafe { HidP_MaxUsageListLength(report_type, 0, self.0.as_ptr() as _) };
    ret as usize
  }

  #[inline]
  pub fn get_usages(
    &self, report_type: HIDP_REPORT_TYPE, usage_page: USAGE,
    usages: &mut [USAGE], hid_report: &mut [u8],
  ) -> Result<ULONG, HIDP_STATUS> {
    let link_collection = 0; // handle this later?
    let mut usage_length: ULONG = usages.len().try_into().unwrap();
    let report_length: ULONG = hid_report.len().try_into().unwrap();
    let ret = HIDP_STATUS(unsafe {
      HidP_GetUsages(
        report_type,
        usage_page,
        link_collection,
        usages.as_mut_ptr(),
        &mut usage_length,
        self.0.as_ptr() as _,
        hid_report.as_mut_ptr().cast::<CHAR>(),
        report_length,
      )
    });
    if ret == HIDP_STATUS::SUCCESS {
      Ok(usage_length)
    } else {
      Err(ret)
    }
  }
}
