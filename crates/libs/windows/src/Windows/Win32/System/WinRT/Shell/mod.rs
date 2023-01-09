#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
#[repr(transparent)]
pub struct IDDEInitializer(::windows::core::IUnknown);
impl IDDEInitializer {
    #[doc = "*Required features: `\"Win32_UI_Shell\"`*"]
    #[cfg(feature = "Win32_UI_Shell")]
    pub unsafe fn Initialize<P0, P1, P2, P3, P4, P5, P6, P7>(&self, fileextensionorprotocol: P0, method: CreateProcessMethod, currentdirectory: P1, exectarget: P2, site: P3, application: P4, targetfile: P5, arguments: P6, verb: P7) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::super::super::UI::Shell::IShellItem>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P6: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P7: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), fileextensionorprotocol.into().abi(), method, currentdirectory.into().abi(), exectarget.into().abi(), site.into().abi(), application.into().abi(), targetfile.into().abi(), arguments.into().abi(), verb.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IDDEInitializer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDDEInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDDEInitializer {
    type Vtable = IDDEInitializer_Vtbl;
}
unsafe impl ::windows::core::Interface for IDDEInitializer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30dc931f_33fc_4ffd_a168_942258cf3ca4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDDEInitializer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileextensionorprotocol: ::windows::core::PCWSTR, method: CreateProcessMethod, currentdirectory: ::windows::core::PCWSTR, exectarget: *mut ::core::ffi::c_void, site: *mut ::core::ffi::c_void, application: ::windows::core::PCWSTR, targetfile: ::windows::core::PCWSTR, arguments: ::windows::core::PCWSTR, verb: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell"))]
    Initialize: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CreateProcessMethod(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
pub const CpCreateProcess: CreateProcessMethod = CreateProcessMethod(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
pub const CpCreateProcessAsUser: CreateProcessMethod = CreateProcessMethod(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`*"]
pub const CpAicLaunchAdminProcess: CreateProcessMethod = CreateProcessMethod(2i32);
impl ::core::marker::Copy for CreateProcessMethod {}
impl ::core::clone::Clone for CreateProcessMethod {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CreateProcessMethod {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
