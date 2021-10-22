#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_UI_Accessibility")]
pub mod Accessibility;
#[cfg(feature = "Win32_UI_Animation")]
pub mod Animation;
#[cfg(feature = "Win32_UI_ColorSystem")]
pub mod ColorSystem;
#[cfg(feature = "Win32_UI_Controls")]
pub mod Controls;
#[cfg(feature = "Win32_UI_DisplayDevices")]
pub mod DisplayDevices;
#[cfg(feature = "Win32_UI_HiDpi")]
pub mod HiDpi;
#[cfg(feature = "Win32_UI_InkInput")]
pub mod InkInput;
#[cfg(feature = "Win32_UI_InteractionContext")]
pub mod InteractionContext;
#[cfg(feature = "Win32_UI_KeyboardAndMouseInput")]
pub mod KeyboardAndMouseInput;
#[cfg(feature = "Win32_UI_LegacyWindowsEnvironmentFeatures")]
pub mod LegacyWindowsEnvironmentFeatures;
#[cfg(feature = "Win32_UI_Magnification")]
pub mod Magnification;
#[cfg(feature = "Win32_UI_Notifications")]
pub mod Notifications;
#[cfg(feature = "Win32_UI_PointerInput")]
pub mod PointerInput;
#[cfg(feature = "Win32_UI_RadialInput")]
pub mod RadialInput;
#[cfg(feature = "Win32_UI_Ribbon")]
pub mod Ribbon;
#[cfg(feature = "Win32_UI_Shell")]
pub mod Shell;
#[cfg(feature = "Win32_UI_TabletPC")]
pub mod TabletPC;
#[cfg(feature = "Win32_UI_TextServices")]
pub mod TextServices;
#[cfg(feature = "Win32_UI_TouchInput")]
pub mod TouchInput;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub mod WindowsAndMessaging;
#[cfg(feature = "Win32_UI_Wpf")]
pub mod Wpf;
#[cfg(feature = "Win32_UI_XInput")]
pub mod XInput;
#[cfg(feature = "Win32_UI_Xaml")]
pub mod Xaml;
