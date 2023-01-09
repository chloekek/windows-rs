#[cfg(feature = "Phone_ApplicationModel")]
pub mod ApplicationModel;
#[cfg(feature = "Phone_Devices")]
pub mod Devices;
#[cfg(feature = "Phone_Management")]
pub mod Management;
#[cfg(feature = "Phone_Media")]
pub mod Media;
#[cfg(feature = "Phone_Notification")]
pub mod Notification;
#[cfg(feature = "Phone_PersonalInformation")]
pub mod PersonalInformation;
#[cfg(feature = "Phone_Speech")]
pub mod Speech;
#[cfg(feature = "Phone_StartScreen")]
pub mod StartScreen;
#[cfg(feature = "Phone_System")]
pub mod System;
#[cfg(feature = "Phone_UI")]
pub mod UI;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
