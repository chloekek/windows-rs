#[cfg(feature = "Media_Streaming_Adaptive")]
pub mod Adaptive;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
