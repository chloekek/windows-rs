#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIAssociatedIdentityProvider(::windows::core::IUnknown);
impl AsyncIAssociatedIdentityProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_AssociateIdentity<P0>(&self, hwndparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).Begin_AssociateIdentity)(::windows::core::Vtable::as_raw(self), hwndparent.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_AssociateIdentity(&self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Finish_AssociateIdentity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_DisassociateIdentity<P0, P1>(&self, hwndparent: P0, lpszuniqueid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_DisassociateIdentity)(::windows::core::Vtable::as_raw(self), hwndparent.into(), lpszuniqueid.into().abi()).ok()
    }
    pub unsafe fn Finish_DisassociateIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_DisassociateIdentity)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_ChangeCredential<P0, P1>(&self, hwndparent: P0, lpszuniqueid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_ChangeCredential)(::windows::core::Vtable::as_raw(self), hwndparent.into(), lpszuniqueid.into().abi()).ok()
    }
    pub unsafe fn Finish_ChangeCredential(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_ChangeCredential)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(AsyncIAssociatedIdentityProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIAssociatedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for AsyncIAssociatedIdentityProvider {
    type Vtable = AsyncIAssociatedIdentityProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIAssociatedIdentityProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2834d6ed_297e_4e72_8a51_961e86f05152);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAssociatedIdentityProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_AssociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_AssociateIdentity: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_AssociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_AssociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_DisassociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_DisassociateIdentity: usize,
    pub Finish_DisassociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_ChangeCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_ChangeCredential: usize,
    pub Finish_ChangeCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIConnectedIdentityProvider(::windows::core::IUnknown);
impl AsyncIConnectedIdentityProvider {
    pub unsafe fn Begin_ConnectIdentity(&self, authbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_ConnectIdentity)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(authbuffer.as_ptr()), authbuffer.len() as _).ok()
    }
    pub unsafe fn Finish_ConnectIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_ConnectIdentity)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Begin_DisconnectIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_DisconnectIdentity)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Finish_DisconnectIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_DisconnectIdentity)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Begin_IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_IsConnected)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Finish_IsConnected(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Finish_IsConnected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Begin_GetUrl<P0>(&self, identifier: IDENTITY_URL, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::super::System::Com::IBindCtx>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_GetUrl)(::windows::core::Vtable::as_raw(self), identifier, context.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Finish_GetUrl(&self, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_GetUrl)(::windows::core::Vtable::as_raw(self), postdata, url).ok()
    }
    pub unsafe fn Begin_GetAccountState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_GetAccountState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Finish_GetAccountState(&self) -> ::windows::core::Result<ACCOUNT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Finish_GetAccountState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(AsyncIConnectedIdentityProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIConnectedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for AsyncIConnectedIdentityProvider {
    type Vtable = AsyncIConnectedIdentityProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIConnectedIdentityProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ce55141_bce9_4e15_824d_43d79f512f93);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIConnectedIdentityProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_ConnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT,
    pub Finish_ConnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_DisconnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_DisconnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Finish_IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Finish_IsConnected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Begin_GetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Begin_GetUrl: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Finish_GetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Finish_GetUrl: usize,
    pub Begin_GetAccountState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_GetAccountState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityAdvise(::windows::core::IUnknown);
impl AsyncIIdentityAdvise {
    pub unsafe fn Begin_IdentityUpdated<P0>(&self, dwidentityupdateevents: u32, lpszuniqueid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_IdentityUpdated)(::windows::core::Vtable::as_raw(self), dwidentityupdateevents, lpszuniqueid.into().abi()).ok()
    }
    pub unsafe fn Finish_IdentityUpdated(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_IdentityUpdated)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(AsyncIIdentityAdvise, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIIdentityAdvise {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for AsyncIIdentityAdvise {
    type Vtable = AsyncIIdentityAdvise_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIIdentityAdvise {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ab4c8da_d038_4830_8dd9_3253c55a127f);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityAdvise_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_IdentityUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Finish_IdentityUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityAuthentication(::windows::core::IUnknown);
impl AsyncIIdentityAuthentication {
    pub unsafe fn Begin_SetIdentityCredential(&self, credbuffer: ::core::option::Option<&[u8]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_SetIdentityCredential)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(credbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), credbuffer.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn Finish_SetIdentityCredential(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_SetIdentityCredential)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Begin_ValidateIdentityCredential(&self, credbuffer: &[u8], ppidentityproperties: ::core::option::Option<*mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_ValidateIdentityCredential)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(credbuffer.as_ptr()), credbuffer.len() as _, ::core::mem::transmute(ppidentityproperties.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_ValidateIdentityCredential(&self, ppidentityproperties: ::core::option::Option<*mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_ValidateIdentityCredential)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppidentityproperties.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(AsyncIIdentityAuthentication, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIIdentityAuthentication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for AsyncIIdentityAuthentication {
    type Vtable = AsyncIIdentityAuthentication_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIIdentityAuthentication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9a2f918_feca_4e9c_9633_61cbf13ed34d);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityAuthentication_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_SetIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT,
    pub Finish_SetIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Begin_ValidateIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Begin_ValidateIdentityCredential: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_ValidateIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_ValidateIdentityCredential: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityProvider(::windows::core::IUnknown);
impl AsyncIIdentityProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Begin_GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: ::core::option::Option<*const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>, pfilterpropvarvalue: ::core::option::Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_GetIdentityEnum)(::windows::core::Vtable::as_raw(self), eidentitytype, ::core::mem::transmute(pfilterkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfilterpropvarvalue.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Finish_GetIdentityEnum(&self) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Finish_GetIdentityEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_Create<P0>(&self, lpszusername: P0, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_Create)(::windows::core::Vtable::as_raw(self), lpszusername.into().abi(), pkeywordstoadd).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_Create(&self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Finish_Create)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Begin_Import<P0>(&self, ppropertystore: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_Import)(::windows::core::Vtable::as_raw(self), ppropertystore.into().abi()).ok()
    }
    pub unsafe fn Finish_Import(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_Import)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_Delete<P0>(&self, lpszuniqueid: P0, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_Delete)(::windows::core::Vtable::as_raw(self), lpszuniqueid.into().abi(), pkeywordstodelete).ok()
    }
    pub unsafe fn Finish_Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Begin_FindByUniqueID<P0>(&self, lpszuniqueid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_FindByUniqueID)(::windows::core::Vtable::as_raw(self), lpszuniqueid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_FindByUniqueID(&self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Finish_FindByUniqueID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Begin_GetProviderPropertyStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_GetProviderPropertyStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_GetProviderPropertyStore(&self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Finish_GetProviderPropertyStore)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Begin_Advise<P0>(&self, pidentityadvise: P0, dwidentityupdateevents: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IIdentityAdvise>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_Advise)(::windows::core::Vtable::as_raw(self), pidentityadvise.into().abi(), dwidentityupdateevents).ok()
    }
    pub unsafe fn Finish_Advise(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Finish_Advise)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Begin_UnAdvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_UnAdvise)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn Finish_UnAdvise(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_UnAdvise)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(AsyncIIdentityProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for AsyncIIdentityProvider {
    type Vtable = AsyncIIdentityProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIIdentityProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6fc9901_c433_4646_8f48_4e4687aae2a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Begin_GetIdentityEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Begin_GetIdentityEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Finish_GetIdentityEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Finish_GetIdentityEnum: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Begin_Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszusername: ::windows::core::PCWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Begin_Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Begin_Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertystore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Begin_Import: usize,
    pub Finish_Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Begin_Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Begin_Delete: usize,
    pub Finish_Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_FindByUniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_FindByUniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_FindByUniqueID: usize,
    pub Begin_GetProviderPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_GetProviderPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_GetProviderPropertyStore: usize,
    pub Begin_Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentityadvise: *mut ::core::ffi::c_void, dwidentityupdateevents: u32) -> ::windows::core::HRESULT,
    pub Finish_Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub Begin_UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub Finish_UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityStore(::windows::core::IUnknown);
impl AsyncIIdentityStore {
    pub unsafe fn Begin_GetCount(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_GetCount)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Finish_GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Finish_GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Begin_GetAt(&self, dwprovider: u32, pprovguid: ::core::option::Option<*mut ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_GetAt)(::windows::core::Vtable::as_raw(self), dwprovider, ::core::mem::transmute(pprovguid.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Finish_GetAt(&self, pprovguid: ::core::option::Option<*mut ::windows::core::GUID>, ppidentityprovider: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_GetAt)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pprovguid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppidentityprovider)).ok()
    }
    pub unsafe fn Begin_AddToCache<P0>(&self, lpszuniqueid: P0, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_AddToCache)(::windows::core::Vtable::as_raw(self), lpszuniqueid.into().abi(), providerguid).ok()
    }
    pub unsafe fn Finish_AddToCache(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_AddToCache)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Begin_ConvertToSid<P0>(&self, lpszuniqueid: P0, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: ::core::option::Option<*mut u8>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_ConvertToSid)(::windows::core::Vtable::as_raw(self), lpszuniqueid.into().abi(), providerguid, cbsid, ::core::mem::transmute(psid.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Finish_ConvertToSid(&self, psid: ::core::option::Option<*mut u8>, pcbrequiredsid: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_ConvertToSid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psid.unwrap_or(::std::ptr::null_mut())), pcbrequiredsid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Begin_EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: ::core::option::Option<*const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>, pfilterpropvarvalue: ::core::option::Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_EnumerateIdentities)(::windows::core::Vtable::as_raw(self), eidentitytype, ::core::mem::transmute(pfilterkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfilterpropvarvalue.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Finish_EnumerateIdentities(&self) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Finish_EnumerateIdentities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Begin_Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Begin_Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Finish_Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(AsyncIIdentityStore, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIIdentityStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for AsyncIIdentityStore {
    type Vtable = AsyncIIdentityStore_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIIdentityStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeefa1616_48de_4872_aa64_6e6206535a51);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityStore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT,
    pub Begin_GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Finish_GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_AddToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Finish_AddToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_ConvertToSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8) -> ::windows::core::HRESULT,
    pub Finish_ConvertToSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Begin_EnumerateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Begin_EnumerateIdentities: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Finish_EnumerateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Finish_EnumerateIdentities: usize,
    pub Begin_Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct AsyncIIdentityStoreEx(::windows::core::IUnknown);
impl AsyncIIdentityStoreEx {
    pub unsafe fn Begin_CreateConnectedIdentity<P0, P1>(&self, localname: P0, connectedname: P1, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_CreateConnectedIdentity)(::windows::core::Vtable::as_raw(self), localname.into().abi(), connectedname.into().abi(), providerguid).ok()
    }
    pub unsafe fn Finish_CreateConnectedIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_CreateConnectedIdentity)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Begin_DeleteConnectedIdentity<P0>(&self, connectedname: P0, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Begin_DeleteConnectedIdentity)(::windows::core::Vtable::as_raw(self), connectedname.into().abi(), providerguid).ok()
    }
    pub unsafe fn Finish_DeleteConnectedIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish_DeleteConnectedIdentity)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(AsyncIIdentityStoreEx, ::windows::core::IUnknown);
impl ::core::clone::Clone for AsyncIIdentityStoreEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for AsyncIIdentityStoreEx {
    type Vtable = AsyncIIdentityStoreEx_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncIIdentityStoreEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfca3af9a_8a07_4eae_8632_ec3de658a36a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityStoreEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_CreateConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localname: ::windows::core::PCWSTR, connectedname: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Finish_CreateConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectedname: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Finish_DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IAssociatedIdentityProvider(::windows::core::IUnknown);
impl IAssociatedIdentityProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn AssociateIdentity<P0>(&self, hwndparent: P0) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>
    where
        P0: ::std::convert::Into<super::super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AssociateIdentity)(::windows::core::Vtable::as_raw(self), hwndparent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisassociateIdentity<P0, P1>(&self, hwndparent: P0, lpszuniqueid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DisassociateIdentity)(::windows::core::Vtable::as_raw(self), hwndparent.into(), lpszuniqueid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeCredential<P0, P1>(&self, hwndparent: P0, lpszuniqueid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).ChangeCredential)(::windows::core::Vtable::as_raw(self), hwndparent.into(), lpszuniqueid.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IAssociatedIdentityProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAssociatedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAssociatedIdentityProvider {
    type Vtable = IAssociatedIdentityProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IAssociatedIdentityProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2af066b3_4cbb_4cba_a798_204b6af68cc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssociatedIdentityProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub AssociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    AssociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisassociateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisassociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangeCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangeCredential: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IConnectedIdentityProvider(::windows::core::IUnknown);
impl IConnectedIdentityProvider {
    pub unsafe fn ConnectIdentity(&self, authbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ConnectIdentity)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(authbuffer.as_ptr()), authbuffer.len() as _).ok()
    }
    pub unsafe fn DisconnectIdentity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DisconnectIdentity)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsConnected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetUrl<P0>(&self, identifier: IDENTITY_URL, context: P0, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::super::System::Com::IBindCtx>>,
    {
        (::windows::core::Vtable::vtable(self).GetUrl)(::windows::core::Vtable::as_raw(self), identifier, context.into().abi(), postdata, url).ok()
    }
    pub unsafe fn GetAccountState(&self) -> ::windows::core::Result<ACCOUNT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAccountState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IConnectedIdentityProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IConnectedIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IConnectedIdentityProvider {
    type Vtable = IConnectedIdentityProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IConnectedIdentityProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7417b54_e08c_429b_96c8_678d1369ecb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedIdentityProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> ::windows::core::HRESULT,
    pub DisconnectIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: IDENTITY_URL, context: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetUrl: usize,
    pub GetAccountState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityAdvise(::windows::core::IUnknown);
impl IIdentityAdvise {
    pub unsafe fn IdentityUpdated<P0>(&self, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).IdentityUpdated)(::windows::core::Vtable::as_raw(self), dwidentityupdateevents, lpszuniqueid.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IIdentityAdvise, ::windows::core::IUnknown);
impl ::core::clone::Clone for IIdentityAdvise {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IIdentityAdvise {
    type Vtable = IIdentityAdvise_Vtbl;
}
unsafe impl ::windows::core::Interface for IIdentityAdvise {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e982fed_d14b_440c_b8d6_bb386453d386);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityAdvise_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub IdentityUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityAuthentication(::windows::core::IUnknown);
impl IIdentityAuthentication {
    pub unsafe fn SetIdentityCredential(&self, credbuffer: ::core::option::Option<&[u8]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetIdentityCredential)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(credbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), credbuffer.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn ValidateIdentityCredential(&self, credbuffer: &[u8], ppidentityproperties: ::core::option::Option<*mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ValidateIdentityCredential)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(credbuffer.as_ptr()), credbuffer.len() as _, ::core::mem::transmute(ppidentityproperties.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(IIdentityAuthentication, ::windows::core::IUnknown);
impl ::core::clone::Clone for IIdentityAuthentication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IIdentityAuthentication {
    type Vtable = IIdentityAuthentication_Vtbl;
}
unsafe impl ::windows::core::Interface for IIdentityAuthentication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e7ef254_979f_43b5_b74e_06e4eb7df0f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityAuthentication_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub ValidateIdentityCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    ValidateIdentityCredential: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityProvider(::windows::core::IUnknown);
impl IIdentityProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: ::core::option::Option<*const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>, pfilterpropvarvalue: ::core::option::Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIdentityEnum)(::windows::core::Vtable::as_raw(self), eidentitytype, ::core::mem::transmute(pfilterkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfilterpropvarvalue.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Create<P0>(&self, lpszusername: P0, pppropertystore: *mut ::core::option::Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Create)(::windows::core::Vtable::as_raw(self), lpszusername.into().abi(), ::core::mem::transmute(pppropertystore), pkeywordstoadd).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Import<P0>(&self, ppropertystore: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    {
        (::windows::core::Vtable::vtable(self).Import)(::windows::core::Vtable::as_raw(self), ppropertystore.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Delete<P0>(&self, lpszuniqueid: P0, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self), lpszuniqueid.into().abi(), pkeywordstodelete).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn FindByUniqueID<P0>(&self, lpszuniqueid: P0) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindByUniqueID)(::windows::core::Vtable::as_raw(self), lpszuniqueid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetProviderPropertyStore(&self) -> ::windows::core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProviderPropertyStore)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Advise<P0>(&self, pidentityadvise: P0, dwidentityupdateevents: IdentityUpdateEvent) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IIdentityAdvise>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Advise)(::windows::core::Vtable::as_raw(self), pidentityadvise.into().abi(), dwidentityupdateevents, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnAdvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnAdvise)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
}
::windows::core::interface_hierarchy!(IIdentityProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IIdentityProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IIdentityProvider {
    type Vtable = IIdentityProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IIdentityProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d1b9e0c_e8ba_4f55_a81b_bce934b948f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetIdentityEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetIdentityEnum: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszusername: ::windows::core::PCWSTR, pppropertystore: *mut *mut ::core::ffi::c_void, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropertystore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Import: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Delete: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub FindByUniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    FindByUniqueID: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProviderPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProviderPropertyStore: usize,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentityadvise: *mut ::core::ffi::c_void, dwidentityupdateevents: IdentityUpdateEvent, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityStore(::windows::core::IUnknown);
impl IIdentityStore {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, dwprovider: u32, pprovguid: ::core::option::Option<*mut ::windows::core::GUID>, ppidentityprovider: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), dwprovider, ::core::mem::transmute(pprovguid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppidentityprovider)).ok()
    }
    pub unsafe fn AddToCache<P0>(&self, lpszuniqueid: P0, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).AddToCache)(::windows::core::Vtable::as_raw(self), lpszuniqueid.into().abi(), providerguid).ok()
    }
    pub unsafe fn ConvertToSid<P0>(&self, lpszuniqueid: P0, providerguid: *const ::windows::core::GUID, psid: ::core::option::Option<&mut [u8]>, pcbrequiredsid: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).ConvertToSid)(::windows::core::Vtable::as_raw(self), lpszuniqueid.into().abi(), providerguid, psid.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(psid.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcbrequiredsid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: ::core::option::Option<*const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>, pfilterpropvarvalue: ::core::option::Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateIdentities)(::windows::core::Vtable::as_raw(self), eidentitytype, ::core::mem::transmute(pfilterkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfilterpropvarvalue.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IIdentityStore, ::windows::core::IUnknown);
impl ::core::clone::Clone for IIdentityStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IIdentityStore {
    type Vtable = IIdentityStore_Vtbl;
}
unsafe impl ::windows::core::Interface for IIdentityStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf586fa5_6f35_44f1_b209_b38e169772eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityStore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwproviders: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprovider: u32, pprovguid: *mut ::windows::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ConvertToSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszuniqueid: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub EnumerateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    EnumerateIdentities: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
pub struct IIdentityStoreEx(::windows::core::IUnknown);
impl IIdentityStoreEx {
    pub unsafe fn CreateConnectedIdentity<P0, P1>(&self, localname: P0, connectedname: P1, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).CreateConnectedIdentity)(::windows::core::Vtable::as_raw(self), localname.into().abi(), connectedname.into().abi(), providerguid).ok()
    }
    pub unsafe fn DeleteConnectedIdentity<P0>(&self, connectedname: P0, providerguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteConnectedIdentity)(::windows::core::Vtable::as_raw(self), connectedname.into().abi(), providerguid).ok()
    }
}
::windows::core::interface_hierarchy!(IIdentityStoreEx, ::windows::core::IUnknown);
impl ::core::clone::Clone for IIdentityStoreEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IIdentityStoreEx {
    type Vtable = IIdentityStoreEx_Vtbl;
}
unsafe impl ::windows::core::Interface for IIdentityStoreEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9f9eb98_8f7f_4e38_9577_6980114ce32b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityStoreEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localname: ::windows::core::PCWSTR, connectedname: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectedname: ::windows::core::PCWSTR, providerguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CIdentityProfileHandler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecf5bf46_e3b6_449a_b56b_43f58f867814);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CoClassIdentityStore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30d49246_d217_465f_b00b_ac9ddd652eb7);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_ASSOCIATED: ::windows::core::PCWSTR = ::windows::w!("associated");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_CONNECTED: ::windows::core::PCWSTR = ::windows::w!("connected");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_HOMEGROUP: ::windows::core::PCWSTR = ::windows::w!("homegroup");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_LOCAL: ::windows::core::PCWSTR = ::windows::w!("local");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const OID_OAssociatedIdentityProviderObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98c5a3dd_db68_4f1a_8d2b_9079cdfeaf61);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_COMPLETE_ACCOUNT: ::windows::core::PCWSTR = ::windows::w!("CompleteAccount");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_MODERN_SETTINGS_ADD_USER: ::windows::core::PCWSTR = ::windows::w!("ModernSettingsAddUser");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_NTH_USER_FIRST_AUTH: ::windows::core::PCWSTR = ::windows::w!("NthUserFirstAuth");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_OUT_OF_BOX_EXPERIENCE: ::windows::core::PCWSTR = ::windows::w!("OutOfBoxExperience");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_OUT_OF_BOX_UPGRADE_EXPERIENCE: ::windows::core::PCWSTR = ::windows::w!("OutOfBoxUpgradeExperience");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_PROPERTY_STORE: ::windows::core::PCWSTR = ::windows::w!("PropertyStore");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_USER_NAME: ::windows::core::PCWSTR = ::windows::w!("Username");
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACCOUNT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const NOT_CONNECTED: ACCOUNT_STATE = ACCOUNT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CONNECTING: ACCOUNT_STATE = ACCOUNT_STATE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CONNECT_COMPLETED: ACCOUNT_STATE = ACCOUNT_STATE(2i32);
impl ::core::marker::Copy for ACCOUNT_STATE {}
impl ::core::clone::Clone for ACCOUNT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACCOUNT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IDENTITY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITIES_ALL: IDENTITY_TYPE = IDENTITY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITIES_ME_ONLY: IDENTITY_TYPE = IDENTITY_TYPE(1i32);
impl ::core::marker::Copy for IDENTITY_TYPE {}
impl ::core::clone::Clone for IDENTITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IDENTITY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IDENTITY_URL(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CREATE_ACCOUNT_WIZARD: IDENTITY_URL = IDENTITY_URL(0i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_SIGN_IN_WIZARD: IDENTITY_URL = IDENTITY_URL(1i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CHANGE_PASSWORD_WIZARD: IDENTITY_URL = IDENTITY_URL(2i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_IFEXISTS_WIZARD: IDENTITY_URL = IDENTITY_URL(3i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_ACCOUNT_SETTINGS: IDENTITY_URL = IDENTITY_URL(4i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_RESTORE_WIZARD: IDENTITY_URL = IDENTITY_URL(5i32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CONNECT_WIZARD: IDENTITY_URL = IDENTITY_URL(6i32);
impl ::core::marker::Copy for IDENTITY_URL {}
impl ::core::clone::Clone for IDENTITY_URL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IDENTITY_URL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IdentityUpdateEvent(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_ASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(1u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DISASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(2u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_CREATED: IdentityUpdateEvent = IdentityUpdateEvent(4u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_IMPORTED: IdentityUpdateEvent = IdentityUpdateEvent(8u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DELETED: IdentityUpdateEvent = IdentityUpdateEvent(16u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_PROPCHANGED: IdentityUpdateEvent = IdentityUpdateEvent(32u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_CONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(64u32);
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DISCONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(128u32);
impl ::core::marker::Copy for IdentityUpdateEvent {}
impl ::core::clone::Clone for IdentityUpdateEvent {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IdentityUpdateEvent {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
