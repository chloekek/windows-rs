#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeInterfaceContextTable(interfacecontexttable: *const NET_INTERFACE_CONTEXT_TABLE) {
    ::windows::core::link ! ( "ondemandconnroutehelper.dll""system" fn FreeInterfaceContextTable ( interfacecontexttable : *const NET_INTERFACE_CONTEXT_TABLE ) -> ( ) );
    FreeInterfaceContextTable(interfacecontexttable)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetInterfaceContextTableForHostName<P0, P1>(hostname: P0, proxyname: P1, flags: u32, connectionprofilefilterrawdata: ::core::option::Option<&[u8]>) -> ::windows::core::Result<*mut NET_INTERFACE_CONTEXT_TABLE>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ondemandconnroutehelper.dll""system" fn GetInterfaceContextTableForHostName ( hostname : :: windows::core::PCWSTR , proxyname : :: windows::core::PCWSTR , flags : u32 , connectionprofilefilterrawdata : *const u8 , connectionprofilefilterrawdatasize : u32 , interfacecontexttable : *mut *mut NET_INTERFACE_CONTEXT_TABLE ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetInterfaceContextTableForHostName(hostname.into().abi(), proxyname.into().abi(), flags, ::core::mem::transmute(connectionprofilefilterrawdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), connectionprofilefilterrawdata.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[inline]
pub unsafe fn OnDemandGetRoutingHint<P0>(destinationhostname: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ondemandconnroutehelper.dll""system" fn OnDemandGetRoutingHint ( destinationhostname : :: windows::core::PCWSTR , interfaceindex : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OnDemandGetRoutingHint(destinationhostname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OnDemandRegisterNotification(callback: ONDEMAND_NOTIFICATION_CALLBACK, callbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    ::windows::core::link ! ( "ondemandconnroutehelper.dll""system" fn OnDemandRegisterNotification ( callback : ONDEMAND_NOTIFICATION_CALLBACK , callbackcontext : *const ::core::ffi::c_void , registrationhandle : *mut super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OnDemandRegisterNotification(callback, ::core::mem::transmute(callbackcontext.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OnDemandUnRegisterNotification<P0>(registrationhandle: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ondemandconnroutehelper.dll""system" fn OnDemandUnRegisterNotification ( registrationhandle : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    OnDemandUnRegisterNotification(registrationhandle.into()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[inline]
pub unsafe fn WcmFreeMemory(pmemory: *mut ::core::ffi::c_void) {
    ::windows::core::link ! ( "wcmapi.dll""system" fn WcmFreeMemory ( pmemory : *mut ::core::ffi::c_void ) -> ( ) );
    WcmFreeMemory(pmemory)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[inline]
pub unsafe fn WcmGetProfileList(preserved: ::core::option::Option<*mut ::core::ffi::c_void>, ppprofilelist: *mut *mut WCM_PROFILE_INFO_LIST) -> u32 {
    ::windows::core::link ! ( "wcmapi.dll""system" fn WcmGetProfileList ( preserved : *mut ::core::ffi::c_void , ppprofilelist : *mut *mut WCM_PROFILE_INFO_LIST ) -> u32 );
    WcmGetProfileList(::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut())), ppprofilelist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[inline]
pub unsafe fn WcmQueryProperty<P0>(pinterface: ::core::option::Option<*const ::windows::core::GUID>, strprofilename: P0, property: WCM_PROPERTY, preserved: ::core::option::Option<*mut ::core::ffi::c_void>, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "wcmapi.dll""system" fn WcmQueryProperty ( pinterface : *const :: windows::core::GUID , strprofilename : :: windows::core::PCWSTR , property : WCM_PROPERTY , preserved : *mut ::core::ffi::c_void , pdwdatasize : *mut u32 , ppdata : *mut *mut u8 ) -> u32 );
    WcmQueryProperty(::core::mem::transmute(pinterface.unwrap_or(::std::ptr::null())), strprofilename.into().abi(), property, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut())), pdwdatasize, ppdata)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcmSetProfileList<P0>(pprofilelist: *const WCM_PROFILE_INFO_LIST, dwposition: u32, fignoreunknownprofiles: P0, preserved: ::core::option::Option<*mut ::core::ffi::c_void>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "wcmapi.dll""system" fn WcmSetProfileList ( pprofilelist : *const WCM_PROFILE_INFO_LIST , dwposition : u32 , fignoreunknownprofiles : super::super::Foundation:: BOOL , preserved : *mut ::core::ffi::c_void ) -> u32 );
    WcmSetProfileList(pprofilelist, dwposition, fignoreunknownprofiles.into(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[inline]
pub unsafe fn WcmSetProperty<P0>(pinterface: ::core::option::Option<*const ::windows::core::GUID>, strprofilename: P0, property: WCM_PROPERTY, preserved: ::core::option::Option<*mut ::core::ffi::c_void>, pbdata: ::core::option::Option<&[u8]>) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "wcmapi.dll""system" fn WcmSetProperty ( pinterface : *const :: windows::core::GUID , strprofilename : :: windows::core::PCWSTR , property : WCM_PROPERTY , preserved : *mut ::core::ffi::c_void , dwdatasize : u32 , pbdata : *const u8 ) -> u32 );
    WcmSetProperty(::core::mem::transmute(pinterface.unwrap_or(::std::ptr::null())), strprofilename.into().abi(), property, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut())), pbdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const NET_INTERFACE_FLAG_CONNECT_IF_NEEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const NET_INTERFACE_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_API_VERSION_1_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_MAX_PROFILE_NAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WCM_CONNECTION_COST(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_UNKNOWN: WCM_CONNECTION_COST = WCM_CONNECTION_COST(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_UNRESTRICTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_FIXED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_VARIABLE: WCM_CONNECTION_COST = WCM_CONNECTION_COST(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_OVERDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(65536i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_CONGESTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(131072i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_ROAMING: WCM_CONNECTION_COST = WCM_CONNECTION_COST(262144i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_APPROACHINGDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(524288i32);
impl ::core::marker::Copy for WCM_CONNECTION_COST {}
impl ::core::clone::Clone for WCM_CONNECTION_COST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WCM_CONNECTION_COST {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WCM_CONNECTION_COST_SOURCE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_SOURCE_DEFAULT: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_SOURCE_GP: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_SOURCE_USER: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_SOURCE_OPERATOR: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(3i32);
impl ::core::marker::Copy for WCM_CONNECTION_COST_SOURCE {}
impl ::core::clone::Clone for WCM_CONNECTION_COST_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WCM_CONNECTION_COST_SOURCE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WCM_MEDIA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_unknown: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_ethernet: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_wlan: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_mbn: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_invalid: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_max: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(5i32);
impl ::core::marker::Copy for WCM_MEDIA_TYPE {}
impl ::core::clone::Clone for WCM_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WCM_MEDIA_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WCM_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_global_property_domain_policy: WCM_PROPERTY = WCM_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_global_property_minimize_policy: WCM_PROPERTY = WCM_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_global_property_roaming_policy: WCM_PROPERTY = WCM_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_global_property_powermanagement_policy: WCM_PROPERTY = WCM_PROPERTY(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_intf_property_connection_cost: WCM_PROPERTY = WCM_PROPERTY(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_intf_property_dataplan_status: WCM_PROPERTY = WCM_PROPERTY(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_intf_property_hotspot_profile: WCM_PROPERTY = WCM_PROPERTY(6i32);
impl ::core::marker::Copy for WCM_PROPERTY {}
impl ::core::clone::Clone for WCM_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WCM_PROPERTY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub struct NET_INTERFACE_CONTEXT {
    pub InterfaceIndex: u32,
    pub ConfigurationName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for NET_INTERFACE_CONTEXT {}
impl ::core::clone::Clone for NET_INTERFACE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NET_INTERFACE_CONTEXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_INTERFACE_CONTEXT_TABLE {
    pub InterfaceContextHandle: super::super::Foundation::HANDLE,
    pub NumberOfEntries: u32,
    pub InterfaceContextArray: *mut NET_INTERFACE_CONTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_INTERFACE_CONTEXT_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_INTERFACE_CONTEXT_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NET_INTERFACE_CONTEXT_TABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_BILLING_CYCLE_INFO {
    pub StartDate: super::super::Foundation::FILETIME,
    pub Duration: WCM_TIME_INTERVAL,
    pub Reset: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_BILLING_CYCLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_BILLING_CYCLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WCM_BILLING_CYCLE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub struct WCM_CONNECTION_COST_DATA {
    pub ConnectionCost: u32,
    pub CostSource: WCM_CONNECTION_COST_SOURCE,
}
impl ::core::marker::Copy for WCM_CONNECTION_COST_DATA {}
impl ::core::clone::Clone for WCM_CONNECTION_COST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WCM_CONNECTION_COST_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_DATAPLAN_STATUS {
    pub UsageData: WCM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub BillingCycle: WCM_BILLING_CYCLE_INFO,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_DATAPLAN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WCM_DATAPLAN_STATUS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_POLICY_VALUE {
    pub fValue: super::super::Foundation::BOOL,
    pub fIsGroupPolicy: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_POLICY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_POLICY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WCM_POLICY_VALUE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub struct WCM_PROFILE_INFO {
    pub strProfileName: [u16; 256],
    pub AdapterGUID: ::windows::core::GUID,
    pub Media: WCM_MEDIA_TYPE,
}
impl ::core::marker::Copy for WCM_PROFILE_INFO {}
impl ::core::clone::Clone for WCM_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WCM_PROFILE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub struct WCM_PROFILE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub ProfileInfo: [WCM_PROFILE_INFO; 1],
}
impl ::core::marker::Copy for WCM_PROFILE_INFO_LIST {}
impl ::core::clone::Clone for WCM_PROFILE_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WCM_PROFILE_INFO_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub struct WCM_TIME_INTERVAL {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl ::core::marker::Copy for WCM_TIME_INTERVAL {}
impl ::core::clone::Clone for WCM_TIME_INTERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WCM_TIME_INTERVAL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_USAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WCM_USAGE_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub type ONDEMAND_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *const ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
