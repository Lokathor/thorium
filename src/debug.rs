use super::*;

/// Prints GL debug messages to stdout.
///
/// ## Safety
/// * The `length` and `message` values must be valid for making a `&[u8]`.
#[allow(clippy::missing_inline_in_public_items)]
pub unsafe extern "system" fn gl_debug_print_callback(
  source: u32, type_: u32, _id: u32, severity: u32, length: u32,
  message: *const u8, _user_param: *const c_void,
) {
  let source = match source {
    GL_DEBUG_SOURCE_API_KHR => "[Api]",
    GL_DEBUG_SOURCE_WINDOW_SYSTEM_KHR => "[WindowSystem]",
    GL_DEBUG_SOURCE_SHADER_COMPILER_KHR => "[ShaderCompiler]",
    GL_DEBUG_SOURCE_THIRD_PARTY_KHR => "[3rdParty]",
    GL_DEBUG_SOURCE_APPLICATION_KHR => "[Application]",
    GL_DEBUG_SOURCE_OTHER_KHR => "[OtherSource]",
    _ => "[UnknownSrc]",
  };
  let type_ = match type_ {
    GL_DEBUG_TYPE_ERROR_KHR => "[Error]",
    GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR => "[Deprecated]",
    GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR => "[Undefined]",
    GL_DEBUG_TYPE_PORTABILITY_KHR => "[Portability]",
    GL_DEBUG_TYPE_PERFORMANCE_KHR => "[Performance]",
    GL_DEBUG_TYPE_OTHER_KHR => "[Other]",
    GL_DEBUG_TYPE_MARKER_KHR => "[Marker]",
    _ => "[UnknownType]",
  };
  let severity = match severity {
    GL_DEBUG_SEVERITY_HIGH_KHR => "[SeverityHigh]",
    GL_DEBUG_SEVERITY_MEDIUM_KHR => "[SeverityMedium]",
    GL_DEBUG_SEVERITY_LOW_KHR => "[SeverityLow]",
    GL_DEBUG_SEVERITY_NOTIFICATION_KHR => "[Note]",
    _ => "[SeverityUnknown]",
  };
  let message_slice = unsafe {
    core::slice::from_raw_parts(message, length.try_into().unwrap_or(0))
  };
  let message_str =
    core::str::from_utf8(message_slice).unwrap_or("message was not UTF8");
  println!("GL{source}{type_}{severity}> {message_str}");
}
