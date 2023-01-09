#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn MultinetGetConnectionPerformanceA(lpnetresource: *const NETRESOURCEA, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn MultinetGetConnectionPerformanceA ( lpnetresource : *const NETRESOURCEA , lpnetconnectinfostruct : *mut NETCONNECTINFOSTRUCT ) -> u32 );
    MultinetGetConnectionPerformanceA(lpnetresource, lpnetconnectinfostruct)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn MultinetGetConnectionPerformanceW(lpnetresource: *const NETRESOURCEW, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn MultinetGetConnectionPerformanceW ( lpnetresource : *const NETRESOURCEW , lpnetconnectinfostruct : *mut NETCONNECTINFOSTRUCT ) -> u32 );
    MultinetGetConnectionPerformanceW(lpnetresource, lpnetconnectinfostruct)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPAddConnection<P0, P1>(lpnetresource: *const NETRESOURCEW, lppassword: P0, lpusername: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "davclnt.dll""system" fn NPAddConnection ( lpnetresource : *const NETRESOURCEW , lppassword : :: windows::core::PCWSTR , lpusername : :: windows::core::PCWSTR ) -> u32 );
    NPAddConnection(lpnetresource, lppassword.into().abi(), lpusername.into().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPAddConnection3<P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lppassword: P1, lpusername: P2, dwflags: NET_USE_CONNECT_FLAGS) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "davclnt.dll""system" fn NPAddConnection3 ( hwndowner : super::super::Foundation:: HWND , lpnetresource : *const NETRESOURCEW , lppassword : :: windows::core::PCWSTR , lpusername : :: windows::core::PCWSTR , dwflags : NET_USE_CONNECT_FLAGS ) -> u32 );
    NPAddConnection3(hwndowner.into(), lpnetresource, lppassword.into().abi(), lpusername.into().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPAddConnection4<P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lpauthbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbauthbuffer: u32, dwflags: u32, lpuseoptions: ::core::option::Option<&[u8]>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "ntlanman.dll""system" fn NPAddConnection4 ( hwndowner : super::super::Foundation:: HWND , lpnetresource : *const NETRESOURCEW , lpauthbuffer : *const ::core::ffi::c_void , cbauthbuffer : u32 , dwflags : u32 , lpuseoptions : *const u8 , cbuseoptions : u32 ) -> u32 );
    NPAddConnection4(hwndowner.into(), lpnetresource, ::core::mem::transmute(lpauthbuffer.unwrap_or(::std::ptr::null())), cbauthbuffer, dwflags, ::core::mem::transmute(lpuseoptions.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpuseoptions.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPCancelConnection<P0, P1>(lpname: P0, fforce: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "davclnt.dll""system" fn NPCancelConnection ( lpname : :: windows::core::PCWSTR , fforce : super::super::Foundation:: BOOL ) -> u32 );
    NPCancelConnection(lpname.into().abi(), fforce.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPCancelConnection2<P0, P1>(lpname: P0, fforce: P1, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ntlanman.dll""system" fn NPCancelConnection2 ( lpname : :: windows::core::PCWSTR , fforce : super::super::Foundation:: BOOL , dwflags : u32 ) -> u32 );
    NPCancelConnection2(lpname.into().abi(), fforce.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPCloseEnum<P0>(henum: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "davclnt.dll""system" fn NPCloseEnum ( henum : super::super::Foundation:: HANDLE ) -> u32 );
    NPCloseEnum(henum.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPEnumResource<P0>(henum: P0, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "davclnt.dll""system" fn NPEnumResource ( henum : super::super::Foundation:: HANDLE , lpccount : *mut u32 , lpbuffer : *mut ::core::ffi::c_void , lpbuffersize : *mut u32 ) -> u32 );
    NPEnumResource(henum.into(), lpccount, lpbuffer, lpbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPFormatNetworkName<P0>(lpremotename: P0, lpformattedname: ::windows::core::PWSTR, lpnlength: *mut u32, dwflags: NETWORK_NAME_FORMAT_FLAGS, dwavecharperline: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "davclnt.dll""system" fn NPFormatNetworkName ( lpremotename : :: windows::core::PCWSTR , lpformattedname : :: windows::core::PWSTR , lpnlength : *mut u32 , dwflags : NETWORK_NAME_FORMAT_FLAGS , dwavecharperline : u32 ) -> u32 );
    NPFormatNetworkName(lpremotename.into().abi(), ::core::mem::transmute(lpformattedname), lpnlength, dwflags, dwavecharperline)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetCaps(ndex: u32) -> u32 {
    ::windows::core::link ! ( "davclnt.dll""system" fn NPGetCaps ( ndex : u32 ) -> u32 );
    NPGetCaps(ndex)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetConnection<P0>(lplocalname: P0, lpremotename: ::windows::core::PWSTR, lpnbufferlen: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "davclnt.dll""system" fn NPGetConnection ( lplocalname : :: windows::core::PCWSTR , lpremotename : :: windows::core::PWSTR , lpnbufferlen : *mut u32 ) -> u32 );
    NPGetConnection(lplocalname.into().abi(), ::core::mem::transmute(lpremotename), lpnbufferlen)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetConnection3<P0>(lplocalname: P0, dwlevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntlanman.dll""system" fn NPGetConnection3 ( lplocalname : :: windows::core::PCWSTR , dwlevel : u32 , lpbuffer : *mut ::core::ffi::c_void , lpbuffersize : *mut u32 ) -> u32 );
    NPGetConnection3(lplocalname.into().abi(), dwlevel, lpbuffer, lpbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetConnectionPerformance<P0>(lpremotename: P0, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntlanman.dll""system" fn NPGetConnectionPerformance ( lpremotename : :: windows::core::PCWSTR , lpnetconnectinfo : *mut NETCONNECTINFOSTRUCT ) -> u32 );
    NPGetConnectionPerformance(lpremotename.into().abi(), lpnetconnectinfo)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetPersistentUseOptionsForConnection<P0>(lpremotepath: P0, lpreaduseoptions: ::core::option::Option<&[u8]>, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntlanman.dll""system" fn NPGetPersistentUseOptionsForConnection ( lpremotepath : :: windows::core::PCWSTR , lpreaduseoptions : *const u8 , cbreaduseoptions : u32 , lpwriteuseoptions : *mut u8 , lpsizewriteuseoptions : *mut u32 ) -> u32 );
    NPGetPersistentUseOptionsForConnection(lpremotepath.into().abi(), ::core::mem::transmute(lpreaduseoptions.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpreaduseoptions.as_deref().map_or(0, |slice| slice.len() as _), lpwriteuseoptions, lpsizewriteuseoptions)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetResourceInformation(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut ::windows::core::PWSTR) -> u32 {
    ::windows::core::link ! ( "davclnt.dll""system" fn NPGetResourceInformation ( lpnetresource : *const NETRESOURCEW , lpbuffer : *mut ::core::ffi::c_void , lpbuffersize : *mut u32 , lplpsystem : *mut :: windows::core::PWSTR ) -> u32 );
    NPGetResourceInformation(lpnetresource, lpbuffer, lpbuffersize, lplpsystem)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetResourceParent(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32 {
    ::windows::core::link ! ( "davclnt.dll""system" fn NPGetResourceParent ( lpnetresource : *const NETRESOURCEW , lpbuffer : *mut ::core::ffi::c_void , lpbuffersize : *mut u32 ) -> u32 );
    NPGetResourceParent(lpnetresource, lpbuffer, lpbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetUniversalName<P0>(lplocalpath: P0, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "davclnt.dll""system" fn NPGetUniversalName ( lplocalpath : :: windows::core::PCWSTR , dwinfolevel : UNC_INFO_LEVEL , lpbuffer : *mut ::core::ffi::c_void , lpbuffersize : *mut u32 ) -> u32 );
    NPGetUniversalName(lplocalpath.into().abi(), dwinfolevel, lpbuffer, lpbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetUser<P0>(lpname: P0, lpusername: ::windows::core::PWSTR, lpnbufferlen: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "davclnt.dll""system" fn NPGetUser ( lpname : :: windows::core::PCWSTR , lpusername : :: windows::core::PWSTR , lpnbufferlen : *mut u32 ) -> u32 );
    NPGetUser(lpname.into().abi(), ::core::mem::transmute(lpusername), lpnbufferlen)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPOpenEnum(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: ::core::option::Option<*const NETRESOURCEW>, lphenum: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows::core::link ! ( "davclnt.dll""system" fn NPOpenEnum ( dwscope : u32 , dwtype : u32 , dwusage : u32 , lpnetresource : *const NETRESOURCEW , lphenum : *mut super::super::Foundation:: HANDLE ) -> u32 );
    NPOpenEnum(dwscope, dwtype, dwusage, ::core::mem::transmute(lpnetresource.unwrap_or(::std::ptr::null())), lphenum)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetAddConnection2A<P0, P1>(lpnetresource: *const NETRESOURCEA, lppassword: P0, lpusername: P1, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetAddConnection2A ( lpnetresource : *const NETRESOURCEA , lppassword : :: windows::core::PCSTR , lpusername : :: windows::core::PCSTR , dwflags : u32 ) -> u32 );
    WNetAddConnection2A(lpnetresource, lppassword.into().abi(), lpusername.into().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetAddConnection2W<P0, P1>(lpnetresource: *const NETRESOURCEW, lppassword: P0, lpusername: P1, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetAddConnection2W ( lpnetresource : *const NETRESOURCEW , lppassword : :: windows::core::PCWSTR , lpusername : :: windows::core::PCWSTR , dwflags : u32 ) -> u32 );
    WNetAddConnection2W(lpnetresource, lppassword.into().abi(), lpusername.into().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetAddConnection3A<P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, lppassword: P1, lpusername: P2, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetAddConnection3A ( hwndowner : super::super::Foundation:: HWND , lpnetresource : *const NETRESOURCEA , lppassword : :: windows::core::PCSTR , lpusername : :: windows::core::PCSTR , dwflags : u32 ) -> u32 );
    WNetAddConnection3A(hwndowner.into(), lpnetresource, lppassword.into().abi(), lpusername.into().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetAddConnection3W<P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lppassword: P1, lpusername: P2, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetAddConnection3W ( hwndowner : super::super::Foundation:: HWND , lpnetresource : *const NETRESOURCEW , lppassword : :: windows::core::PCWSTR , lpusername : :: windows::core::PCWSTR , dwflags : u32 ) -> u32 );
    WNetAddConnection3W(hwndowner.into(), lpnetresource, lppassword.into().abi(), lpusername.into().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetAddConnection4A<P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: &[u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetAddConnection4A ( hwndowner : super::super::Foundation:: HWND , lpnetresource : *const NETRESOURCEA , pauthbuffer : *const ::core::ffi::c_void , cbauthbuffer : u32 , dwflags : u32 , lpuseoptions : *const u8 , cbuseoptions : u32 ) -> u32 );
    WNetAddConnection4A(hwndowner.into(), lpnetresource, pauthbuffer, cbauthbuffer, dwflags, ::core::mem::transmute(lpuseoptions.as_ptr()), lpuseoptions.len() as _)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetAddConnection4W<P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: &[u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetAddConnection4W ( hwndowner : super::super::Foundation:: HWND , lpnetresource : *const NETRESOURCEW , pauthbuffer : *const ::core::ffi::c_void , cbauthbuffer : u32 , dwflags : u32 , lpuseoptions : *const u8 , cbuseoptions : u32 ) -> u32 );
    WNetAddConnection4W(hwndowner.into(), lpnetresource, pauthbuffer, cbauthbuffer, dwflags, ::core::mem::transmute(lpuseoptions.as_ptr()), lpuseoptions.len() as _)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetAddConnectionA<P0, P1, P2>(lpremotename: P0, lppassword: P1, lplocalname: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetAddConnectionA ( lpremotename : :: windows::core::PCSTR , lppassword : :: windows::core::PCSTR , lplocalname : :: windows::core::PCSTR ) -> u32 );
    WNetAddConnectionA(lpremotename.into().abi(), lppassword.into().abi(), lplocalname.into().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetAddConnectionW<P0, P1, P2>(lpremotename: P0, lppassword: P1, lplocalname: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetAddConnectionW ( lpremotename : :: windows::core::PCWSTR , lppassword : :: windows::core::PCWSTR , lplocalname : :: windows::core::PCWSTR ) -> u32 );
    WNetAddConnectionW(lpremotename.into().abi(), lppassword.into().abi(), lplocalname.into().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetCancelConnection2A<P0, P1>(lpname: P0, dwflags: u32, fforce: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetCancelConnection2A ( lpname : :: windows::core::PCSTR , dwflags : u32 , fforce : super::super::Foundation:: BOOL ) -> u32 );
    WNetCancelConnection2A(lpname.into().abi(), dwflags, fforce.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetCancelConnection2W<P0, P1>(lpname: P0, dwflags: u32, fforce: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetCancelConnection2W ( lpname : :: windows::core::PCWSTR , dwflags : u32 , fforce : super::super::Foundation:: BOOL ) -> u32 );
    WNetCancelConnection2W(lpname.into().abi(), dwflags, fforce.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetCancelConnectionA<P0, P1>(lpname: P0, fforce: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetCancelConnectionA ( lpname : :: windows::core::PCSTR , fforce : super::super::Foundation:: BOOL ) -> u32 );
    WNetCancelConnectionA(lpname.into().abi(), fforce.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetCancelConnectionW<P0, P1>(lpname: P0, fforce: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetCancelConnectionW ( lpname : :: windows::core::PCWSTR , fforce : super::super::Foundation:: BOOL ) -> u32 );
    WNetCancelConnectionW(lpname.into().abi(), fforce.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetCloseEnum<P0>(henum: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetCloseEnum ( henum : super::super::Foundation:: HANDLE ) -> u32 );
    WNetCloseEnum(henum.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetConnectionDialog<P0>(hwnd: P0, dwtype: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetConnectionDialog ( hwnd : super::super::Foundation:: HWND , dwtype : u32 ) -> u32 );
    WNetConnectionDialog(hwnd.into(), dwtype)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetConnectionDialog1A(lpconndlgstruct: *mut CONNECTDLGSTRUCTA) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetConnectionDialog1A ( lpconndlgstruct : *mut CONNECTDLGSTRUCTA ) -> u32 );
    WNetConnectionDialog1A(lpconndlgstruct)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetConnectionDialog1W(lpconndlgstruct: *mut CONNECTDLGSTRUCTW) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetConnectionDialog1W ( lpconndlgstruct : *mut CONNECTDLGSTRUCTW ) -> u32 );
    WNetConnectionDialog1W(lpconndlgstruct)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetDisconnectDialog<P0>(hwnd: P0, dwtype: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetDisconnectDialog ( hwnd : super::super::Foundation:: HWND , dwtype : u32 ) -> u32 );
    WNetDisconnectDialog(hwnd.into(), dwtype)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetDisconnectDialog1A(lpconndlgstruct: *const DISCDLGSTRUCTA) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetDisconnectDialog1A ( lpconndlgstruct : *const DISCDLGSTRUCTA ) -> u32 );
    WNetDisconnectDialog1A(lpconndlgstruct)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetDisconnectDialog1W(lpconndlgstruct: *const DISCDLGSTRUCTW) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetDisconnectDialog1W ( lpconndlgstruct : *const DISCDLGSTRUCTW ) -> u32 );
    WNetDisconnectDialog1W(lpconndlgstruct)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetEnumResourceA<P0>(henum: P0, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetEnumResourceA ( henum : super::super::Foundation:: HANDLE , lpccount : *mut u32 , lpbuffer : *mut ::core::ffi::c_void , lpbuffersize : *mut u32 ) -> u32 );
    WNetEnumResourceA(henum.into(), lpccount, lpbuffer, lpbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetEnumResourceW<P0>(henum: P0, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetEnumResourceW ( henum : super::super::Foundation:: HANDLE , lpccount : *mut u32 , lpbuffer : *mut ::core::ffi::c_void , lpbuffersize : *mut u32 ) -> u32 );
    WNetEnumResourceW(henum.into(), lpccount, lpbuffer, lpbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetConnectionA<P0>(lplocalname: P0, lpremotename: ::windows::core::PSTR, lpnlength: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetConnectionA ( lplocalname : :: windows::core::PCSTR , lpremotename : :: windows::core::PSTR , lpnlength : *mut u32 ) -> u32 );
    WNetGetConnectionA(lplocalname.into().abi(), ::core::mem::transmute(lpremotename), lpnlength)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetConnectionW<P0>(lplocalname: P0, lpremotename: ::windows::core::PWSTR, lpnlength: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetConnectionW ( lplocalname : :: windows::core::PCWSTR , lpremotename : :: windows::core::PWSTR , lpnlength : *mut u32 ) -> u32 );
    WNetGetConnectionW(lplocalname.into().abi(), ::core::mem::transmute(lpremotename), lpnlength)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetLastErrorA(lperror: *mut u32, lperrorbuf: &mut [u8], lpnamebuf: &mut [u8]) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetLastErrorA ( lperror : *mut u32 , lperrorbuf : :: windows::core::PSTR , nerrorbufsize : u32 , lpnamebuf : :: windows::core::PSTR , nnamebufsize : u32 ) -> u32 );
    WNetGetLastErrorA(lperror, ::core::mem::transmute(lperrorbuf.as_ptr()), lperrorbuf.len() as _, ::core::mem::transmute(lpnamebuf.as_ptr()), lpnamebuf.len() as _)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetLastErrorW(lperror: *mut u32, lperrorbuf: &mut [u16], lpnamebuf: &mut [u16]) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetLastErrorW ( lperror : *mut u32 , lperrorbuf : :: windows::core::PWSTR , nerrorbufsize : u32 , lpnamebuf : :: windows::core::PWSTR , nnamebufsize : u32 ) -> u32 );
    WNetGetLastErrorW(lperror, ::core::mem::transmute(lperrorbuf.as_ptr()), lperrorbuf.len() as _, ::core::mem::transmute(lpnamebuf.as_ptr()), lpnamebuf.len() as _)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetGetNetworkInformationA<P0>(lpprovider: P0, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetNetworkInformationA ( lpprovider : :: windows::core::PCSTR , lpnetinfostruct : *mut NETINFOSTRUCT ) -> u32 );
    WNetGetNetworkInformationA(lpprovider.into().abi(), lpnetinfostruct)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetGetNetworkInformationW<P0>(lpprovider: P0, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetNetworkInformationW ( lpprovider : :: windows::core::PCWSTR , lpnetinfostruct : *mut NETINFOSTRUCT ) -> u32 );
    WNetGetNetworkInformationW(lpprovider.into().abi(), lpnetinfostruct)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetProviderNameA(dwnettype: u32, lpprovidername: ::windows::core::PSTR, lpbuffersize: *mut u32) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetProviderNameA ( dwnettype : u32 , lpprovidername : :: windows::core::PSTR , lpbuffersize : *mut u32 ) -> u32 );
    WNetGetProviderNameA(dwnettype, ::core::mem::transmute(lpprovidername), lpbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetProviderNameW(dwnettype: u32, lpprovidername: ::windows::core::PWSTR, lpbuffersize: *mut u32) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetProviderNameW ( dwnettype : u32 , lpprovidername : :: windows::core::PWSTR , lpbuffersize : *mut u32 ) -> u32 );
    WNetGetProviderNameW(dwnettype, ::core::mem::transmute(lpprovidername), lpbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetResourceInformationA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut ::windows::core::PSTR) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetResourceInformationA ( lpnetresource : *const NETRESOURCEA , lpbuffer : *mut ::core::ffi::c_void , lpcbbuffer : *mut u32 , lplpsystem : *mut :: windows::core::PSTR ) -> u32 );
    WNetGetResourceInformationA(lpnetresource, lpbuffer, lpcbbuffer, lplpsystem)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetResourceInformationW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut ::windows::core::PWSTR) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetResourceInformationW ( lpnetresource : *const NETRESOURCEW , lpbuffer : *mut ::core::ffi::c_void , lpcbbuffer : *mut u32 , lplpsystem : *mut :: windows::core::PWSTR ) -> u32 );
    WNetGetResourceInformationW(lpnetresource, lpbuffer, lpcbbuffer, lplpsystem)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetResourceParentA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetResourceParentA ( lpnetresource : *const NETRESOURCEA , lpbuffer : *mut ::core::ffi::c_void , lpcbbuffer : *mut u32 ) -> u32 );
    WNetGetResourceParentA(lpnetresource, lpbuffer, lpcbbuffer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetResourceParentW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetResourceParentW ( lpnetresource : *const NETRESOURCEW , lpbuffer : *mut ::core::ffi::c_void , lpcbbuffer : *mut u32 ) -> u32 );
    WNetGetResourceParentW(lpnetresource, lpbuffer, lpcbbuffer)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetUniversalNameA<P0>(lplocalpath: P0, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetUniversalNameA ( lplocalpath : :: windows::core::PCSTR , dwinfolevel : UNC_INFO_LEVEL , lpbuffer : *mut ::core::ffi::c_void , lpbuffersize : *mut u32 ) -> u32 );
    WNetGetUniversalNameA(lplocalpath.into().abi(), dwinfolevel, lpbuffer, lpbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetUniversalNameW<P0>(lplocalpath: P0, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetUniversalNameW ( lplocalpath : :: windows::core::PCWSTR , dwinfolevel : UNC_INFO_LEVEL , lpbuffer : *mut ::core::ffi::c_void , lpbuffersize : *mut u32 ) -> u32 );
    WNetGetUniversalNameW(lplocalpath.into().abi(), dwinfolevel, lpbuffer, lpbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetUserA<P0>(lpname: P0, lpusername: ::windows::core::PSTR, lpnlength: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetUserA ( lpname : :: windows::core::PCSTR , lpusername : :: windows::core::PSTR , lpnlength : *mut u32 ) -> u32 );
    WNetGetUserA(lpname.into().abi(), ::core::mem::transmute(lpusername), lpnlength)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetUserW<P0>(lpname: P0, lpusername: ::windows::core::PWSTR, lpnlength: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetGetUserW ( lpname : :: windows::core::PCWSTR , lpusername : :: windows::core::PWSTR , lpnlength : *mut u32 ) -> u32 );
    WNetGetUserW(lpname.into().abi(), ::core::mem::transmute(lpusername), lpnlength)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetOpenEnumA(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: ::core::option::Option<*const NETRESOURCEA>, lphenum: *mut NetEnumHandle) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetOpenEnumA ( dwscope : NET_RESOURCE_SCOPE , dwtype : NET_RESOURCE_TYPE , dwusage : WNET_OPEN_ENUM_USAGE , lpnetresource : *const NETRESOURCEA , lphenum : *mut NetEnumHandle ) -> u32 );
    WNetOpenEnumA(dwscope, dwtype, dwusage, ::core::mem::transmute(lpnetresource.unwrap_or(::std::ptr::null())), lphenum)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetOpenEnumW(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: ::core::option::Option<*const NETRESOURCEW>, lphenum: *mut NetEnumHandle) -> u32 {
    ::windows::core::link ! ( "mpr.dll""system" fn WNetOpenEnumW ( dwscope : NET_RESOURCE_SCOPE , dwtype : NET_RESOURCE_TYPE , dwusage : WNET_OPEN_ENUM_USAGE , lpnetresource : *const NETRESOURCEW , lphenum : *mut NetEnumHandle ) -> u32 );
    WNetOpenEnumW(dwscope, dwtype, dwusage, ::core::mem::transmute(lpnetresource.unwrap_or(::std::ptr::null())), lphenum)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetSetLastErrorA<P0, P1>(err: u32, lperror: P0, lpproviders: P1)
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetSetLastErrorA ( err : u32 , lperror : :: windows::core::PCSTR , lpproviders : :: windows::core::PCSTR ) -> ( ) );
    WNetSetLastErrorA(err, lperror.into().abi(), lpproviders.into().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetSetLastErrorW<P0, P1>(err: u32, lperror: P0, lpproviders: P1)
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetSetLastErrorW ( err : u32 , lperror : :: windows::core::PCWSTR , lpproviders : :: windows::core::PCWSTR ) -> ( ) );
    WNetSetLastErrorW(err, lperror.into().abi(), lpproviders.into().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetUseConnection4A<P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, pauthbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbauthbuffer: u32, dwflags: u32, lpuseoptions: ::core::option::Option<&[u8]>, lpaccessname: ::windows::core::PSTR, lpbuffersize: ::core::option::Option<*mut u32>, lpresult: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetUseConnection4A ( hwndowner : super::super::Foundation:: HWND , lpnetresource : *const NETRESOURCEA , pauthbuffer : *const ::core::ffi::c_void , cbauthbuffer : u32 , dwflags : u32 , lpuseoptions : *const u8 , cbuseoptions : u32 , lpaccessname : :: windows::core::PSTR , lpbuffersize : *mut u32 , lpresult : *mut u32 ) -> u32 );
    WNetUseConnection4A(hwndowner.into(), lpnetresource, ::core::mem::transmute(pauthbuffer.unwrap_or(::std::ptr::null())), cbauthbuffer, dwflags, ::core::mem::transmute(lpuseoptions.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpuseoptions.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpaccessname), ::core::mem::transmute(lpbuffersize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpresult.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetUseConnection4W<P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, pauthbuffer: ::core::option::Option<*const ::core::ffi::c_void>, cbauthbuffer: u32, dwflags: u32, lpuseoptions: ::core::option::Option<&[u8]>, lpaccessname: ::windows::core::PWSTR, lpbuffersize: ::core::option::Option<*mut u32>, lpresult: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetUseConnection4W ( hwndowner : super::super::Foundation:: HWND , lpnetresource : *const NETRESOURCEW , pauthbuffer : *const ::core::ffi::c_void , cbauthbuffer : u32 , dwflags : u32 , lpuseoptions : *const u8 , cbuseoptions : u32 , lpaccessname : :: windows::core::PWSTR , lpbuffersize : *mut u32 , lpresult : *mut u32 ) -> u32 );
    WNetUseConnection4W(hwndowner.into(), lpnetresource, ::core::mem::transmute(pauthbuffer.unwrap_or(::std::ptr::null())), cbauthbuffer, dwflags, ::core::mem::transmute(lpuseoptions.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpuseoptions.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpaccessname), ::core::mem::transmute(lpbuffersize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpresult.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetUseConnectionA<P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, lppassword: P1, lpuserid: P2, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: ::windows::core::PSTR, lpbuffersize: ::core::option::Option<*mut u32>, lpresult: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetUseConnectionA ( hwndowner : super::super::Foundation:: HWND , lpnetresource : *const NETRESOURCEA , lppassword : :: windows::core::PCSTR , lpuserid : :: windows::core::PCSTR , dwflags : NET_USE_CONNECT_FLAGS , lpaccessname : :: windows::core::PSTR , lpbuffersize : *mut u32 , lpresult : *mut u32 ) -> u32 );
    WNetUseConnectionA(hwndowner.into(), lpnetresource, lppassword.into().abi(), lpuserid.into().abi(), dwflags, ::core::mem::transmute(lpaccessname), ::core::mem::transmute(lpbuffersize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpresult.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetUseConnectionW<P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lppassword: P1, lpuserid: P2, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: ::windows::core::PWSTR, lpbuffersize: ::core::option::Option<*mut u32>, lpresult: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mpr.dll""system" fn WNetUseConnectionW ( hwndowner : super::super::Foundation:: HWND , lpnetresource : *const NETRESOURCEW , lppassword : :: windows::core::PCWSTR , lpuserid : :: windows::core::PCWSTR , dwflags : NET_USE_CONNECT_FLAGS , lpaccessname : :: windows::core::PWSTR , lpbuffersize : *mut u32 , lpresult : *mut u32 ) -> u32 );
    WNetUseConnectionW(hwndowner.into(), lpnetresource, lppassword.into().abi(), lpuserid.into().abi(), dwflags, ::core::mem::transmute(lpaccessname), ::core::mem::transmute(lpbuffersize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpresult.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_CRED_RESET: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_CURRENT_MEDIA: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_GLOBAL_MAPPING: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_LOCALDRIVE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_NEED_DRIVE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_REFCOUNT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_REQUIRE_INTEGRITY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_REQUIRE_PRIVACY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_RESERVED: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_WRITE_THROUGH_SEMANTICS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NETPROPERTY_PERSISTENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NOTIFY_POST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NOTIFY_PRE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEDISPLAYTYPE_DIRECTORY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEDISPLAYTYPE_NETWORK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEDISPLAYTYPE_ROOT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCETYPE_RESERVED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCETYPE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_NOLOCALDEVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_RESERVED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_SIBLING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCE_RECENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNCON_DYNAMIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNCON_FORNETCARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNCON_NOTROUTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNCON_SLOWLINK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNDT_NETWORK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNDT_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNFMT_CONNECTION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNFMT_INENUM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNGETCON_CONNECTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNGETCON_DISCONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ADMIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ADM_DIRECTORYNOTIFY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ADM_GETDIRECTORYTYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CONNECTION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CONNECTION_FLAGS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_ADDCONNECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_ADDCONNECTION3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_ADDCONNECTION4: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_CANCELCONNECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_CANCELCONNECTION2: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_DEFER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_GETCONNECTIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_GETPERFORMANCE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DIALOG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_DEVICEMODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_FORMATNETWORKNAME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_GETRESOURCEINFORMATION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_GETRESOURCEPARENT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_PERMISSIONEDITOR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_PROPERTYDIALOG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_SEARCHDIALOG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DRIVER_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ENUMERATION: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ENUM_CONTEXT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ENUM_GLOBAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ENUM_LOCAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ENUM_SHAREABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_NET_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_NET_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_SPEC_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_SPEC_VERSION51: u32 = 327681u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_START: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_USER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_USR_GETUSER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_WAIT_FOR_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERMC_AUDIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERMC_OWNER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERMC_PERM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNSRCH_REFRESH_FIRST_LEVEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNTYPE_COMM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNTYPE_DRIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNTYPE_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNTYPE_PRINTER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_CREDENTIAL_CLASS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_NETWORK_CLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_NT_PASSWORD_CHANGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_PRIMARY_AUTHENT_CLASS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_SERVICE_CLASS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_VALID_LOGON_ACCOUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONNECTDLGSTRUCT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_RO_PATH: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_CONN_POINT: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_USE_MRU: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_HIDE_BOX: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_PERSIST: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_NOT_PERSIST: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(32u32);
impl ::core::marker::Copy for CONNECTDLGSTRUCT_FLAGS {}
impl ::core::clone::Clone for CONNECTDLGSTRUCT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONNECTDLGSTRUCT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISCDLGSTRUCT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const DISC_UPDATE_PROFILE: DISCDLGSTRUCT_FLAGS = DISCDLGSTRUCT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const DISC_NO_FORCE: DISCDLGSTRUCT_FLAGS = DISCDLGSTRUCT_FLAGS(64u32);
impl ::core::marker::Copy for DISCDLGSTRUCT_FLAGS {}
impl ::core::clone::Clone for DISCDLGSTRUCT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DISCDLGSTRUCT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETINFOSTRUCT_CHARACTERISTICS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NETINFO_DLL16: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NETINFO_DISKRED: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NETINFO_PRINTERRED: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(8u32);
impl ::core::marker::Copy for NETINFOSTRUCT_CHARACTERISTICS {}
impl ::core::clone::Clone for NETINFOSTRUCT_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NETINFOSTRUCT_CHARACTERISTICS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETWORK_NAME_FORMAT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNFMT_MULTILINE: NETWORK_NAME_FORMAT_FLAGS = NETWORK_NAME_FORMAT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNFMT_ABBREVIATED: NETWORK_NAME_FORMAT_FLAGS = NETWORK_NAME_FORMAT_FLAGS(2u32);
impl ::core::marker::Copy for NETWORK_NAME_FORMAT_FLAGS {}
impl ::core::clone::Clone for NETWORK_NAME_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NETWORK_NAME_FORMAT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_RESOURCE_SCOPE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCE_CONNECTED: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCE_CONTEXT: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(5u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCE_GLOBALNET: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCE_REMEMBERED: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(3u32);
impl ::core::marker::Copy for NET_RESOURCE_SCOPE {}
impl ::core::clone::Clone for NET_RESOURCE_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NET_RESOURCE_SCOPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_RESOURCE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCETYPE_ANY: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCETYPE_DISK: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCETYPE_PRINT: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(2u32);
impl ::core::marker::Copy for NET_RESOURCE_TYPE {}
impl ::core::clone::Clone for NET_RESOURCE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NET_RESOURCE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_USE_CONNECT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_INTERACTIVE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_PROMPT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_REDIRECT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_UPDATE_PROFILE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_COMMANDLINE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_CMD_SAVECRED: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_TEMPORARY: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_DEFERRED: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_UPDATE_RECENT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(2u32);
impl ::core::marker::Copy for NET_USE_CONNECT_FLAGS {}
impl ::core::clone::Clone for NET_USE_CONNECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NET_USE_CONNECT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NPDIRECTORY_NOTIFY_OPERATION(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNDN_MKDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNDN_RMDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNDN_MVDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(3u32);
impl ::core::marker::Copy for NPDIRECTORY_NOTIFY_OPERATION {}
impl ::core::clone::Clone for NPDIRECTORY_NOTIFY_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NPDIRECTORY_NOTIFY_OPERATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NP_PROPERTY_DIALOG_SELECTION(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPS_FILE: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPS_DIR: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPS_MULT: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(2u32);
impl ::core::marker::Copy for NP_PROPERTY_DIALOG_SELECTION {}
impl ::core::clone::Clone for NP_PROPERTY_DIALOG_SELECTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NP_PROPERTY_DIALOG_SELECTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UNC_INFO_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const UNIVERSAL_NAME_INFO_LEVEL: UNC_INFO_LEVEL = UNC_INFO_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const REMOTE_NAME_INFO_LEVEL: UNC_INFO_LEVEL = UNC_INFO_LEVEL(2u32);
impl ::core::marker::Copy for UNC_INFO_LEVEL {}
impl ::core::clone::Clone for UNC_INFO_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UNC_INFO_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WNET_OPEN_ENUM_USAGE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_NONE: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_CONNECTABLE: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_CONTAINER: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_ATTACHED: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_ALL: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(19u32);
impl ::core::marker::Copy for WNET_OPEN_ENUM_USAGE {}
impl ::core::clone::Clone for WNET_OPEN_ENUM_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WNET_OPEN_ENUM_USAGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WNPERM_DLG(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERM_DLG_PERM: WNPERM_DLG = WNPERM_DLG(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERM_DLG_AUDIT: WNPERM_DLG = WNPERM_DLG(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERM_DLG_OWNER: WNPERM_DLG = WNPERM_DLG(2u32);
impl ::core::marker::Copy for WNPERM_DLG {}
impl ::core::clone::Clone for WNPERM_DLG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WNPERM_DLG {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONNECTDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEA,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONNECTDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONNECTDLGSTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONNECTDLGSTRUCTA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONNECTDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEW,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONNECTDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONNECTDLGSTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONNECTDLGSTRUCTW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISCDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: ::windows::core::PSTR,
    pub lpRemoteName: ::windows::core::PSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISCDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISCDLGSTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISCDLGSTRUCTA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISCDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: ::windows::core::PWSTR,
    pub lpRemoteName: ::windows::core::PWSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISCDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISCDLGSTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISCDLGSTRUCTW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct NETCONNECTINFOSTRUCT {
    pub cbStructure: u32,
    pub dwFlags: u32,
    pub dwSpeed: u32,
    pub dwDelay: u32,
    pub dwOptDataSize: u32,
}
impl ::core::marker::Copy for NETCONNECTINFOSTRUCT {}
impl ::core::clone::Clone for NETCONNECTINFOSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NETCONNECTINFOSTRUCT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETINFOSTRUCT {
    pub cbStructure: u32,
    pub dwProviderVersion: u32,
    pub dwStatus: super::super::Foundation::WIN32_ERROR,
    pub dwCharacteristics: NETINFOSTRUCT_CHARACTERISTICS,
    pub dwHandle: usize,
    pub wNetType: u16,
    pub dwPrinters: u32,
    pub dwDrives: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETINFOSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETINFOSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NETINFOSTRUCT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct NETRESOURCEA {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: ::windows::core::PSTR,
    pub lpRemoteName: ::windows::core::PSTR,
    pub lpComment: ::windows::core::PSTR,
    pub lpProvider: ::windows::core::PSTR,
}
impl ::core::marker::Copy for NETRESOURCEA {}
impl ::core::clone::Clone for NETRESOURCEA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NETRESOURCEA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct NETRESOURCEW {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: ::windows::core::PWSTR,
    pub lpRemoteName: ::windows::core::PWSTR,
    pub lpComment: ::windows::core::PWSTR,
    pub lpProvider: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for NETRESOURCEW {}
impl ::core::clone::Clone for NETRESOURCEW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NETRESOURCEW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFYADD {
    pub hwndOwner: super::super::Foundation::HWND,
    pub NetResource: NETRESOURCEA,
    pub dwAddFlags: NET_USE_CONNECT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NOTIFYADD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NOTIFYADD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NOTIFYADD {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFYCANCEL {
    pub lpName: ::windows::core::PWSTR,
    pub lpProvider: ::windows::core::PWSTR,
    pub dwFlags: u32,
    pub fForce: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NOTIFYCANCEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NOTIFYCANCEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NOTIFYCANCEL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct NOTIFYINFO {
    pub dwNotifyStatus: u32,
    pub dwOperationStatus: u32,
    pub lpContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NOTIFYINFO {}
impl ::core::clone::Clone for NOTIFYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NOTIFYINFO {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetEnumHandle(pub isize);
impl NetEnumHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for NetEnumHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NetEnumHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NetEnumHandle {}
impl ::core::fmt::Debug for NetEnumHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetEnumHandle").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<NetEnumHandle>> for NetEnumHandle {
    fn from(optional: ::core::option::Option<NetEnumHandle>) -> NetEnumHandle {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for NetEnumHandle {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct REMOTE_NAME_INFOA {
    pub lpUniversalName: ::windows::core::PSTR,
    pub lpConnectionName: ::windows::core::PSTR,
    pub lpRemainingPath: ::windows::core::PSTR,
}
impl ::core::marker::Copy for REMOTE_NAME_INFOA {}
impl ::core::clone::Clone for REMOTE_NAME_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REMOTE_NAME_INFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct REMOTE_NAME_INFOW {
    pub lpUniversalName: ::windows::core::PWSTR,
    pub lpConnectionName: ::windows::core::PWSTR,
    pub lpRemainingPath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for REMOTE_NAME_INFOW {}
impl ::core::clone::Clone for REMOTE_NAME_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REMOTE_NAME_INFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct UNIVERSAL_NAME_INFOA {
    pub lpUniversalName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for UNIVERSAL_NAME_INFOA {}
impl ::core::clone::Clone for UNIVERSAL_NAME_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UNIVERSAL_NAME_INFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct UNIVERSAL_NAME_INFOW {
    pub lpUniversalName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for UNIVERSAL_NAME_INFOW {}
impl ::core::clone::Clone for UNIVERSAL_NAME_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UNIVERSAL_NAME_INFOW {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_AddConnectNotify = ::core::option::Option<unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpaddinfo: *const NOTIFYADD) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_CancelConnectNotify = ::core::option::Option<unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpcancelinfo: *const NOTIFYCANCEL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPAddConnection = ::core::option::Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lppassword: ::windows::core::PCWSTR, lpusername: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPAddConnection3 = ::core::option::Option<unsafe extern "system" fn(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: ::windows::core::PCWSTR, lpusername: ::windows::core::PCWSTR, dwflags: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPAddConnection4 = ::core::option::Option<unsafe extern "system" fn(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCancelConnection = ::core::option::Option<unsafe extern "system" fn(lpname: ::windows::core::PCWSTR, fforce: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCancelConnection2 = ::core::option::Option<unsafe extern "system" fn(lpname: ::windows::core::PCWSTR, fforce: super::super::Foundation::BOOL, dwflags: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCloseEnum = ::core::option::Option<unsafe extern "system" fn(henum: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPDeviceMode = ::core::option::Option<unsafe extern "system" fn(hparent: super::super::Foundation::HWND) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPDirectoryNotify = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, lpdir: ::windows::core::PCWSTR, dwoper: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPEnumResource = ::core::option::Option<unsafe extern "system" fn(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFMXEditPerm = ::core::option::Option<unsafe extern "system" fn(lpdrivename: ::windows::core::PCWSTR, hwndfmx: super::super::Foundation::HWND, ndialogtype: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPFMXGetPermCaps = ::core::option::Option<unsafe extern "system" fn(lpdrivename: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFMXGetPermHelp = ::core::option::Option<unsafe extern "system" fn(lpdrivename: ::windows::core::PCWSTR, ndialogtype: u32, fdirectory: super::super::Foundation::BOOL, lpfilenamebuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lpnhelpcontext: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPFormatNetworkName = ::core::option::Option<unsafe extern "system" fn(lpremotename: ::windows::core::PCWSTR, lpformattedname: ::windows::core::PWSTR, lpnlength: *mut u32, dwflags: u32, dwavecharperline: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetCaps = ::core::option::Option<unsafe extern "system" fn(ndex: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetConnection = ::core::option::Option<unsafe extern "system" fn(lplocalname: ::windows::core::PCWSTR, lpremotename: ::windows::core::PWSTR, lpnbufferlen: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetConnection3 = ::core::option::Option<unsafe extern "system" fn(lplocalname: ::windows::core::PCWSTR, dwlevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetConnectionPerformance = ::core::option::Option<unsafe extern "system" fn(lpremotename: ::windows::core::PCWSTR, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetDirectoryType = ::core::option::Option<unsafe extern "system" fn(lpname: ::windows::core::PCWSTR, lptype: *const i32, bflushcache: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetPersistentUseOptionsForConnection = ::core::option::Option<unsafe extern "system" fn(lpremotepath: ::windows::core::PCWSTR, lpreaduseoptions: *const u8, cbreaduseoptions: u32, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetPropertyText = ::core::option::Option<unsafe extern "system" fn(ibutton: u32, npropsel: u32, lpname: ::windows::core::PCWSTR, lpbuttonname: ::windows::core::PWSTR, nbuttonnamelen: u32, ntype: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetResourceInformation = ::core::option::Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut ::windows::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetResourceParent = ::core::option::Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetUniversalName = ::core::option::Option<unsafe extern "system" fn(lplocalpath: ::windows::core::PCWSTR, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpnbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetUser = ::core::option::Option<unsafe extern "system" fn(lpname: ::windows::core::PCWSTR, lpusername: ::windows::core::PWSTR, lpnbufferlen: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPLogonNotify = ::core::option::Option<unsafe extern "system" fn(lplogonid: *const super::super::Foundation::LUID, lpauthentinfotype: ::windows::core::PCWSTR, lpauthentinfo: *const ::core::ffi::c_void, lppreviousauthentinfotype: ::windows::core::PCWSTR, lppreviousauthentinfo: *const ::core::ffi::c_void, lpstationname: ::windows::core::PCWSTR, stationhandle: *const ::core::ffi::c_void, lplogonscript: *mut ::windows::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPOpenEnum = ::core::option::Option<unsafe extern "system" fn(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: *const NETRESOURCEW, lphenum: *mut super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPPasswordChangeNotify = ::core::option::Option<unsafe extern "system" fn(lpauthentinfotype: ::windows::core::PCWSTR, lpauthentinfo: *const ::core::ffi::c_void, lppreviousauthentinfotype: ::windows::core::PCWSTR, lppreviousauthentinfo: *const ::core::ffi::c_void, lpstationname: ::windows::core::PCWSTR, stationhandle: *const ::core::ffi::c_void, dwchangeinfo: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPPropertyDialog = ::core::option::Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, ibuttondlg: u32, npropsel: u32, lpfilename: ::windows::core::PCWSTR, ntype: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPSearchDialog = ::core::option::Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, lpnflags: *mut u32) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
