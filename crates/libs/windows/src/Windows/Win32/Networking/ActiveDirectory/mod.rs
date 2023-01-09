#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ADsBuildEnumerator<P0>(padscontainer: P0) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>
where
    P0: ::std::convert::Into<::windows::core::InParam<IADsContainer>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn ADsBuildEnumerator ( padscontainer : * mut::core::ffi::c_void , ppenumvariant : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    ADsBuildEnumerator(padscontainer.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ADsBuildVarArrayInt(lpdwobjecttypes: *mut u32, dwobjecttypes: u32, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "activeds.dll""system" fn ADsBuildVarArrayInt ( lpdwobjecttypes : *mut u32 , dwobjecttypes : u32 , pvar : *mut super::super::System::Com:: VARIANT ) -> :: windows::core::HRESULT );
    ADsBuildVarArrayInt(lpdwobjecttypes, dwobjecttypes, pvar).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ADsBuildVarArrayStr(lpppathnames: &[::windows::core::PCWSTR], pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "activeds.dll""system" fn ADsBuildVarArrayStr ( lpppathnames : *const :: windows::core::PCWSTR , dwpathnames : u32 , pvar : *mut super::super::System::Com:: VARIANT ) -> :: windows::core::HRESULT );
    ADsBuildVarArrayStr(::core::mem::transmute(lpppathnames.as_ptr()), lpppathnames.len() as _, pvar).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn ADsDecodeBinaryData<P0>(szsrcdata: P0, ppbdestdata: *mut *mut u8, pdwdestlen: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn ADsDecodeBinaryData ( szsrcdata : :: windows::core::PCWSTR , ppbdestdata : *mut *mut u8 , pdwdestlen : *mut u32 ) -> :: windows::core::HRESULT );
    ADsDecodeBinaryData(szsrcdata.into().abi(), ppbdestdata, pdwdestlen).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn ADsEncodeBinaryData(pbsrcdata: *mut u8, dwsrclen: u32, ppszdestdata: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "activeds.dll""system" fn ADsEncodeBinaryData ( pbsrcdata : *mut u8 , dwsrclen : u32 , ppszdestdata : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    ADsEncodeBinaryData(pbsrcdata, dwsrclen, ppszdestdata).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ADsEnumerateNext<P0>(penumvariant: P0, celements: u32, pvar: *mut super::super::System::Com::VARIANT, pcelementsfetched: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Ole::IEnumVARIANT>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn ADsEnumerateNext ( penumvariant : * mut::core::ffi::c_void , celements : u32 , pvar : *mut super::super::System::Com:: VARIANT , pcelementsfetched : *mut u32 ) -> :: windows::core::HRESULT );
    ADsEnumerateNext(penumvariant.into().abi(), celements, pvar, pcelementsfetched).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
#[inline]
pub unsafe fn ADsFreeEnumerator<P0>(penumvariant: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Ole::IEnumVARIANT>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn ADsFreeEnumerator ( penumvariant : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    ADsFreeEnumerator(penumvariant.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn ADsGetLastError(lperror: *mut u32, lperrorbuf: &mut [u16], lpnamebuf: &mut [u16]) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "activeds.dll""system" fn ADsGetLastError ( lperror : *mut u32 , lperrorbuf : :: windows::core::PWSTR , dwerrorbuflen : u32 , lpnamebuf : :: windows::core::PWSTR , dwnamebuflen : u32 ) -> :: windows::core::HRESULT );
    ADsGetLastError(lperror, ::core::mem::transmute(lperrorbuf.as_ptr()), lperrorbuf.len() as _, ::core::mem::transmute(lpnamebuf.as_ptr()), lpnamebuf.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn ADsGetObject<P0>(lpszpathname: P0, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn ADsGetObject ( lpszpathname : :: windows::core::PCWSTR , riid : *const :: windows::core::GUID , ppobject : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    ADsGetObject(lpszpathname.into().abi(), riid, ppobject).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn ADsOpenObject<P0, P1, P2>(lpszpathname: P0, lpszusername: P1, lpszpassword: P2, dwreserved: ADS_AUTHENTICATION_ENUM, riid: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn ADsOpenObject ( lpszpathname : :: windows::core::PCWSTR , lpszusername : :: windows::core::PCWSTR , lpszpassword : :: windows::core::PCWSTR , dwreserved : ADS_AUTHENTICATION_ENUM , riid : *const :: windows::core::GUID , ppobject : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    ADsOpenObject(lpszpathname.into().abi(), lpszusername.into().abi(), lpszpassword.into().abi(), dwreserved, riid, ppobject).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropCheckIfWritable<P0>(pwzattr: P0, pwritableattrs: *const ADS_ATTR_INFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "dsprop.dll""system" fn ADsPropCheckIfWritable ( pwzattr : :: windows::core::PCWSTR , pwritableattrs : *const ADS_ATTR_INFO ) -> super::super::Foundation:: BOOL );
    ADsPropCheckIfWritable(pwzattr.into().abi(), pwritableattrs)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ADsPropCreateNotifyObj<P0, P1>(pappthddataobj: P0, pwzadsobjname: P1, phnotifyobj: *mut super::super::Foundation::HWND) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDataObject>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "dsprop.dll""system" fn ADsPropCreateNotifyObj ( pappthddataobj : * mut::core::ffi::c_void , pwzadsobjname : :: windows::core::PCWSTR , phnotifyobj : *mut super::super::Foundation:: HWND ) -> :: windows::core::HRESULT );
    ADsPropCreateNotifyObj(pappthddataobj.into().abi(), pwzadsobjname.into().abi(), phnotifyobj).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropGetInitInfo<P0>(hnotifyobj: P0, pinitparams: *mut ADSPROPINITPARAMS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "dsprop.dll""system" fn ADsPropGetInitInfo ( hnotifyobj : super::super::Foundation:: HWND , pinitparams : *mut ADSPROPINITPARAMS ) -> super::super::Foundation:: BOOL );
    ADsPropGetInitInfo(hnotifyobj.into(), pinitparams)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropSendErrorMessage<P0>(hnotifyobj: P0, perror: *mut ADSPROPERROR) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "dsprop.dll""system" fn ADsPropSendErrorMessage ( hnotifyobj : super::super::Foundation:: HWND , perror : *mut ADSPROPERROR ) -> super::super::Foundation:: BOOL );
    ADsPropSendErrorMessage(hnotifyobj.into(), perror)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropSetHwnd<P0, P1>(hnotifyobj: P0, hpage: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "dsprop.dll""system" fn ADsPropSetHwnd ( hnotifyobj : super::super::Foundation:: HWND , hpage : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    ADsPropSetHwnd(hnotifyobj.into(), hpage.into())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropSetHwndWithTitle<P0, P1>(hnotifyobj: P0, hpage: P1, ptztitle: *const i8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "dsprop.dll""system" fn ADsPropSetHwndWithTitle ( hnotifyobj : super::super::Foundation:: HWND , hpage : super::super::Foundation:: HWND , ptztitle : *const i8 ) -> super::super::Foundation:: BOOL );
    ADsPropSetHwndWithTitle(hnotifyobj.into(), hpage.into(), ptztitle)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropShowErrorDialog<P0, P1>(hnotifyobj: P0, hpage: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "dsprop.dll""system" fn ADsPropShowErrorDialog ( hnotifyobj : super::super::Foundation:: HWND , hpage : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    ADsPropShowErrorDialog(hnotifyobj.into(), hpage.into())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn ADsSetLastError<P0, P1>(dwerr: u32, pszerror: P0, pszprovider: P1)
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn ADsSetLastError ( dwerr : u32 , pszerror : :: windows::core::PCWSTR , pszprovider : :: windows::core::PCWSTR ) -> ( ) );
    ADsSetLastError(dwerr, pszerror.into().abi(), pszprovider.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdsFreeAdsValues(padsvalues: *mut ADSVALUE, dwnumvalues: u32) {
    ::windows::core::link ! ( "activeds.dll""system" fn AdsFreeAdsValues ( padsvalues : *mut ADSVALUE , dwnumvalues : u32 ) -> ( ) );
    AdsFreeAdsValues(padsvalues, dwnumvalues)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn AdsTypeToPropVariant(padsvalues: *mut ADSVALUE, dwnumvalues: u32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "activeds.dll""system" fn AdsTypeToPropVariant ( padsvalues : *mut ADSVALUE , dwnumvalues : u32 , pvariant : *mut super::super::System::Com:: VARIANT ) -> :: windows::core::HRESULT );
    AdsTypeToPropVariant(padsvalues, dwnumvalues, pvariant).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn AllocADsMem(cb: u32) -> *mut ::core::ffi::c_void {
    ::windows::core::link ! ( "activeds.dll""system" fn AllocADsMem ( cb : u32 ) -> *mut ::core::ffi::c_void );
    AllocADsMem(cb)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn AllocADsStr<P0>(pstr: P0) -> ::windows::core::PWSTR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn AllocADsStr ( pstr : :: windows::core::PCWSTR ) -> :: windows::core::PWSTR );
    AllocADsStr(pstr.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn BinarySDToSecurityDescriptor<P0, P1, P2, P3>(psecuritydescriptor: P0, pvarsec: *mut super::super::System::Com::VARIANT, pszservername: P1, username: P2, password: P3, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn BinarySDToSecurityDescriptor ( psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR , pvarsec : *mut super::super::System::Com:: VARIANT , pszservername : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR , password : :: windows::core::PCWSTR , dwflags : u32 ) -> :: windows::core::HRESULT );
    BinarySDToSecurityDescriptor(psecuritydescriptor.into(), pvarsec, pszservername.into().abi(), username.into().abi(), password.into().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsAddSidHistoryA<P0, P1, P2, P3, P4, P5>(hds: P0, flags: u32, srcdomain: P1, srcprincipal: P2, srcdomaincontroller: P3, srcdomaincreds: ::core::option::Option<*const ::core::ffi::c_void>, dstdomain: P4, dstprincipal: P5) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsAddSidHistoryA ( hds : super::super::Foundation:: HANDLE , flags : u32 , srcdomain : :: windows::core::PCSTR , srcprincipal : :: windows::core::PCSTR , srcdomaincontroller : :: windows::core::PCSTR , srcdomaincreds : *const ::core::ffi::c_void , dstdomain : :: windows::core::PCSTR , dstprincipal : :: windows::core::PCSTR ) -> u32 );
    DsAddSidHistoryA(hds.into(), flags, srcdomain.into().abi(), srcprincipal.into().abi(), srcdomaincontroller.into().abi(), ::core::mem::transmute(srcdomaincreds.unwrap_or(::std::ptr::null())), dstdomain.into().abi(), dstprincipal.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsAddSidHistoryW<P0, P1, P2, P3, P4, P5>(hds: P0, flags: u32, srcdomain: P1, srcprincipal: P2, srcdomaincontroller: P3, srcdomaincreds: ::core::option::Option<*const ::core::ffi::c_void>, dstdomain: P4, dstprincipal: P5) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsAddSidHistoryW ( hds : super::super::Foundation:: HANDLE , flags : u32 , srcdomain : :: windows::core::PCWSTR , srcprincipal : :: windows::core::PCWSTR , srcdomaincontroller : :: windows::core::PCWSTR , srcdomaincreds : *const ::core::ffi::c_void , dstdomain : :: windows::core::PCWSTR , dstprincipal : :: windows::core::PCWSTR ) -> u32 );
    DsAddSidHistoryW(hds.into(), flags, srcdomain.into().abi(), srcprincipal.into().abi(), srcdomaincontroller.into().abi(), ::core::mem::transmute(srcdomaincreds.unwrap_or(::std::ptr::null())), dstdomain.into().abi(), dstprincipal.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DsAddressToSiteNamesA<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsAddressToSiteNamesA ( computername : :: windows::core::PCSTR , entrycount : u32 , socketaddresses : *const super::WinSock:: SOCKET_ADDRESS , sitenames : *mut *mut :: windows::core::PSTR ) -> u32 );
    DsAddressToSiteNamesA(computername.into().abi(), socketaddresses.len() as _, ::core::mem::transmute(socketaddresses.as_ptr()), sitenames)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DsAddressToSiteNamesExA<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut ::windows::core::PSTR, subnetnames: *mut *mut ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsAddressToSiteNamesExA ( computername : :: windows::core::PCSTR , entrycount : u32 , socketaddresses : *const super::WinSock:: SOCKET_ADDRESS , sitenames : *mut *mut :: windows::core::PSTR , subnetnames : *mut *mut :: windows::core::PSTR ) -> u32 );
    DsAddressToSiteNamesExA(computername.into().abi(), socketaddresses.len() as _, ::core::mem::transmute(socketaddresses.as_ptr()), sitenames, subnetnames)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DsAddressToSiteNamesExW<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut ::windows::core::PWSTR, subnetnames: *mut *mut ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsAddressToSiteNamesExW ( computername : :: windows::core::PCWSTR , entrycount : u32 , socketaddresses : *const super::WinSock:: SOCKET_ADDRESS , sitenames : *mut *mut :: windows::core::PWSTR , subnetnames : *mut *mut :: windows::core::PWSTR ) -> u32 );
    DsAddressToSiteNamesExW(computername.into().abi(), socketaddresses.len() as _, ::core::mem::transmute(socketaddresses.as_ptr()), sitenames, subnetnames)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DsAddressToSiteNamesW<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsAddressToSiteNamesW ( computername : :: windows::core::PCWSTR , entrycount : u32 , socketaddresses : *const super::WinSock:: SOCKET_ADDRESS , sitenames : *mut *mut :: windows::core::PWSTR ) -> u32 );
    DsAddressToSiteNamesW(computername.into().abi(), socketaddresses.len() as _, ::core::mem::transmute(socketaddresses.as_ptr()), sitenames)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindA<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindA ( domaincontrollername : :: windows::core::PCSTR , dnsdomainname : :: windows::core::PCSTR , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindA(domaincontrollername.into().abi(), dnsdomainname.into().abi(), phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindByInstanceA<P0, P1, P2, P3>(servername: P0, annotation: P1, instanceguid: ::core::option::Option<*const ::windows::core::GUID>, dnsdomainname: P2, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P3, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindByInstanceA ( servername : :: windows::core::PCSTR , annotation : :: windows::core::PCSTR , instanceguid : *const :: windows::core::GUID , dnsdomainname : :: windows::core::PCSTR , authidentity : *const ::core::ffi::c_void , serviceprincipalname : :: windows::core::PCSTR , bindflags : u32 , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindByInstanceA(servername.into().abi(), annotation.into().abi(), ::core::mem::transmute(instanceguid.unwrap_or(::std::ptr::null())), dnsdomainname.into().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into().abi(), bindflags, phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindByInstanceW<P0, P1, P2, P3>(servername: P0, annotation: P1, instanceguid: ::core::option::Option<*const ::windows::core::GUID>, dnsdomainname: P2, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P3, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindByInstanceW ( servername : :: windows::core::PCWSTR , annotation : :: windows::core::PCWSTR , instanceguid : *const :: windows::core::GUID , dnsdomainname : :: windows::core::PCWSTR , authidentity : *const ::core::ffi::c_void , serviceprincipalname : :: windows::core::PCWSTR , bindflags : u32 , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindByInstanceW(servername.into().abi(), annotation.into().abi(), ::core::mem::transmute(instanceguid.unwrap_or(::std::ptr::null())), dnsdomainname.into().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into().abi(), bindflags, phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindToISTGA<P0>(sitename: P0, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindToISTGA ( sitename : :: windows::core::PCSTR , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindToISTGA(sitename.into().abi(), phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindToISTGW<P0>(sitename: P0, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindToISTGW ( sitename : :: windows::core::PCWSTR , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindToISTGW(sitename.into().abi(), phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindW<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindW ( domaincontrollername : :: windows::core::PCWSTR , dnsdomainname : :: windows::core::PCWSTR , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindW(domaincontrollername.into().abi(), dnsdomainname.into().abi(), phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithCredA<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindWithCredA ( domaincontrollername : :: windows::core::PCSTR , dnsdomainname : :: windows::core::PCSTR , authidentity : *const ::core::ffi::c_void , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindWithCredA(domaincontrollername.into().abi(), dnsdomainname.into().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithCredW<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindWithCredW ( domaincontrollername : :: windows::core::PCWSTR , dnsdomainname : :: windows::core::PCWSTR , authidentity : *const ::core::ffi::c_void , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindWithCredW(domaincontrollername.into().abi(), dnsdomainname.into().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithSpnA<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P2, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindWithSpnA ( domaincontrollername : :: windows::core::PCSTR , dnsdomainname : :: windows::core::PCSTR , authidentity : *const ::core::ffi::c_void , serviceprincipalname : :: windows::core::PCSTR , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindWithSpnA(domaincontrollername.into().abi(), dnsdomainname.into().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into().abi(), phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithSpnExA<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P2, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindWithSpnExA ( domaincontrollername : :: windows::core::PCSTR , dnsdomainname : :: windows::core::PCSTR , authidentity : *const ::core::ffi::c_void , serviceprincipalname : :: windows::core::PCSTR , bindflags : u32 , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindWithSpnExA(domaincontrollername.into().abi(), dnsdomainname.into().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into().abi(), bindflags, phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithSpnExW<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P2, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindWithSpnExW ( domaincontrollername : :: windows::core::PCWSTR , dnsdomainname : :: windows::core::PCWSTR , authidentity : *const ::core::ffi::c_void , serviceprincipalname : :: windows::core::PCWSTR , bindflags : u32 , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindWithSpnExW(domaincontrollername.into().abi(), dnsdomainname.into().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into().abi(), bindflags, phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithSpnW<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P2, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindWithSpnW ( domaincontrollername : :: windows::core::PCWSTR , dnsdomainname : :: windows::core::PCWSTR , authidentity : *const ::core::ffi::c_void , serviceprincipalname : :: windows::core::PCWSTR , phds : *mut super::super::Foundation:: HANDLE ) -> u32 );
    DsBindWithSpnW(domaincontrollername.into().abi(), dnsdomainname.into().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into().abi(), phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindingSetTimeout<P0>(hds: P0, ctimeoutsecs: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsBindingSetTimeout ( hds : super::super::Foundation:: HANDLE , ctimeoutsecs : u32 ) -> u32 );
    DsBindingSetTimeout(hds.into(), ctimeoutsecs)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
#[inline]
pub unsafe fn DsBrowseForContainerA(pinfo: *mut DSBROWSEINFOA) -> i32 {
    ::windows::core::link ! ( "dsuiext.dll""system" fn DsBrowseForContainerA ( pinfo : *mut DSBROWSEINFOA ) -> i32 );
    DsBrowseForContainerA(pinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
#[inline]
pub unsafe fn DsBrowseForContainerW(pinfo: *mut DSBROWSEINFOW) -> i32 {
    ::windows::core::link ! ( "dsuiext.dll""system" fn DsBrowseForContainerW ( pinfo : *mut DSBROWSEINFOW ) -> i32 );
    DsBrowseForContainerW(pinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsClientMakeSpnForTargetServerA<P0, P1>(serviceclass: P0, servicename: P1, pcspnlength: *mut u32, pszspn: ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsClientMakeSpnForTargetServerA ( serviceclass : :: windows::core::PCSTR , servicename : :: windows::core::PCSTR , pcspnlength : *mut u32 , pszspn : :: windows::core::PSTR ) -> u32 );
    DsClientMakeSpnForTargetServerA(serviceclass.into().abi(), servicename.into().abi(), pcspnlength, ::core::mem::transmute(pszspn))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsClientMakeSpnForTargetServerW<P0, P1>(serviceclass: P0, servicename: P1, pcspnlength: *mut u32, pszspn: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsClientMakeSpnForTargetServerW ( serviceclass : :: windows::core::PCWSTR , servicename : :: windows::core::PCWSTR , pcspnlength : *mut u32 , pszspn : :: windows::core::PWSTR ) -> u32 );
    DsClientMakeSpnForTargetServerW(serviceclass.into().abi(), servicename.into().abi(), pcspnlength, ::core::mem::transmute(pszspn))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsCrackNamesA<P0>(hds: P0, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, rpnames: &[::windows::core::PCSTR], ppresult: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsCrackNamesA ( hds : super::super::Foundation:: HANDLE , flags : DS_NAME_FLAGS , formatoffered : DS_NAME_FORMAT , formatdesired : DS_NAME_FORMAT , cnames : u32 , rpnames : *const :: windows::core::PCSTR , ppresult : *mut *mut DS_NAME_RESULTA ) -> u32 );
    DsCrackNamesA(hds.into(), flags, formatoffered, formatdesired, rpnames.len() as _, ::core::mem::transmute(rpnames.as_ptr()), ppresult)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsCrackNamesW<P0>(hds: P0, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, rpnames: &[::windows::core::PCWSTR], ppresult: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsCrackNamesW ( hds : super::super::Foundation:: HANDLE , flags : DS_NAME_FLAGS , formatoffered : DS_NAME_FORMAT , formatdesired : DS_NAME_FORMAT , cnames : u32 , rpnames : *const :: windows::core::PCWSTR , ppresult : *mut *mut DS_NAME_RESULTW ) -> u32 );
    DsCrackNamesW(hds.into(), flags, formatoffered, formatdesired, rpnames.len() as _, ::core::mem::transmute(rpnames.as_ptr()), ppresult)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsCrackSpn2A(pszspn: &[u8], pcserviceclass: ::core::option::Option<*mut u32>, serviceclass: ::windows::core::PSTR, pcservicename: ::core::option::Option<*mut u32>, servicename: ::windows::core::PSTR, pcinstancename: ::core::option::Option<*mut u32>, instancename: ::windows::core::PSTR, pinstanceport: ::core::option::Option<*mut u16>) -> u32 {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsCrackSpn2A ( pszspn : :: windows::core::PCSTR , cspn : u32 , pcserviceclass : *mut u32 , serviceclass : :: windows::core::PSTR , pcservicename : *mut u32 , servicename : :: windows::core::PSTR , pcinstancename : *mut u32 , instancename : :: windows::core::PSTR , pinstanceport : *mut u16 ) -> u32 );
    DsCrackSpn2A(::core::mem::transmute(pszspn.as_ptr()), pszspn.len() as _, ::core::mem::transmute(pcserviceclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(serviceclass), ::core::mem::transmute(pcservicename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servicename), ::core::mem::transmute(pcinstancename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(instancename), ::core::mem::transmute(pinstanceport.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsCrackSpn2W(pszspn: &[u16], pcserviceclass: ::core::option::Option<*mut u32>, serviceclass: ::windows::core::PWSTR, pcservicename: ::core::option::Option<*mut u32>, servicename: ::windows::core::PWSTR, pcinstancename: ::core::option::Option<*mut u32>, instancename: ::windows::core::PWSTR, pinstanceport: ::core::option::Option<*mut u16>) -> u32 {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsCrackSpn2W ( pszspn : :: windows::core::PCWSTR , cspn : u32 , pcserviceclass : *mut u32 , serviceclass : :: windows::core::PWSTR , pcservicename : *mut u32 , servicename : :: windows::core::PWSTR , pcinstancename : *mut u32 , instancename : :: windows::core::PWSTR , pinstanceport : *mut u16 ) -> u32 );
    DsCrackSpn2W(::core::mem::transmute(pszspn.as_ptr()), pszspn.len() as _, ::core::mem::transmute(pcserviceclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(serviceclass), ::core::mem::transmute(pcservicename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servicename), ::core::mem::transmute(pcinstancename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(instancename), ::core::mem::transmute(pinstanceport.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsCrackSpn3W<P0>(pszspn: P0, cspn: u32, pchostname: *mut u32, hostname: ::windows::core::PWSTR, pcinstancename: *mut u32, instancename: ::windows::core::PWSTR, pportnumber: *mut u16, pcdomainname: *mut u32, domainname: ::windows::core::PWSTR, pcrealmname: *mut u32, realmname: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "dsparse.dll""system" fn DsCrackSpn3W ( pszspn : :: windows::core::PCWSTR , cspn : u32 , pchostname : *mut u32 , hostname : :: windows::core::PWSTR , pcinstancename : *mut u32 , instancename : :: windows::core::PWSTR , pportnumber : *mut u16 , pcdomainname : *mut u32 , domainname : :: windows::core::PWSTR , pcrealmname : *mut u32 , realmname : :: windows::core::PWSTR ) -> u32 );
    DsCrackSpn3W(pszspn.into().abi(), cspn, pchostname, ::core::mem::transmute(hostname), pcinstancename, ::core::mem::transmute(instancename), pportnumber, pcdomainname, ::core::mem::transmute(domainname), pcrealmname, ::core::mem::transmute(realmname))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsCrackSpn4W<P0>(pszspn: P0, cspn: u32, pchostname: *mut u32, hostname: ::windows::core::PWSTR, pcinstancename: *mut u32, instancename: ::windows::core::PWSTR, pcportname: *mut u32, portname: ::windows::core::PWSTR, pcdomainname: *mut u32, domainname: ::windows::core::PWSTR, pcrealmname: *mut u32, realmname: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "dsparse.dll""system" fn DsCrackSpn4W ( pszspn : :: windows::core::PCWSTR , cspn : u32 , pchostname : *mut u32 , hostname : :: windows::core::PWSTR , pcinstancename : *mut u32 , instancename : :: windows::core::PWSTR , pcportname : *mut u32 , portname : :: windows::core::PWSTR , pcdomainname : *mut u32 , domainname : :: windows::core::PWSTR , pcrealmname : *mut u32 , realmname : :: windows::core::PWSTR ) -> u32 );
    DsCrackSpn4W(pszspn.into().abi(), cspn, pchostname, ::core::mem::transmute(hostname), pcinstancename, ::core::mem::transmute(instancename), pcportname, ::core::mem::transmute(portname), pcdomainname, ::core::mem::transmute(domainname), pcrealmname, ::core::mem::transmute(realmname))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsCrackSpnA<P0>(pszspn: P0, pcserviceclass: ::core::option::Option<*mut u32>, serviceclass: ::windows::core::PSTR, pcservicename: ::core::option::Option<*mut u32>, servicename: ::windows::core::PSTR, pcinstancename: ::core::option::Option<*mut u32>, instancename: ::windows::core::PSTR, pinstanceport: ::core::option::Option<*mut u16>) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "dsparse.dll""system" fn DsCrackSpnA ( pszspn : :: windows::core::PCSTR , pcserviceclass : *mut u32 , serviceclass : :: windows::core::PSTR , pcservicename : *mut u32 , servicename : :: windows::core::PSTR , pcinstancename : *mut u32 , instancename : :: windows::core::PSTR , pinstanceport : *mut u16 ) -> u32 );
    DsCrackSpnA(pszspn.into().abi(), ::core::mem::transmute(pcserviceclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(serviceclass), ::core::mem::transmute(pcservicename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servicename), ::core::mem::transmute(pcinstancename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(instancename), ::core::mem::transmute(pinstanceport.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsCrackSpnW<P0>(pszspn: P0, pcserviceclass: ::core::option::Option<*mut u32>, serviceclass: ::windows::core::PWSTR, pcservicename: ::core::option::Option<*mut u32>, servicename: ::windows::core::PWSTR, pcinstancename: ::core::option::Option<*mut u32>, instancename: ::windows::core::PWSTR, pinstanceport: ::core::option::Option<*mut u16>) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "dsparse.dll""system" fn DsCrackSpnW ( pszspn : :: windows::core::PCWSTR , pcserviceclass : *mut u32 , serviceclass : :: windows::core::PWSTR , pcservicename : *mut u32 , servicename : :: windows::core::PWSTR , pcinstancename : *mut u32 , instancename : :: windows::core::PWSTR , pinstanceport : *mut u16 ) -> u32 );
    DsCrackSpnW(pszspn.into().abi(), ::core::mem::transmute(pcserviceclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(serviceclass), ::core::mem::transmute(pcservicename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servicename), ::core::mem::transmute(pcinstancename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(instancename), ::core::mem::transmute(pinstanceport.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsCrackUnquotedMangledRdnA(pszrdn: &[u8], pguid: ::core::option::Option<*mut ::windows::core::GUID>, pedsmanglefor: ::core::option::Option<*mut DS_MANGLE_FOR>) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsCrackUnquotedMangledRdnA ( pszrdn : :: windows::core::PCSTR , cchrdn : u32 , pguid : *mut :: windows::core::GUID , pedsmanglefor : *mut DS_MANGLE_FOR ) -> super::super::Foundation:: BOOL );
    DsCrackUnquotedMangledRdnA(::core::mem::transmute(pszrdn.as_ptr()), pszrdn.len() as _, ::core::mem::transmute(pguid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pedsmanglefor.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsCrackUnquotedMangledRdnW(pszrdn: &[u16], pguid: ::core::option::Option<*mut ::windows::core::GUID>, pedsmanglefor: ::core::option::Option<*mut DS_MANGLE_FOR>) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsCrackUnquotedMangledRdnW ( pszrdn : :: windows::core::PCWSTR , cchrdn : u32 , pguid : *mut :: windows::core::GUID , pedsmanglefor : *mut DS_MANGLE_FOR ) -> super::super::Foundation:: BOOL );
    DsCrackUnquotedMangledRdnW(::core::mem::transmute(pszrdn.as_ptr()), pszrdn.len() as _, ::core::mem::transmute(pguid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pedsmanglefor.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsDeregisterDnsHostRecordsA<P0, P1, P2>(servername: P0, dnsdomainname: P1, domainguid: ::core::option::Option<*const ::windows::core::GUID>, dsaguid: ::core::option::Option<*const ::windows::core::GUID>, dnshostname: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsDeregisterDnsHostRecordsA ( servername : :: windows::core::PCSTR , dnsdomainname : :: windows::core::PCSTR , domainguid : *const :: windows::core::GUID , dsaguid : *const :: windows::core::GUID , dnshostname : :: windows::core::PCSTR ) -> u32 );
    DsDeregisterDnsHostRecordsA(servername.into().abi(), dnsdomainname.into().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(dsaguid.unwrap_or(::std::ptr::null())), dnshostname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsDeregisterDnsHostRecordsW<P0, P1, P2>(servername: P0, dnsdomainname: P1, domainguid: ::core::option::Option<*const ::windows::core::GUID>, dsaguid: ::core::option::Option<*const ::windows::core::GUID>, dnshostname: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsDeregisterDnsHostRecordsW ( servername : :: windows::core::PCWSTR , dnsdomainname : :: windows::core::PCWSTR , domainguid : *const :: windows::core::GUID , dsaguid : *const :: windows::core::GUID , dnshostname : :: windows::core::PCWSTR ) -> u32 );
    DsDeregisterDnsHostRecordsW(servername.into().abi(), dnsdomainname.into().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(dsaguid.unwrap_or(::std::ptr::null())), dnshostname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsEnumerateDomainTrustsA<P0>(servername: P0, flags: u32, domains: *mut *mut DS_DOMAIN_TRUSTSA, domaincount: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsEnumerateDomainTrustsA ( servername : :: windows::core::PCSTR , flags : u32 , domains : *mut *mut DS_DOMAIN_TRUSTSA , domaincount : *mut u32 ) -> u32 );
    DsEnumerateDomainTrustsA(servername.into().abi(), flags, domains, domaincount)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsEnumerateDomainTrustsW<P0>(servername: P0, flags: u32, domains: *mut *mut DS_DOMAIN_TRUSTSW, domaincount: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsEnumerateDomainTrustsW ( servername : :: windows::core::PCWSTR , flags : u32 , domains : *mut *mut DS_DOMAIN_TRUSTSW , domaincount : *mut u32 ) -> u32 );
    DsEnumerateDomainTrustsW(servername.into().abi(), flags, domains, domaincount)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsFreeDomainControllerInfoA(infolevel: u32, pinfo: &[u8]) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsFreeDomainControllerInfoA ( infolevel : u32 , cinfo : u32 , pinfo : *const ::core::ffi::c_void ) -> ( ) );
    DsFreeDomainControllerInfoA(infolevel, pinfo.len() as _, ::core::mem::transmute(pinfo.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsFreeDomainControllerInfoW(infolevel: u32, pinfo: &[u8]) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsFreeDomainControllerInfoW ( infolevel : u32 , cinfo : u32 , pinfo : *const ::core::ffi::c_void ) -> ( ) );
    DsFreeDomainControllerInfoW(infolevel, pinfo.len() as _, ::core::mem::transmute(pinfo.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsFreeNameResultA(presult: *const DS_NAME_RESULTA) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsFreeNameResultA ( presult : *const DS_NAME_RESULTA ) -> ( ) );
    DsFreeNameResultA(presult)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsFreeNameResultW(presult: *const DS_NAME_RESULTW) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsFreeNameResultW ( presult : *const DS_NAME_RESULTW ) -> ( ) );
    DsFreeNameResultW(presult)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsFreePasswordCredentials(authidentity: *const ::core::ffi::c_void) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsFreePasswordCredentials ( authidentity : *const ::core::ffi::c_void ) -> ( ) );
    DsFreePasswordCredentials(authidentity)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsFreeSchemaGuidMapA(pguidmap: *const DS_SCHEMA_GUID_MAPA) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsFreeSchemaGuidMapA ( pguidmap : *const DS_SCHEMA_GUID_MAPA ) -> ( ) );
    DsFreeSchemaGuidMapA(pguidmap)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsFreeSchemaGuidMapW(pguidmap: *const DS_SCHEMA_GUID_MAPW) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsFreeSchemaGuidMapW ( pguidmap : *const DS_SCHEMA_GUID_MAPW ) -> ( ) );
    DsFreeSchemaGuidMapW(pguidmap)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsFreeSpnArrayA(rpszspn: &mut [::windows::core::PSTR]) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsFreeSpnArrayA ( cspn : u32 , rpszspn : *mut :: windows::core::PSTR ) -> ( ) );
    DsFreeSpnArrayA(rpszspn.len() as _, ::core::mem::transmute(rpszspn.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsFreeSpnArrayW(rpszspn: &mut [::windows::core::PWSTR]) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsFreeSpnArrayW ( cspn : u32 , rpszspn : *mut :: windows::core::PWSTR ) -> ( ) );
    DsFreeSpnArrayW(rpszspn.len() as _, ::core::mem::transmute(rpszspn.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetDcCloseW<P0>(getdccontexthandle: P0)
where
    P0: ::std::convert::Into<GetDcContextHandle>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetDcCloseW ( getdccontexthandle : GetDcContextHandle ) -> ( ) );
    DsGetDcCloseW(getdccontexthandle.into())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetDcNameA<P0, P1, P2>(computername: P0, domainname: P1, domainguid: ::core::option::Option<*const ::windows::core::GUID>, sitename: P2, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOA) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetDcNameA ( computername : :: windows::core::PCSTR , domainname : :: windows::core::PCSTR , domainguid : *const :: windows::core::GUID , sitename : :: windows::core::PCSTR , flags : u32 , domaincontrollerinfo : *mut *mut DOMAIN_CONTROLLER_INFOA ) -> u32 );
    DsGetDcNameA(computername.into().abi(), domainname.into().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), sitename.into().abi(), flags, domaincontrollerinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetDcNameW<P0, P1, P2>(computername: P0, domainname: P1, domainguid: ::core::option::Option<*const ::windows::core::GUID>, sitename: P2, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOW) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetDcNameW ( computername : :: windows::core::PCWSTR , domainname : :: windows::core::PCWSTR , domainguid : *const :: windows::core::GUID , sitename : :: windows::core::PCWSTR , flags : u32 , domaincontrollerinfo : *mut *mut DOMAIN_CONTROLLER_INFOW ) -> u32 );
    DsGetDcNameW(computername.into().abi(), domainname.into().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), sitename.into().abi(), flags, domaincontrollerinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DsGetDcNextA<P0>(getdccontexthandle: P0, sockaddresscount: ::core::option::Option<*mut u32>, sockaddresses: ::core::option::Option<*mut *mut super::WinSock::SOCKET_ADDRESS>, dnshostname: ::core::option::Option<*mut ::windows::core::PSTR>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetDcNextA ( getdccontexthandle : super::super::Foundation:: HANDLE , sockaddresscount : *mut u32 , sockaddresses : *mut *mut super::WinSock:: SOCKET_ADDRESS , dnshostname : *mut :: windows::core::PSTR ) -> u32 );
    DsGetDcNextA(getdccontexthandle.into(), ::core::mem::transmute(sockaddresscount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(sockaddresses.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(dnshostname.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DsGetDcNextW<P0>(getdccontexthandle: P0, sockaddresscount: ::core::option::Option<*mut u32>, sockaddresses: ::core::option::Option<*mut *mut super::WinSock::SOCKET_ADDRESS>, dnshostname: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetDcNextW ( getdccontexthandle : super::super::Foundation:: HANDLE , sockaddresscount : *mut u32 , sockaddresses : *mut *mut super::WinSock:: SOCKET_ADDRESS , dnshostname : *mut :: windows::core::PWSTR ) -> u32 );
    DsGetDcNextW(getdccontexthandle.into(), ::core::mem::transmute(sockaddresscount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(sockaddresses.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(dnshostname.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetDcOpenA<P0, P1, P2>(dnsname: P0, optionflags: u32, sitename: P1, domainguid: ::core::option::Option<*const ::windows::core::GUID>, dnsforestname: P2, dcflags: u32, retgetdccontext: *mut GetDcContextHandle) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetDcOpenA ( dnsname : :: windows::core::PCSTR , optionflags : u32 , sitename : :: windows::core::PCSTR , domainguid : *const :: windows::core::GUID , dnsforestname : :: windows::core::PCSTR , dcflags : u32 , retgetdccontext : *mut GetDcContextHandle ) -> u32 );
    DsGetDcOpenA(dnsname.into().abi(), optionflags, sitename.into().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), dnsforestname.into().abi(), dcflags, retgetdccontext)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetDcOpenW<P0, P1, P2>(dnsname: P0, optionflags: u32, sitename: P1, domainguid: ::core::option::Option<*const ::windows::core::GUID>, dnsforestname: P2, dcflags: u32, retgetdccontext: *mut GetDcContextHandle) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetDcOpenW ( dnsname : :: windows::core::PCWSTR , optionflags : u32 , sitename : :: windows::core::PCWSTR , domainguid : *const :: windows::core::GUID , dnsforestname : :: windows::core::PCWSTR , dcflags : u32 , retgetdccontext : *mut GetDcContextHandle ) -> u32 );
    DsGetDcOpenW(dnsname.into().abi(), optionflags, sitename.into().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), dnsforestname.into().abi(), dcflags, retgetdccontext)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetDcSiteCoverageA<P0>(servername: P0, entrycount: *mut u32, sitenames: *mut *mut ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetDcSiteCoverageA ( servername : :: windows::core::PCSTR , entrycount : *mut u32 , sitenames : *mut *mut :: windows::core::PSTR ) -> u32 );
    DsGetDcSiteCoverageA(servername.into().abi(), entrycount, sitenames)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetDcSiteCoverageW<P0>(servername: P0, entrycount: *mut u32, sitenames: *mut *mut ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetDcSiteCoverageW ( servername : :: windows::core::PCWSTR , entrycount : *mut u32 , sitenames : *mut *mut :: windows::core::PWSTR ) -> u32 );
    DsGetDcSiteCoverageW(servername.into().abi(), entrycount, sitenames)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsGetDomainControllerInfoA<P0, P1>(hds: P0, domainname: P1, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsGetDomainControllerInfoA ( hds : super::super::Foundation:: HANDLE , domainname : :: windows::core::PCSTR , infolevel : u32 , pcout : *mut u32 , ppinfo : *mut *mut ::core::ffi::c_void ) -> u32 );
    DsGetDomainControllerInfoA(hds.into(), domainname.into().abi(), infolevel, pcout, ppinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsGetDomainControllerInfoW<P0, P1>(hds: P0, domainname: P1, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsGetDomainControllerInfoW ( hds : super::super::Foundation:: HANDLE , domainname : :: windows::core::PCWSTR , infolevel : u32 , pcout : *mut u32 , ppinfo : *mut *mut ::core::ffi::c_void ) -> u32 );
    DsGetDomainControllerInfoW(hds.into(), domainname.into().abi(), infolevel, pcout, ppinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
#[inline]
pub unsafe fn DsGetForestTrustInformationW<P0, P1>(servername: P0, trusteddomainname: P1, flags: u32, foresttrustinfo: *mut *mut super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetForestTrustInformationW ( servername : :: windows::core::PCWSTR , trusteddomainname : :: windows::core::PCWSTR , flags : u32 , foresttrustinfo : *mut *mut super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION ) -> u32 );
    DsGetForestTrustInformationW(servername.into().abi(), trusteddomainname.into().abi(), flags, foresttrustinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetFriendlyClassName<P0>(pszobjectclass: P0, pszbuffer: &mut [u16]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "dsuiext.dll""system" fn DsGetFriendlyClassName ( pszobjectclass : :: windows::core::PCWSTR , pszbuffer : :: windows::core::PWSTR , cchbuffer : u32 ) -> :: windows::core::HRESULT );
    DsGetFriendlyClassName(pszobjectclass.into().abi(), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn DsGetIcon<P0>(dwflags: u32, pszobjectclass: P0, cximage: i32, cyimage: i32) -> super::super::UI::WindowsAndMessaging::HICON
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "dsuiext.dll""system" fn DsGetIcon ( dwflags : u32 , pszobjectclass : :: windows::core::PCWSTR , cximage : i32 , cyimage : i32 ) -> super::super::UI::WindowsAndMessaging:: HICON );
    DsGetIcon(dwflags, pszobjectclass.into().abi(), cximage, cyimage)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetRdnW(ppdn: *mut ::windows::core::PWSTR, pcdn: *mut u32, ppkey: *mut ::windows::core::PWSTR, pckey: *mut u32, ppval: *mut ::windows::core::PWSTR, pcval: *mut u32) -> u32 {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsGetRdnW ( ppdn : *mut :: windows::core::PWSTR , pcdn : *mut u32 , ppkey : *mut :: windows::core::PWSTR , pckey : *mut u32 , ppval : *mut :: windows::core::PWSTR , pcval : *mut u32 ) -> u32 );
    DsGetRdnW(ppdn, pcdn, ppkey, pckey, ppval, pcval)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetSiteNameA<P0>(computername: P0, sitename: *mut ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetSiteNameA ( computername : :: windows::core::PCSTR , sitename : *mut :: windows::core::PSTR ) -> u32 );
    DsGetSiteNameA(computername.into().abi(), sitename)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetSiteNameW<P0>(computername: P0, sitename: *mut ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsGetSiteNameW ( computername : :: windows::core::PCWSTR , sitename : *mut :: windows::core::PWSTR ) -> u32 );
    DsGetSiteNameW(computername.into().abi(), sitename)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetSpnA<P0, P1>(servicetype: DS_SPN_NAME_TYPE, serviceclass: P0, servicename: P1, instanceport: u16, cinstancenames: u16, pinstancenames: ::core::option::Option<*const ::windows::core::PCSTR>, pinstanceports: ::core::option::Option<*const u16>, pcspn: *mut u32, prpszspn: *mut *mut ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsGetSpnA ( servicetype : DS_SPN_NAME_TYPE , serviceclass : :: windows::core::PCSTR , servicename : :: windows::core::PCSTR , instanceport : u16 , cinstancenames : u16 , pinstancenames : *const :: windows::core::PCSTR , pinstanceports : *const u16 , pcspn : *mut u32 , prpszspn : *mut *mut :: windows::core::PSTR ) -> u32 );
    DsGetSpnA(servicetype, serviceclass.into().abi(), servicename.into().abi(), instanceport, cinstancenames, ::core::mem::transmute(pinstancenames.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pinstanceports.unwrap_or(::std::ptr::null())), pcspn, prpszspn)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsGetSpnW<P0, P1>(servicetype: DS_SPN_NAME_TYPE, serviceclass: P0, servicename: P1, instanceport: u16, cinstancenames: u16, pinstancenames: ::core::option::Option<*const ::windows::core::PCWSTR>, pinstanceports: ::core::option::Option<*const u16>, pcspn: *mut u32, prpszspn: *mut *mut ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsGetSpnW ( servicetype : DS_SPN_NAME_TYPE , serviceclass : :: windows::core::PCWSTR , servicename : :: windows::core::PCWSTR , instanceport : u16 , cinstancenames : u16 , pinstancenames : *const :: windows::core::PCWSTR , pinstanceports : *const u16 , pcspn : *mut u32 , prpszspn : *mut *mut :: windows::core::PWSTR ) -> u32 );
    DsGetSpnW(servicetype, serviceclass.into().abi(), servicename.into().abi(), instanceport, cinstancenames, ::core::mem::transmute(pinstancenames.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pinstanceports.unwrap_or(::std::ptr::null())), pcspn, prpszspn)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsInheritSecurityIdentityA<P0, P1, P2>(hds: P0, flags: u32, srcprincipal: P1, dstprincipal: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsInheritSecurityIdentityA ( hds : super::super::Foundation:: HANDLE , flags : u32 , srcprincipal : :: windows::core::PCSTR , dstprincipal : :: windows::core::PCSTR ) -> u32 );
    DsInheritSecurityIdentityA(hds.into(), flags, srcprincipal.into().abi(), dstprincipal.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsInheritSecurityIdentityW<P0, P1, P2>(hds: P0, flags: u32, srcprincipal: P1, dstprincipal: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsInheritSecurityIdentityW ( hds : super::super::Foundation:: HANDLE , flags : u32 , srcprincipal : :: windows::core::PCWSTR , dstprincipal : :: windows::core::PCWSTR ) -> u32 );
    DsInheritSecurityIdentityW(hds.into(), flags, srcprincipal.into().abi(), dstprincipal.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsIsMangledDnA<P0>(pszdn: P0, edsmanglefor: DS_MANGLE_FOR) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "dsparse.dll""system" fn DsIsMangledDnA ( pszdn : :: windows::core::PCSTR , edsmanglefor : DS_MANGLE_FOR ) -> super::super::Foundation:: BOOL );
    DsIsMangledDnA(pszdn.into().abi(), edsmanglefor)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsIsMangledDnW<P0>(pszdn: P0, edsmanglefor: DS_MANGLE_FOR) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "dsparse.dll""system" fn DsIsMangledDnW ( pszdn : :: windows::core::PCWSTR , edsmanglefor : DS_MANGLE_FOR ) -> super::super::Foundation:: BOOL );
    DsIsMangledDnW(pszdn.into().abi(), edsmanglefor)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsIsMangledRdnValueA(pszrdn: &[u8], edsmanglefordesired: DS_MANGLE_FOR) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsIsMangledRdnValueA ( pszrdn : :: windows::core::PCSTR , crdn : u32 , edsmanglefordesired : DS_MANGLE_FOR ) -> super::super::Foundation:: BOOL );
    DsIsMangledRdnValueA(::core::mem::transmute(pszrdn.as_ptr()), pszrdn.len() as _, edsmanglefordesired)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsIsMangledRdnValueW(pszrdn: &[u16], edsmanglefordesired: DS_MANGLE_FOR) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsIsMangledRdnValueW ( pszrdn : :: windows::core::PCWSTR , crdn : u32 , edsmanglefordesired : DS_MANGLE_FOR ) -> super::super::Foundation:: BOOL );
    DsIsMangledRdnValueW(::core::mem::transmute(pszrdn.as_ptr()), pszrdn.len() as _, edsmanglefordesired)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListDomainsInSiteA<P0, P1>(hds: P0, site: P1, ppdomains: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListDomainsInSiteA ( hds : super::super::Foundation:: HANDLE , site : :: windows::core::PCSTR , ppdomains : *mut *mut DS_NAME_RESULTA ) -> u32 );
    DsListDomainsInSiteA(hds.into(), site.into().abi(), ppdomains)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListDomainsInSiteW<P0, P1>(hds: P0, site: P1, ppdomains: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListDomainsInSiteW ( hds : super::super::Foundation:: HANDLE , site : :: windows::core::PCWSTR , ppdomains : *mut *mut DS_NAME_RESULTW ) -> u32 );
    DsListDomainsInSiteW(hds.into(), site.into().abi(), ppdomains)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListInfoForServerA<P0, P1>(hds: P0, server: P1, ppinfo: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListInfoForServerA ( hds : super::super::Foundation:: HANDLE , server : :: windows::core::PCSTR , ppinfo : *mut *mut DS_NAME_RESULTA ) -> u32 );
    DsListInfoForServerA(hds.into(), server.into().abi(), ppinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListInfoForServerW<P0, P1>(hds: P0, server: P1, ppinfo: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListInfoForServerW ( hds : super::super::Foundation:: HANDLE , server : :: windows::core::PCWSTR , ppinfo : *mut *mut DS_NAME_RESULTW ) -> u32 );
    DsListInfoForServerW(hds.into(), server.into().abi(), ppinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListRolesA<P0>(hds: P0, pproles: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListRolesA ( hds : super::super::Foundation:: HANDLE , pproles : *mut *mut DS_NAME_RESULTA ) -> u32 );
    DsListRolesA(hds.into(), pproles)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListRolesW<P0>(hds: P0, pproles: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListRolesW ( hds : super::super::Foundation:: HANDLE , pproles : *mut *mut DS_NAME_RESULTW ) -> u32 );
    DsListRolesW(hds.into(), pproles)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListServersForDomainInSiteA<P0, P1, P2>(hds: P0, domain: P1, site: P2, ppservers: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListServersForDomainInSiteA ( hds : super::super::Foundation:: HANDLE , domain : :: windows::core::PCSTR , site : :: windows::core::PCSTR , ppservers : *mut *mut DS_NAME_RESULTA ) -> u32 );
    DsListServersForDomainInSiteA(hds.into(), domain.into().abi(), site.into().abi(), ppservers)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListServersForDomainInSiteW<P0, P1, P2>(hds: P0, domain: P1, site: P2, ppservers: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListServersForDomainInSiteW ( hds : super::super::Foundation:: HANDLE , domain : :: windows::core::PCWSTR , site : :: windows::core::PCWSTR , ppservers : *mut *mut DS_NAME_RESULTW ) -> u32 );
    DsListServersForDomainInSiteW(hds.into(), domain.into().abi(), site.into().abi(), ppservers)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListServersInSiteA<P0, P1>(hds: P0, site: P1, ppservers: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListServersInSiteA ( hds : super::super::Foundation:: HANDLE , site : :: windows::core::PCSTR , ppservers : *mut *mut DS_NAME_RESULTA ) -> u32 );
    DsListServersInSiteA(hds.into(), site.into().abi(), ppservers)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListServersInSiteW<P0, P1>(hds: P0, site: P1, ppservers: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListServersInSiteW ( hds : super::super::Foundation:: HANDLE , site : :: windows::core::PCWSTR , ppservers : *mut *mut DS_NAME_RESULTW ) -> u32 );
    DsListServersInSiteW(hds.into(), site.into().abi(), ppservers)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListSitesA<P0>(hds: P0, ppsites: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListSitesA ( hds : super::super::Foundation:: HANDLE , ppsites : *mut *mut DS_NAME_RESULTA ) -> u32 );
    DsListSitesA(hds.into(), ppsites)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListSitesW<P0>(hds: P0, ppsites: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsListSitesW ( hds : super::super::Foundation:: HANDLE , ppsites : *mut *mut DS_NAME_RESULTW ) -> u32 );
    DsListSitesW(hds.into(), ppsites)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsMakePasswordCredentialsA<P0, P1, P2>(user: P0, domain: P1, password: P2, pauthidentity: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsMakePasswordCredentialsA ( user : :: windows::core::PCSTR , domain : :: windows::core::PCSTR , password : :: windows::core::PCSTR , pauthidentity : *mut *mut ::core::ffi::c_void ) -> u32 );
    DsMakePasswordCredentialsA(user.into().abi(), domain.into().abi(), password.into().abi(), pauthidentity)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsMakePasswordCredentialsW<P0, P1, P2>(user: P0, domain: P1, password: P2, pauthidentity: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsMakePasswordCredentialsW ( user : :: windows::core::PCWSTR , domain : :: windows::core::PCWSTR , password : :: windows::core::PCWSTR , pauthidentity : *mut *mut ::core::ffi::c_void ) -> u32 );
    DsMakePasswordCredentialsW(user.into().abi(), domain.into().abi(), password.into().abi(), pauthidentity)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsMakeSpnA<P0, P1, P2, P3>(serviceclass: P0, servicename: P1, instancename: P2, instanceport: u16, referrer: P3, pcspnlength: *mut u32, pszspn: ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "dsparse.dll""system" fn DsMakeSpnA ( serviceclass : :: windows::core::PCSTR , servicename : :: windows::core::PCSTR , instancename : :: windows::core::PCSTR , instanceport : u16 , referrer : :: windows::core::PCSTR , pcspnlength : *mut u32 , pszspn : :: windows::core::PSTR ) -> u32 );
    DsMakeSpnA(serviceclass.into().abi(), servicename.into().abi(), instancename.into().abi(), instanceport, referrer.into().abi(), pcspnlength, ::core::mem::transmute(pszspn))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsMakeSpnW<P0, P1, P2, P3>(serviceclass: P0, servicename: P1, instancename: P2, instanceport: u16, referrer: P3, pcspnlength: *mut u32, pszspn: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "dsparse.dll""system" fn DsMakeSpnW ( serviceclass : :: windows::core::PCWSTR , servicename : :: windows::core::PCWSTR , instancename : :: windows::core::PCWSTR , instanceport : u16 , referrer : :: windows::core::PCWSTR , pcspnlength : *mut u32 , pszspn : :: windows::core::PWSTR ) -> u32 );
    DsMakeSpnW(serviceclass.into().abi(), servicename.into().abi(), instancename.into().abi(), instanceport, referrer.into().abi(), pcspnlength, ::core::mem::transmute(pszspn))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsMapSchemaGuidsA<P0>(hds: P0, rguids: &[::windows::core::GUID], ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPA) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsMapSchemaGuidsA ( hds : super::super::Foundation:: HANDLE , cguids : u32 , rguids : *const :: windows::core::GUID , ppguidmap : *mut *mut DS_SCHEMA_GUID_MAPA ) -> u32 );
    DsMapSchemaGuidsA(hds.into(), rguids.len() as _, ::core::mem::transmute(rguids.as_ptr()), ppguidmap)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsMapSchemaGuidsW<P0>(hds: P0, rguids: &[::windows::core::GUID], ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPW) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsMapSchemaGuidsW ( hds : super::super::Foundation:: HANDLE , cguids : u32 , rguids : *const :: windows::core::GUID , ppguidmap : *mut *mut DS_SCHEMA_GUID_MAPW ) -> u32 );
    DsMapSchemaGuidsW(hds.into(), rguids.len() as _, ::core::mem::transmute(rguids.as_ptr()), ppguidmap)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
#[inline]
pub unsafe fn DsMergeForestTrustInformationW<P0>(domainname: P0, newforesttrustinfo: *const super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION, oldforesttrustinfo: ::core::option::Option<*const super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION>, mergedforesttrustinfo: *mut *mut super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsMergeForestTrustInformationW ( domainname : :: windows::core::PCWSTR , newforesttrustinfo : *const super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION , oldforesttrustinfo : *const super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION , mergedforesttrustinfo : *mut *mut super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION ) -> u32 );
    DsMergeForestTrustInformationW(domainname.into().abi(), newforesttrustinfo, ::core::mem::transmute(oldforesttrustinfo.unwrap_or(::std::ptr::null())), mergedforesttrustinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsQuerySitesByCostA<P0, P1>(hds: P0, pszfromsite: P1, rgsztosites: &[::windows::core::PCSTR], dwflags: u32, prgsiteinfo: *mut *mut DS_SITE_COST_INFO) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsQuerySitesByCostA ( hds : super::super::Foundation:: HANDLE , pszfromsite : :: windows::core::PCSTR , rgsztosites : *const :: windows::core::PCSTR , ctosites : u32 , dwflags : u32 , prgsiteinfo : *mut *mut DS_SITE_COST_INFO ) -> u32 );
    DsQuerySitesByCostA(hds.into(), pszfromsite.into().abi(), ::core::mem::transmute(rgsztosites.as_ptr()), rgsztosites.len() as _, dwflags, prgsiteinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsQuerySitesByCostW<P0, P1>(hds: P0, pwszfromsite: P1, rgwsztosites: &[::windows::core::PCWSTR], dwflags: u32, prgsiteinfo: *mut *mut DS_SITE_COST_INFO) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsQuerySitesByCostW ( hds : super::super::Foundation:: HANDLE , pwszfromsite : :: windows::core::PCWSTR , rgwsztosites : *const :: windows::core::PCWSTR , ctosites : u32 , dwflags : u32 , prgsiteinfo : *mut *mut DS_SITE_COST_INFO ) -> u32 );
    DsQuerySitesByCostW(hds.into(), pwszfromsite.into().abi(), ::core::mem::transmute(rgwsztosites.as_ptr()), rgwsztosites.len() as _, dwflags, prgsiteinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsQuerySitesFree(rgsiteinfo: *const DS_SITE_COST_INFO) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsQuerySitesFree ( rgsiteinfo : *const DS_SITE_COST_INFO ) -> ( ) );
    DsQuerySitesFree(rgsiteinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsQuoteRdnValueA(psunquotedrdnvalue: &[u8], pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: ::windows::core::PSTR) -> u32 {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsQuoteRdnValueA ( cunquotedrdnvaluelength : u32 , psunquotedrdnvalue : :: windows::core::PCSTR , pcquotedrdnvaluelength : *mut u32 , psquotedrdnvalue : :: windows::core::PSTR ) -> u32 );
    DsQuoteRdnValueA(psunquotedrdnvalue.len() as _, ::core::mem::transmute(psunquotedrdnvalue.as_ptr()), pcquotedrdnvaluelength, ::core::mem::transmute(psquotedrdnvalue))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsQuoteRdnValueW(psunquotedrdnvalue: &[u16], pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: ::windows::core::PWSTR) -> u32 {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsQuoteRdnValueW ( cunquotedrdnvaluelength : u32 , psunquotedrdnvalue : :: windows::core::PCWSTR , pcquotedrdnvaluelength : *mut u32 , psquotedrdnvalue : :: windows::core::PWSTR ) -> u32 );
    DsQuoteRdnValueW(psunquotedrdnvalue.len() as _, ::core::mem::transmute(psunquotedrdnvalue.as_ptr()), pcquotedrdnvaluelength, ::core::mem::transmute(psquotedrdnvalue))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsRemoveDsDomainA<P0, P1>(hds: P0, domaindn: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsRemoveDsDomainA ( hds : super::super::Foundation:: HANDLE , domaindn : :: windows::core::PCSTR ) -> u32 );
    DsRemoveDsDomainA(hds.into(), domaindn.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsRemoveDsDomainW<P0, P1>(hds: P0, domaindn: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsRemoveDsDomainW ( hds : super::super::Foundation:: HANDLE , domaindn : :: windows::core::PCWSTR ) -> u32 );
    DsRemoveDsDomainW(hds.into(), domaindn.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsRemoveDsServerA<P0, P1, P2, P3>(hds: P0, serverdn: P1, domaindn: P2, flastdcindomain: ::core::option::Option<*mut super::super::Foundation::BOOL>, fcommit: P3) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsRemoveDsServerA ( hds : super::super::Foundation:: HANDLE , serverdn : :: windows::core::PCSTR , domaindn : :: windows::core::PCSTR , flastdcindomain : *mut super::super::Foundation:: BOOL , fcommit : super::super::Foundation:: BOOL ) -> u32 );
    DsRemoveDsServerA(hds.into(), serverdn.into().abi(), domaindn.into().abi(), ::core::mem::transmute(flastdcindomain.unwrap_or(::std::ptr::null_mut())), fcommit.into())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsRemoveDsServerW<P0, P1, P2, P3>(hds: P0, serverdn: P1, domaindn: P2, flastdcindomain: ::core::option::Option<*mut super::super::Foundation::BOOL>, fcommit: P3) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsRemoveDsServerW ( hds : super::super::Foundation:: HANDLE , serverdn : :: windows::core::PCWSTR , domaindn : :: windows::core::PCWSTR , flastdcindomain : *mut super::super::Foundation:: BOOL , fcommit : super::super::Foundation:: BOOL ) -> u32 );
    DsRemoveDsServerW(hds.into(), serverdn.into().abi(), domaindn.into().abi(), ::core::mem::transmute(flastdcindomain.unwrap_or(::std::ptr::null_mut())), fcommit.into())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaAddA<P0, P1, P2, P3, P4>(hds: P0, namecontext: P1, sourcedsadn: P2, transportdn: P3, sourcedsaaddress: P4, pschedule: ::core::option::Option<*const SCHEDULE>, options: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaAddA ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCSTR , sourcedsadn : :: windows::core::PCSTR , transportdn : :: windows::core::PCSTR , sourcedsaaddress : :: windows::core::PCSTR , pschedule : *const SCHEDULE , options : u32 ) -> u32 );
    DsReplicaAddA(hds.into(), namecontext.into().abi(), sourcedsadn.into().abi(), transportdn.into().abi(), sourcedsaaddress.into().abi(), ::core::mem::transmute(pschedule.unwrap_or(::std::ptr::null())), options)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaAddW<P0, P1, P2, P3, P4>(hds: P0, namecontext: P1, sourcedsadn: P2, transportdn: P3, sourcedsaaddress: P4, pschedule: ::core::option::Option<*const SCHEDULE>, options: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaAddW ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCWSTR , sourcedsadn : :: windows::core::PCWSTR , transportdn : :: windows::core::PCWSTR , sourcedsaaddress : :: windows::core::PCWSTR , pschedule : *const SCHEDULE , options : u32 ) -> u32 );
    DsReplicaAddW(hds.into(), namecontext.into().abi(), sourcedsadn.into().abi(), transportdn.into().abi(), sourcedsaaddress.into().abi(), ::core::mem::transmute(pschedule.unwrap_or(::std::ptr::null())), options)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaConsistencyCheck<P0>(hds: P0, taskid: DS_KCC_TASKID, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaConsistencyCheck ( hds : super::super::Foundation:: HANDLE , taskid : DS_KCC_TASKID , dwflags : u32 ) -> u32 );
    DsReplicaConsistencyCheck(hds.into(), taskid, dwflags)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaDelA<P0, P1, P2>(hds: P0, namecontext: P1, dsasrc: P2, options: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaDelA ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCSTR , dsasrc : :: windows::core::PCSTR , options : u32 ) -> u32 );
    DsReplicaDelA(hds.into(), namecontext.into().abi(), dsasrc.into().abi(), options)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaDelW<P0, P1, P2>(hds: P0, namecontext: P1, dsasrc: P2, options: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaDelW ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCWSTR , dsasrc : :: windows::core::PCWSTR , options : u32 ) -> u32 );
    DsReplicaDelW(hds.into(), namecontext.into().abi(), dsasrc.into().abi(), options)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsReplicaFreeInfo(infotype: DS_REPL_INFO_TYPE, pinfo: *const ::core::ffi::c_void) {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaFreeInfo ( infotype : DS_REPL_INFO_TYPE , pinfo : *const ::core::ffi::c_void ) -> ( ) );
    DsReplicaFreeInfo(infotype, pinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaGetInfo2W<P0, P1, P2, P3>(hds: P0, infotype: DS_REPL_INFO_TYPE, pszobject: P1, puuidforsourcedsaobjguid: ::core::option::Option<*const ::windows::core::GUID>, pszattributename: P2, pszvalue: P3, dwflags: u32, dwenumerationcontext: u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaGetInfo2W ( hds : super::super::Foundation:: HANDLE , infotype : DS_REPL_INFO_TYPE , pszobject : :: windows::core::PCWSTR , puuidforsourcedsaobjguid : *const :: windows::core::GUID , pszattributename : :: windows::core::PCWSTR , pszvalue : :: windows::core::PCWSTR , dwflags : u32 , dwenumerationcontext : u32 , ppinfo : *mut *mut ::core::ffi::c_void ) -> u32 );
    DsReplicaGetInfo2W(hds.into(), infotype, pszobject.into().abi(), ::core::mem::transmute(puuidforsourcedsaobjguid.unwrap_or(::std::ptr::null())), pszattributename.into().abi(), pszvalue.into().abi(), dwflags, dwenumerationcontext, ppinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaGetInfoW<P0, P1>(hds: P0, infotype: DS_REPL_INFO_TYPE, pszobject: P1, puuidforsourcedsaobjguid: ::core::option::Option<*const ::windows::core::GUID>, ppinfo: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaGetInfoW ( hds : super::super::Foundation:: HANDLE , infotype : DS_REPL_INFO_TYPE , pszobject : :: windows::core::PCWSTR , puuidforsourcedsaobjguid : *const :: windows::core::GUID , ppinfo : *mut *mut ::core::ffi::c_void ) -> u32 );
    DsReplicaGetInfoW(hds.into(), infotype, pszobject.into().abi(), ::core::mem::transmute(puuidforsourcedsaobjguid.unwrap_or(::std::ptr::null())), ppinfo)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaModifyA<P0, P1, P2, P3>(hds: P0, namecontext: P1, puuidsourcedsa: ::core::option::Option<*const ::windows::core::GUID>, transportdn: P2, sourcedsaaddress: P3, pschedule: ::core::option::Option<*const SCHEDULE>, replicaflags: u32, modifyfields: u32, options: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaModifyA ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCSTR , puuidsourcedsa : *const :: windows::core::GUID , transportdn : :: windows::core::PCSTR , sourcedsaaddress : :: windows::core::PCSTR , pschedule : *const SCHEDULE , replicaflags : u32 , modifyfields : u32 , options : u32 ) -> u32 );
    DsReplicaModifyA(hds.into(), namecontext.into().abi(), ::core::mem::transmute(puuidsourcedsa.unwrap_or(::std::ptr::null())), transportdn.into().abi(), sourcedsaaddress.into().abi(), ::core::mem::transmute(pschedule.unwrap_or(::std::ptr::null())), replicaflags, modifyfields, options)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaModifyW<P0, P1, P2, P3>(hds: P0, namecontext: P1, puuidsourcedsa: ::core::option::Option<*const ::windows::core::GUID>, transportdn: P2, sourcedsaaddress: P3, pschedule: ::core::option::Option<*const SCHEDULE>, replicaflags: u32, modifyfields: u32, options: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaModifyW ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCWSTR , puuidsourcedsa : *const :: windows::core::GUID , transportdn : :: windows::core::PCWSTR , sourcedsaaddress : :: windows::core::PCWSTR , pschedule : *const SCHEDULE , replicaflags : u32 , modifyfields : u32 , options : u32 ) -> u32 );
    DsReplicaModifyW(hds.into(), namecontext.into().abi(), ::core::mem::transmute(puuidsourcedsa.unwrap_or(::std::ptr::null())), transportdn.into().abi(), sourcedsaaddress.into().abi(), ::core::mem::transmute(pschedule.unwrap_or(::std::ptr::null())), replicaflags, modifyfields, options)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaSyncA<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const ::windows::core::GUID, options: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaSyncA ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCSTR , puuiddsasrc : *const :: windows::core::GUID , options : u32 ) -> u32 );
    DsReplicaSyncA(hds.into(), namecontext.into().abi(), puuiddsasrc, options)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaSyncAllA<P0, P1>(hds: P0, psznamecontext: P1, ulflags: u32, pfncallback: isize, pcallbackdata: ::core::option::Option<*const ::core::ffi::c_void>, perrors: ::core::option::Option<*mut *mut *mut DS_REPSYNCALL_ERRINFOA>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaSyncAllA ( hds : super::super::Foundation:: HANDLE , psznamecontext : :: windows::core::PCSTR , ulflags : u32 , pfncallback : isize , pcallbackdata : *const ::core::ffi::c_void , perrors : *mut *mut *mut DS_REPSYNCALL_ERRINFOA ) -> u32 );
    DsReplicaSyncAllA(hds.into(), psznamecontext.into().abi(), ulflags, pfncallback, ::core::mem::transmute(pcallbackdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perrors.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaSyncAllW<P0, P1>(hds: P0, psznamecontext: P1, ulflags: u32, pfncallback: isize, pcallbackdata: ::core::option::Option<*const ::core::ffi::c_void>, perrors: ::core::option::Option<*mut *mut *mut DS_REPSYNCALL_ERRINFOW>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaSyncAllW ( hds : super::super::Foundation:: HANDLE , psznamecontext : :: windows::core::PCWSTR , ulflags : u32 , pfncallback : isize , pcallbackdata : *const ::core::ffi::c_void , perrors : *mut *mut *mut DS_REPSYNCALL_ERRINFOW ) -> u32 );
    DsReplicaSyncAllW(hds.into(), psznamecontext.into().abi(), ulflags, pfncallback, ::core::mem::transmute(pcallbackdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perrors.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaSyncW<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const ::windows::core::GUID, options: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaSyncW ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCWSTR , puuiddsasrc : *const :: windows::core::GUID , options : u32 ) -> u32 );
    DsReplicaSyncW(hds.into(), namecontext.into().abi(), puuiddsasrc, options)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaUpdateRefsA<P0, P1, P2>(hds: P0, namecontext: P1, dsadest: P2, puuiddsadest: *const ::windows::core::GUID, options: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaUpdateRefsA ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCSTR , dsadest : :: windows::core::PCSTR , puuiddsadest : *const :: windows::core::GUID , options : u32 ) -> u32 );
    DsReplicaUpdateRefsA(hds.into(), namecontext.into().abi(), dsadest.into().abi(), puuiddsadest, options)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaUpdateRefsW<P0, P1, P2>(hds: P0, namecontext: P1, dsadest: P2, puuiddsadest: *const ::windows::core::GUID, options: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaUpdateRefsW ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCWSTR , dsadest : :: windows::core::PCWSTR , puuiddsadest : *const :: windows::core::GUID , options : u32 ) -> u32 );
    DsReplicaUpdateRefsW(hds.into(), namecontext.into().abi(), dsadest.into().abi(), puuiddsadest, options)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaVerifyObjectsA<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const ::windows::core::GUID, uloptions: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaVerifyObjectsA ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCSTR , puuiddsasrc : *const :: windows::core::GUID , uloptions : u32 ) -> u32 );
    DsReplicaVerifyObjectsA(hds.into(), namecontext.into().abi(), puuiddsasrc, uloptions)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaVerifyObjectsW<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const ::windows::core::GUID, uloptions: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsReplicaVerifyObjectsW ( hds : super::super::Foundation:: HANDLE , namecontext : :: windows::core::PCWSTR , puuiddsasrc : *const :: windows::core::GUID , uloptions : u32 ) -> u32 );
    DsReplicaVerifyObjectsW(hds.into(), namecontext.into().abi(), puuiddsasrc, uloptions)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsRoleFreeMemory(buffer: *mut ::core::ffi::c_void) {
    ::windows::core::link ! ( "netapi32.dll""system" fn DsRoleFreeMemory ( buffer : *mut ::core::ffi::c_void ) -> ( ) );
    DsRoleFreeMemory(buffer)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsRoleGetPrimaryDomainInformation<P0>(lpserver: P0, infolevel: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL, buffer: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsRoleGetPrimaryDomainInformation ( lpserver : :: windows::core::PCWSTR , infolevel : DSROLE_PRIMARY_DOMAIN_INFO_LEVEL , buffer : *mut *mut u8 ) -> u32 );
    DsRoleGetPrimaryDomainInformation(lpserver.into().abi(), infolevel, buffer)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsServerRegisterSpnA<P0, P1>(operation: DS_SPN_WRITE_OP, serviceclass: P0, userobjectdn: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsServerRegisterSpnA ( operation : DS_SPN_WRITE_OP , serviceclass : :: windows::core::PCSTR , userobjectdn : :: windows::core::PCSTR ) -> u32 );
    DsServerRegisterSpnA(operation, serviceclass.into().abi(), userobjectdn.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsServerRegisterSpnW<P0, P1>(operation: DS_SPN_WRITE_OP, serviceclass: P0, userobjectdn: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsServerRegisterSpnW ( operation : DS_SPN_WRITE_OP , serviceclass : :: windows::core::PCWSTR , userobjectdn : :: windows::core::PCWSTR ) -> u32 );
    DsServerRegisterSpnW(operation, serviceclass.into().abi(), userobjectdn.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsUnBindA(phds: *const super::super::Foundation::HANDLE) -> u32 {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsUnBindA ( phds : *const super::super::Foundation:: HANDLE ) -> u32 );
    DsUnBindA(phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsUnBindW(phds: *const super::super::Foundation::HANDLE) -> u32 {
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsUnBindW ( phds : *const super::super::Foundation:: HANDLE ) -> u32 );
    DsUnBindW(phds)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsUnquoteRdnValueA(psquotedrdnvalue: &[u8], pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: ::windows::core::PSTR) -> u32 {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsUnquoteRdnValueA ( cquotedrdnvaluelength : u32 , psquotedrdnvalue : :: windows::core::PCSTR , pcunquotedrdnvaluelength : *mut u32 , psunquotedrdnvalue : :: windows::core::PSTR ) -> u32 );
    DsUnquoteRdnValueA(psquotedrdnvalue.len() as _, ::core::mem::transmute(psquotedrdnvalue.as_ptr()), pcunquotedrdnvaluelength, ::core::mem::transmute(psunquotedrdnvalue))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsUnquoteRdnValueW(psquotedrdnvalue: &[u16], pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: ::windows::core::PWSTR) -> u32 {
    ::windows::core::link ! ( "dsparse.dll""system" fn DsUnquoteRdnValueW ( cquotedrdnvaluelength : u32 , psquotedrdnvalue : :: windows::core::PCWSTR , pcunquotedrdnvaluelength : *mut u32 , psunquotedrdnvalue : :: windows::core::PWSTR ) -> u32 );
    DsUnquoteRdnValueW(psquotedrdnvalue.len() as _, ::core::mem::transmute(psquotedrdnvalue.as_ptr()), pcunquotedrdnvaluelength, ::core::mem::transmute(psunquotedrdnvalue))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsValidateSubnetNameA<P0>(subnetname: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsValidateSubnetNameA ( subnetname : :: windows::core::PCSTR ) -> u32 );
    DsValidateSubnetNameA(subnetname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn DsValidateSubnetNameW<P0>(subnetname: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn DsValidateSubnetNameW ( subnetname : :: windows::core::PCWSTR ) -> u32 );
    DsValidateSubnetNameW(subnetname.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsWriteAccountSpnA<P0, P1>(hds: P0, operation: DS_SPN_WRITE_OP, pszaccount: P1, rpszspn: &[::windows::core::PCSTR]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsWriteAccountSpnA ( hds : super::super::Foundation:: HANDLE , operation : DS_SPN_WRITE_OP , pszaccount : :: windows::core::PCSTR , cspn : u32 , rpszspn : *const :: windows::core::PCSTR ) -> u32 );
    DsWriteAccountSpnA(hds.into(), operation, pszaccount.into().abi(), rpszspn.len() as _, ::core::mem::transmute(rpszspn.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsWriteAccountSpnW<P0, P1>(hds: P0, operation: DS_SPN_WRITE_OP, pszaccount: P1, rpszspn: &[::windows::core::PCWSTR]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ntdsapi.dll""system" fn DsWriteAccountSpnW ( hds : super::super::Foundation:: HANDLE , operation : DS_SPN_WRITE_OP , pszaccount : :: windows::core::PCWSTR , cspn : u32 , rpszspn : *const :: windows::core::PCWSTR ) -> u32 );
    DsWriteAccountSpnW(hds.into(), operation, pszaccount.into().abi(), rpszspn.len() as _, ::core::mem::transmute(rpszspn.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeADsMem(pmem: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "activeds.dll""system" fn FreeADsMem ( pmem : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FreeADsMem(pmem)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeADsStr<P0>(pstr: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn FreeADsStr ( pstr : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    FreeADsStr(pstr.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn PropVariantToAdsType(pvariant: *mut super::super::System::Com::VARIANT, dwnumvariant: u32, ppadsvalues: *mut *mut ADSVALUE, pdwnumvalues: *mut u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "activeds.dll""system" fn PropVariantToAdsType ( pvariant : *mut super::super::System::Com:: VARIANT , dwnumvariant : u32 , ppadsvalues : *mut *mut ADSVALUE , pdwnumvalues : *mut u32 ) -> :: windows::core::HRESULT );
    PropVariantToAdsType(pvariant, dwnumvariant, ppadsvalues, pdwnumvalues).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[inline]
pub unsafe fn ReallocADsMem(poldmem: *mut ::core::ffi::c_void, cbold: u32, cbnew: u32) -> *mut ::core::ffi::c_void {
    ::windows::core::link ! ( "activeds.dll""system" fn ReallocADsMem ( poldmem : *mut ::core::ffi::c_void , cbold : u32 , cbnew : u32 ) -> *mut ::core::ffi::c_void );
    ReallocADsMem(poldmem, cbold, cbnew)
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReallocADsStr<P0>(ppstr: *mut ::windows::core::PWSTR, pstr: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn ReallocADsStr ( ppstr : *mut :: windows::core::PWSTR , pstr : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    ReallocADsStr(ppstr, pstr.into().abi())
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn SecurityDescriptorToBinarySD<P0, P1, P2>(vvarsecdes: super::super::System::Com::VARIANT, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, pdwsdlength: *mut u32, pszservername: P0, username: P1, password: P2, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "activeds.dll""system" fn SecurityDescriptorToBinarySD ( vvarsecdes : super::super::System::Com:: VARIANT , ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR , pdwsdlength : *mut u32 , pszservername : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR , password : :: windows::core::PCWSTR , dwflags : u32 ) -> :: windows::core::HRESULT );
    SecurityDescriptorToBinarySD(::core::mem::transmute(vvarsecdes), ppsecuritydescriptor, pdwsdlength, pszservername.into().abi(), username.into().abi(), password.into().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADs(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADs {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Class)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ADsPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Schema)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put(&self, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Put)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutEx(&self, lncontrolcode: i32, bstrname: &::windows::core::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PutEx)(::windows::core::Vtable::as_raw(self), lncontrolcode, ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetInfoEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADs, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADs {
    type Vtable = IADs_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd8256d0_fd15_11ce_abc4_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Schema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Get: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, vprop: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Put: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetEx: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PutEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lncontrolcode: i32, bstrname: *mut ::core::ffi::c_void, vprop: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PutEx: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetInfoEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetInfoEx: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsADSystemInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsADSystemInfo {
    pub unsafe fn UserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ComputerName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ComputerName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SiteName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SiteName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DomainShortName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DomainShortName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DomainDNSName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DomainDNSName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ForestDNSName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ForestDNSName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PDCRoleOwner(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PDCRoleOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SchemaRoleOwner(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SchemaRoleOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsNativeMode(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsNativeMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAnyDCName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAnyDCName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDCSiteName(&self, szserver: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDCSiteName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(szserver), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RefreshSchemaCache(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RefreshSchemaCache)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetTrees(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTrees)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsADSystemInfo, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsADSystemInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsADSystemInfo {
    type Vtable = IADsADSystemInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsADSystemInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bb11929_afd1_11d2_9cb9_0000f87a369e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsADSystemInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ComputerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SiteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DomainShortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DomainDNSName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ForestDNSName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PDCRoleOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SchemaRoleOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsNativeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsNativeMode: usize,
    pub GetAnyDCName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdcname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDCSiteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szserver: *mut ::core::ffi::c_void, pszsitename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RefreshSchemaCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetTrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvtrees: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetTrees: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsAccessControlEntry(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsAccessControlEntry {
    pub unsafe fn AccessMask(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AccessMask)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccessMask(&self, lnaccessmask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAccessMask)(::windows::core::Vtable::as_raw(self), lnaccessmask).ok()
    }
    pub unsafe fn AceType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AceType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAceType(&self, lnacetype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAceType)(::windows::core::Vtable::as_raw(self), lnacetype).ok()
    }
    pub unsafe fn AceFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AceFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAceFlags(&self, lnaceflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAceFlags)(::windows::core::Vtable::as_raw(self), lnaceflags).ok()
    }
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Flags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFlags(&self, lnflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFlags)(::windows::core::Vtable::as_raw(self), lnflags).ok()
    }
    pub unsafe fn ObjectType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ObjectType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetObjectType(&self, bstrobjecttype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetObjectType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrobjecttype)).ok()
    }
    pub unsafe fn InheritedObjectType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InheritedObjectType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInheritedObjectType(&self, bstrinheritedobjecttype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInheritedObjectType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrinheritedobjecttype)).ok()
    }
    pub unsafe fn Trustee(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Trustee)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTrustee(&self, bstrtrustee: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTrustee)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtrustee)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsAccessControlEntry, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsAccessControlEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsAccessControlEntry {
    type Vtable = IADsAccessControlEntry_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsAccessControlEntry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4f3a14c_9bdd_11d0_852c_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAccessControlEntry_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AccessMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAccessMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnaccessmask: i32) -> ::windows::core::HRESULT,
    pub AceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnacetype: i32) -> ::windows::core::HRESULT,
    pub AceFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAceFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnaceflags: i32) -> ::windows::core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows::core::HRESULT,
    pub ObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjecttype: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InheritedObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetInheritedObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinheritedobjecttype: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Trustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTrustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsAccessControlList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsAccessControlList {
    pub unsafe fn AclRevision(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AclRevision)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAclRevision(&self, lnaclrevision: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAclRevision)(::windows::core::Vtable::as_raw(self), lnaclrevision).ok()
    }
    pub unsafe fn AceCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AceCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAceCount(&self, lnacecount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAceCount)(::windows::core::Vtable::as_raw(self), lnacecount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddAce<P0>(&self, paccesscontrolentry: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).AddAce)(::windows::core::Vtable::as_raw(self), paccesscontrolentry.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveAce<P0>(&self, paccesscontrolentry: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveAce)(::windows::core::Vtable::as_raw(self), paccesscontrolentry.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyAccessList(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CopyAccessList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsAccessControlList, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsAccessControlList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsAccessControlList {
    type Vtable = IADsAccessControlList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsAccessControlList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7ee91cc_9bdd_11d0_852c_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAccessControlList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AclRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAclRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnaclrevision: i32) -> ::windows::core::HRESULT,
    pub AceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnacecount: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddAce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paccesscontrolentry: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddAce: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveAce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paccesscontrolentry: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveAce: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyAccessList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaccesscontrollist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyAccessList: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsAcl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsAcl {
    pub unsafe fn ProtectedAttrName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ProtectedAttrName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProtectedAttrName(&self, bstrprotectedattrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProtectedAttrName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotectedattrname)).ok()
    }
    pub unsafe fn SubjectName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SubjectName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSubjectName(&self, bstrsubjectname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSubjectName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsubjectname)).ok()
    }
    pub unsafe fn Privileges(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Privileges)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivileges(&self, lnprivileges: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPrivileges)(::windows::core::Vtable::as_raw(self), lnprivileges).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyAcl(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CopyAcl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsAcl, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsAcl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsAcl {
    type Vtable = IADsAcl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsAcl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8452d3ab_0869_11d1_a377_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAcl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ProtectedAttrName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetProtectedAttrName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotectedattrname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SubjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubjectname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Privileges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrivileges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnprivileges: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyAcl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppacl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyAcl: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IADsAggregatee(::windows::core::IUnknown);
impl IADsAggregatee {
    pub unsafe fn ConnectAsAggregatee<P0>(&self, pouterunknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).ConnectAsAggregatee)(::windows::core::Vtable::as_raw(self), pouterunknown.into().abi()).ok()
    }
    pub unsafe fn DisconnectAsAggregatee(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DisconnectAsAggregatee)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RelinquishInterface(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RelinquishInterface)(::windows::core::Vtable::as_raw(self), riid).ok()
    }
    pub unsafe fn RestoreInterface(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RestoreInterface)(::windows::core::Vtable::as_raw(self), riid).ok()
    }
}
::windows::core::interface_hierarchy!(IADsAggregatee, ::windows::core::IUnknown);
impl ::core::clone::Clone for IADsAggregatee {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IADsAggregatee {
    type Vtable = IADsAggregatee_Vtbl;
}
unsafe impl ::windows::core::Interface for IADsAggregatee {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1346ce8c_9039_11d0_8528_00c04fd8d503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsAggregatee_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConnectAsAggregatee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pouterunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisconnectAsAggregatee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RelinquishInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RestoreInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IADsAggregator(::windows::core::IUnknown);
impl IADsAggregator {
    pub unsafe fn ConnectAsAggregator<P0>(&self, paggregatee: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).ConnectAsAggregator)(::windows::core::Vtable::as_raw(self), paggregatee.into().abi()).ok()
    }
    pub unsafe fn DisconnectAsAggregator(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DisconnectAsAggregator)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IADsAggregator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IADsAggregator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IADsAggregator {
    type Vtable = IADsAggregator_Vtbl;
}
unsafe impl ::windows::core::Interface for IADsAggregator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52db5fb0_941f_11d0_8529_00c04fd8d503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsAggregator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConnectAsAggregator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregatee: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisconnectAsAggregator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsBackLink(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsBackLink {
    pub unsafe fn RemoteID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RemoteID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRemoteID(&self, lnremoteid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRemoteID)(::windows::core::Vtable::as_raw(self), lnremoteid).ok()
    }
    pub unsafe fn ObjectName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ObjectName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetObjectName(&self, bstrobjectname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetObjectName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrobjectname)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsBackLink, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsBackLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsBackLink {
    type Vtable = IADsBackLink_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsBackLink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd1302bd_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsBackLink_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub RemoteID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetRemoteID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnremoteid: i32) -> ::windows::core::HRESULT,
    pub ObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjectname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsCaseIgnoreList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsCaseIgnoreList {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CaseIgnoreList(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CaseIgnoreList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCaseIgnoreList(&self, vcaseignorelist: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCaseIgnoreList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vcaseignorelist)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsCaseIgnoreList, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsCaseIgnoreList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsCaseIgnoreList {
    type Vtable = IADsCaseIgnoreList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsCaseIgnoreList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b66b533_4680_11d1_a3b4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsCaseIgnoreList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CaseIgnoreList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CaseIgnoreList: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCaseIgnoreList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcaseignorelist: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCaseIgnoreList: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsClass(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsClass {
    pub unsafe fn PrimaryInterface(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrimaryInterface)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCLSID(&self, bstrclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCLSID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrclsid)).ok()
    }
    pub unsafe fn OID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOID(&self, bstroid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstroid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abstract(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Abstract)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAbstract<P0>(&self, fabstract: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAbstract)(::windows::core::Vtable::as_raw(self), fabstract.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Auxiliary(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Auxiliary)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuxiliary<P0>(&self, fauxiliary: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAuxiliary)(::windows::core::Vtable::as_raw(self), fauxiliary.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MandatoryProperties(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MandatoryProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetMandatoryProperties(&self, vmandatoryproperties: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMandatoryProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vmandatoryproperties)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OptionalProperties(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OptionalProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetOptionalProperties(&self, voptionalproperties: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOptionalProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(voptionalproperties)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NamingProperties(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NamingProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetNamingProperties(&self, vnamingproperties: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNamingProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vnamingproperties)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DerivedFrom(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DerivedFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDerivedFrom(&self, vderivedfrom: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDerivedFrom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vderivedfrom)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AuxDerivedFrom(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AuxDerivedFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetAuxDerivedFrom(&self, vauxderivedfrom: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAuxDerivedFrom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vauxderivedfrom)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PossibleSuperiors(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PossibleSuperiors)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPossibleSuperiors(&self, vpossiblesuperiors: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPossibleSuperiors)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vpossiblesuperiors)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Containment(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Containment)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetContainment(&self, vcontainment: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetContainment)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vcontainment)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Container(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Container)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContainer<P0>(&self, fcontainer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetContainer)(::windows::core::Vtable::as_raw(self), fcontainer.into()).ok()
    }
    pub unsafe fn HelpFileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HelpFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHelpFileName(&self, bstrhelpfilename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHelpFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrhelpfilename)).ok()
    }
    pub unsafe fn HelpFileContext(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HelpFileContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHelpFileContext(&self, lnhelpfilecontext: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHelpFileContext)(::windows::core::Vtable::as_raw(self), lnhelpfilecontext).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers(&self) -> ::windows::core::Result<IADsCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Qualifiers)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsClass, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsClass {
    type Vtable = IADsClass_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsClass {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8f93dd0_4ae0_11cf_9e73_00aa004a5691);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsClass_Vtbl {
    pub base__: IADs_Vtbl,
    pub PrimaryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclsid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Abstract: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Abstract: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAbstract: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fabstract: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAbstract: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Auxiliary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Auxiliary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuxiliary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fauxiliary: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuxiliary: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MandatoryProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MandatoryProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetMandatoryProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vmandatoryproperties: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetMandatoryProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OptionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OptionalProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetOptionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voptionalproperties: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetOptionalProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NamingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NamingProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetNamingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vnamingproperties: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetNamingProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DerivedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DerivedFrom: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDerivedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vderivedfrom: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDerivedFrom: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AuxDerivedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AuxDerivedFrom: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetAuxDerivedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vauxderivedfrom: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetAuxDerivedFrom: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PossibleSuperiors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PossibleSuperiors: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPossibleSuperiors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vpossiblesuperiors: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPossibleSuperiors: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Containment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Containment: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetContainment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcontainment: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetContainment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Container: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcontainer: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContainer: usize,
    pub HelpFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHelpFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhelpfilename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HelpFileContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetHelpFileContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnhelpfilecontext: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqualifiers: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add(&self, bstrname: &::windows::core::BSTR, vitem: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute(vitem)).ok()
    }
    pub unsafe fn Remove(&self, bstritemtoberemoved: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemtoberemoved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetObject(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetObject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsCollection {
    type Vtable = IADsCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b945e0_253b_11cf_a988_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, vitem: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemtoberemoved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pvitem: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetObject: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsComputer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsComputer {
    pub unsafe fn ComputerID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ComputerID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Site(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Site)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn Location(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Location)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLocation(&self, bstrlocation: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLocation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrlocation)).ok()
    }
    pub unsafe fn PrimaryUser(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrimaryUser)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrimaryUser(&self, bstrprimaryuser: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPrimaryUser)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprimaryuser)).ok()
    }
    pub unsafe fn Owner(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Owner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOwner(&self, bstrowner: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOwner)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrowner)).ok()
    }
    pub unsafe fn Division(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Division)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDivision(&self, bstrdivision: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDivision)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdivision)).ok()
    }
    pub unsafe fn Department(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Department)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDepartment(&self, bstrdepartment: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDepartment)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdepartment)).ok()
    }
    pub unsafe fn Role(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Role)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRole(&self, bstrrole: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRole)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrole)).ok()
    }
    pub unsafe fn OperatingSystem(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OperatingSystem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOperatingSystem(&self, bstroperatingsystem: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOperatingSystem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstroperatingsystem)).ok()
    }
    pub unsafe fn OperatingSystemVersion(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OperatingSystemVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOperatingSystemVersion(&self, bstroperatingsystemversion: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOperatingSystemVersion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstroperatingsystemversion)).ok()
    }
    pub unsafe fn Model(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Model)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetModel(&self, bstrmodel: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetModel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmodel)).ok()
    }
    pub unsafe fn Processor(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Processor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProcessor(&self, bstrprocessor: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProcessor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprocessor)).ok()
    }
    pub unsafe fn ProcessorCount(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ProcessorCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProcessorCount(&self, bstrprocessorcount: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProcessorCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprocessorcount)).ok()
    }
    pub unsafe fn MemorySize(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MemorySize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMemorySize(&self, bstrmemorysize: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMemorySize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmemorysize)).ok()
    }
    pub unsafe fn StorageCapacity(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StorageCapacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStorageCapacity(&self, bstrstoragecapacity: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStorageCapacity)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrstoragecapacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NetAddresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NetAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetNetAddresses(&self, vnetaddresses: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNetAddresses)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vnetaddresses)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsComputer, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsComputer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsComputer {
    type Vtable = IADsComputer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsComputer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefe3cc70_1d9f_11cf_b1f3_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsComputer_Vtbl {
    pub base__: IADs_Vtbl,
    pub ComputerID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Site: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlocation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrimaryUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrimaryUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprimaryuser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Owner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrowner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Division: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDivision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdivision: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Department: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdepartment: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Role: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrole: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroperatingsystem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OperatingSystemVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOperatingSystemVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroperatingsystemversion: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmodel: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Processor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetProcessor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprocessor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetProcessorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprocessorcount: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MemorySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMemorySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmemorysize: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StorageCapacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStorageCapacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstoragecapacity: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NetAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NetAddresses: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetNetAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vnetaddresses: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetNetAddresses: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsComputerOperations(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsComputerOperations {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Status)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Shutdown<P0>(&self, breboot: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Shutdown)(::windows::core::Vtable::as_raw(self), breboot.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsComputerOperations, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsComputerOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsComputerOperations {
    type Vtable = IADsComputerOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsComputerOperations {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef497680_1d9f_11cf_b1f3_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsComputerOperations_Vtbl {
    pub base__: IADs_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, breboot: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Shutdown: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsContainer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsContainer {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Filter(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Filter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetFilter(&self, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFilter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(var)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Hints(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Hints)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetHints(&self, vhints: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHints)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vhints)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetObject(&self, classname: &::windows::core::BSTR, relativename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetObject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(classname), ::core::mem::transmute_copy(relativename), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create(&self, classname: &::windows::core::BSTR, relativename: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Create)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(classname), ::core::mem::transmute_copy(relativename), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self, bstrclassname: &::windows::core::BSTR, bstrrelativename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrclassname), ::core::mem::transmute_copy(bstrrelativename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyHere(&self, sourcename: &::windows::core::BSTR, newname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CopyHere)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(sourcename), ::core::mem::transmute_copy(newname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoveHere(&self, sourcename: &::windows::core::BSTR, newname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveHere)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(sourcename), ::core::mem::transmute_copy(newname), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsContainer, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsContainer {
    type Vtable = IADsContainer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x001677d0_fd16_11ce_abc4_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsContainer_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Filter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Filter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, var: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Hints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Hints: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetHints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vhints: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetHints: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classname: *mut ::core::ffi::c_void, relativename: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classname: *mut ::core::ffi::c_void, relativename: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclassname: *mut ::core::ffi::c_void, bstrrelativename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyHere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcename: *mut ::core::ffi::c_void, newname: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyHere: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MoveHere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcename: *mut ::core::ffi::c_void, newname: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoveHere: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsDNWithBinary(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsDNWithBinary {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BinaryValue(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BinaryValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBinaryValue(&self, vbinaryvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBinaryValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vbinaryvalue)).ok()
    }
    pub unsafe fn DNString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DNString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDNString(&self, bstrdnstring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDNString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdnstring)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsDNWithBinary, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsDNWithBinary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsDNWithBinary {
    type Vtable = IADsDNWithBinary_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsDNWithBinary {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e99c0a2_f935_11d2_ba96_00c04fb6d0d1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDNWithBinary_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BinaryValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BinaryValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBinaryValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbinaryvalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBinaryValue: usize,
    pub DNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdnstring: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsDNWithString(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsDNWithString {
    pub unsafe fn StringValue(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StringValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStringValue(&self, bstrstringvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStringValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrstringvalue)).ok()
    }
    pub unsafe fn DNString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DNString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDNString(&self, bstrdnstring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDNString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdnstring)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsDNWithString, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsDNWithString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsDNWithString {
    type Vtable = IADsDNWithString_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsDNWithString {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x370df02e_f934_11d2_ba96_00c04fb6d0d1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDNWithString_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstringvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdnstring: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsDeleteOps(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsDeleteOps {
    pub unsafe fn DeleteObject(&self, lnflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteObject)(::windows::core::Vtable::as_raw(self), lnflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsDeleteOps, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsDeleteOps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsDeleteOps {
    type Vtable = IADsDeleteOps_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsDeleteOps {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2bd0902_8878_11d1_8c21_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDeleteOps_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DeleteObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsDomain(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsDomain {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkgroup(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsWorkgroup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MinPasswordLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MinPasswordLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMinPasswordLength(&self, lnminpasswordlength: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMinPasswordLength)(::windows::core::Vtable::as_raw(self), lnminpasswordlength).ok()
    }
    pub unsafe fn MinPasswordAge(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MinPasswordAge)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMinPasswordAge(&self, lnminpasswordage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMinPasswordAge)(::windows::core::Vtable::as_raw(self), lnminpasswordage).ok()
    }
    pub unsafe fn MaxPasswordAge(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaxPasswordAge)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxPasswordAge(&self, lnmaxpasswordage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMaxPasswordAge)(::windows::core::Vtable::as_raw(self), lnmaxpasswordage).ok()
    }
    pub unsafe fn MaxBadPasswordsAllowed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaxBadPasswordsAllowed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxBadPasswordsAllowed(&self, lnmaxbadpasswordsallowed: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMaxBadPasswordsAllowed)(::windows::core::Vtable::as_raw(self), lnmaxbadpasswordsallowed).ok()
    }
    pub unsafe fn PasswordHistoryLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PasswordHistoryLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPasswordHistoryLength(&self, lnpasswordhistorylength: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPasswordHistoryLength)(::windows::core::Vtable::as_raw(self), lnpasswordhistorylength).ok()
    }
    pub unsafe fn PasswordAttributes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PasswordAttributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPasswordAttributes(&self, lnpasswordattributes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPasswordAttributes)(::windows::core::Vtable::as_raw(self), lnpasswordattributes).ok()
    }
    pub unsafe fn AutoUnlockInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoUnlockInterval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAutoUnlockInterval(&self, lnautounlockinterval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAutoUnlockInterval)(::windows::core::Vtable::as_raw(self), lnautounlockinterval).ok()
    }
    pub unsafe fn LockoutObservationInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LockoutObservationInterval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLockoutObservationInterval(&self, lnlockoutobservationinterval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLockoutObservationInterval)(::windows::core::Vtable::as_raw(self), lnlockoutobservationinterval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsDomain, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsDomain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsDomain {
    type Vtable = IADsDomain_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsDomain {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00e4c220_fd16_11ce_abc4_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDomain_Vtbl {
    pub base__: IADs_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWorkgroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWorkgroup: usize,
    pub MinPasswordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMinPasswordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnminpasswordlength: i32) -> ::windows::core::HRESULT,
    pub MinPasswordAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMinPasswordAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnminpasswordage: i32) -> ::windows::core::HRESULT,
    pub MaxPasswordAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxPasswordAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxpasswordage: i32) -> ::windows::core::HRESULT,
    pub MaxBadPasswordsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxBadPasswordsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxbadpasswordsallowed: i32) -> ::windows::core::HRESULT,
    pub PasswordHistoryLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetPasswordHistoryLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnpasswordhistorylength: i32) -> ::windows::core::HRESULT,
    pub PasswordAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetPasswordAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnpasswordattributes: i32) -> ::windows::core::HRESULT,
    pub AutoUnlockInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAutoUnlockInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnautounlockinterval: i32) -> ::windows::core::HRESULT,
    pub LockoutObservationInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetLockoutObservationInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnlockoutobservationinterval: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsEmail(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsEmail {
    pub unsafe fn Type(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetType(&self, lntype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetType)(::windows::core::Vtable::as_raw(self), lntype).ok()
    }
    pub unsafe fn Address(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Address)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAddress(&self, bstraddress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAddress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstraddress)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsEmail, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsEmail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsEmail {
    type Vtable = IADsEmail_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsEmail {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97af011a_478e_11d1_a3b4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsEmail_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IADsExtension(::windows::core::IUnknown);
impl IADsExtension {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operate(&self, dwcode: u32, vardata1: super::super::System::Com::VARIANT, vardata2: super::super::System::Com::VARIANT, vardata3: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Operate)(::windows::core::Vtable::as_raw(self), dwcode, ::core::mem::transmute(vardata1), ::core::mem::transmute(vardata2), ::core::mem::transmute(vardata3)).ok()
    }
    pub unsafe fn PrivateGetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateGetIDsOfNames)(::windows::core::Vtable::as_raw(self), riid, rgsznames, cnames, lcid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PrivateInvoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PrivateInvoke)(::windows::core::Vtable::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, pvarresult, pexcepinfo, puargerr).ok()
    }
}
::windows::core::interface_hierarchy!(IADsExtension, ::windows::core::IUnknown);
impl ::core::clone::Clone for IADsExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IADsExtension {
    type Vtable = IADsExtension_Vtbl;
}
unsafe impl ::windows::core::Interface for IADsExtension {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d35553c_d2b0_11d1_b17b_0000f87593a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsExtension_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Operate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcode: u32, vardata1: super::super::System::Com::VARIANT, vardata2: super::super::System::Com::VARIANT, vardata3: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Operate: usize,
    pub PrivateGetIDsOfNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PrivateInvoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PrivateInvoke: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsFaxNumber(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsFaxNumber {
    pub unsafe fn TelephoneNumber(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TelephoneNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTelephoneNumber(&self, bstrtelephonenumber: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTelephoneNumber)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtelephonenumber)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetParameters(&self, vparameters: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetParameters)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vparameters)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsFaxNumber, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsFaxNumber {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsFaxNumber {
    type Vtable = IADsFaxNumber_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsFaxNumber {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa910dea9_4680_11d1_a3b4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFaxNumber_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub TelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtelephonenumber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Parameters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vparameters: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetParameters: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsFileService(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsFileService {
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn MaxUserCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaxUserCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxUserCount(&self, lnmaxusercount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMaxUserCount)(::windows::core::Vtable::as_raw(self), lnmaxusercount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsFileService, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs, IADsService);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsFileService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsFileService {
    type Vtable = IADsFileService_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsFileService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa89d1900_31ca_11cf_a98a_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileService_Vtbl {
    pub base__: IADsService_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MaxUserCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxUserCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsFileServiceOperations(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsFileServiceOperations {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sessions(&self) -> ::windows::core::Result<IADsCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Sessions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resources(&self) -> ::windows::core::Result<IADsCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Resources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsFileServiceOperations, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs, IADsServiceOperations);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsFileServiceOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsFileServiceOperations {
    type Vtable = IADsFileServiceOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsFileServiceOperations {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa02ded10_31ca_11cf_a98a_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileServiceOperations_Vtbl {
    pub base__: IADsServiceOperations_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Sessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsessions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sessions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resources: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsFileShare(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsFileShare {
    pub unsafe fn CurrentUserCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentUserCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn HostComputer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HostComputer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHostComputer(&self, bstrhostcomputer: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHostComputer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrhostcomputer)).ok()
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPath(&self, bstrpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpath)).ok()
    }
    pub unsafe fn MaxUserCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaxUserCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxUserCount(&self, lnmaxusercount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMaxUserCount)(::windows::core::Vtable::as_raw(self), lnmaxusercount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsFileShare, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsFileShare {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsFileShare {
    type Vtable = IADsFileShare_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsFileShare {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb6dcaf0_4b83_11cf_a995_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileShare_Vtbl {
    pub base__: IADs_Vtbl,
    pub CurrentUserCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HostComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHostComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhostcomputer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MaxUserCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxUserCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsGroup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsGroup {
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Members(&self) -> ::windows::core::Result<IADsMembers> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Members)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMember(&self, bstrmember: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsMember)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmember), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Add(&self, bstrnewitem: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnewitem)).ok()
    }
    pub unsafe fn Remove(&self, bstritemtoberemoved: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstritemtoberemoved)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsGroup, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsGroup {
    type Vtable = IADsGroup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27636b00_410f_11cf_b1ff_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsGroup_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmembers: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Members: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmember: *mut ::core::ffi::c_void, bmember: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMember: usize,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnewitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemtoberemoved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsHold(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsHold {
    pub unsafe fn ObjectName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ObjectName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetObjectName(&self, bstrobjectname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetObjectName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrobjectname)).ok()
    }
    pub unsafe fn Amount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Amount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAmount(&self, lnamount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAmount)(::windows::core::Vtable::as_raw(self), lnamount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsHold, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsHold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsHold {
    type Vtable = IADsHold_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsHold {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3eb3b37_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsHold_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjectname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Amount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnamount: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsLargeInteger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsLargeInteger {
    pub unsafe fn HighPart(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HighPart)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHighPart(&self, lnhighpart: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHighPart)(::windows::core::Vtable::as_raw(self), lnhighpart).ok()
    }
    pub unsafe fn LowPart(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LowPart)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLowPart(&self, lnlowpart: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLowPart)(::windows::core::Vtable::as_raw(self), lnlowpart).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsLargeInteger, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsLargeInteger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsLargeInteger {
    type Vtable = IADsLargeInteger_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsLargeInteger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9068270b_0939_11d1_8be1_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsLargeInteger_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub HighPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetHighPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnhighpart: i32) -> ::windows::core::HRESULT,
    pub LowPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetLowPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnlowpart: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsLocality(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsLocality {
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn LocalityName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalityName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLocalityName(&self, bstrlocalityname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLocalityName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrlocalityname)).ok()
    }
    pub unsafe fn PostalAddress(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PostalAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPostalAddress(&self, bstrpostaladdress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPostalAddress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpostaladdress)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SeeAlso(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SeeAlso)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSeeAlso)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vseealso)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsLocality, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsLocality {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsLocality {
    type Vtable = IADsLocality_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsLocality {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa05e03a2_effe_11cf_8abc_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsLocality_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlocalityname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpostaladdress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSeeAlso: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsMembers(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsMembers {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Filter(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Filter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetFilter(&self, pvfilter: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFilter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pvfilter)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsMembers, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsMembers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsMembers {
    type Vtable = IADsMembers_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsMembers {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x451a0030_72ec_11cf_b03b_00aa006e0975);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsMembers_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Filter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Filter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvfilter: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetFilter: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsNameTranslate(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsNameTranslate {
    pub unsafe fn SetChaseReferral(&self, lnchasereferral: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetChaseReferral)(::windows::core::Vtable::as_raw(self), lnchasereferral).ok()
    }
    pub unsafe fn Init(&self, lnsettype: i32, bstradspath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Init)(::windows::core::Vtable::as_raw(self), lnsettype, ::core::mem::transmute_copy(bstradspath)).ok()
    }
    pub unsafe fn InitEx(&self, lnsettype: i32, bstradspath: &::windows::core::BSTR, bstruserid: &::windows::core::BSTR, bstrdomain: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InitEx)(::windows::core::Vtable::as_raw(self), lnsettype, ::core::mem::transmute_copy(bstradspath), ::core::mem::transmute_copy(bstruserid), ::core::mem::transmute_copy(bstrdomain), ::core::mem::transmute_copy(bstrpassword)).ok()
    }
    pub unsafe fn Set(&self, lnsettype: i32, bstradspath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Set)(::windows::core::Vtable::as_raw(self), lnsettype, ::core::mem::transmute_copy(bstradspath)).ok()
    }
    pub unsafe fn Get(&self, lnformattype: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Get)(::windows::core::Vtable::as_raw(self), lnformattype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetEx(&self, lnformattype: i32, pvar: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEx)(::windows::core::Vtable::as_raw(self), lnformattype, ::core::mem::transmute(pvar)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetEx(&self, lnformattype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEx)(::windows::core::Vtable::as_raw(self), lnformattype, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsNameTranslate, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsNameTranslate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsNameTranslate {
    type Vtable = IADsNameTranslate_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsNameTranslate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1b272a3_3625_11d1_a3a4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNameTranslate_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetChaseReferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnchasereferral: i32) -> ::windows::core::HRESULT,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InitEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: *mut ::core::ffi::c_void, bstruserid: *mut ::core::ffi::c_void, bstrdomain: *mut ::core::ffi::c_void, bstrpassword: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetEx: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetEx: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsNamespaces(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsNamespaces {
    pub unsafe fn DefaultContainer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DefaultContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDefaultContainer(&self, bstrdefaultcontainer: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultContainer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdefaultcontainer)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsNamespaces, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsNamespaces {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsNamespaces {
    type Vtable = IADsNamespaces_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsNamespaces {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28b96ba0_b330_11cf_a9ad_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNamespaces_Vtbl {
    pub base__: IADs_Vtbl,
    pub DefaultContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDefaultContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdefaultcontainer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsNetAddress(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsNetAddress {
    pub unsafe fn AddressType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddressType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAddressType(&self, lnaddresstype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAddressType)(::windows::core::Vtable::as_raw(self), lnaddresstype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Address(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Address)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetAddress(&self, vaddress: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAddress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vaddress)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsNetAddress, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsNetAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsNetAddress {
    type Vtable = IADsNetAddress_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsNetAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb21a50a9_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNetAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AddressType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAddressType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnaddresstype: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Address: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vaddress: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetAddress: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsO(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsO {
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn LocalityName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalityName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLocalityName(&self, bstrlocalityname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLocalityName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrlocalityname)).ok()
    }
    pub unsafe fn PostalAddress(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PostalAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPostalAddress(&self, bstrpostaladdress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPostalAddress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpostaladdress)).ok()
    }
    pub unsafe fn TelephoneNumber(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TelephoneNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTelephoneNumber(&self, bstrtelephonenumber: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTelephoneNumber)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtelephonenumber)).ok()
    }
    pub unsafe fn FaxNumber(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FaxNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFaxNumber(&self, bstrfaxnumber: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFaxNumber)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfaxnumber)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SeeAlso(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SeeAlso)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSeeAlso)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vseealso)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsO, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsO {
    type Vtable = IADsO_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsO {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1cd2dc6_effe_11cf_8abc_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsO_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlocalityname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpostaladdress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtelephonenumber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxnumber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSeeAlso: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsOU(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsOU {
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn LocalityName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalityName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLocalityName(&self, bstrlocalityname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLocalityName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrlocalityname)).ok()
    }
    pub unsafe fn PostalAddress(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PostalAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPostalAddress(&self, bstrpostaladdress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPostalAddress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpostaladdress)).ok()
    }
    pub unsafe fn TelephoneNumber(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TelephoneNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTelephoneNumber(&self, bstrtelephonenumber: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTelephoneNumber)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtelephonenumber)).ok()
    }
    pub unsafe fn FaxNumber(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FaxNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFaxNumber(&self, bstrfaxnumber: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFaxNumber)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfaxnumber)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SeeAlso(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SeeAlso)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSeeAlso)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vseealso)).ok()
    }
    pub unsafe fn BusinessCategory(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BusinessCategory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBusinessCategory(&self, bstrbusinesscategory: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBusinessCategory)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrbusinesscategory)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsOU, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsOU {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsOU {
    type Vtable = IADsOU_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsOU {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2f733b8_effe_11cf_8abc_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOU_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlocalityname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpostaladdress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtelephonenumber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxnumber: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSeeAlso: usize,
    pub BusinessCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBusinessCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbusinesscategory: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsObjectOptions(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsObjectOptions {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetOption(&self, lnoption: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOption)(::windows::core::Vtable::as_raw(self), lnoption, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetOption(&self, lnoption: i32, vvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOption)(::windows::core::Vtable::as_raw(self), lnoption, ::core::mem::transmute(vvalue)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsObjectOptions, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsObjectOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsObjectOptions {
    type Vtable = IADsObjectOptions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsObjectOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46f14fda_232b_11d1_a808_00c04fd8d5a8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsObjectOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnoption: i32, pvvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetOption: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnoption: i32, vvalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetOption: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsOctetList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsOctetList {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OctetList(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OctetList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetOctetList(&self, voctetlist: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOctetList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(voctetlist)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsOctetList, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsOctetList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsOctetList {
    type Vtable = IADsOctetList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsOctetList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b28b80f_4680_11d1_a3b4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOctetList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OctetList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OctetList: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetOctetList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voctetlist: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetOctetList: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsOpenDSObject(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsOpenDSObject {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenDSObject(&self, lpszdnname: &::windows::core::BSTR, lpszusername: &::windows::core::BSTR, lpszpassword: &::windows::core::BSTR, lnreserved: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenDSObject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(lpszdnname), ::core::mem::transmute_copy(lpszusername), ::core::mem::transmute_copy(lpszpassword), lnreserved, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsOpenDSObject, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsOpenDSObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsOpenDSObject {
    type Vtable = IADsOpenDSObject_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsOpenDSObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddf2891e_0f9c_11d0_8ad4_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOpenDSObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenDSObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszdnname: *mut ::core::ffi::c_void, lpszusername: *mut ::core::ffi::c_void, lpszpassword: *mut ::core::ffi::c_void, lnreserved: i32, ppoledsobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenDSObject: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPath(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPath {
    pub unsafe fn Type(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetType(&self, lntype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetType)(::windows::core::Vtable::as_raw(self), lntype).ok()
    }
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).VolumeName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVolumeName(&self, bstrvolumename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVolumeName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrvolumename)).ok()
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPath(&self, bstrpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpath)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPath, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPath {
    type Vtable = IADsPath_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPath {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb287fcd5_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPath_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows::core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvolumename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPathname(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPathname {
    pub unsafe fn Set(&self, bstradspath: &::windows::core::BSTR, lnsettype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Set)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstradspath), lnsettype).ok()
    }
    pub unsafe fn SetDisplayType(&self, lndisplaytype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDisplayType)(::windows::core::Vtable::as_raw(self), lndisplaytype).ok()
    }
    pub unsafe fn Retrieve(&self, lnformattype: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Retrieve)(::windows::core::Vtable::as_raw(self), lnformattype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNumElements(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNumElements)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetElement(&self, lnelementindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetElement)(::windows::core::Vtable::as_raw(self), lnelementindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddLeafElement(&self, bstrleafelement: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddLeafElement)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrleafelement)).ok()
    }
    pub unsafe fn RemoveLeafElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveLeafElement)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyPath(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CopyPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEscapedElement(&self, lnreserved: i32, bstrinstr: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEscapedElement)(::windows::core::Vtable::as_raw(self), lnreserved, ::core::mem::transmute_copy(bstrinstr), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EscapedMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EscapedMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEscapedMode(&self, lnescapedmode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEscapedMode)(::windows::core::Vtable::as_raw(self), lnescapedmode).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPathname, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPathname {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPathname {
    type Vtable = IADsPathname_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPathname {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd592aed4_f420_11d0_a36e_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPathname_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradspath: *mut ::core::ffi::c_void, lnsettype: i32) -> ::windows::core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lndisplaytype: i32) -> ::windows::core::HRESULT,
    pub Retrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plnnumpathelements: *mut i32) -> ::windows::core::HRESULT,
    pub GetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnelementindex: i32, pbstrelement: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddLeafElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrleafelement: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveLeafElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppadspath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyPath: usize,
    pub GetEscapedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnreserved: i32, bstrinstr: *mut ::core::ffi::c_void, pbstroutstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EscapedMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetEscapedMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnescapedmode: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPostalAddress(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPostalAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PostalAddress(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PostalAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPostalAddress(&self, vpostaladdress: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPostalAddress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vpostaladdress)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPostalAddress, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPostalAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPostalAddress {
    type Vtable = IADsPostalAddress_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPostalAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7adecf29_4680_11d1_a3b4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPostalAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PostalAddress: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vpostaladdress: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPostalAddress: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPrintJob(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintJob {
    pub unsafe fn HostPrintQueue(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HostPrintQueue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn User(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).User)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TimeSubmitted(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TimeSubmitted)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalPages(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalPages)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Size(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Size)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Priority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPriority(&self, lnpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPriority)(::windows::core::Vtable::as_raw(self), lnpriority).ok()
    }
    pub unsafe fn StartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StartTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStartTime(&self, dastarttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStartTime)(::windows::core::Vtable::as_raw(self), dastarttime).ok()
    }
    pub unsafe fn UntilTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UntilTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUntilTime(&self, dauntiltime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUntilTime)(::windows::core::Vtable::as_raw(self), dauntiltime).ok()
    }
    pub unsafe fn Notify(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Notify)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNotify(&self, bstrnotify: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNotify)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnotify)).ok()
    }
    pub unsafe fn NotifyPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NotifyPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNotifyPath(&self, bstrnotifypath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNotifyPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnotifypath)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPrintJob, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPrintJob {
    type Vtable = IADsPrintJob_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPrintJob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32fb6780_1ed0_11cf_a988_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintJob_Vtbl {
    pub base__: IADs_Vtbl,
    pub HostPrintQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TimeSubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub TotalPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows::core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows::core::HRESULT,
    pub UntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub SetUntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows::core::HRESULT,
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnotify: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNotifyPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnotifypath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPrintJobOperations(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintJobOperations {
    pub unsafe fn Status(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Status)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TimeElapsed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TimeElapsed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PagesPrinted(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PagesPrinted)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Position(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Position)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPosition(&self, lnposition: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPosition)(::windows::core::Vtable::as_raw(self), lnposition).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPrintJobOperations, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPrintJobOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPrintJobOperations {
    type Vtable = IADsPrintJobOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPrintJobOperations {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a52db30_1ecf_11cf_a988_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintJobOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub TimeElapsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub PagesPrinted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnposition: i32) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPrintQueue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintQueue {
    pub unsafe fn PrinterPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrinterPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrinterPath(&self, bstrprinterpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPrinterPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprinterpath)).ok()
    }
    pub unsafe fn Model(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Model)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetModel(&self, bstrmodel: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetModel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmodel)).ok()
    }
    pub unsafe fn Datatype(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Datatype)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDatatype(&self, bstrdatatype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDatatype)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdatatype)).ok()
    }
    pub unsafe fn PrintProcessor(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrintProcessor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrintProcessor(&self, bstrprintprocessor: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPrintProcessor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprintprocessor)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn Location(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Location)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLocation(&self, bstrlocation: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLocation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrlocation)).ok()
    }
    pub unsafe fn StartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StartTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStartTime(&self, dastarttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStartTime)(::windows::core::Vtable::as_raw(self), dastarttime).ok()
    }
    pub unsafe fn UntilTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UntilTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUntilTime(&self, dauntiltime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUntilTime)(::windows::core::Vtable::as_raw(self), dauntiltime).ok()
    }
    pub unsafe fn DefaultJobPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DefaultJobPriority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDefaultJobPriority(&self, lndefaultjobpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultJobPriority)(::windows::core::Vtable::as_raw(self), lndefaultjobpriority).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Priority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPriority(&self, lnpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPriority)(::windows::core::Vtable::as_raw(self), lnpriority).ok()
    }
    pub unsafe fn BannerPage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BannerPage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBannerPage(&self, bstrbannerpage: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBannerPage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrbannerpage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PrintDevices(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrintDevices)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPrintDevices(&self, vprintdevices: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPrintDevices)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vprintdevices)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NetAddresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NetAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetNetAddresses(&self, vnetaddresses: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNetAddresses)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vnetaddresses)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPrintQueue, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPrintQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPrintQueue {
    type Vtable = IADsPrintQueue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPrintQueue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb15160d0_1226_11cf_a985_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintQueue_Vtbl {
    pub base__: IADs_Vtbl,
    pub PrinterPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrinterPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprinterpath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmodel: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Datatype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDatatype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdatatype: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrintProcessor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrintProcessor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprintprocessor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlocation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows::core::HRESULT,
    pub UntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub SetUntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows::core::HRESULT,
    pub DefaultJobPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetDefaultJobPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lndefaultjobpriority: i32) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows::core::HRESULT,
    pub BannerPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBannerPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbannerpage: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PrintDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PrintDevices: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPrintDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vprintdevices: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPrintDevices: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NetAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NetAddresses: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetNetAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vnetaddresses: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetNetAddresses: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPrintQueueOperations(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintQueueOperations {
    pub unsafe fn Status(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Status)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrintJobs(&self) -> ::windows::core::Result<IADsCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrintJobs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Purge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Purge)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPrintQueueOperations, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPrintQueueOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPrintQueueOperations {
    type Vtable = IADsPrintQueueOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPrintQueueOperations {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x124be5c0_156e_11cf_a986_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintQueueOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PrintJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrintJobs: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Purge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsProperty {
    pub unsafe fn OID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOID(&self, bstroid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstroid)).ok()
    }
    pub unsafe fn Syntax(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Syntax)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSyntax(&self, bstrsyntax: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSyntax)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrsyntax)).ok()
    }
    pub unsafe fn MaxRange(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaxRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxRange(&self, lnmaxrange: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMaxRange)(::windows::core::Vtable::as_raw(self), lnmaxrange).ok()
    }
    pub unsafe fn MinRange(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MinRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMinRange(&self, lnminrange: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMinRange)(::windows::core::Vtable::as_raw(self), lnminrange).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MultiValued(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MultiValued)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMultiValued<P0>(&self, fmultivalued: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetMultiValued)(::windows::core::Vtable::as_raw(self), fmultivalued.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers(&self) -> ::windows::core::Result<IADsCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Qualifiers)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsProperty, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsProperty {
    type Vtable = IADsProperty_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8f93dd3_4ae0_11cf_9e73_00aa004a5691);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsProperty_Vtbl {
    pub base__: IADs_Vtbl,
    pub OID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Syntax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSyntax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsyntax: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MaxRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxrange: i32) -> ::windows::core::HRESULT,
    pub MinRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMinRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnminrange: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MultiValued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MultiValued: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMultiValued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmultivalued: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMultiValued: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqualifiers: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPropertyEntry(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyEntry {
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn ADsType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ADsType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetADsType(&self, lnadstype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetADsType)(::windows::core::Vtable::as_raw(self), lnadstype).ok()
    }
    pub unsafe fn ControlCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ControlCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetControlCode(&self, lncontrolcode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetControlCode)(::windows::core::Vtable::as_raw(self), lncontrolcode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Values(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Values)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValues(&self, vvalues: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValues)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vvalues)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPropertyEntry, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPropertyEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPropertyEntry {
    type Vtable = IADsPropertyEntry_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPropertyEntry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05792c8e_941f_11d0_8529_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyEntry_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ADsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetADsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows::core::HRESULT,
    pub ControlCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetControlCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lncontrolcode: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Values: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Values: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vvalues: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValues: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPropertyList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyList {
    pub unsafe fn PropertyCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), pvariant)
    }
    pub unsafe fn Skip(&self, celements: i32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celements)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Item)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varindex), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPropertyItem(&self, bstrname: &::windows::core::BSTR, lnadstype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPropertyItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), lnadstype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutPropertyItem(&self, vardata: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PutPropertyItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vardata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ResetPropertyItem(&self, varentry: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ResetPropertyItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varentry)).ok()
    }
    pub unsafe fn PurgePropertyList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PurgePropertyList)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPropertyList, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPropertyList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPropertyList {
    type Vtable = IADsPropertyList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPropertyList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6f602b6_8f69_11d0_8528_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub PropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celements: i32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Com::VARIANT, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, lnadstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PutPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardata: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PutPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ResetPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varentry: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ResetPropertyItem: usize,
    pub PurgePropertyList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPropertyValue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyValue {
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ADsType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ADsType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetADsType(&self, lnadstype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetADsType)(::windows::core::Vtable::as_raw(self), lnadstype).ok()
    }
    pub unsafe fn DNString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DNString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDNString(&self, bstrdnstring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDNString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdnstring)).ok()
    }
    pub unsafe fn CaseExactString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CaseExactString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCaseExactString(&self, bstrcaseexactstring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCaseExactString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcaseexactstring)).ok()
    }
    pub unsafe fn CaseIgnoreString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CaseIgnoreString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCaseIgnoreString(&self, bstrcaseignorestring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCaseIgnoreString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcaseignorestring)).ok()
    }
    pub unsafe fn PrintableString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrintableString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrintableString(&self, bstrprintablestring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPrintableString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprintablestring)).ok()
    }
    pub unsafe fn NumericString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NumericString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNumericString(&self, bstrnumericstring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNumericString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnumericstring)).ok()
    }
    pub unsafe fn Boolean(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Boolean)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBoolean(&self, lnboolean: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBoolean)(::windows::core::Vtable::as_raw(self), lnboolean).ok()
    }
    pub unsafe fn Integer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Integer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInteger(&self, lninteger: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInteger)(::windows::core::Vtable::as_raw(self), lninteger).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OctetString(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OctetString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetOctetString(&self, voctetstring: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOctetString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(voctetstring)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityDescriptor(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SecurityDescriptor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<P0>(&self, psecuritydescriptor: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).SetSecurityDescriptor)(::windows::core::Vtable::as_raw(self), psecuritydescriptor.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LargeInteger(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LargeInteger)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetLargeInteger<P0>(&self, plargeinteger: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).SetLargeInteger)(::windows::core::Vtable::as_raw(self), plargeinteger.into().abi()).ok()
    }
    pub unsafe fn UTCTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UTCTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUTCTime(&self, dautctime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUTCTime)(::windows::core::Vtable::as_raw(self), dautctime).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPropertyValue, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPropertyValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPropertyValue {
    type Vtable = IADsPropertyValue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPropertyValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79fa9ad0_a97c_11d0_8534_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyValue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ADsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetADsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows::core::HRESULT,
    pub DNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdnstring: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CaseExactString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCaseExactString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcaseexactstring: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CaseIgnoreString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCaseIgnoreString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcaseignorestring: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrintableString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrintableString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprintablestring: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NumericString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNumericString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnumericstring: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Boolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnboolean: i32) -> ::windows::core::HRESULT,
    pub Integer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lninteger: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OctetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OctetString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetOctetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voctetstring: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetOctetString: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecuritydescriptor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LargeInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LargeInteger: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetLargeInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plargeinteger: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetLargeInteger: usize,
    pub UTCTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub SetUTCTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dautctime: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsPropertyValue2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyValue2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetObjectProperty(&self, lnadstype: *mut i32, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetObjectProperty)(::windows::core::Vtable::as_raw(self), lnadstype, pvprop).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutObjectProperty(&self, lnadstype: i32, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PutObjectProperty)(::windows::core::Vtable::as_raw(self), lnadstype, ::core::mem::transmute(vprop)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsPropertyValue2, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsPropertyValue2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsPropertyValue2 {
    type Vtable = IADsPropertyValue2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsPropertyValue2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x306e831c_5bc7_11d1_a3b8_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyValue2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetObjectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnadstype: *mut i32, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetObjectProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PutObjectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnadstype: i32, vprop: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PutObjectProperty: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsReplicaPointer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsReplicaPointer {
    pub unsafe fn ServerName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServerName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetServerName(&self, bstrservername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetServerName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrservername)).ok()
    }
    pub unsafe fn ReplicaType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReplicaType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetReplicaType(&self, lnreplicatype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetReplicaType)(::windows::core::Vtable::as_raw(self), lnreplicatype).ok()
    }
    pub unsafe fn ReplicaNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReplicaNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetReplicaNumber(&self, lnreplicanumber: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetReplicaNumber)(::windows::core::Vtable::as_raw(self), lnreplicanumber).ok()
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCount(&self, lncount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCount)(::windows::core::Vtable::as_raw(self), lncount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReplicaAddressHints(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReplicaAddressHints)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetReplicaAddressHints(&self, vreplicaaddresshints: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetReplicaAddressHints)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vreplicaaddresshints)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsReplicaPointer, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsReplicaPointer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsReplicaPointer {
    type Vtable = IADsReplicaPointer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsReplicaPointer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf60fb803_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsReplicaPointer_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ServerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetServerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrservername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReplicaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetReplicaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnreplicatype: i32) -> ::windows::core::HRESULT,
    pub ReplicaNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetReplicaNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnreplicanumber: i32) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lncount: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReplicaAddressHints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReplicaAddressHints: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetReplicaAddressHints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vreplicaaddresshints: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetReplicaAddressHints: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsResource(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsResource {
    pub unsafe fn User(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).User)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LockCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LockCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsResource, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsResource {
    type Vtable = IADsResource_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a05b20_4aab_11cf_ae2c_00aa006ebfb9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsResource_Vtbl {
    pub base__: IADs_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LockCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsSecurityDescriptor(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsSecurityDescriptor {
    pub unsafe fn Revision(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Revision)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRevision(&self, lnrevision: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRevision)(::windows::core::Vtable::as_raw(self), lnrevision).ok()
    }
    pub unsafe fn Control(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Control)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetControl(&self, lncontrol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetControl)(::windows::core::Vtable::as_raw(self), lncontrol).ok()
    }
    pub unsafe fn Owner(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Owner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOwner(&self, bstrowner: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOwner)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrowner)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerDefaulted(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OwnerDefaulted)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOwnerDefaulted<P0>(&self, fownerdefaulted: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetOwnerDefaulted)(::windows::core::Vtable::as_raw(self), fownerdefaulted.into()).ok()
    }
    pub unsafe fn Group(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Group)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGroup(&self, bstrgroup: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGroup)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrgroup)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GroupDefaulted(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GroupDefaulted)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGroupDefaulted<P0>(&self, fgroupdefaulted: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetGroupDefaulted)(::windows::core::Vtable::as_raw(self), fgroupdefaulted.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DiscretionaryAcl(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DiscretionaryAcl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDiscretionaryAcl<P0>(&self, pdiscretionaryacl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).SetDiscretionaryAcl)(::windows::core::Vtable::as_raw(self), pdiscretionaryacl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DaclDefaulted(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DaclDefaulted)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDaclDefaulted<P0>(&self, fdacldefaulted: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetDaclDefaulted)(::windows::core::Vtable::as_raw(self), fdacldefaulted.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SystemAcl(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SystemAcl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSystemAcl<P0>(&self, psystemacl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).SetSystemAcl)(::windows::core::Vtable::as_raw(self), psystemacl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaclDefaulted(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SaclDefaulted)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSaclDefaulted<P0>(&self, fsacldefaulted: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetSaclDefaulted)(::windows::core::Vtable::as_raw(self), fsacldefaulted.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopySecurityDescriptor(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CopySecurityDescriptor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsSecurityDescriptor, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsSecurityDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsSecurityDescriptor {
    type Vtable = IADsSecurityDescriptor_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsSecurityDescriptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8c787ca_9bdd_11d0_852c_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSecurityDescriptor_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnrevision: i32) -> ::windows::core::HRESULT,
    pub Control: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lncontrol: i32) -> ::windows::core::HRESULT,
    pub Owner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrowner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OwnerDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OwnerDefaulted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOwnerDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fownerdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOwnerDefaulted: usize,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroup: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GroupDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GroupDefaulted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGroupDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgroupdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGroupDefaulted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DiscretionaryAcl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DiscretionaryAcl: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDiscretionaryAcl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdiscretionaryacl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDiscretionaryAcl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DaclDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DaclDefaulted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDaclDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDaclDefaulted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SystemAcl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SystemAcl: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSystemAcl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psystemacl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSystemAcl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SaclDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaclDefaulted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSaclDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSaclDefaulted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopySecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecuritydescriptor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopySecurityDescriptor: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsSecurityUtility(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsSecurityUtility {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSecurityDescriptor(&self, varpath: super::super::System::Com::VARIANT, lpathformat: i32, lformat: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSecurityDescriptor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varpath), lpathformat, lformat, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSecurityDescriptor(&self, varpath: super::super::System::Com::VARIANT, lpathformat: i32, vardata: super::super::System::Com::VARIANT, ldataformat: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSecurityDescriptor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varpath), lpathformat, ::core::mem::transmute(vardata), ldataformat).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ConvertSecurityDescriptor(&self, varsd: super::super::System::Com::VARIANT, ldataformat: i32, loutformat: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ConvertSecurityDescriptor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varsd), ldataformat, loutformat, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SecurityMask(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SecurityMask)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSecurityMask(&self, lnsecuritymask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSecurityMask)(::windows::core::Vtable::as_raw(self), lnsecuritymask).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsSecurityUtility, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsSecurityUtility {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsSecurityUtility {
    type Vtable = IADsSecurityUtility_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsSecurityUtility {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa63251b2_5f21_474b_ab52_4a8efad10895);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSecurityUtility_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varpath: super::super::System::Com::VARIANT, lpathformat: i32, lformat: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetSecurityDescriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varpath: super::super::System::Com::VARIANT, lpathformat: i32, vardata: super::super::System::Com::VARIANT, ldataformat: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSecurityDescriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ConvertSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsd: super::super::System::Com::VARIANT, ldataformat: i32, loutformat: i32, presult: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ConvertSecurityDescriptor: usize,
    pub SecurityMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetSecurityMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnsecuritymask: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsService(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsService {
    pub unsafe fn HostComputer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HostComputer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHostComputer(&self, bstrhostcomputer: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHostComputer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrhostcomputer)).ok()
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDisplayName(&self, bstrdisplayname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDisplayName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdisplayname)).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Version)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVersion(&self, bstrversion: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVersion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrversion)).ok()
    }
    pub unsafe fn ServiceType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServiceType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetServiceType(&self, lnservicetype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetServiceType)(::windows::core::Vtable::as_raw(self), lnservicetype).ok()
    }
    pub unsafe fn StartType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StartType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStartType(&self, lnstarttype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStartType)(::windows::core::Vtable::as_raw(self), lnstarttype).ok()
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPath(&self, bstrpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrpath)).ok()
    }
    pub unsafe fn StartupParameters(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StartupParameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStartupParameters(&self, bstrstartupparameters: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStartupParameters)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrstartupparameters)).ok()
    }
    pub unsafe fn ErrorControl(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ErrorControl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetErrorControl(&self, lnerrorcontrol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetErrorControl)(::windows::core::Vtable::as_raw(self), lnerrorcontrol).ok()
    }
    pub unsafe fn LoadOrderGroup(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoadOrderGroup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLoadOrderGroup(&self, bstrloadordergroup: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLoadOrderGroup)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrloadordergroup)).ok()
    }
    pub unsafe fn ServiceAccountName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServiceAccountName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetServiceAccountName(&self, bstrserviceaccountname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetServiceAccountName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrserviceaccountname)).ok()
    }
    pub unsafe fn ServiceAccountPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServiceAccountPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetServiceAccountPath(&self, bstrserviceaccountpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetServiceAccountPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrserviceaccountpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Dependencies(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Dependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDependencies(&self, vdependencies: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDependencies)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vdependencies)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsService, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsService {
    type Vtable = IADsService_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68af66e0_31ca_11cf_a98a_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsService_Vtbl {
    pub base__: IADs_Vtbl,
    pub HostComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHostComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhostcomputer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdisplayname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrversion: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ServiceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetServiceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnservicetype: i32) -> ::windows::core::HRESULT,
    pub StartType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetStartType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnstarttype: i32) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartupParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStartupParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstartupparameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ErrorControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetErrorControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnerrorcontrol: i32) -> ::windows::core::HRESULT,
    pub LoadOrderGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLoadOrderGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrloadordergroup: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ServiceAccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetServiceAccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserviceaccountname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ServiceAccountPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetServiceAccountPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserviceaccountpath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Dependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Dependencies: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vdependencies: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDependencies: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsServiceOperations(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsServiceOperations {
    pub unsafe fn Status(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Status)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Continue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Continue)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetPassword(&self, bstrnewpassword: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPassword)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnewpassword)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsServiceOperations, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsServiceOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsServiceOperations {
    type Vtable = IADsServiceOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsServiceOperations {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d7b33f0_31ca_11cf_a98a_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsServiceOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Continue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnewpassword: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsSession(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsSession {
    pub unsafe fn User(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).User)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Computer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Computer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ComputerPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ComputerPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ConnectTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ConnectTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IdleTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IdleTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsSession, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsSession {
    type Vtable = IADsSession_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x398b7da0_4aab_11cf_ae2c_00aa006ebfb9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSession_Vtbl {
    pub base__: IADs_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Computer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ComputerPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConnectTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub IdleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsSyntax(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsSyntax {
    pub unsafe fn OleAutoDataType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OleAutoDataType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOleAutoDataType(&self, lnoleautodatatype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOleAutoDataType)(::windows::core::Vtable::as_raw(self), lnoleautodatatype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsSyntax, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsSyntax {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsSyntax {
    type Vtable = IADsSyntax_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsSyntax {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8f93dd2_4ae0_11cf_9e73_00aa004a5691);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSyntax_Vtbl {
    pub base__: IADs_Vtbl,
    pub OleAutoDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetOleAutoDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnoleautodatatype: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsTimestamp(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsTimestamp {
    pub unsafe fn WholeSeconds(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WholeSeconds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetWholeSeconds(&self, lnwholeseconds: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetWholeSeconds)(::windows::core::Vtable::as_raw(self), lnwholeseconds).ok()
    }
    pub unsafe fn EventID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EventID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEventID(&self, lneventid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEventID)(::windows::core::Vtable::as_raw(self), lneventid).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsTimestamp, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsTimestamp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsTimestamp {
    type Vtable = IADsTimestamp_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsTimestamp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2f5a901_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTimestamp_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub WholeSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetWholeSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnwholeseconds: i32) -> ::windows::core::HRESULT,
    pub EventID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetEventID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lneventid: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsTypedName(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsTypedName {
    pub unsafe fn ObjectName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ObjectName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetObjectName(&self, bstrobjectname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetObjectName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrobjectname)).ok()
    }
    pub unsafe fn Level(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Level)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, lnlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLevel)(::windows::core::Vtable::as_raw(self), lnlevel).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Interval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInterval(&self, lninterval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInterval)(::windows::core::Vtable::as_raw(self), lninterval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsTypedName, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsTypedName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsTypedName {
    type Vtable = IADsTypedName_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsTypedName {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb371a349_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTypedName_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjectname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnlevel: i32) -> ::windows::core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lninterval: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsUser(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsUser {
    pub unsafe fn BadLoginAddress(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BadLoginAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BadLoginCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BadLoginCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastLogin(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastLogin)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastLogoff(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastLogoff)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastFailedLogin(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastFailedLogin)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PasswordLastChanged(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PasswordLastChanged)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdescription)).ok()
    }
    pub unsafe fn Division(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Division)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDivision(&self, bstrdivision: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDivision)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdivision)).ok()
    }
    pub unsafe fn Department(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Department)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDepartment(&self, bstrdepartment: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDepartment)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdepartment)).ok()
    }
    pub unsafe fn EmployeeID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EmployeeID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEmployeeID(&self, bstremployeeid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEmployeeID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstremployeeid)).ok()
    }
    pub unsafe fn FullName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FullName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFullName(&self, bstrfullname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFullName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfullname)).ok()
    }
    pub unsafe fn FirstName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FirstName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFirstName(&self, bstrfirstname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFirstName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfirstname)).ok()
    }
    pub unsafe fn LastName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLastName(&self, bstrlastname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLastName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrlastname)).ok()
    }
    pub unsafe fn OtherName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OtherName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOtherName(&self, bstrothername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOtherName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrothername)).ok()
    }
    pub unsafe fn NamePrefix(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NamePrefix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNamePrefix(&self, bstrnameprefix: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNamePrefix)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnameprefix)).ok()
    }
    pub unsafe fn NameSuffix(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NameSuffix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNameSuffix(&self, bstrnamesuffix: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNameSuffix)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrnamesuffix)).ok()
    }
    pub unsafe fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Title)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTitle(&self, bstrtitle: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTitle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtitle)).ok()
    }
    pub unsafe fn Manager(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Manager)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetManager(&self, bstrmanager: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetManager)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrmanager)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TelephoneHome(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TelephoneHome)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetTelephoneHome(&self, vtelephonehome: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTelephoneHome)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vtelephonehome)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TelephoneMobile(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TelephoneMobile)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetTelephoneMobile(&self, vtelephonemobile: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTelephoneMobile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vtelephonemobile)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TelephoneNumber(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TelephoneNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetTelephoneNumber(&self, vtelephonenumber: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTelephoneNumber)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vtelephonenumber)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TelephonePager(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TelephonePager)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetTelephonePager(&self, vtelephonepager: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTelephonePager)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vtelephonepager)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FaxNumber(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FaxNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetFaxNumber(&self, vfaxnumber: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFaxNumber)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vfaxnumber)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OfficeLocations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OfficeLocations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetOfficeLocations(&self, vofficelocations: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOfficeLocations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vofficelocations)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PostalAddresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PostalAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPostalAddresses(&self, vpostaladdresses: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPostalAddresses)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vpostaladdresses)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PostalCodes(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PostalCodes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPostalCodes(&self, vpostalcodes: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPostalCodes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vpostalcodes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SeeAlso(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SeeAlso)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSeeAlso)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vseealso)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AccountDisabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AccountDisabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAccountDisabled<P0>(&self, faccountdisabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAccountDisabled)(::windows::core::Vtable::as_raw(self), faccountdisabled.into()).ok()
    }
    pub unsafe fn AccountExpirationDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AccountExpirationDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccountExpirationDate(&self, daaccountexpirationdate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAccountExpirationDate)(::windows::core::Vtable::as_raw(self), daaccountexpirationdate).ok()
    }
    pub unsafe fn GraceLoginsAllowed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GraceLoginsAllowed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGraceLoginsAllowed(&self, lngraceloginsallowed: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGraceLoginsAllowed)(::windows::core::Vtable::as_raw(self), lngraceloginsallowed).ok()
    }
    pub unsafe fn GraceLoginsRemaining(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GraceLoginsRemaining)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGraceLoginsRemaining(&self, lngraceloginsremaining: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGraceLoginsRemaining)(::windows::core::Vtable::as_raw(self), lngraceloginsremaining).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAccountLocked(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsAccountLocked)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsAccountLocked<P0>(&self, fisaccountlocked: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetIsAccountLocked)(::windows::core::Vtable::as_raw(self), fisaccountlocked.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LoginHours(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoginHours)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetLoginHours(&self, vloginhours: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLoginHours)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vloginhours)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LoginWorkstations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoginWorkstations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetLoginWorkstations(&self, vloginworkstations: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLoginWorkstations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vloginworkstations)).ok()
    }
    pub unsafe fn MaxLogins(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaxLogins)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxLogins(&self, lnmaxlogins: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMaxLogins)(::windows::core::Vtable::as_raw(self), lnmaxlogins).ok()
    }
    pub unsafe fn MaxStorage(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MaxStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxStorage(&self, lnmaxstorage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMaxStorage)(::windows::core::Vtable::as_raw(self), lnmaxstorage).ok()
    }
    pub unsafe fn PasswordExpirationDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PasswordExpirationDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPasswordExpirationDate(&self, dapasswordexpirationdate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPasswordExpirationDate)(::windows::core::Vtable::as_raw(self), dapasswordexpirationdate).ok()
    }
    pub unsafe fn PasswordMinimumLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PasswordMinimumLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPasswordMinimumLength(&self, lnpasswordminimumlength: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPasswordMinimumLength)(::windows::core::Vtable::as_raw(self), lnpasswordminimumlength).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PasswordRequired(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PasswordRequired)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPasswordRequired<P0>(&self, fpasswordrequired: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetPasswordRequired)(::windows::core::Vtable::as_raw(self), fpasswordrequired.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequireUniquePassword(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequireUniquePassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRequireUniquePassword<P0>(&self, frequireuniquepassword: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetRequireUniquePassword)(::windows::core::Vtable::as_raw(self), frequireuniquepassword.into()).ok()
    }
    pub unsafe fn EmailAddress(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EmailAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEmailAddress(&self, bstremailaddress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEmailAddress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstremailaddress)).ok()
    }
    pub unsafe fn HomeDirectory(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HomeDirectory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHomeDirectory(&self, bstrhomedirectory: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHomeDirectory)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrhomedirectory)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Languages(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Languages)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetLanguages(&self, vlanguages: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLanguages)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vlanguages)).ok()
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Profile)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProfile(&self, bstrprofile: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProfile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprofile)).ok()
    }
    pub unsafe fn LoginScript(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoginScript)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLoginScript(&self, bstrloginscript: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLoginScript)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrloginscript)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Picture(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Picture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPicture(&self, vpicture: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPicture)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vpicture)).ok()
    }
    pub unsafe fn HomePage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HomePage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHomePage(&self, bstrhomepage: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHomePage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrhomepage)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Groups(&self) -> ::windows::core::Result<IADsMembers> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Groups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPassword(&self, newpassword: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPassword)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(newpassword)).ok()
    }
    pub unsafe fn ChangePassword(&self, bstroldpassword: &::windows::core::BSTR, bstrnewpassword: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ChangePassword)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstroldpassword), ::core::mem::transmute_copy(bstrnewpassword)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsUser, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsUser {
    type Vtable = IADsUser_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e37e320_17e2_11cf_abc4_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsUser_Vtbl {
    pub base__: IADs_Vtbl,
    pub BadLoginAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BadLoginCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub LastLogin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub LastLogoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub LastFailedLogin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub PasswordLastChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Division: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDivision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdivision: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Department: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdepartment: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmployeeID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEmployeeID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstremployeeid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfullname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfirstname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlastname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OtherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOtherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrothername: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnameprefix: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NameSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNameSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnamesuffix: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtitle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Manager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TelephoneHome: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TelephoneHome: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetTelephoneHome: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtelephonehome: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetTelephoneHome: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TelephoneMobile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TelephoneMobile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetTelephoneMobile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtelephonemobile: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetTelephoneMobile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TelephoneNumber: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetTelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtelephonenumber: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetTelephoneNumber: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TelephonePager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TelephonePager: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetTelephonePager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtelephonepager: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetTelephonePager: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FaxNumber: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vfaxnumber: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetFaxNumber: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OfficeLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OfficeLocations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetOfficeLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vofficelocations: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetOfficeLocations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PostalAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PostalAddresses: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPostalAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vpostaladdresses: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPostalAddresses: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PostalCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PostalCodes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPostalCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vpostalcodes: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPostalCodes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSeeAlso: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AccountDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AccountDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAccountDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, faccountdisabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAccountDisabled: usize,
    pub AccountExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub SetAccountExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, daaccountexpirationdate: f64) -> ::windows::core::HRESULT,
    pub GraceLoginsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetGraceLoginsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lngraceloginsallowed: i32) -> ::windows::core::HRESULT,
    pub GraceLoginsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetGraceLoginsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lngraceloginsremaining: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAccountLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAccountLocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsAccountLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fisaccountlocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsAccountLocked: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LoginHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LoginHours: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetLoginHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vloginhours: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetLoginHours: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LoginWorkstations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LoginWorkstations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetLoginWorkstations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vloginworkstations: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetLoginWorkstations: usize,
    pub MaxLogins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxLogins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxlogins: i32) -> ::windows::core::HRESULT,
    pub MaxStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxstorage: i32) -> ::windows::core::HRESULT,
    pub PasswordExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub SetPasswordExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dapasswordexpirationdate: f64) -> ::windows::core::HRESULT,
    pub PasswordMinimumLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub SetPasswordMinimumLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnpasswordminimumlength: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PasswordRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PasswordRequired: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPasswordRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fpasswordrequired: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPasswordRequired: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequireUniquePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequireUniquePassword: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRequireUniquePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequireuniquepassword: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRequireUniquePassword: usize,
    pub EmailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEmailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstremailaddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HomeDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHomeDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhomedirectory: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Languages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vlanguages: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetLanguages: usize,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoginScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLoginScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrloginscript: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Picture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Picture: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vpicture: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPicture: usize,
    pub HomePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHomePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhomepage: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newpassword: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ChangePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroldpassword: *mut ::core::ffi::c_void, bstrnewpassword: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IADsWinNTSystemInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsWinNTSystemInfo {
    pub unsafe fn UserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ComputerName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ComputerName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DomainName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DomainName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PDC(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PDC)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IADsWinNTSystemInfo, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IADsWinNTSystemInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IADsWinNTSystemInfo {
    type Vtable = IADsWinNTSystemInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IADsWinNTSystemInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c6d65dc_afd1_11d2_9cb9_0000f87a369e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsWinNTSystemInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ComputerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct ICommonQuery(::windows::core::IUnknown);
impl ICommonQuery {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OpenQueryWindow<P0>(&self, hwndparent: P0, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).OpenQueryWindow)(::windows::core::Vtable::as_raw(self), hwndparent.into(), pquerywnd, ::core::mem::transmute(ppdataobject)).ok()
    }
}
::windows::core::interface_hierarchy!(ICommonQuery, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICommonQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ICommonQuery {
    type Vtable = ICommonQuery_Vtbl;
}
unsafe impl ::windows::core::Interface for ICommonQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab50dec0_6f1d_11d0_a1c4_00aa00c16e65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonQuery_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub OpenQueryWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    OpenQueryWindow: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDirectoryObject(::windows::core::IUnknown);
impl IDirectoryObject {
    pub unsafe fn GetObjectInformation(&self) -> ::windows::core::Result<*mut ADS_OBJECT_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetObjectInformation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectAttributes(&self, pattributenames: *const ::windows::core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetObjectAttributes)(::windows::core::Vtable::as_raw(self), pattributenames, dwnumberattributes, ppattributeentries, pdwnumattributesreturned).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetObjectAttributes(&self, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SetObjectAttributes)(::windows::core::Vtable::as_raw(self), pattributeentries, dwnumattributes, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDSObject<P0>(&self, pszrdnname: P0, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows::core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDSObject)(::windows::core::Vtable::as_raw(self), pszrdnname.into().abi(), pattributeentries, dwnumattributes, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DeleteDSObject<P0>(&self, pszrdnname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteDSObject)(::windows::core::Vtable::as_raw(self), pszrdnname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectoryObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectoryObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDirectoryObject {
    type Vtable = IDirectoryObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectoryObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe798de2c_22e4_11d0_84fe_00c04fd8d503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectoryObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetObjectInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobjinfo: *mut *mut ADS_OBJECT_INFO) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObjectAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattributenames: *const ::windows::core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObjectAttributes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetObjectAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, pdwnumattributesmodified: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetObjectAttributes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateDSObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszrdnname: ::windows::core::PCWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateDSObject: usize,
    pub DeleteDSObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszrdnname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDirectorySchemaMgmt(::windows::core::IUnknown);
impl IDirectorySchemaMgmt {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAttributes(&self, ppszattrnames: *const ::windows::core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnumAttributes)(::windows::core::Vtable::as_raw(self), ppszattrnames, dwnumattributes, ppattrdefinition, pdwnumattributes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAttributeDefinition<P0>(&self, pszattributename: P0, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).CreateAttributeDefinition)(::windows::core::Vtable::as_raw(self), pszattributename.into().abi(), pattributedefinition).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributeDefinition<P0>(&self, pszattributename: P0, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteAttributeDefinition)(::windows::core::Vtable::as_raw(self), pszattributename.into().abi(), pattributedefinition).ok()
    }
    pub unsafe fn DeleteAttributeDefinition<P0>(&self, pszattributename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteAttributeDefinition)(::windows::core::Vtable::as_raw(self), pszattributename.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumClasses(&self, ppszclassnames: *const ::windows::core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnumClasses)(::windows::core::Vtable::as_raw(self), ppszclassnames, dwnumclasses, ppclassdefinition, pdwnumclasses).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteClassDefinition<P0>(&self, pszclassname: P0, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteClassDefinition)(::windows::core::Vtable::as_raw(self), pszclassname.into().abi(), pclassdefinition).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateClassDefinition<P0>(&self, pszclassname: P0, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).CreateClassDefinition)(::windows::core::Vtable::as_raw(self), pszclassname.into().abi(), pclassdefinition).ok()
    }
    pub unsafe fn DeleteClassDefinition<P0>(&self, pszclassname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteClassDefinition)(::windows::core::Vtable::as_raw(self), pszclassname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectorySchemaMgmt, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectorySchemaMgmt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDirectorySchemaMgmt {
    type Vtable = IDirectorySchemaMgmt_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectorySchemaMgmt {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75db3b9c_a4d8_11d0_a79c_00c04fd8d5a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectorySchemaMgmt_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszattrnames: *const ::windows::core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumAttributes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateAttributeDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributename: ::windows::core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateAttributeDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteAttributeDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributename: ::windows::core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteAttributeDefinition: usize,
    pub DeleteAttributeDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszclassnames: *const ::windows::core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumClasses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteClassDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszclassname: ::windows::core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteClassDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateClassDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszclassname: ::windows::core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateClassDefinition: usize,
    pub DeleteClassDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszclassname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDirectorySearch(::windows::core::IUnknown);
impl IDirectorySearch {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSearchPreference(&self, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSearchPreference)(::windows::core::Vtable::as_raw(self), psearchprefs, dwnumprefs).ok()
    }
    pub unsafe fn ExecuteSearch<P0>(&self, pszsearchfilter: P0, pattributenames: *const ::windows::core::PCWSTR, dwnumberattributes: u32) -> ::windows::core::Result<ADS_SEARCH_HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExecuteSearch)(::windows::core::Vtable::as_raw(self), pszsearchfilter.into().abi(), pattributenames, dwnumberattributes, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AbandonSearch<P0>(&self, phsearchresult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<ADS_SEARCH_HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).AbandonSearch)(::windows::core::Vtable::as_raw(self), phsearchresult.into()).ok()
    }
    pub unsafe fn GetFirstRow<P0>(&self, hsearchresult: P0) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<ADS_SEARCH_HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).GetFirstRow)(::windows::core::Vtable::as_raw(self), hsearchresult.into())
    }
    pub unsafe fn GetNextRow<P0>(&self, hsearchresult: P0) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<ADS_SEARCH_HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).GetNextRow)(::windows::core::Vtable::as_raw(self), hsearchresult.into())
    }
    pub unsafe fn GetPreviousRow<P0>(&self, hsearchresult: P0) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<ADS_SEARCH_HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).GetPreviousRow)(::windows::core::Vtable::as_raw(self), hsearchresult.into())
    }
    pub unsafe fn GetNextColumnName<P0>(&self, hsearchhandle: P0, ppszcolumnname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<ADS_SEARCH_HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).GetNextColumnName)(::windows::core::Vtable::as_raw(self), hsearchhandle.into(), ppszcolumnname)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColumn<P0, P1>(&self, hsearchresult: P0, szcolumnname: P1, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<ADS_SEARCH_HANDLE>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetColumn)(::windows::core::Vtable::as_raw(self), hsearchresult.into(), szcolumnname.into().abi(), psearchcolumn).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FreeColumn(&self, psearchcolumn: *const ADS_SEARCH_COLUMN) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FreeColumn)(::windows::core::Vtable::as_raw(self), psearchcolumn).ok()
    }
    pub unsafe fn CloseSearchHandle<P0>(&self, hsearchresult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<ADS_SEARCH_HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).CloseSearchHandle)(::windows::core::Vtable::as_raw(self), hsearchresult.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IDirectorySearch, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDirectorySearch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDirectorySearch {
    type Vtable = IDirectorySearch_Vtbl;
}
unsafe impl ::windows::core::Interface for IDirectorySearch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x109ba8ec_92f0_11d0_a790_00c04fd8d5a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectorySearch_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSearchPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSearchPreference: usize,
    pub ExecuteSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsearchfilter: ::windows::core::PCWSTR, pattributenames: *const ::windows::core::PCWSTR, dwnumberattributes: u32, phsearchresult: *mut ADS_SEARCH_HANDLE) -> ::windows::core::HRESULT,
    pub AbandonSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phsearchresult: ADS_SEARCH_HANDLE) -> ::windows::core::HRESULT,
    pub GetFirstRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows::core::HRESULT,
    pub GetNextRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows::core::HRESULT,
    pub GetPreviousRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows::core::HRESULT,
    pub GetNextColumnName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchhandle: ADS_SEARCH_HANDLE, ppszcolumnname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: ::windows::core::PCWSTR, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetColumn: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FreeColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psearchcolumn: *const ADS_SEARCH_COLUMN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FreeColumn: usize,
    pub CloseSearchHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDsAdminCreateObj(::windows::core::IUnknown);
impl IDsAdminCreateObj {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, padscontainerobj: P0, padscopysource: P1, lpszclassname: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IADsContainer>>,
        P1: ::std::convert::Into<::windows::core::InParam<IADs>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), padscontainerobj.into().abi(), padscopysource.into().abi(), lpszclassname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateModal<P0>(&self, hwndparent: P0) -> ::windows::core::Result<IADs>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateModal)(::windows::core::Vtable::as_raw(self), hwndparent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDsAdminCreateObj, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDsAdminCreateObj {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDsAdminCreateObj {
    type Vtable = IDsAdminCreateObj_Vtbl;
}
unsafe impl ::windows::core::Interface for IDsAdminCreateObj {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53554a38_f902_11d2_82b9_00c04f68928b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsAdminCreateObj_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padscontainerobj: *mut ::core::ffi::c_void, padscopysource: *mut ::core::ffi::c_void, lpszclassname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppadsobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateModal: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDsAdminNewObj(::windows::core::IUnknown);
impl IDsAdminNewObj {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetButtons<P0>(&self, ncurrindex: u32, bvalid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetButtons)(::windows::core::Vtable::as_raw(self), ncurrindex, bvalid.into()).ok()
    }
    pub unsafe fn GetPageCounts(&self, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPageCounts)(::windows::core::Vtable::as_raw(self), pntotal, pnstartindex).ok()
    }
}
::windows::core::interface_hierarchy!(IDsAdminNewObj, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDsAdminNewObj {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDsAdminNewObj {
    type Vtable = IDsAdminNewObj_Vtbl;
}
unsafe impl ::windows::core::Interface for IDsAdminNewObj {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2573587_e6fc_11d2_82af_00c04f68928b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsAdminNewObj_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetButtons: usize,
    pub GetPageCounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDsAdminNewObjExt(::windows::core::IUnknown);
impl IDsAdminNewObjExt {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn Initialize<P0, P1, P2, P3>(&self, padscontainerobj: P0, padscopysource: P1, lpszclassname: P2, pdsadminnewobj: P3, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IADsContainer>>,
        P1: ::std::convert::Into<::windows::core::InParam<IADs>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<IDsAdminNewObj>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), padscontainerobj.into().abi(), padscopysource.into().abi(), lpszclassname.into().abi(), pdsadminnewobj.into().abi(), pdispinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub unsafe fn AddPages<P0>(&self, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).AddPages)(::windows::core::Vtable::as_raw(self), lpfnaddpage, lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetObject<P0>(&self, padsobj: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IADs>>,
    {
        (::windows::core::Vtable::vtable(self).SetObject)(::windows::core::Vtable::as_raw(self), padsobj.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteData<P0>(&self, hwnd: P0, ucontext: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).WriteData)(::windows::core::Vtable::as_raw(self), hwnd.into(), ucontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnError<P0>(&self, hwnd: P0, hr: ::windows::core::HRESULT, ucontext: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).OnError)(::windows::core::Vtable::as_raw(self), hwnd.into(), hr, ucontext).ok()
    }
    pub unsafe fn GetSummaryInfo(&self, pbstrtext: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSummaryInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrtext)).ok()
    }
}
::windows::core::interface_hierarchy!(IDsAdminNewObjExt, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDsAdminNewObjExt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDsAdminNewObjExt {
    type Vtable = IDsAdminNewObjExt_Vtbl;
}
unsafe impl ::windows::core::Interface for IDsAdminNewObjExt {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6088eae2_e7bf_11d2_82af_00c04f68928b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsAdminNewObjExt_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padscontainerobj: *mut ::core::ffi::c_void, padscopysource: *mut ::core::ffi::c_void, lpszclassname: ::windows::core::PCWSTR, pdsadminnewobj: *mut ::core::ffi::c_void, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging")))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub AddPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls")))]
    AddPages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padsobj: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, hr: ::windows::core::HRESULT, ucontext: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnError: usize,
    pub GetSummaryInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDsAdminNewObjPrimarySite(::windows::core::IUnknown);
impl IDsAdminNewObjPrimarySite {
    pub unsafe fn CreateNew<P0>(&self, pszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).CreateNew)(::windows::core::Vtable::as_raw(self), pszname.into().abi()).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IDsAdminNewObjPrimarySite, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDsAdminNewObjPrimarySite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDsAdminNewObjPrimarySite {
    type Vtable = IDsAdminNewObjPrimarySite_Vtbl;
}
unsafe impl ::windows::core::Interface for IDsAdminNewObjPrimarySite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe2b487e_f904_11d2_82b9_00c04f68928b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsAdminNewObjPrimarySite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDsAdminNotifyHandler(::windows::core::IUnknown);
impl IDsAdminNotifyHandler {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pextrainfo: P0, pueventflags: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), pextrainfo.into().abi(), pueventflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Begin<P0, P1>(&self, uevent: u32, parg1: P0, parg2: P1, puflags: *mut u32, pbstr: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDataObject>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).Begin)(::windows::core::Vtable::as_raw(self), uevent, parg1.into().abi(), parg2.into().abi(), puflags, ::core::mem::transmute(pbstr)).ok()
    }
    pub unsafe fn Notify(&self, nitem: u32, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Notify)(::windows::core::Vtable::as_raw(self), nitem, uflags).ok()
    }
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).End)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IDsAdminNotifyHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDsAdminNotifyHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDsAdminNotifyHandler {
    type Vtable = IDsAdminNotifyHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IDsAdminNotifyHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4a2b8b3_5a18_11d2_97c1_00a0c9a06d2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsAdminNotifyHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextrainfo: *mut ::core::ffi::c_void, pueventflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Begin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uevent: u32, parg1: *mut ::core::ffi::c_void, parg2: *mut ::core::ffi::c_void, puflags: *mut u32, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Begin: usize,
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitem: u32, uflags: u32) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDsBrowseDomainTree(::windows::core::IUnknown);
impl IDsBrowseDomainTree {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BrowseTo<P0>(&self, hwndparent: P0, ppsztargetpath: *mut ::windows::core::PWSTR, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).BrowseTo)(::windows::core::Vtable::as_raw(self), hwndparent.into(), ppsztargetpath, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDomains(&self, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDomains)(::windows::core::Vtable::as_raw(self), ppdomaintree, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FreeDomains(&self, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FreeDomains)(::windows::core::Vtable::as_raw(self), ppdomaintree).ok()
    }
    pub unsafe fn FlushCachedDomains(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FlushCachedDomains)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetComputer<P0, P1, P2>(&self, pszcomputername: P0, pszusername: P1, pszpassword: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetComputer)(::windows::core::Vtable::as_raw(self), pszcomputername.into().abi(), pszusername.into().abi(), pszpassword.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IDsBrowseDomainTree, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDsBrowseDomainTree {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDsBrowseDomainTree {
    type Vtable = IDsBrowseDomainTree_Vtbl;
}
unsafe impl ::windows::core::Interface for IDsBrowseDomainTree {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cabcf1e_78f5_11d2_960c_00c04fa31a86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsBrowseDomainTree_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BrowseTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut ::windows::core::PWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BrowseTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDomains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDomains: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FreeDomains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FreeDomains: usize,
    pub FlushCachedDomains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcomputername: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, pszpassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDsDisplaySpecifier(::windows::core::IUnknown);
impl IDsDisplaySpecifier {
    pub unsafe fn SetServer<P0, P1, P2>(&self, pszserver: P0, pszusername: P1, pszpassword: P2, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetServer)(::windows::core::Vtable::as_raw(self), pszserver.into().abi(), pszusername.into().abi(), pszpassword.into().abi(), dwflags).ok()
    }
    pub unsafe fn SetLanguageID(&self, langid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLanguageID)(::windows::core::Vtable::as_raw(self), langid).ok()
    }
    pub unsafe fn GetDisplaySpecifier<P0>(&self, pszobjectclass: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetDisplaySpecifier)(::windows::core::Vtable::as_raw(self), pszobjectclass.into().abi(), riid, ppv).ok()
    }
    pub unsafe fn GetIconLocation<P0>(&self, pszobjectclass: P0, dwflags: u32, pszbuffer: &mut [u16], presid: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetIconLocation)(::windows::core::Vtable::as_raw(self), pszobjectclass.into().abi(), dwflags, ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len() as _, presid).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon<P0>(&self, pszobjectclass: P0, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetIcon)(::windows::core::Vtable::as_raw(self), pszobjectclass.into().abi(), dwflags, cxicon, cyicon)
    }
    pub unsafe fn GetFriendlyClassName<P0>(&self, pszobjectclass: P0, pszbuffer: &mut [u16]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetFriendlyClassName)(::windows::core::Vtable::as_raw(self), pszobjectclass.into().abi(), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len() as _).ok()
    }
    pub unsafe fn GetFriendlyAttributeName<P0, P1>(&self, pszobjectclass: P0, pszattributename: P1, pszbuffer: &mut [u16]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetFriendlyAttributeName)(::windows::core::Vtable::as_raw(self), pszobjectclass.into().abi(), pszattributename.into().abi(), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClassContainer<P0, P1>(&self, pszobjectclass: P0, pszadspath: P1, dwflags: u32) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).IsClassContainer)(::windows::core::Vtable::as_raw(self), pszobjectclass.into().abi(), pszadspath.into().abi(), dwflags)
    }
    pub unsafe fn GetClassCreationInfo<P0>(&self, pszobjectclass: P0, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetClassCreationInfo)(::windows::core::Vtable::as_raw(self), pszobjectclass.into().abi(), ppdscci).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumClassAttributes<P0, P1>(&self, pszobjectclass: P0, pcbenum: LPDSENUMATTRIBUTES, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).EnumClassAttributes)(::windows::core::Vtable::as_raw(self), pszobjectclass.into().abi(), pcbenum, lparam.into()).ok()
    }
    pub unsafe fn GetAttributeADsType<P0>(&self, pszattributename: P0) -> ADSTYPE
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetAttributeADsType)(::windows::core::Vtable::as_raw(self), pszattributename.into().abi())
    }
}
::windows::core::interface_hierarchy!(IDsDisplaySpecifier, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDsDisplaySpecifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDsDisplaySpecifier {
    type Vtable = IDsDisplaySpecifier_Vtbl;
}
unsafe impl ::windows::core::Interface for IDsDisplaySpecifier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ab4a8c0_6a0b_11d2_ad49_00c04fa31a86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsDisplaySpecifier_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserver: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, pszpassword: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub SetLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT,
    pub GetDisplaySpecifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIconLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows::core::PCWSTR, dwflags: u32, pszbuffer: ::windows::core::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows::core::PCWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    pub GetFriendlyClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows::core::PCWSTR, pszbuffer: ::windows::core::PWSTR, cchbuffer: i32) -> ::windows::core::HRESULT,
    pub GetFriendlyAttributeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows::core::PCWSTR, pszattributename: ::windows::core::PCWSTR, pszbuffer: ::windows::core::PWSTR, cchbuffer: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClassContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows::core::PCWSTR, pszadspath: ::windows::core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClassContainer: usize,
    pub GetClassCreationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows::core::PCWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumClassAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows::core::PCWSTR, pcbenum: LPDSENUMATTRIBUTES, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumClassAttributes: usize,
    pub GetAttributeADsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributename: ::windows::core::PCWSTR) -> ADSTYPE,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDsObjectPicker(::windows::core::IUnknown);
impl IDsObjectPicker {
    pub unsafe fn Initialize(&self, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), pinitinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InvokeDialog<P0>(&self, hwndparent: P0) -> ::windows::core::Result<super::super::System::Com::IDataObject>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InvokeDialog)(::windows::core::Vtable::as_raw(self), hwndparent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDsObjectPicker, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDsObjectPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDsObjectPicker {
    type Vtable = IDsObjectPicker_Vtbl;
}
unsafe impl ::windows::core::Interface for IDsObjectPicker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c87e64e_3b7a_11d2_b9e0_00c04fd8dbf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsObjectPicker_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InvokeDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppdoselections: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InvokeDialog: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IDsObjectPickerCredentials(::windows::core::IUnknown);
impl IDsObjectPickerCredentials {
    pub unsafe fn SetCredentials<P0, P1>(&self, szusername: P0, szpassword: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetCredentials)(::windows::core::Vtable::as_raw(self), szusername.into().abi(), szpassword.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IDsObjectPickerCredentials, ::windows::core::IUnknown, IDsObjectPicker);
impl ::core::clone::Clone for IDsObjectPickerCredentials {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDsObjectPickerCredentials {
    type Vtable = IDsObjectPickerCredentials_Vtbl;
}
unsafe impl ::windows::core::Interface for IDsObjectPickerCredentials {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2d3ec9b_d041_445a_8f16_4748de8fb1cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsObjectPickerCredentials_Vtbl {
    pub base__: IDsObjectPicker_Vtbl,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szusername: ::windows::core::PCWSTR, szpassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPersistQuery(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPersistQuery {
    pub unsafe fn WriteString<P0, P1, P2>(&self, psection: P0, pvaluename: P1, pvalue: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteString)(::windows::core::Vtable::as_raw(self), psection.into().abi(), pvaluename.into().abi(), pvalue.into().abi()).ok()
    }
    pub unsafe fn ReadString<P0, P1>(&self, psection: P0, pvaluename: P1, pbuffer: ::windows::core::PWSTR, cchbuffer: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).ReadString)(::windows::core::Vtable::as_raw(self), psection.into().abi(), pvaluename.into().abi(), ::core::mem::transmute(pbuffer), cchbuffer).ok()
    }
    pub unsafe fn WriteInt<P0, P1>(&self, psection: P0, pvaluename: P1, value: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteInt)(::windows::core::Vtable::as_raw(self), psection.into().abi(), pvaluename.into().abi(), value).ok()
    }
    pub unsafe fn ReadInt<P0, P1>(&self, psection: P0, pvaluename: P1, pvalue: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).ReadInt)(::windows::core::Vtable::as_raw(self), psection.into().abi(), pvaluename.into().abi(), pvalue).ok()
    }
    pub unsafe fn WriteStruct<P0, P1>(&self, psection: P0, pvaluename: P1, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteStruct)(::windows::core::Vtable::as_raw(self), psection.into().abi(), pvaluename.into().abi(), pstruct, cbstruct).ok()
    }
    pub unsafe fn ReadStruct<P0, P1>(&self, psection: P0, pvaluename: P1, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).ReadStruct)(::windows::core::Vtable::as_raw(self), psection.into().abi(), pvaluename.into().abi(), pstruct, cbstruct).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPersistQuery, ::windows::core::IUnknown, super::super::System::Com::IPersist);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPersistQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPersistQuery {
    type Vtable = IPersistQuery_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPersistQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a3114b8_a62e_11d0_a6c5_00a0c906af45);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistQuery_Vtbl {
    pub base__: super::super::System::Com::IPersist_Vtbl,
    pub WriteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows::core::PCWSTR, pvaluename: ::windows::core::PCWSTR, pvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ReadString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows::core::PCWSTR, pvaluename: ::windows::core::PCWSTR, pbuffer: ::windows::core::PWSTR, cchbuffer: i32) -> ::windows::core::HRESULT,
    pub WriteInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows::core::PCWSTR, pvaluename: ::windows::core::PCWSTR, value: i32) -> ::windows::core::HRESULT,
    pub ReadInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows::core::PCWSTR, pvaluename: ::windows::core::PCWSTR, pvalue: *mut i32) -> ::windows::core::HRESULT,
    pub WriteStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows::core::PCWSTR, pvaluename: ::windows::core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::HRESULT,
    pub ReadStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows::core::PCWSTR, pvaluename: ::windows::core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IPrivateDispatch(::windows::core::IUnknown);
impl IPrivateDispatch {
    pub unsafe fn ADSIInitializeDispatchManager(&self, dwextensionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ADSIInitializeDispatchManager)(::windows::core::Vtable::as_raw(self), dwextensionid).ok()
    }
    pub unsafe fn ADSIGetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ADSIGetTypeInfoCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ADSIGetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ADSIGetTypeInfo)(::windows::core::Vtable::as_raw(self), itinfo, lcid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ADSIGetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ADSIGetIDsOfNames)(::windows::core::Vtable::as_raw(self), riid, rgsznames, cnames, lcid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ADSIInvoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ADSIInvoke)(::windows::core::Vtable::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, pvarresult, pexcepinfo, puargerr).ok()
    }
}
::windows::core::interface_hierarchy!(IPrivateDispatch, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrivateDispatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrivateDispatch {
    type Vtable = IPrivateDispatch_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrivateDispatch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86ab4bbe_65f6_11d1_8c13_00c04fd8d503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrivateDispatch_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ADSIInitializeDispatchManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwextensionid: i32) -> ::windows::core::HRESULT,
    pub ADSIGetTypeInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ADSIGetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ADSIGetTypeInfo: usize,
    pub ADSIGetIDsOfNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ADSIInvoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ADSIInvoke: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IPrivateUnknown(::windows::core::IUnknown);
impl IPrivateUnknown {
    pub unsafe fn ADSIInitializeObject(&self, lpszusername: &::windows::core::BSTR, lpszpassword: &::windows::core::BSTR, lnreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ADSIInitializeObject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(lpszusername), ::core::mem::transmute_copy(lpszpassword), lnreserved).ok()
    }
    pub unsafe fn ADSIReleaseObject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ADSIReleaseObject)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPrivateUnknown, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrivateUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrivateUnknown {
    type Vtable = IPrivateUnknown_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrivateUnknown {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89126bab_6ead_11d1_8c18_00c04fd8d503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrivateUnknown_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ADSIInitializeObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszusername: *mut ::core::ffi::c_void, lpszpassword: *mut ::core::ffi::c_void, lnreserved: i32) -> ::windows::core::HRESULT,
    pub ADSIReleaseObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
pub struct IQueryForm(::windows::core::IUnknown);
impl IQueryForm {
    #[doc = "*Required features: `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<P0>(&self, hkform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), hkform.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn AddForms<P0>(&self, paddformsproc: LPCQADDFORMSPROC, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).AddForms)(::windows::core::Vtable::as_raw(self), paddformsproc, lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn AddPages<P0>(&self, paddpagesproc: LPCQADDPAGESPROC, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).AddPages)(::windows::core::Vtable::as_raw(self), paddpagesproc, lparam.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IQueryForm, ::windows::core::IUnknown);
impl ::core::clone::Clone for IQueryForm {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IQueryForm {
    type Vtable = IQueryForm_Vtbl;
}
unsafe impl ::windows::core::Interface for IQueryForm {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cfcee30_39bd_11d0_b8d1_00a024ab2dbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryForm_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkform: super::super::System::Registry::HKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub AddForms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddformsproc: LPCQADDFORMSPROC, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    AddForms: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub AddPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddpagesproc: LPCQADDPAGESPROC, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    AddPages: usize,
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ACTRL_DS_CONTROL_ACCESS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ACTRL_DS_CREATE_CHILD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ACTRL_DS_DELETE_CHILD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ACTRL_DS_DELETE_TREE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ACTRL_DS_LIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ACTRL_DS_LIST_OBJECT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ACTRL_DS_OPEN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ACTRL_DS_READ_PROP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ACTRL_DS_SELF: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ACTRL_DS_WRITE_PROP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_REPL_AUTHENTICATION_MODE_MUTUAL_AUTH_REQUIRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE_PASS_THROUGH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_FSMO_NAMING_STRING: ::windows::core::PCSTR = ::windows::s!("naming");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_FSMO_NAMING_STRING_W: ::windows::core::PCWSTR = ::windows::w!("naming");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_FSMO_SCHEMA_STRING: ::windows::core::PCSTR = ::windows::s!("schema");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_FSMO_SCHEMA_STRING_W: ::windows::core::PCWSTR = ::windows::w!("schema");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_FSMO_STRING: ::windows::core::PCSTR = ::windows::s!("fsmo:");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_FSMO_STRING_W: ::windows::core::PCWSTR = ::windows::w!("fsmo:");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_INSTANCE_NAME_STRING: ::windows::core::PCSTR = ::windows::s!("instance:");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_INSTANCE_NAME_STRING_W: ::windows::core::PCWSTR = ::windows::w!("instance:");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_PARTITION_STRING: ::windows::core::PCSTR = ::windows::s!("partition:");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_PARTITION_STRING_W: ::windows::core::PCWSTR = ::windows::w!("partition:");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_SITE_NAME_STRING: ::windows::core::PCSTR = ::windows::s!("site:");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADAM_SCP_SITE_NAME_STRING_W: ::windows::core::PCWSTR = ::windows::w!("site:");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ATTR_APPEND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ATTR_CLEAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ATTR_DELETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ATTR_UPDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_EXT_INITCREDENTIALS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_EXT_INITIALIZE_COMPLETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_EXT_MAXEXTDISPID: u32 = 16777215u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_EXT_MINEXTDISPID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSystemInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50b6327f_afd1_11d2_9cb9_0000f87a369e);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADsSecurityUtility: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf270c64a_ffb8_4ae4_85fe_3a75e5347966);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const AccessControlEntry: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb75ac000_9bdd_11d0_852c_00c04fd8d503);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const AccessControlList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb85ea052_9bdd_11d0_852c_00c04fd8d503);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const BackLink: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcbf906f_4080_11d1_a3ac_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CFSTR_DSDISPLAYSPECOPTIONS: ::windows::core::PCWSTR = ::windows::w!("DsDisplaySpecOptions");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CFSTR_DSOBJECTNAMES: ::windows::core::PCWSTR = ::windows::w!("DsObjectNames");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CFSTR_DSOP_DS_SELECTION_LIST: ::windows::core::PCWSTR = ::windows::w!("CFSTR_DSOP_DS_SELECTION_LIST");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CFSTR_DSPROPERTYPAGEINFO: ::windows::core::PCWSTR = ::windows::w!("DsPropPageInfo");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CFSTR_DSQUERYPARAMS: ::windows::core::PCWSTR = ::windows::w!("DsQueryParameters");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CFSTR_DSQUERYSCOPE: ::windows::core::PCWSTR = ::windows::w!("DsQueryScope");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CFSTR_DS_DISPLAY_SPEC_OPTIONS: ::windows::core::PCWSTR = ::windows::w!("DsDisplaySpecOptions");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_CommonQuery: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83bc5ec0_6f2a_11d0_a1c4_00aa00c16e65);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsAdminCreateObj: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe301a009_f901_11d2_82b9_00c04f68928b);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsDisplaySpecifier: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ab4a8c0_6a0b_11d2_ad49_00c04fa31a86);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsDomainTreeBrowser: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1698790a_e2b4_11d0_b0b1_00c04fd8dca6);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFindAdvanced: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83ee3fe3_57d9_11d0_b932_00a024ab2dbb);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFindComputer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16006700_87ad_11d0_9140_00aa00c16e65);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFindContainer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1b3cbf2_886a_11d0_9140_00aa00c16e65);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFindDomainController: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x538c7b7e_d25e_11d0_9742_00a0c906af45);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFindFrsMembers: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94ce4b18_b3d3_11d1_b9b4_00c04fd8d5b0);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFindObjects: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83ee3fe1_57d9_11d0_b932_00a024ab2dbb);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFindPeople: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83ee3fe2_57d9_11d0_b932_00a024ab2dbb);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFindPrinter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb577f070_7ee2_11d0_913f_00aa00c16e65);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFindVolume: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1b3cbf1_886a_11d0_9140_00aa00c16e65);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFindWriteableDomainController: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cbef079_aa84_444b_bc70_68e41283eabc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsFolderProperties: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e51e0d0_6e0f_11d2_9601_00c04fa31a86);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsObjectPicker: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17d6ccd8_3b7b_11d2_b9e0_00c04fd8dbf7);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsPropertyPages: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d45d530_764b_11d0_a1ca_00aa00c16e65);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_DsQuery: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a23e65e_31c2_11d0_891c_00a024ab2dbb);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CLSID_MicrosoftDS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe1290f0_cfbd_11cf_a330_00aa00c16e65);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQFF_ISOPTIONAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQFF_NOGLOBALPAGES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQPM_CLEARFORM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQPM_ENABLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQPM_GETPARAMETERS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQPM_HANDLERSPECIFIC: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQPM_HELP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQPM_INITIALIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQPM_PERSIST: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQPM_RELEASE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CQPM_SETDEFAULTPARAMETERS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const CaseIgnoreList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15f88a55_4680_11d1_a3b4_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DBDTF_RETURNEXTERNAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DBDTF_RETURNFQDN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DBDTF_RETURNINBOUND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DBDTF_RETURNINOUTBOUND: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DBDTF_RETURNMIXEDDOMAINS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DNWithBinary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e99c0a3_f935_11d2_ba96_00c04fb6d0d1);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DNWithString: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x334857cc_f934_11d2_ba96_00c04fb6d0d1);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSA_NEWOBJ_CTX_CLEANUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSA_NEWOBJ_CTX_COMMIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSA_NEWOBJ_CTX_POSTCOMMIT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSA_NEWOBJ_CTX_PRECOMMIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSA_NOTIFY_DEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSA_NOTIFY_FLAG_ADDITIONAL_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSA_NOTIFY_FLAG_FORCE_ADDITIONAL_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSA_NOTIFY_MOV: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSA_NOTIFY_PROP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSA_NOTIFY_REN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBF_DISPLAYNAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBF_ICONLOCATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBF_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBID_BANNER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBID_CONTAINERLIST: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_CHECKBOXES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_DONTSIGNSEAL: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_ENTIREDIRECTORY: u32 = 589824u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_EXPANDONOPEN: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_HASCREDENTIALS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_IGNORETREATASLEAF: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_INCLUDEHIDDEN: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_NOBUTTONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_NOLINES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_NOLINESATROOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_NOROOT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_RETURNOBJECTCLASS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_RETURN_FORMAT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBI_SIMPLEAUTHENTICATE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBM_CHANGEIMAGESTATE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBM_CONTEXTMENU: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBM_HELP: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBM_QUERYINSERT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBM_QUERYINSERTA: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBM_QUERYINSERTW: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBS_CHECKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBS_HIDDEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSBS_ROOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSB_MAX_DISPLAYNAME_CHARS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSCCIF_HASWIZARDDIALOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSCCIF_HASWIZARDPRIMARYPAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSDSOF_DONTSIGNSEAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSDSOF_DSAVAILABLE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSDSOF_HASUSERANDSERVERINFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSDSOF_SIMPLEAUTHENTICATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSECAF_NOTLISTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSGIF_DEFAULTISCONTAINER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSGIF_GETDEFAULTICON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSGIF_ISDISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSGIF_ISMASK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSGIF_ISNORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSGIF_ISOPEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSICCF_IGNORETREATASLEAF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOBJECT_ISCONTAINER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOBJECT_READONLYPAGES: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_ALL_APP_PACKAGES: u32 = 2281701376u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_ALL_WELLKNOWN_SIDS: u32 = 2147614720u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_ANONYMOUS: u32 = 2147483712u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_AUTHENTICATED_USER: u32 = 2147483680u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_BATCH: u32 = 2147483776u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_COMPUTERS: u32 = 2147483656u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_CREATOR_GROUP: u32 = 2147484160u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_CREATOR_OWNER: u32 = 2147483904u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_DIALUP: u32 = 2147484672u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_EXCLUDE_BUILTIN_GROUPS: u32 = 2147516416u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_GLOBAL_GROUPS: u32 = 2147483652u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_IIS_APP_POOL: u32 = 2214592512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_INTERACTIVE: u32 = 2147485696u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_INTERNET_USER: u32 = 2149580800u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_ACCOUNTS: u32 = 2415919104u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_GROUPS: u32 = 2147483650u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_LOGON: u32 = 2164260864u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_SERVICE: u32 = 2147745792u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_NETWORK: u32 = 2147487744u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_NETWORK_SERVICE: u32 = 2148007936u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_OWNER_RIGHTS: u32 = 2151677952u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_REMOTE_LOGON: u32 = 2148532224u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_SERVICE: u32 = 2147491840u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_SERVICES: u32 = 2155872256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_SYSTEM: u32 = 2147500032u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_TERMINAL_SERVER: u32 = 2147549184u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_THIS_ORG_CERT: u32 = 2181038080u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_USERS: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_DOWNLEVEL_FILTER_WORLD: u32 = 2147483664u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_BUILTIN_GROUPS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_COMPUTERS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_CONTACTS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_DOMAIN_LOCAL_GROUPS_DL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_DOMAIN_LOCAL_GROUPS_SE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_GLOBAL_GROUPS_DL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_GLOBAL_GROUPS_SE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_INCLUDE_ADVANCED_VIEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_PASSWORDSETTINGS_OBJECTS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_SERVICE_ACCOUNTS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_UNIVERSAL_GROUPS_DL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_UNIVERSAL_GROUPS_SE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_USERS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FILTER_WELL_KNOWN_PRINCIPALS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FLAG_MULTISELECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_FLAG_SKIP_TARGET_COMPUTER_DC_CHECK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_COMPUTERS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_CONTACTS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_GROUPS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_PASSWORDSETTINGS_OBJECTS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_SERVICE_ACCOUNTS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_USERS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_STARTING_SCOPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_WANT_DOWNLEVEL_BUILTIN_PATH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_GC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_LDAP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_WINNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_FLAG_WANT_SID_PATH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_TYPE_DOWNLEVEL_JOINED_DOMAIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_TYPE_ENTERPRISE_DOMAIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_TYPE_EXTERNAL_DOWNLEVEL_DOMAIN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_TYPE_EXTERNAL_UPLEVEL_DOMAIN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_TYPE_GLOBAL_CATALOG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_TYPE_TARGET_COMPUTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_TYPE_UPLEVEL_JOINED_DOMAIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_TYPE_USER_ENTERED_DOWNLEVEL_SCOPE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_TYPE_USER_ENTERED_UPLEVEL_SCOPE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSOP_SCOPE_TYPE_WORKGROUP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSPROP_ATTRCHANGED_MSG: ::windows::core::PCWSTR = ::windows::w!("DsPropAttrChanged");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSPROVIDER_ADVANCED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSPROVIDER_AD_LDS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSPROVIDER_UNUSED_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSPROVIDER_UNUSED_1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSPROVIDER_UNUSED_2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSPROVIDER_UNUSED_3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSQPF_ENABLEADMINFEATURES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSQPF_ENABLEADVANCEDFEATURES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSQPF_HASCREDENTIALS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSQPF_NOCHOOSECOLUMNS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSQPF_NOSAVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSQPF_SAVELOCATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSQPF_SHOWHIDDENOBJECTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSQPM_GETCLASSLIST: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSQPM_HELPTOPICS: u32 = 268435457u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSROLE_PRIMARY_DOMAIN_GUID_PRESENT: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSROLE_PRIMARY_DS_MIXED_MODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSROLE_PRIMARY_DS_READONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSROLE_PRIMARY_DS_RUNNING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSROLE_UPGRADE_IN_PROGRESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSSSF_DONTSIGNSEAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSSSF_DSAVAILABLE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DSSSF_SIMPLEAUTHENTICATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_AVOID_SELF: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BACKGROUND_ONLY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_LONGHORN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WIN2000: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WIN2003: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WIN2003_WITH_MIXED_DOMAINS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WIN2008: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WIN2008R2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WIN2012: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WIN2012R2: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WIN2016: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WIN7: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WIN8: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WINBLUE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_BEHAVIOR_WINTHRESHOLD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_CLOSEST_FLAG: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DIRECTORY_SERVICE_10_REQUIRED: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DIRECTORY_SERVICE_6_REQUIRED: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DIRECTORY_SERVICE_8_REQUIRED: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DIRECTORY_SERVICE_9_REQUIRED: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DIRECTORY_SERVICE_PREFERRED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DIRECTORY_SERVICE_REQUIRED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DNS_CONTROLLER_FLAG: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DNS_DOMAIN_FLAG: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DNS_FOREST_FLAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DOMAIN_DIRECT_INBOUND: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DOMAIN_DIRECT_OUTBOUND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DOMAIN_IN_FOREST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DOMAIN_NATIVE_MODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DOMAIN_PRIMARY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DOMAIN_TREE_ROOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DS_10_FLAG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DS_8_FLAG: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DS_9_FLAG: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DS_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_EXIST_ADVISORY_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_FORCE_REDISCOVERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_FULL_SECRET_DOMAIN_6_FLAG: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_GC_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_GC_SERVER_REQUIRED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_GFTI_UPDATE_TDO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_GFTI_VALID_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_GOOD_TIMESERV_FLAG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_GOOD_TIMESERV_PREFERRED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_INSTANCETYPE_IS_NC_HEAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_INSTANCETYPE_NC_COMING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_INSTANCETYPE_NC_GOING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_INSTANCETYPE_NC_IS_WRITEABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_IP_REQUIRED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_IS_DNS_NAME: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_IS_FLAT_NAME: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_KCC_FLAG_ASYNC_OP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_KCC_FLAG_DAMPED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_KDC_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_KDC_REQUIRED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_KEY_LIST_FLAG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_KEY_LIST_SUPPORT_REQUIRED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_LDAP_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_LIST_ACCOUNT_OBJECT_FOR_SERVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_LIST_DNS_HOST_NAME_FOR_SERVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_LIST_DSA_OBJECT_FOR_SERVER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NDNC_FLAG: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NOTIFY_AFTER_SITE_RECORDS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_ONLY_DO_SITE_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_ONLY_LDAP_NEEDED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_PDC_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_PDC_REQUIRED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_PING_FLAGS: u32 = 1048575u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_PROP_ADMIN_PREFIX: ::windows::core::PCWSTR = ::windows::w!("admin");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_PROP_SHELL_PREFIX: ::windows::core::PCWSTR = ::windows::w!("shell");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_ASYNCHRONOUS_REPLICA: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_CRITICAL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_DISABLE_NOTIFICATION: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_DISABLE_PERIODIC: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_INITIAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_INTERSITE_MESSAGING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_NEVER_NOTIFY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_NONGC_RO_REPLICA: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_PERIODIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_SELECT_SECRETS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_TWO_WAY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_USE_COMPRESSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPADD_WRITEABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPDEL_ASYNCHRONOUS_OPERATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPDEL_IGNORE_ERRORS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPDEL_INTERSITE_MESSAGING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPDEL_LOCAL_ONLY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPDEL_NO_SOURCE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPDEL_REF_OK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPDEL_WRITEABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_FLAG_IMPROVE_LINKED_ATTRS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_COMPRESS_CHANGES: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_DISABLE_SCHEDULED_SYNC: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_DO_SCHEDULED_SYNCS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_FULL_SYNC_IN_PROGRESS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_FULL_SYNC_NEXT_PACKET: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_GCSPN: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_IGNORE_CHANGE_NOTIFICATIONS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_NEVER_SYNCED: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_NONGC_RO_REPLICA: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_NO_CHANGE_NOTIFICATIONS: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_PARTIAL_ATTRIBUTE_SET: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_PREEMPTED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_RETURN_OBJECT_PARENTS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_SELECT_SECRETS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_SYNC_ON_STARTUP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_TWO_WAY_SYNC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_USE_ASYNC_INTERSITE_TRANSPORT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_NBR_WRITEABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPMOD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPMOD_UPDATE_ADDRESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPMOD_UPDATE_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPMOD_UPDATE_INSTANCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPMOD_UPDATE_RESULT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPMOD_UPDATE_SCHEDULE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPMOD_UPDATE_TRANSPORT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPMOD_WRITEABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_ABORT_IF_SERVER_UNAVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_CROSS_SITE_BOUNDARIES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_DO_NOT_SYNC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_ID_SERVERS_BY_DN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_NO_OPTIONS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_PUSH_CHANGES_OUTWARD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_SKIP_INITIAL_CHECK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_SYNC_ADJACENT_SERVERS_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_ABANDONED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_ADD_REFERENCE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_ASYNCHRONOUS_OPERATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_ASYNCHRONOUS_REPLICA: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_CRITICAL: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_FORCE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_FULL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_FULL_IN_PROGRESS: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_INITIAL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_INITIAL_IN_PROGRESS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_INTERSITE_MESSAGING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_NEVER_COMPLETED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_NEVER_NOTIFY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_NONGC_RO_REPLICA: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_NOTIFICATION: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_NO_DISCARD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_PARTIAL_ATTRIBUTE_SET: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_PERIODIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_PREEMPTED: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_REQUEUE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_SELECT_SECRETS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_TWO_WAY: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_URGENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_USE_COMPRESSION: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNC_WRITEABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPUPD_ADD_REFERENCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPUPD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPUPD_DELETE_REFERENCE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPUPD_REFERENCE_GCSPN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPUPD_WRITEABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_RETURN_DNS_NAME: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_RETURN_FLAT_NAME: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_ROLE_DOMAIN_OWNER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_ROLE_INFRASTRUCTURE_OWNER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_ROLE_PDC_OWNER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_ROLE_RID_OWNER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_ROLE_SCHEMA_OWNER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SCHEMA_GUID_ATTR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SCHEMA_GUID_ATTR_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SCHEMA_GUID_CLASS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SCHEMA_GUID_CONTROL_RIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SCHEMA_GUID_NOT_FOUND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SELECT_SECRET_DOMAIN_6_FLAG: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SYNCED_EVENT_NAME: ::windows::core::PCSTR = ::windows::s!("NTDSInitialSyncsCompleted");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SYNCED_EVENT_NAME_W: ::windows::core::PCWSTR = ::windows::w!("NTDSInitialSyncsCompleted");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_TIMESERV_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_TIMESERV_REQUIRED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_TRY_NEXTCLOSEST_SITE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_WEB_SERVICE_REQUIRED: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_WRITABLE_FLAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_WRITABLE_REQUIRED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_WS_FLAG: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const Email: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f92a857_478e_11d1_a3b4_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const FACILITY_BACKUP: u32 = 2047u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const FACILITY_NTDSB: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const FACILITY_SYSTEM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const FLAG_DISABLABLE_OPTIONAL_FEATURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const FLAG_DOMAIN_OPTIONAL_FEATURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const FLAG_FOREST_OPTIONAL_FEATURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const FLAG_SERVER_OPTIONAL_FEATURE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const FRSCONN_MAX_PRIORITY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const FRSCONN_PRIORITY_MASK: u32 = 1879048192u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const FaxNumber: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5062215_4681_11d1_a3b4_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_COMPUTRS_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("aa312825768811d1aded00c04fd8d5cd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_COMPUTRS_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("aa312825768811d1aded00c04fd8d5cd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_DELETED_OBJECTS_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("18e2ea80684f11d2b9aa00c04f79f805");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_DELETED_OBJECTS_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("18e2ea80684f11d2b9aa00c04f79f805");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_DOMAIN_CONTROLLERS_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("a361b2ffffd211d1aa4b00c04fd7d83a");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_DOMAIN_CONTROLLERS_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("a361b2ffffd211d1aa4b00c04fd7d83a");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("22b70c67d56e4efb91e9300fca3dc1aa");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("22b70c67d56e4efb91e9300fca3dc1aa");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_INFRASTRUCTURE_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("2fbac1870ade11d297c400c04fd8d5cd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_INFRASTRUCTURE_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("2fbac1870ade11d297c400c04fd8d5cd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_KEYS_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("683A24E2E8164BD3AF86AC3C2CF3F981");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_LOSTANDFOUND_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("ab8153b7768811d1aded00c04fd8d5cd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_LOSTANDFOUND_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("ab8153b7768811d1aded00c04fd8d5cd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_MANAGED_SERVICE_ACCOUNTS_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("1EB93889E40C45DF9F0C64D23BBB6237");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("f4be92a4c777485e878e9421d53087db");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("f4be92a4c777485e878e9421d53087db");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_NTDS_QUOTAS_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("6227f0af1fc2410d8e3bb10615bb5b0f");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_NTDS_QUOTAS_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("6227f0af1fc2410d8e3bb10615bb5b0f");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_A: ::windows::core::PCSTR = ::windows::s!("73e843ece8cc4046b4ab07ffe4ab5bcd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_W: ::windows::core::PCWSTR = ::windows::w!("73e843ece8cc4046b4ab07ffe4ab5bcd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_PROGRAM_DATA_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("09460c08ae1e4a4ea0f64aee7daa1e5a");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_PROGRAM_DATA_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("09460c08ae1e4a4ea0f64aee7daa1e5a");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_RECYCLE_BIN_OPTIONAL_FEATURE_A: ::windows::core::PCSTR = ::windows::s!("d8dc6d76d0ac5e44f3b9a7f9b6744f2a");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_RECYCLE_BIN_OPTIONAL_FEATURE_W: ::windows::core::PCWSTR = ::windows::w!("d8dc6d76d0ac5e44f3b9a7f9b6744f2a");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_SYSTEMS_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("ab1d30f3768811d1aded00c04fd8d5cd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_SYSTEMS_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("ab1d30f3768811d1aded00c04fd8d5cd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_USERS_CONTAINER_A: ::windows::core::PCSTR = ::windows::s!("a9d1ca15768811d1aded00c04fd8d5cd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const GUID_USERS_CONTAINER_W: ::windows::core::PCWSTR = ::windows::w!("a9d1ca15768811d1aded00c04fd8d5cd");
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const Hold: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3ad3e13_4080_11d1_a3ac_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const LargeInteger: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x927971f5_0939_11d1_8be1_00c04fd8d503);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSAPI_BIND_ALLOW_DELEGATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSAPI_BIND_FIND_BINDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSAPI_BIND_FORCE_KERBEROS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_GC_TOPOLOGY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_INTERSITE_GC_TOPOLOGY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_INTERSITE_TOPOLOGY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_MINIMIZE_HOPS_TOPOLOGY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_NO_REASON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_OSCILLATING_CONNECTION_TOPOLOGY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_REDUNDANT_SERVER_TOPOLOGY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_RING_TOPOLOGY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_SERVER_FAILOVER_TOPOLOGY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_SITE_FAILOVER_TOPOLOGY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_KCC_STALE_SERVERS_TOPOLOGY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_OPT_DISABLE_INTERSITE_COMPRESSION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_OPT_IGNORE_SCHEDULE_MASK: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_OPT_IS_GENERATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_OPT_OVERRIDE_NOTIFY_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_OPT_RODC_TOPOLOGY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_OPT_TWOWAY_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_OPT_USER_OWNED_SCHEDULE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSCONN_OPT_USE_NOTIFY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSDSA_OPT_BLOCK_RPC: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSDSA_OPT_DISABLE_INBOUND_REPL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSDSA_OPT_DISABLE_NTDSCONN_XLATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSDSA_OPT_DISABLE_OUTBOUND_REPL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSDSA_OPT_DISABLE_SPN_REGISTRATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSDSA_OPT_GENERATE_OWN_TOPO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSDSA_OPT_IS_GC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_DEFAULT_SERVER_REDUNDANCY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_FORCE_KCC_W2K_ELECTION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_FORCE_KCC_WHISTLER_BEHAVIOR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_IS_AUTO_TOPOLOGY_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_IS_GROUP_CACHING_ENABLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_IS_INTER_SITE_AUTO_TOPOLOGY_DISABLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_IS_RAND_BH_SELECTION_DISABLED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_IS_REDUNDANT_SERVER_TOPOLOGY_ENABLED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_IS_SCHEDULE_HASHING_ENABLED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_IS_TOPL_CLEANUP_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_IS_TOPL_DETECT_STALE_DISABLED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_IS_TOPL_MIN_HOPS_DISABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_W2K3_BRIDGES_REQUIRED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSETTINGS_OPT_W2K3_IGNORE_SCHEDULES: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSITECONN_OPT_DISABLE_COMPRESSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSITECONN_OPT_TWOWAY_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSITECONN_OPT_USE_NOTIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSITELINK_OPT_DISABLE_COMPRESSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSITELINK_OPT_TWOWAY_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSSITELINK_OPT_USE_NOTIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSTRANSPORT_OPT_BRIDGES_REQUIRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NTDSTRANSPORT_OPT_IGNORE_SCHEDULES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NameTranslate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x274fae1f_3626_11d1_a3a4_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const NetAddress: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0b71247_4080_11d1_a3ac_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_DEFAULTFORM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_HIDEMENUS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_HIDESEARCHUI: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_ISSUEONOPEN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_LOADQUERY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_OKCANCEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_PARAMISPROPERTYBAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_REMOVEFORMS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_REMOVESCOPES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_SAVEQUERYONOK: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_SHOWOPTIONAL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OQWF_SINGLESELECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const OctetList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1241400f_4680_11d1_a3b4_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const Path: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2538919_4080_11d1_a3ac_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const Pathname: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x080d0d78_f421_11d0_a36e_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const PostalAddress: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a75afcd_4680_11d1_a3b4_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const PropertyEntry: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d3edc2_a4c4_11d0_8533_00c04fd8d503);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const PropertyValue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b9e38b0_a97c_11d0_8534_00c04fd8d503);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const QUERYFORM_CHANGESFORMLIST: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const QUERYFORM_CHANGESOPTFORMLIST: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ReplicaPointer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5d1badf_4080_11d1_a3ac_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const SCHEDULE_BANDWIDTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const SCHEDULE_INTERVAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const SCHEDULE_PRIORITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const STATUS_SEVERITY_ERROR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const STATUS_SEVERITY_INFORMATIONAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const STATUS_SEVERITY_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const STATUS_SEVERITY_WARNING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const SecurityDescriptor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb958f73c_9bdd_11d0_852c_00c04fd8d503);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const Timestamp: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2bed2eb_4080_11d1_a3ac_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const TypedName: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb33143cb_4080_11d1_a3ac_00c04fb950dc);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const WM_ADSPROP_NOTIFY_APPLY: u32 = 2128u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const WM_ADSPROP_NOTIFY_CHANGE: u32 = 2127u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const WM_ADSPROP_NOTIFY_ERROR: u32 = 2134u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const WM_ADSPROP_NOTIFY_EXIT: u32 = 2131u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const WM_ADSPROP_NOTIFY_FOREGROUND: u32 = 2130u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const WM_ADSPROP_NOTIFY_PAGEHWND: u32 = 2126u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const WM_ADSPROP_NOTIFY_PAGEINIT: u32 = 2125u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const WM_ADSPROP_NOTIFY_SETFOCUS: u32 = 2129u32;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const WinNTSystemInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66182ec4_afd1_11d2_9cb9_0000f87a369e);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrAccessDenied: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522189i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrAfterInitialization: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522246i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrAlreadyInitialized: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523066i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrAlreadyOpen: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589627i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrAlreadyPrepared: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522489i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBFInUse: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523894i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBFNotSynchronous: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013265720i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBFPageNotFound: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013265719i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBackupDirectoryNotEmpty: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523592i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBackupInProgress: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523591i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBackupNotAllowedYet: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523573i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBadBackupDatabaseSize: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523535i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBadCheckpointSignature: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523564i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBadColumnId: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522579i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBadDbSignature: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523565i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBadItagSequence: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522578i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBadLogSignature: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523566i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBadLogVersion: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523582i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBufferTooSmall: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523058i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrBufferTruncated: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264914i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrCannotBeTagged: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522575i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrCannotRename: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522790i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrCheckpointCorrupt: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523563i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrCircularLogging: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589621i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumn2ndSysMaint: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522586i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnCannotIndex: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522583i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnDoesNotFit: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522593i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnDuplicate: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522588i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnInUse: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523050i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnIndexed: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522591i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnLong: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522595i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnMaxTruncated: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264408i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnNotFound: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522589i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnNotUpdatable: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523048i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnNull: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264916i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnSetNull: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264852i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrColumnTooBig: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522590i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrCommunicationError: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589619i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrConsistentTimeMismatch: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523545i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrContainerNotEmpty: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523053i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrContentsExpired: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589615i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrCouldNotConnect: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589625i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrCreateIndexFailed: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264511i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrCurrencyStackOutOfMemory: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523026i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDatabaseAttached: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264913i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDatabaseCorrupted: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522890i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDatabaseDuplicate: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522895i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDatabaseInUse: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522894i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDatabaseInconsistent: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523546i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDatabaseInvalidName: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522892i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDatabaseInvalidPages: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522891i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDatabaseLocked: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522889i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDatabaseNotFound: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522893i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDeleteBackupFileFail: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523572i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDensityInvalid: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522789i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDiskFull: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522288i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrDiskIO: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523074i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrError: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589630i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrExistingLogFileHasBadSignature: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013265362i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrExistingLogFileIsNotContiguous: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013265361i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrFLDKeyTooBig: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013265520i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrFLDNullKey: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013265518i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrFLDTooManySegments: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523695i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrFeatureNotAvailable: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523095i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrFileAccessDenied: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523064i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrFileClose: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523994i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrFileNotFound: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522285i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrFileOpenReadOnly: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264107i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrFullBackupNotTaken: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589618i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrGivenLogFileHasBadSignature: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523541i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrGivenLogFileIsNotContiguous: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523540i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrIllegalOperation: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522784i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInTransaction: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522988i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrIncrementalBackupDisabled: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589623i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrIndexCantBuild: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522695i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrIndexDuplicate: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522693i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrIndexHasClustered: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522688i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrIndexHasPrimary: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522694i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrIndexInUse: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523045i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrIndexInvalidDef: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522690i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrIndexMustStay: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522691i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrIndexNotFound: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522692i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidBackup: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523570i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidBackupSequence: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523575i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidBookmark: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523051i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidBufferSize: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523049i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidCodePage: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523033i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidColumnType: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522585i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidCountry: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523035i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidDatabase: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523068i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidDatabaseId: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523086i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidFilename: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523052i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidHandle: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589629i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidLanguageId: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523034i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidLogSequence: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523581i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidName: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523094i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidObject: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522780i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidOnSort: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522394i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidOperation: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522190i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidParam: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589631i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidParameter: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523093i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidPath: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523073i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidRecips: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589626i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidSesid: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522992i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrInvalidTableId: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522786i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrKeyChanged: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264302i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrKeyDuplicate: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522491i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrKeyIsMade: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522580i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrKeyNotMade: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522488i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrLogBufferTooSmall: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523579i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrLogCorrupted: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522244i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrLogDiskFull: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523567i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrLogFileCorrupt: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523595i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrLogFileNotFound: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589622i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrLogSequenceEnd: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523577i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrLogWriteFail: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523586i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrLoggingDisabled: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523580i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrMakeBackupDirectoryFail: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523571i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrMissingExpiryToken: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589617i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrMissingFullBackup: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523536i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrMissingLogFile: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523568i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrMissingPreviousLogFile: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523587i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrMissingRestoreLogFiles: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523539i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNoBackup: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523576i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNoBackupDirectory: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523593i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNoCurrentIndex: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522581i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNoCurrentRecord: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522493i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNoFullRestore: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589620i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNoIdleActivity: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264862i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNoWriteLock: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264853i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNone: ::windows::core::HRESULT = ::windows::core::HRESULT(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNotInTransaction: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523042i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNotInitialized: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523067i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNullInvalid: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522592i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNullKeyDisallowed: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523043i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrNyi: ::windows::core::HRESULT = ::windows::core::HRESULT(-1073741823i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrObjectDuplicate: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522782i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrObjectNotFound: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522791i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrOutOfBuffers: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523082i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrOutOfCursors: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523083i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrOutOfDatabaseSpace: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523084i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrOutOfFileHandles: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523076i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrOutOfMemory: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523085i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrOutOfSessions: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522995i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrOutOfThreads: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523993i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrPMRecDeleted: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523794i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrPatchFileMismatch: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523544i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrPermissionDenied: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522287i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrReadVerifyFailure: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523078i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrRecordClusteredChanged: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522492i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrRecordDeleted: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523079i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrRecordNotFound: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522495i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrRecordTooBig: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523070i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrRecoveredWithErrors: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523569i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrRemainingVersions: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013265599i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrRestoreInProgress: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589628i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrRestoreLogTooHigh: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523542i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrRestoreLogTooLow: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523543i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrRestoreMapExists: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589624i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrSeekNotEqual: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264881i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrSessionWriteConflict: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522989i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTableDuplicate: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522793i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTableEmpty: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264619i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTableInUse: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522792i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTableLocked: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522794i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTableNotEmpty: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522788i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTaggedNotNULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522582i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTempFileOpenError: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522293i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTermInProgress: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523096i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTooManyActiveUsers: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523037i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTooManyAttachedDatabases: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522291i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTooManyColumns: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523056i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTooManyIO: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523991i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTooManyIndexes: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523081i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTooManyKeys: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523080i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTooManyOpenDatabases: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523069i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTooManyOpenIndexes: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522686i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTooManyOpenTables: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522785i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTooManySorts: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522395i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrTransTooDeep: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522993i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrUnknownExpiryTokenFormat: ::windows::core::HRESULT = ::windows::core::HRESULT(-939589616i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrUpdateNotPrepared: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522487i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrVersionStoreOutOfMemory: ::windows::core::HRESULT = ::windows::core::HRESULT(-939523027i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrWriteConflict: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522994i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrerrDataHasChanged: ::windows::core::HRESULT = ::windows::core::HRESULT(-939522485i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const hrwrnDataHasChanged: ::windows::core::HRESULT = ::windows::core::HRESULT(-2013264310i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADSI_DIALECT_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSI_DIALECT_LDAP: ADSI_DIALECT_ENUM = ADSI_DIALECT_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSI_DIALECT_SQL: ADSI_DIALECT_ENUM = ADSI_DIALECT_ENUM(1i32);
impl ::core::marker::Copy for ADSI_DIALECT_ENUM {}
impl ::core::clone::Clone for ADSI_DIALECT_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADSI_DIALECT_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADSTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_INVALID: ADSTYPE = ADSTYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_DN_STRING: ADSTYPE = ADSTYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_CASE_EXACT_STRING: ADSTYPE = ADSTYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_CASE_IGNORE_STRING: ADSTYPE = ADSTYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_PRINTABLE_STRING: ADSTYPE = ADSTYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_NUMERIC_STRING: ADSTYPE = ADSTYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_BOOLEAN: ADSTYPE = ADSTYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_INTEGER: ADSTYPE = ADSTYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_OCTET_STRING: ADSTYPE = ADSTYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_UTC_TIME: ADSTYPE = ADSTYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_LARGE_INTEGER: ADSTYPE = ADSTYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_PROV_SPECIFIC: ADSTYPE = ADSTYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_OBJECT_CLASS: ADSTYPE = ADSTYPE(12i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_CASEIGNORE_LIST: ADSTYPE = ADSTYPE(13i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_OCTET_LIST: ADSTYPE = ADSTYPE(14i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_PATH: ADSTYPE = ADSTYPE(15i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_POSTALADDRESS: ADSTYPE = ADSTYPE(16i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_TIMESTAMP: ADSTYPE = ADSTYPE(17i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_BACKLINK: ADSTYPE = ADSTYPE(18i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_TYPEDNAME: ADSTYPE = ADSTYPE(19i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_HOLD: ADSTYPE = ADSTYPE(20i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_NETADDRESS: ADSTYPE = ADSTYPE(21i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_REPLICAPOINTER: ADSTYPE = ADSTYPE(22i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_FAXNUMBER: ADSTYPE = ADSTYPE(23i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_EMAIL: ADSTYPE = ADSTYPE(24i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_NT_SECURITY_DESCRIPTOR: ADSTYPE = ADSTYPE(25i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_UNKNOWN: ADSTYPE = ADSTYPE(26i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_DN_WITH_BINARY: ADSTYPE = ADSTYPE(27i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSTYPE_DN_WITH_STRING: ADSTYPE = ADSTYPE(28i32);
impl ::core::marker::Copy for ADSTYPE {}
impl ::core::clone::Clone for ADSTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADSTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_ACEFLAG_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACEFLAG_INHERIT_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACEFLAG_NO_PROPAGATE_INHERIT_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACEFLAG_INHERIT_ONLY_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACEFLAG_INHERITED_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACEFLAG_VALID_INHERIT_FLAGS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(31i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACEFLAG_SUCCESSFUL_ACCESS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACEFLAG_FAILED_ACCESS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(128i32);
impl ::core::marker::Copy for ADS_ACEFLAG_ENUM {}
impl ::core::clone::Clone for ADS_ACEFLAG_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_ACEFLAG_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_ACETYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_ACCESS_ALLOWED: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_ACCESS_DENIED: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_SYSTEM_AUDIT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_ACCESS_ALLOWED_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_ACCESS_DENIED_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_SYSTEM_AUDIT_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(7i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_SYSTEM_ALARM_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(9i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_ACCESS_DENIED_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(10i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(11i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_ACCESS_DENIED_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(12i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(13i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_SYSTEM_ALARM_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(14i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(15i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ACETYPE_SYSTEM_ALARM_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(16i32);
impl ::core::marker::Copy for ADS_ACETYPE_ENUM {}
impl ::core::clone::Clone for ADS_ACETYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_ACETYPE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_AUTHENTICATION_ENUM(pub u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SECURE_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(1u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_USE_ENCRYPTION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_USE_SSL: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_READONLY_SERVER: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(4u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_PROMPT_CREDENTIALS: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(8u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NO_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(16u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FAST_BIND: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(32u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_USE_SIGNING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(64u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_USE_SEALING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(128u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_USE_DELEGATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(256u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SERVER_BIND: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(512u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NO_REFERRAL_CHASING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(1024u32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_AUTH_RESERVED: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2147483648u32);
impl ::core::marker::Copy for ADS_AUTHENTICATION_ENUM {}
impl ::core::clone::Clone for ADS_AUTHENTICATION_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_AUTHENTICATION_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_CHASE_REFERRALS_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_CHASE_REFERRALS_NEVER: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_CHASE_REFERRALS_SUBORDINATE: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_CHASE_REFERRALS_EXTERNAL: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_CHASE_REFERRALS_ALWAYS: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(96i32);
impl ::core::marker::Copy for ADS_CHASE_REFERRALS_ENUM {}
impl ::core::clone::Clone for ADS_CHASE_REFERRALS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_CHASE_REFERRALS_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_DEREFENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_DEREF_NEVER: ADS_DEREFENUM = ADS_DEREFENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_DEREF_SEARCHING: ADS_DEREFENUM = ADS_DEREFENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_DEREF_FINDING: ADS_DEREFENUM = ADS_DEREFENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_DEREF_ALWAYS: ADS_DEREFENUM = ADS_DEREFENUM(3i32);
impl ::core::marker::Copy for ADS_DEREFENUM {}
impl ::core::clone::Clone for ADS_DEREFENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_DEREFENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_DISPLAY_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_DISPLAY_FULL: ADS_DISPLAY_ENUM = ADS_DISPLAY_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_DISPLAY_VALUE_ONLY: ADS_DISPLAY_ENUM = ADS_DISPLAY_ENUM(2i32);
impl ::core::marker::Copy for ADS_DISPLAY_ENUM {}
impl ::core::clone::Clone for ADS_DISPLAY_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_DISPLAY_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_ESCAPE_MODE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ESCAPEDMODE_DEFAULT: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ESCAPEDMODE_ON: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ESCAPEDMODE_OFF: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_ESCAPEDMODE_OFF_EX: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(4i32);
impl ::core::marker::Copy for ADS_ESCAPE_MODE_ENUM {}
impl ::core::clone::Clone for ADS_ESCAPE_MODE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_ESCAPE_MODE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_FLAGTYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FLAG_OBJECT_TYPE_PRESENT: ADS_FLAGTYPE_ENUM = ADS_FLAGTYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FLAG_INHERITED_OBJECT_TYPE_PRESENT: ADS_FLAGTYPE_ENUM = ADS_FLAGTYPE_ENUM(2i32);
impl ::core::marker::Copy for ADS_FLAGTYPE_ENUM {}
impl ::core::clone::Clone for ADS_FLAGTYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_FLAGTYPE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_FORMAT_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_WINDOWS: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_WINDOWS_NO_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_WINDOWS_DN: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_WINDOWS_PARENT: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_X500: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_X500_NO_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_X500_DN: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(7i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_X500_PARENT: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(9i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_PROVIDER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(10i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_FORMAT_LEAF: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(11i32);
impl ::core::marker::Copy for ADS_FORMAT_ENUM {}
impl ::core::clone::Clone for ADS_FORMAT_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_FORMAT_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_GROUP_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_GROUP_TYPE_GLOBAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_GROUP_TYPE_DOMAIN_LOCAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_GROUP_TYPE_LOCAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_GROUP_TYPE_UNIVERSAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_GROUP_TYPE_SECURITY_ENABLED: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(-2147483648i32);
impl ::core::marker::Copy for ADS_GROUP_TYPE_ENUM {}
impl ::core::clone::Clone for ADS_GROUP_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_GROUP_TYPE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_NAME_INITTYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_INITTYPE_DOMAIN: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_INITTYPE_SERVER: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_INITTYPE_GC: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(3i32);
impl ::core::marker::Copy for ADS_NAME_INITTYPE_ENUM {}
impl ::core::clone::Clone for ADS_NAME_INITTYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_NAME_INITTYPE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_NAME_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_1779: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_CANONICAL: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_NT4: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_DISPLAY: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_DOMAIN_SIMPLE: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_ENTERPRISE_SIMPLE: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_GUID: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(7i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_UNKNOWN: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_USER_PRINCIPAL_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(9i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_CANONICAL_EX: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(10i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_SERVICE_PRINCIPAL_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(11i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_NAME_TYPE_SID_OR_SID_HISTORY_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(12i32);
impl ::core::marker::Copy for ADS_NAME_TYPE_ENUM {}
impl ::core::clone::Clone for ADS_NAME_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_NAME_TYPE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_OPTION_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_OPTION_SERVERNAME: ADS_OPTION_ENUM = ADS_OPTION_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_OPTION_REFERRALS: ADS_OPTION_ENUM = ADS_OPTION_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_OPTION_PAGE_SIZE: ADS_OPTION_ENUM = ADS_OPTION_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_OPTION_SECURITY_MASK: ADS_OPTION_ENUM = ADS_OPTION_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_OPTION_MUTUAL_AUTH_STATUS: ADS_OPTION_ENUM = ADS_OPTION_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_OPTION_QUOTA: ADS_OPTION_ENUM = ADS_OPTION_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_OPTION_PASSWORD_PORTNUMBER: ADS_OPTION_ENUM = ADS_OPTION_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_OPTION_PASSWORD_METHOD: ADS_OPTION_ENUM = ADS_OPTION_ENUM(7i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_OPTION_ACCUMULATIVE_MODIFICATION: ADS_OPTION_ENUM = ADS_OPTION_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_OPTION_SKIP_SID_LOOKUP: ADS_OPTION_ENUM = ADS_OPTION_ENUM(9i32);
impl ::core::marker::Copy for ADS_OPTION_ENUM {}
impl ::core::clone::Clone for ADS_OPTION_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_OPTION_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_PASSWORD_ENCODING_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_PASSWORD_ENCODE_REQUIRE_SSL: ADS_PASSWORD_ENCODING_ENUM = ADS_PASSWORD_ENCODING_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_PASSWORD_ENCODE_CLEAR: ADS_PASSWORD_ENCODING_ENUM = ADS_PASSWORD_ENCODING_ENUM(1i32);
impl ::core::marker::Copy for ADS_PASSWORD_ENCODING_ENUM {}
impl ::core::clone::Clone for ADS_PASSWORD_ENCODING_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_PASSWORD_ENCODING_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_PATHTYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_PATH_FILE: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_PATH_FILESHARE: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_PATH_REGISTRY: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(3i32);
impl ::core::marker::Copy for ADS_PATHTYPE_ENUM {}
impl ::core::clone::Clone for ADS_PATHTYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_PATHTYPE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_PREFERENCES_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_ASYNCHRONOUS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_DEREF_ALIASES: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_SIZE_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_TIME_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_ATTRIBTYPES_ONLY: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_SEARCH_SCOPE: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_TIMEOUT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_PAGESIZE: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(7i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_PAGED_TIME_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_CHASE_REFERRALS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(9i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_SORT_ON: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(10i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_CACHE_RESULTS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(11i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADSIPROP_ADSIFLAG: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(12i32);
impl ::core::marker::Copy for ADS_PREFERENCES_ENUM {}
impl ::core::clone::Clone for ADS_PREFERENCES_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_PREFERENCES_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_PROPERTY_OPERATION_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_PROPERTY_CLEAR: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_PROPERTY_UPDATE: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_PROPERTY_APPEND: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_PROPERTY_DELETE: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(4i32);
impl ::core::marker::Copy for ADS_PROPERTY_OPERATION_ENUM {}
impl ::core::clone::Clone for ADS_PROPERTY_OPERATION_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_PROPERTY_OPERATION_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_RIGHTS_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_DELETE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(65536i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_READ_CONTROL: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(131072i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_WRITE_DAC: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(262144i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_WRITE_OWNER: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(524288i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_SYNCHRONIZE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1048576i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_ACCESS_SYSTEM_SECURITY: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(16777216i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_GENERIC_READ: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(-2147483648i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_GENERIC_WRITE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1073741824i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_GENERIC_EXECUTE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(536870912i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_GENERIC_ALL: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(268435456i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_DS_CREATE_CHILD: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_DS_DELETE_CHILD: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_ACTRL_DS_LIST: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_DS_SELF: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_DS_READ_PROP: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_DS_WRITE_PROP: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_DS_DELETE_TREE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_DS_LIST_OBJECT: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(128i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_RIGHT_DS_CONTROL_ACCESS: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(256i32);
impl ::core::marker::Copy for ADS_RIGHTS_ENUM {}
impl ::core::clone::Clone for ADS_RIGHTS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_RIGHTS_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SCOPEENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SCOPE_BASE: ADS_SCOPEENUM = ADS_SCOPEENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SCOPE_ONELEVEL: ADS_SCOPEENUM = ADS_SCOPEENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SCOPE_SUBTREE: ADS_SCOPEENUM = ADS_SCOPEENUM(2i32);
impl ::core::marker::Copy for ADS_SCOPEENUM {}
impl ::core::clone::Clone for ADS_SCOPEENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_SCOPEENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SD_CONTROL_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_OWNER_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_GROUP_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_DACL_PRESENT: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_DACL_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_SACL_PRESENT: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_SACL_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_DACL_AUTO_INHERIT_REQ: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(256i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_SACL_AUTO_INHERIT_REQ: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(512i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_DACL_AUTO_INHERITED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(1024i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_SACL_AUTO_INHERITED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(2048i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_DACL_PROTECTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(4096i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_SACL_PROTECTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(8192i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_CONTROL_SE_SELF_RELATIVE: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(32768i32);
impl ::core::marker::Copy for ADS_SD_CONTROL_ENUM {}
impl ::core::clone::Clone for ADS_SD_CONTROL_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_SD_CONTROL_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SD_FORMAT_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_FORMAT_IID: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_FORMAT_RAW: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_FORMAT_HEXSTRING: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(3i32);
impl ::core::marker::Copy for ADS_SD_FORMAT_ENUM {}
impl ::core::clone::Clone for ADS_SD_FORMAT_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_SD_FORMAT_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SD_REVISION_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SD_REVISION_DS: ADS_SD_REVISION_ENUM = ADS_SD_REVISION_ENUM(4i32);
impl ::core::marker::Copy for ADS_SD_REVISION_ENUM {}
impl ::core::clone::Clone for ADS_SD_REVISION_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_SD_REVISION_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SEARCHPREF_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_ASYNCHRONOUS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_DEREF_ALIASES: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_SIZE_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_TIME_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_ATTRIBTYPES_ONLY: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_SEARCH_SCOPE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_TIMEOUT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_PAGESIZE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(7i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_PAGED_TIME_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_CHASE_REFERRALS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(9i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_SORT_ON: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(10i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_CACHE_RESULTS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(11i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_DIRSYNC: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(12i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_TOMBSTONE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(13i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_VLV: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(14i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_ATTRIBUTE_QUERY: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(15i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_SECURITY_MASK: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_DIRSYNC_FLAG: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(17i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SEARCHPREF_EXTENDED_DN: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(18i32);
impl ::core::marker::Copy for ADS_SEARCHPREF_ENUM {}
impl ::core::clone::Clone for ADS_SEARCHPREF_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_SEARCHPREF_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SECURITY_INFO_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SECURITY_INFO_OWNER: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SECURITY_INFO_GROUP: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SECURITY_INFO_DACL: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SECURITY_INFO_SACL: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(8i32);
impl ::core::marker::Copy for ADS_SECURITY_INFO_ENUM {}
impl ::core::clone::Clone for ADS_SECURITY_INFO_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_SECURITY_INFO_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SETTYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SETTYPE_FULL: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SETTYPE_PROVIDER: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SETTYPE_SERVER: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SETTYPE_DN: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(4i32);
impl ::core::marker::Copy for ADS_SETTYPE_ENUM {}
impl ::core::clone::Clone for ADS_SETTYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_SETTYPE_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_STATUSENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_STATUS_S_OK: ADS_STATUSENUM = ADS_STATUSENUM(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_STATUS_INVALID_SEARCHPREF: ADS_STATUSENUM = ADS_STATUSENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_STATUS_INVALID_SEARCHPREFVALUE: ADS_STATUSENUM = ADS_STATUSENUM(2i32);
impl ::core::marker::Copy for ADS_STATUSENUM {}
impl ::core::clone::Clone for ADS_STATUSENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_STATUSENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SYSTEMFLAG_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SYSTEMFLAG_DISALLOW_DELETE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(-2147483648i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_RENAME: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1073741824i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(536870912i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_LIMITED_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(268435456i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SYSTEMFLAG_DOMAIN_DISALLOW_RENAME: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(134217728i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SYSTEMFLAG_DOMAIN_DISALLOW_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(67108864i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SYSTEMFLAG_CR_NTDS_NC: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SYSTEMFLAG_CR_NTDS_DOMAIN: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SYSTEMFLAG_ATTR_NOT_REPLICATED: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_SYSTEMFLAG_ATTR_IS_CONSTRUCTED: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(4i32);
impl ::core::marker::Copy for ADS_SYSTEMFLAG_ENUM {}
impl ::core::clone::Clone for ADS_SYSTEMFLAG_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_SYSTEMFLAG_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_USER_FLAG_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_SCRIPT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_ACCOUNTDISABLE: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_HOMEDIR_REQUIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_LOCKOUT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_PASSWD_NOTREQD: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_PASSWD_CANT_CHANGE: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(128i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_TEMP_DUPLICATE_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(256i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_NORMAL_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(512i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_INTERDOMAIN_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2048i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_WORKSTATION_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(4096i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_SERVER_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8192i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_DONT_EXPIRE_PASSWD: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(65536i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_MNS_LOGON_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(131072i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_SMARTCARD_REQUIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(262144i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_TRUSTED_FOR_DELEGATION: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(524288i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_NOT_DELEGATED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(1048576i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_USE_DES_KEY_ONLY: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2097152i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_DONT_REQUIRE_PREAUTH: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(4194304i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_PASSWORD_EXPIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8388608i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const ADS_UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(16777216i32);
impl ::core::marker::Copy for ADS_USER_FLAG_ENUM {}
impl ::core::clone::Clone for ADS_USER_FLAG_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_USER_FLAG_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSROLE_MACHINE_ROLE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRole_RoleStandaloneWorkstation: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRole_RoleMemberWorkstation: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRole_RoleStandaloneServer: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRole_RoleMemberServer: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRole_RoleBackupDomainController: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRole_RolePrimaryDomainController: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(5i32);
impl ::core::marker::Copy for DSROLE_MACHINE_ROLE {}
impl ::core::clone::Clone for DSROLE_MACHINE_ROLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSROLE_MACHINE_ROLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSROLE_OPERATION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRoleOperationIdle: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRoleOperationActive: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRoleOperationNeedReboot: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(2i32);
impl ::core::marker::Copy for DSROLE_OPERATION_STATE {}
impl ::core::clone::Clone for DSROLE_OPERATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSROLE_OPERATION_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRolePrimaryDomainInfoBasic: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRoleUpgradeStatus: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRoleOperationState: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(3i32);
impl ::core::marker::Copy for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {}
impl ::core::clone::Clone for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSROLE_SERVER_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRoleServerUnknown: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRoleServerPrimary: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DsRoleServerBackup: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(2i32);
impl ::core::marker::Copy for DSROLE_SERVER_STATE {}
impl ::core::clone::Clone for DSROLE_SERVER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSROLE_SERVER_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_KCC_TASKID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_KCC_TASKID_UPDATE_TOPOLOGY: DS_KCC_TASKID = DS_KCC_TASKID(0i32);
impl ::core::marker::Copy for DS_KCC_TASKID {}
impl ::core::clone::Clone for DS_KCC_TASKID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_KCC_TASKID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_MANGLE_FOR(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_MANGLE_UNKNOWN: DS_MANGLE_FOR = DS_MANGLE_FOR(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_MANGLE_OBJECT_RDN_FOR_DELETION: DS_MANGLE_FOR = DS_MANGLE_FOR(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_MANGLE_OBJECT_RDN_FOR_NAME_CONFLICT: DS_MANGLE_FOR = DS_MANGLE_FOR(2i32);
impl ::core::marker::Copy for DS_MANGLE_FOR {}
impl ::core::clone::Clone for DS_MANGLE_FOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_MANGLE_FOR {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_NAME_ERROR(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_NO_ERROR: DS_NAME_ERROR = DS_NAME_ERROR(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_ERROR_RESOLVING: DS_NAME_ERROR = DS_NAME_ERROR(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_ERROR_NOT_FOUND: DS_NAME_ERROR = DS_NAME_ERROR(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_ERROR_NOT_UNIQUE: DS_NAME_ERROR = DS_NAME_ERROR(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_ERROR_NO_MAPPING: DS_NAME_ERROR = DS_NAME_ERROR(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_ERROR_DOMAIN_ONLY: DS_NAME_ERROR = DS_NAME_ERROR(5i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: DS_NAME_ERROR = DS_NAME_ERROR(6i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_ERROR_TRUST_REFERRAL: DS_NAME_ERROR = DS_NAME_ERROR(7i32);
impl ::core::marker::Copy for DS_NAME_ERROR {}
impl ::core::clone::Clone for DS_NAME_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_NAME_ERROR {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_NAME_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_NO_FLAGS: DS_NAME_FLAGS = DS_NAME_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_FLAG_SYNTACTICAL_ONLY: DS_NAME_FLAGS = DS_NAME_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_FLAG_EVAL_AT_DC: DS_NAME_FLAGS = DS_NAME_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_FLAG_GCVERIFY: DS_NAME_FLAGS = DS_NAME_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NAME_FLAG_TRUST_REFERRAL: DS_NAME_FLAGS = DS_NAME_FLAGS(8i32);
impl ::core::marker::Copy for DS_NAME_FLAGS {}
impl ::core::clone::Clone for DS_NAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_NAME_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_NAME_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_UNKNOWN_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_FQDN_1779_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_NT4_ACCOUNT_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DISPLAY_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_UNIQUE_ID_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(6i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_CANONICAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(7i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_USER_PRINCIPAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_CANONICAL_NAME_EX: DS_NAME_FORMAT = DS_NAME_FORMAT(9i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SERVICE_PRINCIPAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(10i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SID_OR_SID_HISTORY_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(11i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_DNS_DOMAIN_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(12i32);
impl ::core::marker::Copy for DS_NAME_FORMAT {}
impl ::core::clone::Clone for DS_NAME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_NAME_FORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_REPL_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_NEIGHBORS: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_CURSORS_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_METADATA_FOR_OBJ: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_KCC_DSA_CONNECT_FAILURES: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_KCC_DSA_LINK_FAILURES: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_PENDING_OPS: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_METADATA_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_CURSORS_2_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_CURSORS_3_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_METADATA_2_FOR_OBJ: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_METADATA_2_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_METADATA_EXT_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_INFO_TYPE_MAX: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(12i32);
impl ::core::marker::Copy for DS_REPL_INFO_TYPE {}
impl ::core::clone::Clone for DS_REPL_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPL_INFO_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_REPL_OP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_OP_TYPE_SYNC: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_OP_TYPE_ADD: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_OP_TYPE_DELETE: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_OP_TYPE_MODIFY: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPL_OP_TYPE_UPDATE_REFS: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(4i32);
impl ::core::marker::Copy for DS_REPL_OP_TYPE {}
impl ::core::clone::Clone for DS_REPL_OP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPL_OP_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_REPSYNCALL_ERROR(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_WIN32_ERROR_CONTACTING_SERVER: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_WIN32_ERROR_REPLICATING: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_SERVER_UNREACHABLE: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(2i32);
impl ::core::marker::Copy for DS_REPSYNCALL_ERROR {}
impl ::core::clone::Clone for DS_REPSYNCALL_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPSYNCALL_ERROR {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_REPSYNCALL_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_EVENT_ERROR: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_EVENT_SYNC_STARTED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_EVENT_SYNC_COMPLETED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_REPSYNCALL_EVENT_FINISHED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(3i32);
impl ::core::marker::Copy for DS_REPSYNCALL_EVENT {}
impl ::core::clone::Clone for DS_REPSYNCALL_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPSYNCALL_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_SPN_NAME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SPN_DNS_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SPN_DN_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SPN_NB_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SPN_DOMAIN: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SPN_NB_DOMAIN: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SPN_SERVICE: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(5i32);
impl ::core::marker::Copy for DS_SPN_NAME_TYPE {}
impl ::core::clone::Clone for DS_SPN_NAME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_SPN_NAME_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_SPN_WRITE_OP(pub i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SPN_ADD_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(0i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SPN_REPLACE_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(1i32);
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub const DS_SPN_DELETE_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(2i32);
impl ::core::marker::Copy for DS_SPN_WRITE_OP {}
impl ::core::clone::Clone for DS_SPN_WRITE_OP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_SPN_WRITE_OP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADSPROPERROR {
    pub hwndPage: super::super::Foundation::HWND,
    pub pszPageTitle: ::windows::core::PWSTR,
    pub pszObjPath: ::windows::core::PWSTR,
    pub pszObjClass: ::windows::core::PWSTR,
    pub hr: ::windows::core::HRESULT,
    pub pszError: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADSPROPERROR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADSPROPERROR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADSPROPERROR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADSPROPINITPARAMS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hr: ::windows::core::HRESULT,
    pub pDsObj: ::windows::core::ManuallyDrop<IDirectoryObject>,
    pub pwzCN: ::windows::core::PWSTR,
    pub pWritableAttrs: *mut ADS_ATTR_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADSPROPINITPARAMS {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADSPROPINITPARAMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADSVALUE {
    pub dwType: ADSTYPE,
    pub Anonymous: ADSVALUE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADSVALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADSVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADSVALUE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union ADSVALUE_0 {
    pub DNString: *mut u16,
    pub CaseExactString: *mut u16,
    pub CaseIgnoreString: *mut u16,
    pub PrintableString: *mut u16,
    pub NumericString: *mut u16,
    pub Boolean: u32,
    pub Integer: u32,
    pub OctetString: ADS_OCTET_STRING,
    pub UTCTime: super::super::Foundation::SYSTEMTIME,
    pub LargeInteger: i64,
    pub ClassName: *mut u16,
    pub ProviderSpecific: ADS_PROV_SPECIFIC,
    pub pCaseIgnoreList: *mut ADS_CASEIGNORE_LIST,
    pub pOctetList: *mut ADS_OCTET_LIST,
    pub pPath: *mut ADS_PATH,
    pub pPostalAddress: *mut ADS_POSTALADDRESS,
    pub Timestamp: ADS_TIMESTAMP,
    pub BackLink: ADS_BACKLINK,
    pub pTypedName: *mut ADS_TYPEDNAME,
    pub Hold: ADS_HOLD,
    pub pNetAddress: *mut ADS_NETADDRESS,
    pub pReplicaPointer: *mut ADS_REPLICAPOINTER,
    pub pFaxNumber: *mut ADS_FAXNUMBER,
    pub Email: ADS_EMAIL,
    pub SecurityDescriptor: ADS_NT_SECURITY_DESCRIPTOR,
    pub pDNWithBinary: *mut ADS_DN_WITH_BINARY,
    pub pDNWithString: *mut ADS_DN_WITH_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADSVALUE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADSVALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADSVALUE_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_ATTR_DEF {
    pub pszAttrName: ::windows::core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub dwMinRange: u32,
    pub dwMaxRange: u32,
    pub fMultiValued: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_ATTR_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_ATTR_DEF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADS_ATTR_DEF {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_ATTR_INFO {
    pub pszAttrName: ::windows::core::PWSTR,
    pub dwControlCode: u32,
    pub dwADsType: ADSTYPE,
    pub pADsValues: *mut ADSVALUE,
    pub dwNumValues: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_ATTR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_ATTR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADS_ATTR_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_BACKLINK {
    pub RemoteID: u32,
    pub ObjectName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ADS_BACKLINK {}
impl ::core::clone::Clone for ADS_BACKLINK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_BACKLINK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_CASEIGNORE_LIST {
    pub Next: *mut ADS_CASEIGNORE_LIST,
    pub String: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ADS_CASEIGNORE_LIST {}
impl ::core::clone::Clone for ADS_CASEIGNORE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_CASEIGNORE_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_CLASS_DEF {
    pub pszClassName: ::windows::core::PWSTR,
    pub dwMandatoryAttrs: u32,
    pub ppszMandatoryAttrs: *mut ::windows::core::PWSTR,
    pub optionalAttrs: u32,
    pub ppszOptionalAttrs: *mut *mut ::windows::core::PWSTR,
    pub dwNamingAttrs: u32,
    pub ppszNamingAttrs: *mut *mut ::windows::core::PWSTR,
    pub dwSuperClasses: u32,
    pub ppszSuperClasses: *mut *mut ::windows::core::PWSTR,
    pub fIsContainer: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_CLASS_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_CLASS_DEF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADS_CLASS_DEF {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_DN_WITH_BINARY {
    pub dwLength: u32,
    pub lpBinaryValue: *mut u8,
    pub pszDNString: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ADS_DN_WITH_BINARY {}
impl ::core::clone::Clone for ADS_DN_WITH_BINARY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_DN_WITH_BINARY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_DN_WITH_STRING {
    pub pszStringValue: ::windows::core::PWSTR,
    pub pszDNString: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ADS_DN_WITH_STRING {}
impl ::core::clone::Clone for ADS_DN_WITH_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_DN_WITH_STRING {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_EMAIL {
    pub Address: ::windows::core::PWSTR,
    pub Type: u32,
}
impl ::core::marker::Copy for ADS_EMAIL {}
impl ::core::clone::Clone for ADS_EMAIL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_EMAIL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_FAXNUMBER {
    pub TelephoneNumber: ::windows::core::PWSTR,
    pub NumberOfBits: u32,
    pub Parameters: *mut u8,
}
impl ::core::marker::Copy for ADS_FAXNUMBER {}
impl ::core::clone::Clone for ADS_FAXNUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_FAXNUMBER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_HOLD {
    pub ObjectName: ::windows::core::PWSTR,
    pub Amount: u32,
}
impl ::core::marker::Copy for ADS_HOLD {}
impl ::core::clone::Clone for ADS_HOLD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_HOLD {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_NETADDRESS {
    pub AddressType: u32,
    pub AddressLength: u32,
    pub Address: *mut u8,
}
impl ::core::marker::Copy for ADS_NETADDRESS {}
impl ::core::clone::Clone for ADS_NETADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_NETADDRESS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_NT_SECURITY_DESCRIPTOR {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl ::core::marker::Copy for ADS_NT_SECURITY_DESCRIPTOR {}
impl ::core::clone::Clone for ADS_NT_SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_NT_SECURITY_DESCRIPTOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_OBJECT_INFO {
    pub pszRDN: ::windows::core::PWSTR,
    pub pszObjectDN: ::windows::core::PWSTR,
    pub pszParentDN: ::windows::core::PWSTR,
    pub pszSchemaDN: ::windows::core::PWSTR,
    pub pszClassName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ADS_OBJECT_INFO {}
impl ::core::clone::Clone for ADS_OBJECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_OBJECT_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_OCTET_LIST {
    pub Next: *mut ADS_OCTET_LIST,
    pub Length: u32,
    pub Data: *mut u8,
}
impl ::core::marker::Copy for ADS_OCTET_LIST {}
impl ::core::clone::Clone for ADS_OCTET_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_OCTET_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl ::core::marker::Copy for ADS_OCTET_STRING {}
impl ::core::clone::Clone for ADS_OCTET_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_OCTET_STRING {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_PATH {
    pub Type: u32,
    pub VolumeName: ::windows::core::PWSTR,
    pub Path: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for ADS_PATH {}
impl ::core::clone::Clone for ADS_PATH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_PATH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_POSTALADDRESS {
    pub PostalAddress: [::windows::core::PWSTR; 6],
}
impl ::core::marker::Copy for ADS_POSTALADDRESS {}
impl ::core::clone::Clone for ADS_POSTALADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_POSTALADDRESS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_PROV_SPECIFIC {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl ::core::marker::Copy for ADS_PROV_SPECIFIC {}
impl ::core::clone::Clone for ADS_PROV_SPECIFIC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_PROV_SPECIFIC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_REPLICAPOINTER {
    pub ServerName: ::windows::core::PWSTR,
    pub ReplicaType: u32,
    pub ReplicaNumber: u32,
    pub Count: u32,
    pub ReplicaAddressHints: *mut ADS_NETADDRESS,
}
impl ::core::marker::Copy for ADS_REPLICAPOINTER {}
impl ::core::clone::Clone for ADS_REPLICAPOINTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_REPLICAPOINTER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_SEARCHPREF_INFO {
    pub dwSearchPref: ADS_SEARCHPREF_ENUM,
    pub vValue: ADSVALUE,
    pub dwStatus: ADS_STATUSENUM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_SEARCHPREF_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_SEARCHPREF_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADS_SEARCHPREF_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_SEARCH_COLUMN {
    pub pszAttrName: ::windows::core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub pADsValues: *mut ADSVALUE,
    pub dwNumValues: u32,
    pub hReserved: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_SEARCH_COLUMN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_SEARCH_COLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADS_SEARCH_COLUMN {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SEARCH_HANDLE(pub isize);
impl ADS_SEARCH_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for ADS_SEARCH_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for ADS_SEARCH_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for ADS_SEARCH_HANDLE {}
impl ::core::fmt::Debug for ADS_SEARCH_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SEARCH_HANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<ADS_SEARCH_HANDLE>> for ADS_SEARCH_HANDLE {
    fn from(optional: ::core::option::Option<ADS_SEARCH_HANDLE>) -> ADS_SEARCH_HANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for ADS_SEARCH_HANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_SORTKEY {
    pub pszAttrType: ::windows::core::PWSTR,
    pub pszReserved: ::windows::core::PWSTR,
    pub fReverseorder: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_SORTKEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_SORTKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADS_SORTKEY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_TIMESTAMP {
    pub WholeSeconds: u32,
    pub EventID: u32,
}
impl ::core::marker::Copy for ADS_TIMESTAMP {}
impl ::core::clone::Clone for ADS_TIMESTAMP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_TIMESTAMP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_TYPEDNAME {
    pub ObjectName: ::windows::core::PWSTR,
    pub Level: u32,
    pub Interval: u32,
}
impl ::core::marker::Copy for ADS_TYPEDNAME {}
impl ::core::clone::Clone for ADS_TYPEDNAME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_TYPEDNAME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct ADS_VLV {
    pub dwBeforeCount: u32,
    pub dwAfterCount: u32,
    pub dwOffset: u32,
    pub dwContentCount: u32,
    pub pszTarget: ::windows::core::PWSTR,
    pub dwContextIDLength: u32,
    pub lpContextID: *mut u8,
}
impl ::core::marker::Copy for ADS_VLV {}
impl ::core::clone::Clone for ADS_VLV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADS_VLV {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct CQFORM {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub clsid: ::windows::core::GUID,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pszTitle: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for CQFORM {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for CQFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for CQFORM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CQPAGE {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub pPageProc: LPCQPAGEPROC,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub idPageName: i32,
    pub idPageTemplate: i32,
    pub pDlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CQPAGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CQPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for CQPAGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOMAINDESC {
    pub pszName: ::windows::core::PWSTR,
    pub pszPath: ::windows::core::PWSTR,
    pub pszNCName: ::windows::core::PWSTR,
    pub pszTrustParent: ::windows::core::PWSTR,
    pub pszObjectClass: ::windows::core::PWSTR,
    pub ulFlags: u32,
    pub fDownLevel: super::super::Foundation::BOOL,
    pub pdChildList: *mut DOMAINDESC,
    pub pdNextSibling: *mut DOMAINDESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOMAINDESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOMAINDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOMAINDESC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DOMAIN_CONTROLLER_INFOA {
    pub DomainControllerName: ::windows::core::PSTR,
    pub DomainControllerAddress: ::windows::core::PSTR,
    pub DomainControllerAddressType: u32,
    pub DomainGuid: ::windows::core::GUID,
    pub DomainName: ::windows::core::PSTR,
    pub DnsForestName: ::windows::core::PSTR,
    pub Flags: u32,
    pub DcSiteName: ::windows::core::PSTR,
    pub ClientSiteName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for DOMAIN_CONTROLLER_INFOA {}
impl ::core::clone::Clone for DOMAIN_CONTROLLER_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOMAIN_CONTROLLER_INFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DOMAIN_CONTROLLER_INFOW {
    pub DomainControllerName: ::windows::core::PWSTR,
    pub DomainControllerAddress: ::windows::core::PWSTR,
    pub DomainControllerAddressType: u32,
    pub DomainGuid: ::windows::core::GUID,
    pub DomainName: ::windows::core::PWSTR,
    pub DnsForestName: ::windows::core::PWSTR,
    pub Flags: u32,
    pub DcSiteName: ::windows::core::PWSTR,
    pub ClientSiteName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DOMAIN_CONTROLLER_INFOW {}
impl ::core::clone::Clone for DOMAIN_CONTROLLER_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOMAIN_CONTROLLER_INFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOMAIN_TREE {
    pub dsSize: u32,
    pub dwCount: u32,
    pub aDomains: [DOMAINDESC; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOMAIN_TREE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOMAIN_TREE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOMAIN_TREE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct DSA_NEWOBJ_DISPINFO {
    pub dwSize: u32,
    pub hObjClassIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub lpszWizTitle: ::windows::core::PWSTR,
    pub lpszContDisplayName: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for DSA_NEWOBJ_DISPINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for DSA_NEWOBJ_DISPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for DSA_NEWOBJ_DISPINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSBITEMA {
    pub cbStruct: u32,
    pub pszADsPath: ::windows::core::PCWSTR,
    pub pszClass: ::windows::core::PCWSTR,
    pub dwMask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szDisplayName: [super::super::Foundation::CHAR; 64],
    pub szIconLocation: [super::super::Foundation::CHAR; 260],
    pub iIconResID: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSBITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSBITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSBITEMA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSBITEMW {
    pub cbStruct: u32,
    pub pszADsPath: ::windows::core::PCWSTR,
    pub pszClass: ::windows::core::PCWSTR,
    pub dwMask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szDisplayName: [u16; 64],
    pub szIconLocation: [u16; 260],
    pub iIconResID: i32,
}
impl ::core::marker::Copy for DSBITEMW {}
impl ::core::clone::Clone for DSBITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSBITEMW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub struct DSBROWSEINFOA {
    pub cbStruct: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pszCaption: ::windows::core::PCSTR,
    pub pszTitle: ::windows::core::PCSTR,
    pub pszRoot: ::windows::core::PCWSTR,
    pub pszPath: ::windows::core::PWSTR,
    pub cchPath: u32,
    pub dwFlags: u32,
    pub pfnCallback: super::super::UI::Shell::BFFCALLBACK,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwReturnFormat: u32,
    pub pUserName: ::windows::core::PCWSTR,
    pub pPassword: ::windows::core::PCWSTR,
    pub pszObjectClass: ::windows::core::PWSTR,
    pub cchObjectClass: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::marker::Copy for DSBROWSEINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::clone::Clone for DSBROWSEINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
unsafe impl ::windows::core::Abi for DSBROWSEINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub struct DSBROWSEINFOW {
    pub cbStruct: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pszCaption: ::windows::core::PCWSTR,
    pub pszTitle: ::windows::core::PCWSTR,
    pub pszRoot: ::windows::core::PCWSTR,
    pub pszPath: ::windows::core::PWSTR,
    pub cchPath: u32,
    pub dwFlags: u32,
    pub pfnCallback: super::super::UI::Shell::BFFCALLBACK,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwReturnFormat: u32,
    pub pUserName: ::windows::core::PCWSTR,
    pub pPassword: ::windows::core::PCWSTR,
    pub pszObjectClass: ::windows::core::PWSTR,
    pub cchObjectClass: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::marker::Copy for DSBROWSEINFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::clone::Clone for DSBROWSEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
unsafe impl ::windows::core::Abi for DSBROWSEINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSCLASSCREATIONINFO {
    pub dwFlags: u32,
    pub clsidWizardDialog: ::windows::core::GUID,
    pub clsidWizardPrimaryPage: ::windows::core::GUID,
    pub cWizardExtensions: u32,
    pub aWizardExtensions: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for DSCLASSCREATIONINFO {}
impl ::core::clone::Clone for DSCLASSCREATIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSCLASSCREATIONINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSCOLUMN {
    pub dwFlags: u32,
    pub fmt: i32,
    pub cx: i32,
    pub idsName: i32,
    pub offsetProperty: i32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DSCOLUMN {}
impl ::core::clone::Clone for DSCOLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSCOLUMN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSDISPLAYSPECOPTIONS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub offsetAttribPrefix: u32,
    pub offsetUserName: u32,
    pub offsetPassword: u32,
    pub offsetServer: u32,
    pub offsetServerConfigPath: u32,
}
impl ::core::marker::Copy for DSDISPLAYSPECOPTIONS {}
impl ::core::clone::Clone for DSDISPLAYSPECOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSDISPLAYSPECOPTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSOBJECT {
    pub dwFlags: u32,
    pub dwProviderFlags: u32,
    pub offsetName: u32,
    pub offsetClass: u32,
}
impl ::core::marker::Copy for DSOBJECT {}
impl ::core::clone::Clone for DSOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSOBJECT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSOBJECTNAMES {
    pub clsidNamespace: ::windows::core::GUID,
    pub cItems: u32,
    pub aObjects: [DSOBJECT; 1],
}
impl ::core::marker::Copy for DSOBJECTNAMES {}
impl ::core::clone::Clone for DSOBJECTNAMES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSOBJECTNAMES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSOP_FILTER_FLAGS {
    pub Uplevel: DSOP_UPLEVEL_FILTER_FLAGS,
    pub flDownlevel: u32,
}
impl ::core::marker::Copy for DSOP_FILTER_FLAGS {}
impl ::core::clone::Clone for DSOP_FILTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSOP_FILTER_FLAGS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSOP_INIT_INFO {
    pub cbSize: u32,
    pub pwzTargetComputer: ::windows::core::PCWSTR,
    pub cDsScopeInfos: u32,
    pub aDsScopeInfos: *mut DSOP_SCOPE_INIT_INFO,
    pub flOptions: u32,
    pub cAttributesToFetch: u32,
    pub apwzAttributeNames: *mut ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DSOP_INIT_INFO {}
impl ::core::clone::Clone for DSOP_INIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSOP_INIT_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSOP_SCOPE_INIT_INFO {
    pub cbSize: u32,
    pub flType: u32,
    pub flScope: u32,
    pub FilterFlags: DSOP_FILTER_FLAGS,
    pub pwzDcName: ::windows::core::PCWSTR,
    pub pwzADsPath: ::windows::core::PCWSTR,
    pub hr: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DSOP_SCOPE_INIT_INFO {}
impl ::core::clone::Clone for DSOP_SCOPE_INIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSOP_SCOPE_INIT_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSOP_UPLEVEL_FILTER_FLAGS {
    pub flBothModes: u32,
    pub flMixedModeOnly: u32,
    pub flNativeModeOnly: u32,
}
impl ::core::marker::Copy for DSOP_UPLEVEL_FILTER_FLAGS {}
impl ::core::clone::Clone for DSOP_UPLEVEL_FILTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSOP_UPLEVEL_FILTER_FLAGS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSPROPERTYPAGEINFO {
    pub offsetString: u32,
}
impl ::core::marker::Copy for DSPROPERTYPAGEINFO {}
impl ::core::clone::Clone for DSPROPERTYPAGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSPROPERTYPAGEINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSQUERYCLASSLIST {
    pub cbStruct: u32,
    pub cClasses: i32,
    pub offsetClass: [u32; 1],
}
impl ::core::marker::Copy for DSQUERYCLASSLIST {}
impl ::core::clone::Clone for DSQUERYCLASSLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSQUERYCLASSLIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSQUERYINITPARAMS {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub pDefaultScope: ::windows::core::PWSTR,
    pub pDefaultSaveLocation: ::windows::core::PWSTR,
    pub pUserName: ::windows::core::PWSTR,
    pub pPassword: ::windows::core::PWSTR,
    pub pServer: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DSQUERYINITPARAMS {}
impl ::core::clone::Clone for DSQUERYINITPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSQUERYINITPARAMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSQUERYPARAMS {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub offsetQuery: i32,
    pub iColumns: i32,
    pub dwReserved: u32,
    pub aColumns: [DSCOLUMN; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSQUERYPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSQUERYPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSQUERYPARAMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSROLE_OPERATION_STATE_INFO {
    pub OperationState: DSROLE_OPERATION_STATE,
}
impl ::core::marker::Copy for DSROLE_OPERATION_STATE_INFO {}
impl ::core::clone::Clone for DSROLE_OPERATION_STATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSROLE_OPERATION_STATE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    pub MachineRole: DSROLE_MACHINE_ROLE,
    pub Flags: u32,
    pub DomainNameFlat: ::windows::core::PWSTR,
    pub DomainNameDns: ::windows::core::PWSTR,
    pub DomainForestName: ::windows::core::PWSTR,
    pub DomainGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {}
impl ::core::clone::Clone for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DSROLE_UPGRADE_STATUS_INFO {
    pub OperationState: u32,
    pub PreviousServerState: DSROLE_SERVER_STATE,
}
impl ::core::marker::Copy for DSROLE_UPGRADE_STATUS_INFO {}
impl ::core::clone::Clone for DSROLE_UPGRADE_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DSROLE_UPGRADE_STATUS_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_1A {
    pub NetbiosName: ::windows::core::PSTR,
    pub DnsHostName: ::windows::core::PSTR,
    pub SiteName: ::windows::core::PSTR,
    pub ComputerObjectName: ::windows::core::PSTR,
    pub ServerObjectName: ::windows::core::PSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_DOMAIN_CONTROLLER_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_1W {
    pub NetbiosName: ::windows::core::PWSTR,
    pub DnsHostName: ::windows::core::PWSTR,
    pub SiteName: ::windows::core::PWSTR,
    pub ComputerObjectName: ::windows::core::PWSTR,
    pub ServerObjectName: ::windows::core::PWSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_DOMAIN_CONTROLLER_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_2A {
    pub NetbiosName: ::windows::core::PSTR,
    pub DnsHostName: ::windows::core::PSTR,
    pub SiteName: ::windows::core::PSTR,
    pub SiteObjectName: ::windows::core::PSTR,
    pub ComputerObjectName: ::windows::core::PSTR,
    pub ServerObjectName: ::windows::core::PSTR,
    pub NtdsDsaObjectName: ::windows::core::PSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: ::windows::core::GUID,
    pub ComputerObjectGuid: ::windows::core::GUID,
    pub ServerObjectGuid: ::windows::core::GUID,
    pub NtdsDsaObjectGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_DOMAIN_CONTROLLER_INFO_2A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_2W {
    pub NetbiosName: ::windows::core::PWSTR,
    pub DnsHostName: ::windows::core::PWSTR,
    pub SiteName: ::windows::core::PWSTR,
    pub SiteObjectName: ::windows::core::PWSTR,
    pub ComputerObjectName: ::windows::core::PWSTR,
    pub ServerObjectName: ::windows::core::PWSTR,
    pub NtdsDsaObjectName: ::windows::core::PWSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: ::windows::core::GUID,
    pub ComputerObjectGuid: ::windows::core::GUID,
    pub ServerObjectGuid: ::windows::core::GUID,
    pub NtdsDsaObjectGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_DOMAIN_CONTROLLER_INFO_2W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_3A {
    pub NetbiosName: ::windows::core::PSTR,
    pub DnsHostName: ::windows::core::PSTR,
    pub SiteName: ::windows::core::PSTR,
    pub SiteObjectName: ::windows::core::PSTR,
    pub ComputerObjectName: ::windows::core::PSTR,
    pub ServerObjectName: ::windows::core::PSTR,
    pub NtdsDsaObjectName: ::windows::core::PSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub fIsRodc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: ::windows::core::GUID,
    pub ComputerObjectGuid: ::windows::core::GUID,
    pub ServerObjectGuid: ::windows::core::GUID,
    pub NtdsDsaObjectGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_3A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_3A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_DOMAIN_CONTROLLER_INFO_3A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_3W {
    pub NetbiosName: ::windows::core::PWSTR,
    pub DnsHostName: ::windows::core::PWSTR,
    pub SiteName: ::windows::core::PWSTR,
    pub SiteObjectName: ::windows::core::PWSTR,
    pub ComputerObjectName: ::windows::core::PWSTR,
    pub ServerObjectName: ::windows::core::PWSTR,
    pub NtdsDsaObjectName: ::windows::core::PWSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub fIsRodc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: ::windows::core::GUID,
    pub ComputerObjectGuid: ::windows::core::GUID,
    pub ServerObjectGuid: ::windows::core::GUID,
    pub NtdsDsaObjectGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_3W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_DOMAIN_CONTROLLER_INFO_3W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_TRUSTSA {
    pub NetbiosDomainName: ::windows::core::PSTR,
    pub DnsDomainName: ::windows::core::PSTR,
    pub Flags: u32,
    pub ParentIndex: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub DomainSid: super::super::Foundation::PSID,
    pub DomainGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_TRUSTSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_TRUSTSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_DOMAIN_TRUSTSA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_TRUSTSW {
    pub NetbiosDomainName: ::windows::core::PWSTR,
    pub DnsDomainName: ::windows::core::PWSTR,
    pub Flags: u32,
    pub ParentIndex: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub DomainSid: super::super::Foundation::PSID,
    pub DomainGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_TRUSTSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_TRUSTSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_DOMAIN_TRUSTSW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_NAME_RESULTA {
    pub cItems: u32,
    pub rItems: *mut DS_NAME_RESULT_ITEMA,
}
impl ::core::marker::Copy for DS_NAME_RESULTA {}
impl ::core::clone::Clone for DS_NAME_RESULTA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_NAME_RESULTA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_NAME_RESULTW {
    pub cItems: u32,
    pub rItems: *mut DS_NAME_RESULT_ITEMW,
}
impl ::core::marker::Copy for DS_NAME_RESULTW {}
impl ::core::clone::Clone for DS_NAME_RESULTW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_NAME_RESULTW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_NAME_RESULT_ITEMA {
    pub status: u32,
    pub pDomain: ::windows::core::PSTR,
    pub pName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for DS_NAME_RESULT_ITEMA {}
impl ::core::clone::Clone for DS_NAME_RESULT_ITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_NAME_RESULT_ITEMA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_NAME_RESULT_ITEMW {
    pub status: u32,
    pub pDomain: ::windows::core::PWSTR,
    pub pName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DS_NAME_RESULT_ITEMW {}
impl ::core::clone::Clone for DS_NAME_RESULT_ITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_NAME_RESULT_ITEMW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_META_DATA {
    pub pszAttributeName: ::windows::core::PWSTR,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows::core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_META_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_ATTR_META_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_META_DATA_2 {
    pub pszAttributeName: ::windows::core::PWSTR,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows::core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub pszLastOriginatingDsaDN: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_META_DATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_ATTR_META_DATA_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_META_DATA_BLOB {
    pub oszAttributeName: u32,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows::core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub oszLastOriginatingDsaDN: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_META_DATA_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_META_DATA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_ATTR_META_DATA_BLOB {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_VALUE_META_DATA {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_VALUE_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_VALUE_META_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_ATTR_VALUE_META_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_VALUE_META_DATA_2 {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA_2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_VALUE_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_ATTR_VALUE_META_DATA_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_VALUE_META_DATA_EXT {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA_EXT; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_VALUE_META_DATA_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_REPL_CURSOR {
    pub uuidSourceDsaInvocationID: ::windows::core::GUID,
    pub usnAttributeFilter: i64,
}
impl ::core::marker::Copy for DS_REPL_CURSOR {}
impl ::core::clone::Clone for DS_REPL_CURSOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPL_CURSOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_REPL_CURSORS {
    pub cNumCursors: u32,
    pub dwReserved: u32,
    pub rgCursor: [DS_REPL_CURSOR; 1],
}
impl ::core::marker::Copy for DS_REPL_CURSORS {}
impl ::core::clone::Clone for DS_REPL_CURSORS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPL_CURSORS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSORS_2 {
    pub cNumCursors: u32,
    pub dwEnumerationContext: u32,
    pub rgCursor: [DS_REPL_CURSOR_2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_CURSORS_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_CURSORS_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_CURSORS_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSORS_3W {
    pub cNumCursors: u32,
    pub dwEnumerationContext: u32,
    pub rgCursor: [DS_REPL_CURSOR_3W; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_CURSORS_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_CURSORS_3W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_CURSORS_3W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSOR_2 {
    pub uuidSourceDsaInvocationID: ::windows::core::GUID,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_CURSOR_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_CURSOR_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_CURSOR_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSOR_3W {
    pub uuidSourceDsaInvocationID: ::windows::core::GUID,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub pszSourceDsaDN: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_CURSOR_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_CURSOR_3W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_CURSOR_3W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSOR_BLOB {
    pub uuidSourceDsaInvocationID: ::windows::core::GUID,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub oszSourceDsaDN: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_CURSOR_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_CURSOR_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_CURSOR_BLOB {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_KCC_DSA_FAILURESW {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgDsaFailure: [DS_REPL_KCC_DSA_FAILUREW; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_KCC_DSA_FAILURESW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_KCC_DSA_FAILURESW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_KCC_DSA_FAILURESW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_KCC_DSA_FAILUREW {
    pub pszDsaDN: ::windows::core::PWSTR,
    pub uuidDsaObjGuid: ::windows::core::GUID,
    pub ftimeFirstFailure: super::super::Foundation::FILETIME,
    pub cNumFailures: u32,
    pub dwLastResult: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_KCC_DSA_FAILUREW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_KCC_DSA_FAILUREW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_KCC_DSA_FAILUREW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_KCC_DSA_FAILUREW_BLOB {
    pub oszDsaDN: u32,
    pub uuidDsaObjGuid: ::windows::core::GUID,
    pub ftimeFirstFailure: super::super::Foundation::FILETIME,
    pub cNumFailures: u32,
    pub dwLastResult: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_KCC_DSA_FAILUREW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_NEIGHBORSW {
    pub cNumNeighbors: u32,
    pub dwReserved: u32,
    pub rgNeighbor: [DS_REPL_NEIGHBORW; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_NEIGHBORSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_NEIGHBORSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_NEIGHBORSW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_NEIGHBORW {
    pub pszNamingContext: ::windows::core::PWSTR,
    pub pszSourceDsaDN: ::windows::core::PWSTR,
    pub pszSourceDsaAddress: ::windows::core::PWSTR,
    pub pszAsyncIntersiteTransportDN: ::windows::core::PWSTR,
    pub dwReplicaFlags: u32,
    pub dwReserved: u32,
    pub uuidNamingContextObjGuid: ::windows::core::GUID,
    pub uuidSourceDsaObjGuid: ::windows::core::GUID,
    pub uuidSourceDsaInvocationID: ::windows::core::GUID,
    pub uuidAsyncIntersiteTransportObjGuid: ::windows::core::GUID,
    pub usnLastObjChangeSynced: i64,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub ftimeLastSyncAttempt: super::super::Foundation::FILETIME,
    pub dwLastSyncResult: u32,
    pub cNumConsecutiveSyncFailures: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_NEIGHBORW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_NEIGHBORW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_NEIGHBORW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_NEIGHBORW_BLOB {
    pub oszNamingContext: u32,
    pub oszSourceDsaDN: u32,
    pub oszSourceDsaAddress: u32,
    pub oszAsyncIntersiteTransportDN: u32,
    pub dwReplicaFlags: u32,
    pub dwReserved: u32,
    pub uuidNamingContextObjGuid: ::windows::core::GUID,
    pub uuidSourceDsaObjGuid: ::windows::core::GUID,
    pub uuidSourceDsaInvocationID: ::windows::core::GUID,
    pub uuidAsyncIntersiteTransportObjGuid: ::windows::core::GUID,
    pub usnLastObjChangeSynced: i64,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub ftimeLastSyncAttempt: super::super::Foundation::FILETIME,
    pub dwLastSyncResult: u32,
    pub cNumConsecutiveSyncFailures: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_NEIGHBORW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_NEIGHBORW_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_NEIGHBORW_BLOB {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OBJ_META_DATA {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgMetaData: [DS_REPL_ATTR_META_DATA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_OBJ_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_OBJ_META_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_OBJ_META_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OBJ_META_DATA_2 {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgMetaData: [DS_REPL_ATTR_META_DATA_2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_OBJ_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_OBJ_META_DATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_OBJ_META_DATA_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OPW {
    pub ftimeEnqueued: super::super::Foundation::FILETIME,
    pub ulSerialNumber: u32,
    pub ulPriority: u32,
    pub OpType: DS_REPL_OP_TYPE,
    pub ulOptions: u32,
    pub pszNamingContext: ::windows::core::PWSTR,
    pub pszDsaDN: ::windows::core::PWSTR,
    pub pszDsaAddress: ::windows::core::PWSTR,
    pub uuidNamingContextObjGuid: ::windows::core::GUID,
    pub uuidDsaObjGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_OPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_OPW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_OPW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OPW_BLOB {
    pub ftimeEnqueued: super::super::Foundation::FILETIME,
    pub ulSerialNumber: u32,
    pub ulPriority: u32,
    pub OpType: DS_REPL_OP_TYPE,
    pub ulOptions: u32,
    pub oszNamingContext: u32,
    pub oszDsaDN: u32,
    pub oszDsaAddress: u32,
    pub uuidNamingContextObjGuid: ::windows::core::GUID,
    pub uuidDsaObjGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_OPW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_OPW_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_OPW_BLOB {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_PENDING_OPSW {
    pub ftimeCurrentOpStarted: super::super::Foundation::FILETIME,
    pub cNumPendingOps: u32,
    pub rgPendingOp: [DS_REPL_OPW; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_PENDING_OPSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_PENDING_OPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_PENDING_OPSW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_QUEUE_STATISTICSW {
    pub ftimeCurrentOpStarted: super::super::Foundation::FILETIME,
    pub cNumPendingOps: u32,
    pub ftimeOldestSync: super::super::Foundation::FILETIME,
    pub ftimeOldestAdd: super::super::Foundation::FILETIME,
    pub ftimeOldestMod: super::super::Foundation::FILETIME,
    pub ftimeOldestDel: super::super::Foundation::FILETIME,
    pub ftimeOldestUpdRefs: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_QUEUE_STATISTICSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_QUEUE_STATISTICSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_QUEUE_STATISTICSW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA {
    pub pszAttributeName: ::windows::core::PWSTR,
    pub pszObjectDn: ::windows::core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows::core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_VALUE_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_VALUE_META_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_VALUE_META_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_2 {
    pub pszAttributeName: ::windows::core::PWSTR,
    pub pszObjectDn: ::windows::core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows::core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub pszLastOriginatingDsaDN: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_VALUE_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_VALUE_META_DATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_VALUE_META_DATA_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_BLOB {
    pub oszAttributeName: u32,
    pub oszObjectDn: u32,
    pub cbData: u32,
    pub obData: u32,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows::core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub oszLastOriginatingDsaDN: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_VALUE_META_DATA_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_VALUE_META_DATA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_VALUE_META_DATA_BLOB {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_BLOB_EXT {
    pub oszAttributeName: u32,
    pub oszObjectDn: u32,
    pub cbData: u32,
    pub obData: u32,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows::core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub oszLastOriginatingDsaDN: u32,
    pub dwUserIdentifier: u32,
    pub dwPriorLinkState: u32,
    pub dwCurrentLinkState: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_VALUE_META_DATA_BLOB_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_EXT {
    pub pszAttributeName: ::windows::core::PWSTR,
    pub pszObjectDn: ::windows::core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows::core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub pszLastOriginatingDsaDN: ::windows::core::PWSTR,
    pub dwUserIdentifier: u32,
    pub dwPriorLinkState: u32,
    pub dwCurrentLinkState: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_VALUE_META_DATA_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_VALUE_META_DATA_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DS_REPL_VALUE_META_DATA_EXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_REPSYNCALL_ERRINFOA {
    pub pszSvrId: ::windows::core::PSTR,
    pub error: DS_REPSYNCALL_ERROR,
    pub dwWin32Err: u32,
    pub pszSrcId: ::windows::core::PSTR,
}
impl ::core::marker::Copy for DS_REPSYNCALL_ERRINFOA {}
impl ::core::clone::Clone for DS_REPSYNCALL_ERRINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPSYNCALL_ERRINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_REPSYNCALL_ERRINFOW {
    pub pszSvrId: ::windows::core::PWSTR,
    pub error: DS_REPSYNCALL_ERROR,
    pub dwWin32Err: u32,
    pub pszSrcId: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DS_REPSYNCALL_ERRINFOW {}
impl ::core::clone::Clone for DS_REPSYNCALL_ERRINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPSYNCALL_ERRINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_REPSYNCALL_SYNCA {
    pub pszSrcId: ::windows::core::PSTR,
    pub pszDstId: ::windows::core::PSTR,
    pub pszNC: ::windows::core::PSTR,
    pub pguidSrc: *mut ::windows::core::GUID,
    pub pguidDst: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for DS_REPSYNCALL_SYNCA {}
impl ::core::clone::Clone for DS_REPSYNCALL_SYNCA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPSYNCALL_SYNCA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_REPSYNCALL_SYNCW {
    pub pszSrcId: ::windows::core::PWSTR,
    pub pszDstId: ::windows::core::PWSTR,
    pub pszNC: ::windows::core::PWSTR,
    pub pguidSrc: *mut ::windows::core::GUID,
    pub pguidDst: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for DS_REPSYNCALL_SYNCW {}
impl ::core::clone::Clone for DS_REPSYNCALL_SYNCW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPSYNCALL_SYNCW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_REPSYNCALL_UPDATEA {
    pub event: DS_REPSYNCALL_EVENT,
    pub pErrInfo: *mut DS_REPSYNCALL_ERRINFOA,
    pub pSync: *mut DS_REPSYNCALL_SYNCA,
}
impl ::core::marker::Copy for DS_REPSYNCALL_UPDATEA {}
impl ::core::clone::Clone for DS_REPSYNCALL_UPDATEA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPSYNCALL_UPDATEA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_REPSYNCALL_UPDATEW {
    pub event: DS_REPSYNCALL_EVENT,
    pub pErrInfo: *mut DS_REPSYNCALL_ERRINFOW,
    pub pSync: *mut DS_REPSYNCALL_SYNCW,
}
impl ::core::marker::Copy for DS_REPSYNCALL_UPDATEW {}
impl ::core::clone::Clone for DS_REPSYNCALL_UPDATEW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_REPSYNCALL_UPDATEW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_SCHEMA_GUID_MAPA {
    pub guid: ::windows::core::GUID,
    pub guidType: u32,
    pub pName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for DS_SCHEMA_GUID_MAPA {}
impl ::core::clone::Clone for DS_SCHEMA_GUID_MAPA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_SCHEMA_GUID_MAPA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_SCHEMA_GUID_MAPW {
    pub guid: ::windows::core::GUID,
    pub guidType: u32,
    pub pName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DS_SCHEMA_GUID_MAPW {}
impl ::core::clone::Clone for DS_SCHEMA_GUID_MAPW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_SCHEMA_GUID_MAPW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DS_SELECTION {
    pub pwzName: ::windows::core::PWSTR,
    pub pwzADsPath: ::windows::core::PWSTR,
    pub pwzClass: ::windows::core::PWSTR,
    pub pwzUPN: ::windows::core::PWSTR,
    pub pvarFetchedAttributes: *mut super::super::System::Com::VARIANT,
    pub flScopeType: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DS_SELECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DS_SELECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for DS_SELECTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct DS_SELECTION_LIST {
    pub cItems: u32,
    pub cFetchedAttributes: u32,
    pub aDsSelection: [DS_SELECTION; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DS_SELECTION_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DS_SELECTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for DS_SELECTION_LIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct DS_SITE_COST_INFO {
    pub errorCode: u32,
    pub cost: u32,
}
impl ::core::marker::Copy for DS_SITE_COST_INFO {}
impl ::core::clone::Clone for DS_SITE_COST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DS_SITE_COST_INFO {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GetDcContextHandle(pub isize);
impl GetDcContextHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for GetDcContextHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for GetDcContextHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for GetDcContextHandle {}
impl ::core::fmt::Debug for GetDcContextHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetDcContextHandle").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<GetDcContextHandle>> for GetDcContextHandle {
    fn from(optional: ::core::option::Option<GetDcContextHandle>) -> GetDcContextHandle {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for GetDcContextHandle {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub struct OPENQUERYWINDOW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub clsidHandler: ::windows::core::GUID,
    pub pHandlerParameters: *mut ::core::ffi::c_void,
    pub clsidDefaultForm: ::windows::core::GUID,
    pub pPersistQuery: ::windows::core::ManuallyDrop<IPersistQuery>,
    pub Anonymous: OPENQUERYWINDOW_0,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for OPENQUERYWINDOW {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows::core::Abi for OPENQUERYWINDOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub union OPENQUERYWINDOW_0 {
    pub pFormParameters: *mut ::core::ffi::c_void,
    pub ppbFormParameters: ::std::mem::ManuallyDrop<::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyBag>>,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for OPENQUERYWINDOW_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
unsafe impl ::windows::core::Abi for OPENQUERYWINDOW_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct SCHEDULE {
    pub Size: u32,
    pub Bandwidth: u32,
    pub NumberOfSchedules: u32,
    pub Schedules: [SCHEDULE_HEADER; 1],
}
impl ::core::marker::Copy for SCHEDULE {}
impl ::core::clone::Clone for SCHEDULE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCHEDULE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`*"]
pub struct SCHEDULE_HEADER {
    pub Type: u32,
    pub Offset: u32,
}
impl ::core::marker::Copy for SCHEDULE_HEADER {}
impl ::core::clone::Clone for SCHEDULE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCHEDULE_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPCQADDFORMSPROC = ::core::option::Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, pform: *mut CQFORM) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPCQADDPAGESPROC = ::core::option::Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, clsidform: *const ::windows::core::GUID, ppage: *mut CQPAGE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPCQPAGEPROC = ::core::option::Option<unsafe extern "system" fn(ppage: *mut CQPAGE, hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Networking_ActiveDirectory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMATTRIBUTES = ::core::option::Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, pszattributename: ::windows::core::PCWSTR, pszdisplayname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
