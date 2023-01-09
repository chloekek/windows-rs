#[doc = "*Required features: `\"Win32_UI_Input_Radial\"`*"]
#[repr(transparent)]
pub struct IRadialControllerConfigurationInterop(::windows::core::IUnknown);
impl IRadialControllerConfigurationInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, hwnd: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), hwnd.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRadialControllerConfigurationInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IRadialControllerConfigurationInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRadialControllerConfigurationInterop {
    type Vtable = IRadialControllerConfigurationInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IRadialControllerConfigurationInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x787cdaac_3186_476d_87e4_b9374a7b9970);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_UI_Input_Radial\"`*"]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSourceInterop(::windows::core::IUnknown);
impl IRadialControllerIndependentInputSourceInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<P0, T>(&self, hwnd: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateForWindow)(::windows::core::Vtable::as_raw(self), hwnd.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRadialControllerIndependentInputSourceInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IRadialControllerIndependentInputSourceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRadialControllerIndependentInputSourceInterop {
    type Vtable = IRadialControllerIndependentInputSourceInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IRadialControllerIndependentInputSourceInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577eff_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSourceInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
#[doc = "*Required features: `\"Win32_UI_Input_Radial\"`*"]
#[repr(transparent)]
pub struct IRadialControllerInterop(::windows::core::IUnknown);
impl IRadialControllerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<P0, T>(&self, hwnd: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateForWindow)(::windows::core::Vtable::as_raw(self), hwnd.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRadialControllerInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IRadialControllerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRadialControllerInterop {
    type Vtable = IRadialControllerInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IRadialControllerInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b0535c9_57ad_45c1_9d79_ad5c34360513);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
