#![allow(non_camel_case_types)]

use core::{ffi::c_void, mem::MaybeUninit};

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
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HIDP_REPORT_TYPE(u32);
impl HIDP_REPORT_TYPE {
  pub const INPUT: Self = Self(0);
  pub const OUTPUT: Self = Self(1);
  pub const FEATURE: Self = Self(2);
}

type HIDP_PREPARSED_DATA = c_void;
type USAGE = USHORT;

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
pub struct HIDP_BUTTON_CAPS_Range {
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
pub struct HIDP_BUTTON_CAPS_NotRange {
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
pub union RangeNotRange {
  pub range: HIDP_BUTTON_CAPS_Range,
  pub not_range: HIDP_BUTTON_CAPS_NotRange,
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
  pub u: RangeNotRange,
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
      x.field("range", unsafe { &self.u.range });
    } else {
      x.field("not_range", unsafe { &self.u.not_range });
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
}
