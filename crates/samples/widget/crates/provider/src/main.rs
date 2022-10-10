#![windows_subsystem = "windows"]

mod factory;
mod widget;

use windows::{
    core::{IUnknown, GUID},
    Win32::{
        System::Com::{CoInitializeEx, CoRegisterClassObject, CLSCTX_LOCAL_SERVER, COINIT_MULTITHREADED, REGCLS_MULTIPLEUSE},
        UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, TranslateMessage, MSG},
    },
};

use crate::factory::WidgetProviderClassFactory;

fn main() -> windows::core::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        let unknown: IUnknown = WidgetProviderClassFactory.into();
        CoRegisterClassObject(&GUID::from_u128(0x5833908d_edcc_4911_b0d6_9882dc9f747f), &unknown, CLSCTX_LOCAL_SERVER, REGCLS_MULTIPLEUSE)?;

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}
