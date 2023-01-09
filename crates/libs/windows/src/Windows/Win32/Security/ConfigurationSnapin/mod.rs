#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
#[repr(transparent)]
pub struct ISceSvcAttachmentData(::windows::core::IUnknown);
impl ISceSvcAttachmentData {
    pub unsafe fn GetData(&self, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetData)(::windows::core::Vtable::as_raw(self), scesvchandle, scetype, ppvdata, psceenumhandle).ok()
    }
    pub unsafe fn Initialize<P0>(&self, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: P0, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISceSvcAttachmentPersistInfo>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), lpservicename, lptemplatename, lpscesvcpersistinfo.into().abi(), pscesvchandle).ok()
    }
    pub unsafe fn FreeBuffer(&self, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FreeBuffer)(::windows::core::Vtable::as_raw(self), pvdata).ok()
    }
    pub unsafe fn CloseHandle(&self, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CloseHandle)(::windows::core::Vtable::as_raw(self), scesvchandle).ok()
    }
}
::windows::core::interface_hierarchy!(ISceSvcAttachmentData, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISceSvcAttachmentData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ISceSvcAttachmentData {
    type Vtable = ISceSvcAttachmentData_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceSvcAttachmentData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17c35fde_200d_11d1_affb_00c04fb984f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentData_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: *mut ::core::ffi::c_void, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
#[repr(transparent)]
pub struct ISceSvcAttachmentPersistInfo(::windows::core::IUnknown);
impl ISceSvcAttachmentPersistInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save(&self, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self), lptemplatename, scesvchandle, ppvdata, pboverwriteall).ok()
    }
    pub unsafe fn IsDirty(&self, lptemplatename: *mut i8) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).IsDirty)(::windows::core::Vtable::as_raw(self), lptemplatename)
    }
    pub unsafe fn FreeBuffer(&self, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FreeBuffer)(::windows::core::Vtable::as_raw(self), pvdata).ok()
    }
}
::windows::core::interface_hierarchy!(ISceSvcAttachmentPersistInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISceSvcAttachmentPersistInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ISceSvcAttachmentPersistInfo {
    type Vtable = ISceSvcAttachmentPersistInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceSvcAttachmentPersistInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d90e0d0_200d_11d1_affb_00c04fb984f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentPersistInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8) -> ::windows::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const CCF_SCESVC_ATTACHMENT: ::windows::core::PCWSTR = ::windows::w!("CCF_SCESVC_ATTACHMENT");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const CCF_SCESVC_ATTACHMENT_DATA: ::windows::core::PCWSTR = ::windows::w!("CCF_SCESVC_ATTACHMENT_DATA");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_ACCESS_DENIED: i32 = 9i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_ALREADY_RUNNING: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_BAD_FORMAT: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_BUFFER_TOO_SMALL: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_CANT_DELETE: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_EXCEPTION_IN_SERVER: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_INVALID_DATA: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_INVALID_PARAMETER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_MOD_NOT_FOUND: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_NOT_ENOUGH_RESOURCE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_NO_MAPPING: i32 = 18i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_NO_TEMPLATE_GIVEN: i32 = 17i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_OBJECT_EXIST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_OTHER_ERROR: i32 = 12i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_PREFIX_OVERFLOW: i32 = 11i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_PROFILE_NOT_FOUND: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_RECORD_NOT_FOUND: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_SERVICE_NOT_SUPPORT: i32 = 14i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_SUCCESS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_TRUST_FAIL: i32 = 19i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESVC_ENUMERATION_MAX: i32 = 100i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_ROOT_PATH: ::windows::core::PCWSTR = ::windows::w!("Software\\Microsoft\\Windows NT\\CurrentVersion\\SeCEdit");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const cNodetypeSceAnalysisServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x678050c7_1ff8_11d1_affb_00c04fb984f9);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const cNodetypeSceEventLog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce06698_4bf3_11d1_8c30_00c04fb984f9);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const cNodetypeSceTemplateServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24a7f717_1f0c_11d1_affb_00c04fb984f9);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const lstruuidNodetypeSceAnalysisServices: ::windows::core::PCWSTR = ::windows::w!("{678050c7-1ff8-11d1-affb-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const lstruuidNodetypeSceEventLog: ::windows::core::PCWSTR = ::windows::w!("{2ce06698-4bf3-11d1-8c30-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const lstruuidNodetypeSceTemplateServices: ::windows::core::PCWSTR = ::windows::w!("{24a7f717-1f0c-11d1-affb-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const struuidNodetypeSceAnalysisServices: ::windows::core::PCSTR = ::windows::s!("{678050c7-1ff8-11d1-affb-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const struuidNodetypeSceEventLog: ::windows::core::PCSTR = ::windows::s!("{2ce06698-4bf3-11d1-8c30-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const struuidNodetypeSceTemplateServices: ::windows::core::PCSTR = ::windows::s!("{24a7f717-1f0c-11d1-affb-00c04fb984f9}");
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCESVC_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcConfigurationInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcMergedPolicyInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcAnalysisInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcInternalUse: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(3i32);
impl ::core::marker::Copy for SCESVC_INFO_TYPE {}
impl ::core::clone::Clone for SCESVC_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCESVC_INFO_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCE_LOG_ERR_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_ALWAYS: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_ERROR: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_DETAIL: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(2u32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_DEBUG: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(3u32);
impl ::core::marker::Copy for SCE_LOG_ERR_LEVEL {}
impl ::core::clone::Clone for SCE_LOG_ERR_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCE_LOG_ERR_LEVEL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_ANALYSIS_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_ANALYSIS_LINE,
}
impl ::core::marker::Copy for SCESVC_ANALYSIS_INFO {}
impl ::core::clone::Clone for SCESVC_ANALYSIS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCESVC_ANALYSIS_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_ANALYSIS_LINE {
    pub Key: *mut i8,
    pub Value: *mut u8,
    pub ValueLen: u32,
}
impl ::core::marker::Copy for SCESVC_ANALYSIS_LINE {}
impl ::core::clone::Clone for SCESVC_ANALYSIS_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCESVC_ANALYSIS_LINE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCESVC_CALLBACK_INFO {
    pub sceHandle: *mut ::core::ffi::c_void,
    pub pfQueryInfo: PFSCE_QUERY_INFO,
    pub pfSetInfo: PFSCE_SET_INFO,
    pub pfFreeInfo: PFSCE_FREE_INFO,
    pub pfLogInfo: PFSCE_LOG_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCESVC_CALLBACK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCESVC_CALLBACK_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_CONFIGURATION_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_CONFIGURATION_LINE,
}
impl ::core::marker::Copy for SCESVC_CONFIGURATION_INFO {}
impl ::core::clone::Clone for SCESVC_CONFIGURATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCESVC_CONFIGURATION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_CONFIGURATION_LINE {
    pub Key: *mut i8,
    pub Value: *mut i8,
    pub ValueLen: u32,
}
impl ::core::marker::Copy for SCESVC_CONFIGURATION_LINE {}
impl ::core::clone::Clone for SCESVC_CONFIGURATION_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCESVC_CONFIGURATION_LINE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub type PFSCE_FREE_INFO = ::core::option::Option<unsafe extern "system" fn(pvserviceinfo: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub type PFSCE_LOG_INFO = ::core::option::Option<unsafe extern "system" fn(errlevel: SCE_LOG_ERR_LEVEL, win32rc: u32, perrfmt: *mut i8) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_QUERY_INFO = ::core::option::Option<unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, ppvinfo: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_SET_INFO = ::core::option::Option<unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, pvinfo: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_ConfigAnalyzeService = ::core::option::Option<unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_UpdateService = ::core::option::Option<unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO, serviceinfo: *mut SCESVC_CONFIGURATION_INFO) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
