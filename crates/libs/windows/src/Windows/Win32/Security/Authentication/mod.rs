#[cfg(feature = "Win32_Security_Authentication_Identity")]
pub mod Identity;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
