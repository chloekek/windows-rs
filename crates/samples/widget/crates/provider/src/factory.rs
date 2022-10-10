use windows::{
    core::{implement, IUnknown, Interface},
    Win32::{
        Foundation::{BOOL, CLASS_E_NOAGGREGATION, E_UNEXPECTED},
        System::Com::{IClassFactory, IClassFactory_Impl},
    },
};

use crate::widget::WidgetProvider;

#[implement(IClassFactory)]
pub struct WidgetProviderClassFactory;

impl IClassFactory_Impl for WidgetProviderClassFactory {
    fn CreateInstance(&self, outer: &core::option::Option<windows::core::IUnknown>, iid: *const windows::core::GUID, object: *mut *mut core::ffi::c_void) -> windows::core::Result<()> {
        match outer {
            Some(_unk) => CLASS_E_NOAGGREGATION.ok(),
            None => {
                let unknown: IUnknown = WidgetProvider::default().into();
                unsafe { unknown.query(&*iid, object as *mut _).ok() }
            }
        }
    }

    fn LockServer(&self, _flock: BOOL) -> windows::core::Result<()> {
        E_UNEXPECTED.ok()
    }
}
