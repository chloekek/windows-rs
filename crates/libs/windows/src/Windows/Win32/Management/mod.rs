#[cfg(feature = "Win32_Management_MobileDeviceManagementRegistration")]
pub mod MobileDeviceManagementRegistration;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
