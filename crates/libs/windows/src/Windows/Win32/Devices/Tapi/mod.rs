#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GetTnefStreamCodepage<P0>(lpstream: P0, lpulcodepage: *mut u32, lpulsubcodepage: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
{
    ::windows::core::link ! ( "mapi32.dll""system" fn GetTnefStreamCodepage ( lpstream : * mut::core::ffi::c_void , lpulcodepage : *mut u32 , lpulsubcodepage : *mut u32 ) -> :: windows::core::HRESULT );
    GetTnefStreamCodepage(lpstream.into().abi(), lpulcodepage, lpulsubcodepage).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OpenTnefStream<P0, P1>(lpvsupport: *mut ::core::ffi::c_void, lpstream: P0, lpszstreamname: *const i8, ulflags: u32, lpmessage: P1, wkeyval: u16, lpptnef: *mut ::core::option::Option<ITnef>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::super::System::AddressBook::IMessage>>,
{
    ::windows::core::link ! ( "mapi32.dll""system" fn OpenTnefStream ( lpvsupport : *mut ::core::ffi::c_void , lpstream : * mut::core::ffi::c_void , lpszstreamname : *const i8 , ulflags : u32 , lpmessage : * mut::core::ffi::c_void , wkeyval : u16 , lpptnef : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OpenTnefStream(lpvsupport, lpstream.into().abi(), lpszstreamname, ulflags, lpmessage.into().abi(), wkeyval, ::core::mem::transmute(lpptnef)).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OpenTnefStreamEx<P0, P1, P2>(lpvsupport: *mut ::core::ffi::c_void, lpstream: P0, lpszstreamname: *const i8, ulflags: u32, lpmessage: P1, wkeyval: u16, lpadressbook: P2, lpptnef: *mut ::core::option::Option<ITnef>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::super::System::AddressBook::IMessage>>,
    P2: ::std::convert::Into<::windows::core::InParam<super::super::System::AddressBook::IAddrBook>>,
{
    ::windows::core::link ! ( "mapi32.dll""system" fn OpenTnefStreamEx ( lpvsupport : *mut ::core::ffi::c_void , lpstream : * mut::core::ffi::c_void , lpszstreamname : *const i8 , ulflags : u32 , lpmessage : * mut::core::ffi::c_void , wkeyval : u16 , lpadressbook : * mut::core::ffi::c_void , lpptnef : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OpenTnefStreamEx(lpvsupport, lpstream.into().abi(), lpszstreamname, ulflags, lpmessage.into().abi(), wkeyval, lpadressbook.into().abi(), ::core::mem::transmute(lpptnef)).ok()
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineAccept<P0>(hcall: u32, lpsuseruserinfo: P0, dwsize: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineAccept ( hcall : u32 , lpsuseruserinfo : :: windows::core::PCSTR , dwsize : u32 ) -> i32 );
    lineAccept(hcall, lpsuseruserinfo.into().abi(), dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineAddProvider<P0, P1>(lpszproviderfilename: P0, hwndowner: P1, lpdwpermanentproviderid: *mut u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineAddProvider ( lpszproviderfilename : :: windows::core::PCSTR , hwndowner : super::super::Foundation:: HWND , lpdwpermanentproviderid : *mut u32 ) -> i32 );
    lineAddProvider(lpszproviderfilename.into().abi(), hwndowner.into(), lpdwpermanentproviderid)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineAddProviderA<P0, P1>(lpszproviderfilename: P0, hwndowner: P1, lpdwpermanentproviderid: *mut u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineAddProviderA ( lpszproviderfilename : :: windows::core::PCSTR , hwndowner : super::super::Foundation:: HWND , lpdwpermanentproviderid : *mut u32 ) -> i32 );
    lineAddProviderA(lpszproviderfilename.into().abi(), hwndowner.into(), lpdwpermanentproviderid)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineAddProviderW<P0, P1>(lpszproviderfilename: P0, hwndowner: P1, lpdwpermanentproviderid: *mut u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineAddProviderW ( lpszproviderfilename : :: windows::core::PCWSTR , hwndowner : super::super::Foundation:: HWND , lpdwpermanentproviderid : *mut u32 ) -> i32 );
    lineAddProviderW(lpszproviderfilename.into().abi(), hwndowner.into(), lpdwpermanentproviderid)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineAddToConference(hconfcall: u32, hconsultcall: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineAddToConference ( hconfcall : u32 , hconsultcall : u32 ) -> i32 );
    lineAddToConference(hconfcall, hconsultcall)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineAgentSpecific(hline: u32, dwaddressid: u32, dwagentextensionidindex: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineAgentSpecific ( hline : u32 , dwaddressid : u32 , dwagentextensionidindex : u32 , lpparams : *mut ::core::ffi::c_void , dwsize : u32 ) -> i32 );
    lineAgentSpecific(hline, dwaddressid, dwagentextensionidindex, lpparams, dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineAnswer<P0>(hcall: u32, lpsuseruserinfo: P0, dwsize: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineAnswer ( hcall : u32 , lpsuseruserinfo : :: windows::core::PCSTR , dwsize : u32 ) -> i32 );
    lineAnswer(hcall, lpsuseruserinfo.into().abi(), dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineBlindTransfer<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineBlindTransfer ( hcall : u32 , lpszdestaddress : :: windows::core::PCSTR , dwcountrycode : u32 ) -> i32 );
    lineBlindTransfer(hcall, lpszdestaddress.into().abi(), dwcountrycode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineBlindTransferA<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineBlindTransferA ( hcall : u32 , lpszdestaddress : :: windows::core::PCSTR , dwcountrycode : u32 ) -> i32 );
    lineBlindTransferA(hcall, lpszdestaddress.into().abi(), dwcountrycode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineBlindTransferW<P0>(hcall: u32, lpszdestaddressw: P0, dwcountrycode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineBlindTransferW ( hcall : u32 , lpszdestaddressw : :: windows::core::PCWSTR , dwcountrycode : u32 ) -> i32 );
    lineBlindTransferW(hcall, lpszdestaddressw.into().abi(), dwcountrycode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineClose(hline: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineClose ( hline : u32 ) -> i32 );
    lineClose(hline)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineCompleteCall(hcall: u32, lpdwcompletionid: *mut u32, dwcompletionmode: u32, dwmessageid: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineCompleteCall ( hcall : u32 , lpdwcompletionid : *mut u32 , dwcompletionmode : u32 , dwmessageid : u32 ) -> i32 );
    lineCompleteCall(hcall, lpdwcompletionid, dwcompletionmode, dwmessageid)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineCompleteTransfer(hcall: u32, hconsultcall: u32, lphconfcall: *mut u32, dwtransfermode: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineCompleteTransfer ( hcall : u32 , hconsultcall : u32 , lphconfcall : *mut u32 , dwtransfermode : u32 ) -> i32 );
    lineCompleteTransfer(hcall, hconsultcall, lphconfcall, dwtransfermode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineConfigDialog<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineConfigDialog ( dwdeviceid : u32 , hwndowner : super::super::Foundation:: HWND , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    lineConfigDialog(dwdeviceid, hwndowner.into(), lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineConfigDialogA<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineConfigDialogA ( dwdeviceid : u32 , hwndowner : super::super::Foundation:: HWND , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    lineConfigDialogA(dwdeviceid, hwndowner.into(), lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineConfigDialogEdit<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1, lpdeviceconfigin: *const ::core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineConfigDialogEdit ( dwdeviceid : u32 , hwndowner : super::super::Foundation:: HWND , lpszdeviceclass : :: windows::core::PCSTR , lpdeviceconfigin : *const ::core::ffi::c_void , dwsize : u32 , lpdeviceconfigout : *mut VARSTRING ) -> i32 );
    lineConfigDialogEdit(dwdeviceid, hwndowner.into(), lpszdeviceclass.into().abi(), lpdeviceconfigin, dwsize, lpdeviceconfigout)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineConfigDialogEditA<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1, lpdeviceconfigin: *const ::core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineConfigDialogEditA ( dwdeviceid : u32 , hwndowner : super::super::Foundation:: HWND , lpszdeviceclass : :: windows::core::PCSTR , lpdeviceconfigin : *const ::core::ffi::c_void , dwsize : u32 , lpdeviceconfigout : *mut VARSTRING ) -> i32 );
    lineConfigDialogEditA(dwdeviceid, hwndowner.into(), lpszdeviceclass.into().abi(), lpdeviceconfigin, dwsize, lpdeviceconfigout)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineConfigDialogEditW<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1, lpdeviceconfigin: *const ::core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineConfigDialogEditW ( dwdeviceid : u32 , hwndowner : super::super::Foundation:: HWND , lpszdeviceclass : :: windows::core::PCWSTR , lpdeviceconfigin : *const ::core::ffi::c_void , dwsize : u32 , lpdeviceconfigout : *mut VARSTRING ) -> i32 );
    lineConfigDialogEditW(dwdeviceid, hwndowner.into(), lpszdeviceclass.into().abi(), lpdeviceconfigin, dwsize, lpdeviceconfigout)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineConfigDialogW<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineConfigDialogW ( dwdeviceid : u32 , hwndowner : super::super::Foundation:: HWND , lpszdeviceclass : :: windows::core::PCWSTR ) -> i32 );
    lineConfigDialogW(dwdeviceid, hwndowner.into(), lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineConfigProvider<P0>(hwndowner: P0, dwpermanentproviderid: u32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineConfigProvider ( hwndowner : super::super::Foundation:: HWND , dwpermanentproviderid : u32 ) -> i32 );
    lineConfigProvider(hwndowner.into(), dwpermanentproviderid)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineCreateAgentA<P0, P1>(hline: u32, lpszagentid: P0, lpszagentpin: P1, lphagent: *mut u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineCreateAgentA ( hline : u32 , lpszagentid : :: windows::core::PCSTR , lpszagentpin : :: windows::core::PCSTR , lphagent : *mut u32 ) -> i32 );
    lineCreateAgentA(hline, lpszagentid.into().abi(), lpszagentpin.into().abi(), lphagent)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineCreateAgentSessionA<P0>(hline: u32, hagent: u32, lpszagentpin: P0, dwworkingaddressid: u32, lpgroupid: *mut ::windows::core::GUID, lphagentsession: *mut u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineCreateAgentSessionA ( hline : u32 , hagent : u32 , lpszagentpin : :: windows::core::PCSTR , dwworkingaddressid : u32 , lpgroupid : *mut :: windows::core::GUID , lphagentsession : *mut u32 ) -> i32 );
    lineCreateAgentSessionA(hline, hagent, lpszagentpin.into().abi(), dwworkingaddressid, lpgroupid, lphagentsession)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineCreateAgentSessionW<P0>(hline: u32, hagent: u32, lpszagentpin: P0, dwworkingaddressid: u32, lpgroupid: *mut ::windows::core::GUID, lphagentsession: *mut u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineCreateAgentSessionW ( hline : u32 , hagent : u32 , lpszagentpin : :: windows::core::PCWSTR , dwworkingaddressid : u32 , lpgroupid : *mut :: windows::core::GUID , lphagentsession : *mut u32 ) -> i32 );
    lineCreateAgentSessionW(hline, hagent, lpszagentpin.into().abi(), dwworkingaddressid, lpgroupid, lphagentsession)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineCreateAgentW<P0, P1>(hline: u32, lpszagentid: P0, lpszagentpin: P1, lphagent: *mut u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineCreateAgentW ( hline : u32 , lpszagentid : :: windows::core::PCWSTR , lpszagentpin : :: windows::core::PCWSTR , lphagent : *mut u32 ) -> i32 );
    lineCreateAgentW(hline, lpszagentid.into().abi(), lpszagentpin.into().abi(), lphagent)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineDeallocateCall(hcall: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineDeallocateCall ( hcall : u32 ) -> i32 );
    lineDeallocateCall(hcall)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineDevSpecific(hline: u32, dwaddressid: u32, hcall: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineDevSpecific ( hline : u32 , dwaddressid : u32 , hcall : u32 , lpparams : *mut ::core::ffi::c_void , dwsize : u32 ) -> i32 );
    lineDevSpecific(hline, dwaddressid, hcall, lpparams, dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineDevSpecificFeature(hline: u32, dwfeature: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineDevSpecificFeature ( hline : u32 , dwfeature : u32 , lpparams : *mut ::core::ffi::c_void , dwsize : u32 ) -> i32 );
    lineDevSpecificFeature(hline, dwfeature, lpparams, dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineDial<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineDial ( hcall : u32 , lpszdestaddress : :: windows::core::PCSTR , dwcountrycode : u32 ) -> i32 );
    lineDial(hcall, lpszdestaddress.into().abi(), dwcountrycode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineDialA<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineDialA ( hcall : u32 , lpszdestaddress : :: windows::core::PCSTR , dwcountrycode : u32 ) -> i32 );
    lineDialA(hcall, lpszdestaddress.into().abi(), dwcountrycode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineDialW<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineDialW ( hcall : u32 , lpszdestaddress : :: windows::core::PCWSTR , dwcountrycode : u32 ) -> i32 );
    lineDialW(hcall, lpszdestaddress.into().abi(), dwcountrycode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineDrop<P0>(hcall: u32, lpsuseruserinfo: P0, dwsize: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineDrop ( hcall : u32 , lpsuseruserinfo : :: windows::core::PCSTR , dwsize : u32 ) -> i32 );
    lineDrop(hcall, lpsuseruserinfo.into().abi(), dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineForward(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineForward ( hline : u32 , balladdresses : u32 , dwaddressid : u32 , lpforwardlist : *const LINEFORWARDLIST , dwnumringsnoanswer : u32 , lphconsultcall : *mut u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineForward(hline, balladdresses, dwaddressid, lpforwardlist, dwnumringsnoanswer, lphconsultcall, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineForwardA(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineForwardA ( hline : u32 , balladdresses : u32 , dwaddressid : u32 , lpforwardlist : *const LINEFORWARDLIST , dwnumringsnoanswer : u32 , lphconsultcall : *mut u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineForwardA(hline, balladdresses, dwaddressid, lpforwardlist, dwnumringsnoanswer, lphconsultcall, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineForwardW(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineForwardW ( hline : u32 , balladdresses : u32 , dwaddressid : u32 , lpforwardlist : *const LINEFORWARDLIST , dwnumringsnoanswer : u32 , lphconsultcall : *mut u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineForwardW(hline, balladdresses, dwaddressid, lpforwardlist, dwnumringsnoanswer, lphconsultcall, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGatherDigits<P0>(hcall: u32, dwdigitmodes: u32, lpsdigits: ::core::option::Option<&mut [u8]>, lpszterminationdigits: P0, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGatherDigits ( hcall : u32 , dwdigitmodes : u32 , lpsdigits : :: windows::core::PSTR , dwnumdigits : u32 , lpszterminationdigits : :: windows::core::PCSTR , dwfirstdigittimeout : u32 , dwinterdigittimeout : u32 ) -> i32 );
    lineGatherDigits(hcall, dwdigitmodes, ::core::mem::transmute(lpsdigits.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpsdigits.as_deref().map_or(0, |slice| slice.len() as _), lpszterminationdigits.into().abi(), dwfirstdigittimeout, dwinterdigittimeout)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGatherDigitsA<P0>(hcall: u32, dwdigitmodes: u32, lpsdigits: ::core::option::Option<&mut [u8]>, lpszterminationdigits: P0, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGatherDigitsA ( hcall : u32 , dwdigitmodes : u32 , lpsdigits : :: windows::core::PSTR , dwnumdigits : u32 , lpszterminationdigits : :: windows::core::PCSTR , dwfirstdigittimeout : u32 , dwinterdigittimeout : u32 ) -> i32 );
    lineGatherDigitsA(hcall, dwdigitmodes, ::core::mem::transmute(lpsdigits.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpsdigits.as_deref().map_or(0, |slice| slice.len() as _), lpszterminationdigits.into().abi(), dwfirstdigittimeout, dwinterdigittimeout)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGatherDigitsW<P0>(hcall: u32, dwdigitmodes: u32, lpsdigits: ::core::option::Option<&mut [u16]>, lpszterminationdigits: P0, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGatherDigitsW ( hcall : u32 , dwdigitmodes : u32 , lpsdigits : :: windows::core::PWSTR , dwnumdigits : u32 , lpszterminationdigits : :: windows::core::PCWSTR , dwfirstdigittimeout : u32 , dwinterdigittimeout : u32 ) -> i32 );
    lineGatherDigitsW(hcall, dwdigitmodes, ::core::mem::transmute(lpsdigits.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpsdigits.as_deref().map_or(0, |slice| slice.len() as _), lpszterminationdigits.into().abi(), dwfirstdigittimeout, dwinterdigittimeout)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGenerateDigits<P0>(hcall: u32, dwdigitmode: u32, lpszdigits: P0, dwduration: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGenerateDigits ( hcall : u32 , dwdigitmode : u32 , lpszdigits : :: windows::core::PCSTR , dwduration : u32 ) -> i32 );
    lineGenerateDigits(hcall, dwdigitmode, lpszdigits.into().abi(), dwduration)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGenerateDigitsA<P0>(hcall: u32, dwdigitmode: u32, lpszdigits: P0, dwduration: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGenerateDigitsA ( hcall : u32 , dwdigitmode : u32 , lpszdigits : :: windows::core::PCSTR , dwduration : u32 ) -> i32 );
    lineGenerateDigitsA(hcall, dwdigitmode, lpszdigits.into().abi(), dwduration)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGenerateDigitsW<P0>(hcall: u32, dwdigitmode: u32, lpszdigits: P0, dwduration: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGenerateDigitsW ( hcall : u32 , dwdigitmode : u32 , lpszdigits : :: windows::core::PCWSTR , dwduration : u32 ) -> i32 );
    lineGenerateDigitsW(hcall, dwdigitmode, lpszdigits.into().abi(), dwduration)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGenerateTone(hcall: u32, dwtonemode: u32, dwduration: u32, dwnumtones: u32, lptones: *const LINEGENERATETONE) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGenerateTone ( hcall : u32 , dwtonemode : u32 , dwduration : u32 , dwnumtones : u32 , lptones : *const LINEGENERATETONE ) -> i32 );
    lineGenerateTone(hcall, dwtonemode, dwduration, dwnumtones, lptones)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAddressCaps(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAddressCaps ( hlineapp : u32 , dwdeviceid : u32 , dwaddressid : u32 , dwapiversion : u32 , dwextversion : u32 , lpaddresscaps : *mut LINEADDRESSCAPS ) -> i32 );
    lineGetAddressCaps(hlineapp, dwdeviceid, dwaddressid, dwapiversion, dwextversion, lpaddresscaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAddressCapsA(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAddressCapsA ( hlineapp : u32 , dwdeviceid : u32 , dwaddressid : u32 , dwapiversion : u32 , dwextversion : u32 , lpaddresscaps : *mut LINEADDRESSCAPS ) -> i32 );
    lineGetAddressCapsA(hlineapp, dwdeviceid, dwaddressid, dwapiversion, dwextversion, lpaddresscaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAddressCapsW(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAddressCapsW ( hlineapp : u32 , dwdeviceid : u32 , dwaddressid : u32 , dwapiversion : u32 , dwextversion : u32 , lpaddresscaps : *mut LINEADDRESSCAPS ) -> i32 );
    lineGetAddressCapsW(hlineapp, dwdeviceid, dwaddressid, dwapiversion, dwextversion, lpaddresscaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAddressID<P0>(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: P0, dwsize: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAddressID ( hline : u32 , lpdwaddressid : *mut u32 , dwaddressmode : u32 , lpsaddress : :: windows::core::PCSTR , dwsize : u32 ) -> i32 );
    lineGetAddressID(hline, lpdwaddressid, dwaddressmode, lpsaddress.into().abi(), dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAddressIDA<P0>(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: P0, dwsize: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAddressIDA ( hline : u32 , lpdwaddressid : *mut u32 , dwaddressmode : u32 , lpsaddress : :: windows::core::PCSTR , dwsize : u32 ) -> i32 );
    lineGetAddressIDA(hline, lpdwaddressid, dwaddressmode, lpsaddress.into().abi(), dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAddressIDW<P0>(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: P0, dwsize: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAddressIDW ( hline : u32 , lpdwaddressid : *mut u32 , dwaddressmode : u32 , lpsaddress : :: windows::core::PCWSTR , dwsize : u32 ) -> i32 );
    lineGetAddressIDW(hline, lpdwaddressid, dwaddressmode, lpsaddress.into().abi(), dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAddressStatus(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAddressStatus ( hline : u32 , dwaddressid : u32 , lpaddressstatus : *mut LINEADDRESSSTATUS ) -> i32 );
    lineGetAddressStatus(hline, dwaddressid, lpaddressstatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAddressStatusA(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAddressStatusA ( hline : u32 , dwaddressid : u32 , lpaddressstatus : *mut LINEADDRESSSTATUS ) -> i32 );
    lineGetAddressStatusA(hline, dwaddressid, lpaddressstatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAddressStatusW(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAddressStatusW ( hline : u32 , dwaddressid : u32 , lpaddressstatus : *mut LINEADDRESSSTATUS ) -> i32 );
    lineGetAddressStatusW(hline, dwaddressid, lpaddressstatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAgentActivityListA(hline: u32, dwaddressid: u32, lpagentactivitylist: *mut LINEAGENTACTIVITYLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentActivityListA ( hline : u32 , dwaddressid : u32 , lpagentactivitylist : *mut LINEAGENTACTIVITYLIST ) -> i32 );
    lineGetAgentActivityListA(hline, dwaddressid, lpagentactivitylist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAgentActivityListW(hline: u32, dwaddressid: u32, lpagentactivitylist: *mut LINEAGENTACTIVITYLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentActivityListW ( hline : u32 , dwaddressid : u32 , lpagentactivitylist : *mut LINEAGENTACTIVITYLIST ) -> i32 );
    lineGetAgentActivityListW(hline, dwaddressid, lpagentactivitylist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAgentCapsA(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwappapiversion: u32, lpagentcaps: *mut LINEAGENTCAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentCapsA ( hlineapp : u32 , dwdeviceid : u32 , dwaddressid : u32 , dwappapiversion : u32 , lpagentcaps : *mut LINEAGENTCAPS ) -> i32 );
    lineGetAgentCapsA(hlineapp, dwdeviceid, dwaddressid, dwappapiversion, lpagentcaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAgentCapsW(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwappapiversion: u32, lpagentcaps: *mut LINEAGENTCAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentCapsW ( hlineapp : u32 , dwdeviceid : u32 , dwaddressid : u32 , dwappapiversion : u32 , lpagentcaps : *mut LINEAGENTCAPS ) -> i32 );
    lineGetAgentCapsW(hlineapp, dwdeviceid, dwaddressid, dwappapiversion, lpagentcaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAgentGroupListA(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentGroupListA ( hline : u32 , dwaddressid : u32 , lpagentgrouplist : *mut LINEAGENTGROUPLIST ) -> i32 );
    lineGetAgentGroupListA(hline, dwaddressid, lpagentgrouplist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAgentGroupListW(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentGroupListW ( hline : u32 , dwaddressid : u32 , lpagentgrouplist : *mut LINEAGENTGROUPLIST ) -> i32 );
    lineGetAgentGroupListW(hline, dwaddressid, lpagentgrouplist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn lineGetAgentInfo(hline: u32, hagent: u32, lpagentinfo: *mut LINEAGENTINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentInfo ( hline : u32 , hagent : u32 , lpagentinfo : *mut LINEAGENTINFO ) -> i32 );
    lineGetAgentInfo(hline, hagent, lpagentinfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn lineGetAgentSessionInfo(hline: u32, hagentsession: u32, lpagentsessioninfo: *mut LINEAGENTSESSIONINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentSessionInfo ( hline : u32 , hagentsession : u32 , lpagentsessioninfo : *mut LINEAGENTSESSIONINFO ) -> i32 );
    lineGetAgentSessionInfo(hline, hagentsession, lpagentsessioninfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAgentSessionList(hline: u32, hagent: u32, lpagentsessionlist: *mut LINEAGENTSESSIONLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentSessionList ( hline : u32 , hagent : u32 , lpagentsessionlist : *mut LINEAGENTSESSIONLIST ) -> i32 );
    lineGetAgentSessionList(hline, hagent, lpagentsessionlist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAgentStatusA(hline: u32, dwaddressid: u32, lpagentstatus: *mut LINEAGENTSTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentStatusA ( hline : u32 , dwaddressid : u32 , lpagentstatus : *mut LINEAGENTSTATUS ) -> i32 );
    lineGetAgentStatusA(hline, dwaddressid, lpagentstatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAgentStatusW(hline: u32, dwaddressid: u32, lpagentstatus: *mut LINEAGENTSTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAgentStatusW ( hline : u32 , dwaddressid : u32 , lpagentstatus : *mut LINEAGENTSTATUS ) -> i32 );
    lineGetAgentStatusW(hline, dwaddressid, lpagentstatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAppPriority<P0>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAppPriority ( lpszappfilename : :: windows::core::PCSTR , dwmediamode : u32 , lpextensionid : *mut LINEEXTENSIONID , dwrequestmode : u32 , lpextensionname : *mut VARSTRING , lpdwpriority : *mut u32 ) -> i32 );
    lineGetAppPriority(lpszappfilename.into().abi(), dwmediamode, lpextensionid, dwrequestmode, lpextensionname, lpdwpriority)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAppPriorityA<P0>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAppPriorityA ( lpszappfilename : :: windows::core::PCSTR , dwmediamode : u32 , lpextensionid : *mut LINEEXTENSIONID , dwrequestmode : u32 , lpextensionname : *mut VARSTRING , lpdwpriority : *mut u32 ) -> i32 );
    lineGetAppPriorityA(lpszappfilename.into().abi(), dwmediamode, lpextensionid, dwrequestmode, lpextensionname, lpdwpriority)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetAppPriorityW<P0>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetAppPriorityW ( lpszappfilename : :: windows::core::PCWSTR , dwmediamode : u32 , lpextensionid : *mut LINEEXTENSIONID , dwrequestmode : u32 , lpextensionname : *mut VARSTRING , lpdwpriority : *mut u32 ) -> i32 );
    lineGetAppPriorityW(lpszappfilename.into().abi(), dwmediamode, lpextensionid, dwrequestmode, lpextensionname, lpdwpriority)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetCallInfo(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetCallInfo ( hcall : u32 , lpcallinfo : *mut LINECALLINFO ) -> i32 );
    lineGetCallInfo(hcall, lpcallinfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetCallInfoA(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetCallInfoA ( hcall : u32 , lpcallinfo : *mut LINECALLINFO ) -> i32 );
    lineGetCallInfoA(hcall, lpcallinfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetCallInfoW(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetCallInfoW ( hcall : u32 , lpcallinfo : *mut LINECALLINFO ) -> i32 );
    lineGetCallInfoW(hcall, lpcallinfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineGetCallStatus(hcall: u32, lpcallstatus: *mut LINECALLSTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetCallStatus ( hcall : u32 , lpcallstatus : *mut LINECALLSTATUS ) -> i32 );
    lineGetCallStatus(hcall, lpcallstatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetConfRelatedCalls(hcall: u32, lpcalllist: *mut LINECALLLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetConfRelatedCalls ( hcall : u32 , lpcalllist : *mut LINECALLLIST ) -> i32 );
    lineGetConfRelatedCalls(hcall, lpcalllist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetCountry(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetCountry ( dwcountryid : u32 , dwapiversion : u32 , lplinecountrylist : *mut LINECOUNTRYLIST ) -> i32 );
    lineGetCountry(dwcountryid, dwapiversion, lplinecountrylist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetCountryA(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetCountryA ( dwcountryid : u32 , dwapiversion : u32 , lplinecountrylist : *mut LINECOUNTRYLIST ) -> i32 );
    lineGetCountryA(dwcountryid, dwapiversion, lplinecountrylist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetCountryW(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetCountryW ( dwcountryid : u32 , dwapiversion : u32 , lplinecountrylist : *mut LINECOUNTRYLIST ) -> i32 );
    lineGetCountryW(dwcountryid, dwapiversion, lplinecountrylist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetDevCaps(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetDevCaps ( hlineapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , dwextversion : u32 , lplinedevcaps : *mut LINEDEVCAPS ) -> i32 );
    lineGetDevCaps(hlineapp, dwdeviceid, dwapiversion, dwextversion, lplinedevcaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetDevCapsA(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetDevCapsA ( hlineapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , dwextversion : u32 , lplinedevcaps : *mut LINEDEVCAPS ) -> i32 );
    lineGetDevCapsA(hlineapp, dwdeviceid, dwapiversion, dwextversion, lplinedevcaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetDevCapsW(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetDevCapsW ( hlineapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , dwextversion : u32 , lplinedevcaps : *mut LINEDEVCAPS ) -> i32 );
    lineGetDevCapsW(hlineapp, dwdeviceid, dwapiversion, dwextversion, lplinedevcaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetDevConfig<P0>(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetDevConfig ( dwdeviceid : u32 , lpdeviceconfig : *mut VARSTRING , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    lineGetDevConfig(dwdeviceid, lpdeviceconfig, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetDevConfigA<P0>(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetDevConfigA ( dwdeviceid : u32 , lpdeviceconfig : *mut VARSTRING , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    lineGetDevConfigA(dwdeviceid, lpdeviceconfig, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetDevConfigW<P0>(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetDevConfigW ( dwdeviceid : u32 , lpdeviceconfig : *mut VARSTRING , lpszdeviceclass : :: windows::core::PCWSTR ) -> i32 );
    lineGetDevConfigW(dwdeviceid, lpdeviceconfig, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetGroupListA(hline: u32, lpgrouplist: *mut LINEAGENTGROUPLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetGroupListA ( hline : u32 , lpgrouplist : *mut LINEAGENTGROUPLIST ) -> i32 );
    lineGetGroupListA(hline, lpgrouplist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetGroupListW(hline: u32, lpgrouplist: *mut LINEAGENTGROUPLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetGroupListW ( hline : u32 , lpgrouplist : *mut LINEAGENTGROUPLIST ) -> i32 );
    lineGetGroupListW(hline, lpgrouplist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetID<P0>(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetID ( hline : u32 , dwaddressid : u32 , hcall : u32 , dwselect : u32 , lpdeviceid : *mut VARSTRING , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    lineGetID(hline, dwaddressid, hcall, dwselect, lpdeviceid, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetIDA<P0>(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetIDA ( hline : u32 , dwaddressid : u32 , hcall : u32 , dwselect : u32 , lpdeviceid : *mut VARSTRING , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    lineGetIDA(hline, dwaddressid, hcall, dwselect, lpdeviceid, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetIDW<P0>(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetIDW ( hline : u32 , dwaddressid : u32 , hcall : u32 , dwselect : u32 , lpdeviceid : *mut VARSTRING , lpszdeviceclass : :: windows::core::PCWSTR ) -> i32 );
    lineGetIDW(hline, dwaddressid, hcall, dwselect, lpdeviceid, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetIcon<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut isize) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetIcon ( dwdeviceid : u32 , lpszdeviceclass : :: windows::core::PCSTR , lphicon : *mut isize ) -> i32 );
    lineGetIcon(dwdeviceid, lpszdeviceclass.into().abi(), lphicon)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetIconA<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut isize) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetIconA ( dwdeviceid : u32 , lpszdeviceclass : :: windows::core::PCSTR , lphicon : *mut isize ) -> i32 );
    lineGetIconA(dwdeviceid, lpszdeviceclass.into().abi(), lphicon)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetIconW<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut isize) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetIconW ( dwdeviceid : u32 , lpszdeviceclass : :: windows::core::PCWSTR , lphicon : *mut isize ) -> i32 );
    lineGetIconW(dwdeviceid, lpszdeviceclass.into().abi(), lphicon)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetLineDevStatus(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetLineDevStatus ( hline : u32 , lplinedevstatus : *mut LINEDEVSTATUS ) -> i32 );
    lineGetLineDevStatus(hline, lplinedevstatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetLineDevStatusA(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetLineDevStatusA ( hline : u32 , lplinedevstatus : *mut LINEDEVSTATUS ) -> i32 );
    lineGetLineDevStatusA(hline, lplinedevstatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetLineDevStatusW(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetLineDevStatusW ( hline : u32 , lplinedevstatus : *mut LINEDEVSTATUS ) -> i32 );
    lineGetLineDevStatusW(hline, lplinedevstatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetMessage(hlineapp: u32, lpmessage: *mut LINEMESSAGE, dwtimeout: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetMessage ( hlineapp : u32 , lpmessage : *mut LINEMESSAGE , dwtimeout : u32 ) -> i32 );
    lineGetMessage(hlineapp, lpmessage, dwtimeout)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetNewCalls(hline: u32, dwaddressid: u32, dwselect: u32, lpcalllist: *mut LINECALLLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetNewCalls ( hline : u32 , dwaddressid : u32 , dwselect : u32 , lpcalllist : *mut LINECALLLIST ) -> i32 );
    lineGetNewCalls(hline, dwaddressid, dwselect, lpcalllist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetNumRings(hline: u32, dwaddressid: u32, lpdwnumrings: *mut u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetNumRings ( hline : u32 , dwaddressid : u32 , lpdwnumrings : *mut u32 ) -> i32 );
    lineGetNumRings(hline, dwaddressid, lpdwnumrings)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetProviderList(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetProviderList ( dwapiversion : u32 , lpproviderlist : *mut LINEPROVIDERLIST ) -> i32 );
    lineGetProviderList(dwapiversion, lpproviderlist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetProviderListA(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetProviderListA ( dwapiversion : u32 , lpproviderlist : *mut LINEPROVIDERLIST ) -> i32 );
    lineGetProviderListA(dwapiversion, lpproviderlist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetProviderListW(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetProviderListW ( dwapiversion : u32 , lpproviderlist : *mut LINEPROVIDERLIST ) -> i32 );
    lineGetProviderListW(dwapiversion, lpproviderlist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetProxyStatus(hlineapp: u32, dwdeviceid: u32, dwappapiversion: u32, lplineproxyreqestlist: *mut LINEPROXYREQUESTLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetProxyStatus ( hlineapp : u32 , dwdeviceid : u32 , dwappapiversion : u32 , lplineproxyreqestlist : *mut LINEPROXYREQUESTLIST ) -> i32 );
    lineGetProxyStatus(hlineapp, dwdeviceid, dwappapiversion, lplineproxyreqestlist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetQueueInfo(hline: u32, dwqueueid: u32, lplinequeueinfo: *mut LINEQUEUEINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetQueueInfo ( hline : u32 , dwqueueid : u32 , lplinequeueinfo : *mut LINEQUEUEINFO ) -> i32 );
    lineGetQueueInfo(hline, dwqueueid, lplinequeueinfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetQueueListA(hline: u32, lpgroupid: *mut ::windows::core::GUID, lpqueuelist: *mut LINEQUEUELIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetQueueListA ( hline : u32 , lpgroupid : *mut :: windows::core::GUID , lpqueuelist : *mut LINEQUEUELIST ) -> i32 );
    lineGetQueueListA(hline, lpgroupid, lpqueuelist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetQueueListW(hline: u32, lpgroupid: *mut ::windows::core::GUID, lpqueuelist: *mut LINEQUEUELIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetQueueListW ( hline : u32 , lpgroupid : *mut :: windows::core::GUID , lpqueuelist : *mut LINEQUEUELIST ) -> i32 );
    lineGetQueueListW(hline, lpgroupid, lpqueuelist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetRequest(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut ::core::ffi::c_void) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetRequest ( hlineapp : u32 , dwrequestmode : u32 , lprequestbuffer : *mut ::core::ffi::c_void ) -> i32 );
    lineGetRequest(hlineapp, dwrequestmode, lprequestbuffer)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetRequestA(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut ::core::ffi::c_void) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetRequestA ( hlineapp : u32 , dwrequestmode : u32 , lprequestbuffer : *mut ::core::ffi::c_void ) -> i32 );
    lineGetRequestA(hlineapp, dwrequestmode, lprequestbuffer)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetRequestW(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut ::core::ffi::c_void) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetRequestW ( hlineapp : u32 , dwrequestmode : u32 , lprequestbuffer : *mut ::core::ffi::c_void ) -> i32 );
    lineGetRequestW(hlineapp, dwrequestmode, lprequestbuffer)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetStatusMessages(hline: u32, lpdwlinestates: *mut u32, lpdwaddressstates: *mut u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetStatusMessages ( hline : u32 , lpdwlinestates : *mut u32 , lpdwaddressstates : *mut u32 ) -> i32 );
    lineGetStatusMessages(hline, lpdwlinestates, lpdwaddressstates)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetTranslateCaps(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetTranslateCaps ( hlineapp : u32 , dwapiversion : u32 , lptranslatecaps : *mut LINETRANSLATECAPS ) -> i32 );
    lineGetTranslateCaps(hlineapp, dwapiversion, lptranslatecaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetTranslateCapsA(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetTranslateCapsA ( hlineapp : u32 , dwapiversion : u32 , lptranslatecaps : *mut LINETRANSLATECAPS ) -> i32 );
    lineGetTranslateCapsA(hlineapp, dwapiversion, lptranslatecaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineGetTranslateCapsW(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineGetTranslateCapsW ( hlineapp : u32 , dwapiversion : u32 , lptranslatecaps : *mut LINETRANSLATECAPS ) -> i32 );
    lineGetTranslateCapsW(hlineapp, dwapiversion, lptranslatecaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineHandoff<P0>(hcall: u32, lpszfilename: P0, dwmediamode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineHandoff ( hcall : u32 , lpszfilename : :: windows::core::PCSTR , dwmediamode : u32 ) -> i32 );
    lineHandoff(hcall, lpszfilename.into().abi(), dwmediamode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineHandoffA<P0>(hcall: u32, lpszfilename: P0, dwmediamode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineHandoffA ( hcall : u32 , lpszfilename : :: windows::core::PCSTR , dwmediamode : u32 ) -> i32 );
    lineHandoffA(hcall, lpszfilename.into().abi(), dwmediamode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineHandoffW<P0>(hcall: u32, lpszfilename: P0, dwmediamode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineHandoffW ( hcall : u32 , lpszfilename : :: windows::core::PCWSTR , dwmediamode : u32 ) -> i32 );
    lineHandoffW(hcall, lpszfilename.into().abi(), dwmediamode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineHold(hcall: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineHold ( hcall : u32 ) -> i32 );
    lineHold(hcall)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineInitialize<P0, P1>(lphlineapp: *mut u32, hinstance: P0, lpfncallback: LINECALLBACK, lpszappname: P1, lpdwnumdevs: *mut u32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineInitialize ( lphlineapp : *mut u32 , hinstance : super::super::Foundation:: HINSTANCE , lpfncallback : LINECALLBACK , lpszappname : :: windows::core::PCSTR , lpdwnumdevs : *mut u32 ) -> i32 );
    lineInitialize(lphlineapp, hinstance.into(), lpfncallback, lpszappname.into().abi(), lpdwnumdevs)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineInitializeExA<P0, P1>(lphlineapp: *mut u32, hinstance: P0, lpfncallback: LINECALLBACK, lpszfriendlyappname: P1, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lplineinitializeexparams: *mut LINEINITIALIZEEXPARAMS) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineInitializeExA ( lphlineapp : *mut u32 , hinstance : super::super::Foundation:: HINSTANCE , lpfncallback : LINECALLBACK , lpszfriendlyappname : :: windows::core::PCSTR , lpdwnumdevs : *mut u32 , lpdwapiversion : *mut u32 , lplineinitializeexparams : *mut LINEINITIALIZEEXPARAMS ) -> i32 );
    lineInitializeExA(lphlineapp, hinstance.into(), lpfncallback, lpszfriendlyappname.into().abi(), lpdwnumdevs, lpdwapiversion, lplineinitializeexparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineInitializeExW<P0, P1>(lphlineapp: *mut u32, hinstance: P0, lpfncallback: LINECALLBACK, lpszfriendlyappname: P1, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lplineinitializeexparams: *mut LINEINITIALIZEEXPARAMS) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineInitializeExW ( lphlineapp : *mut u32 , hinstance : super::super::Foundation:: HINSTANCE , lpfncallback : LINECALLBACK , lpszfriendlyappname : :: windows::core::PCWSTR , lpdwnumdevs : *mut u32 , lpdwapiversion : *mut u32 , lplineinitializeexparams : *mut LINEINITIALIZEEXPARAMS ) -> i32 );
    lineInitializeExW(lphlineapp, hinstance.into(), lpfncallback, lpszfriendlyappname.into().abi(), lpdwnumdevs, lpdwapiversion, lplineinitializeexparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineMakeCall<P0>(hline: u32, lphcall: *mut u32, lpszdestaddress: P0, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineMakeCall ( hline : u32 , lphcall : *mut u32 , lpszdestaddress : :: windows::core::PCSTR , dwcountrycode : u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineMakeCall(hline, lphcall, lpszdestaddress.into().abi(), dwcountrycode, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineMakeCallA<P0>(hline: u32, lphcall: *mut u32, lpszdestaddress: P0, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineMakeCallA ( hline : u32 , lphcall : *mut u32 , lpszdestaddress : :: windows::core::PCSTR , dwcountrycode : u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineMakeCallA(hline, lphcall, lpszdestaddress.into().abi(), dwcountrycode, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineMakeCallW<P0>(hline: u32, lphcall: *mut u32, lpszdestaddress: P0, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineMakeCallW ( hline : u32 , lphcall : *mut u32 , lpszdestaddress : :: windows::core::PCWSTR , dwcountrycode : u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineMakeCallW(hline, lphcall, lpszdestaddress.into().abi(), dwcountrycode, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineMonitorDigits(hcall: u32, dwdigitmodes: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineMonitorDigits ( hcall : u32 , dwdigitmodes : u32 ) -> i32 );
    lineMonitorDigits(hcall, dwdigitmodes)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineMonitorMedia(hcall: u32, dwmediamodes: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineMonitorMedia ( hcall : u32 , dwmediamodes : u32 ) -> i32 );
    lineMonitorMedia(hcall, dwmediamodes)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineMonitorTones(hcall: u32, lptonelist: *const LINEMONITORTONE, dwnumentries: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineMonitorTones ( hcall : u32 , lptonelist : *const LINEMONITORTONE , dwnumentries : u32 ) -> i32 );
    lineMonitorTones(hcall, lptonelist, dwnumentries)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineNegotiateAPIVersion(hlineapp: u32, dwdeviceid: u32, dwapilowversion: u32, dwapihighversion: u32, lpdwapiversion: *mut u32, lpextensionid: *mut LINEEXTENSIONID) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineNegotiateAPIVersion ( hlineapp : u32 , dwdeviceid : u32 , dwapilowversion : u32 , dwapihighversion : u32 , lpdwapiversion : *mut u32 , lpextensionid : *mut LINEEXTENSIONID ) -> i32 );
    lineNegotiateAPIVersion(hlineapp, dwdeviceid, dwapilowversion, dwapihighversion, lpdwapiversion, lpextensionid)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineNegotiateExtVersion(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextlowversion: u32, dwexthighversion: u32, lpdwextversion: *mut u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineNegotiateExtVersion ( hlineapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , dwextlowversion : u32 , dwexthighversion : u32 , lpdwextversion : *mut u32 ) -> i32 );
    lineNegotiateExtVersion(hlineapp, dwdeviceid, dwapiversion, dwextlowversion, dwexthighversion, lpdwextversion)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineOpen(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineOpen ( hlineapp : u32 , dwdeviceid : u32 , lphline : *mut u32 , dwapiversion : u32 , dwextversion : u32 , dwcallbackinstance : usize , dwprivileges : u32 , dwmediamodes : u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineOpen(hlineapp, dwdeviceid, lphline, dwapiversion, dwextversion, dwcallbackinstance, dwprivileges, dwmediamodes, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineOpenA(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineOpenA ( hlineapp : u32 , dwdeviceid : u32 , lphline : *mut u32 , dwapiversion : u32 , dwextversion : u32 , dwcallbackinstance : usize , dwprivileges : u32 , dwmediamodes : u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineOpenA(hlineapp, dwdeviceid, lphline, dwapiversion, dwextversion, dwcallbackinstance, dwprivileges, dwmediamodes, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineOpenW(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineOpenW ( hlineapp : u32 , dwdeviceid : u32 , lphline : *mut u32 , dwapiversion : u32 , dwextversion : u32 , dwcallbackinstance : usize , dwprivileges : u32 , dwmediamodes : u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineOpenW(hlineapp, dwdeviceid, lphline, dwapiversion, dwextversion, dwcallbackinstance, dwprivileges, dwmediamodes, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn linePark<P0>(hcall: u32, dwparkmode: u32, lpszdiraddress: P0, lpnondiraddress: *mut VARSTRING) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn linePark ( hcall : u32 , dwparkmode : u32 , lpszdiraddress : :: windows::core::PCSTR , lpnondiraddress : *mut VARSTRING ) -> i32 );
    linePark(hcall, dwparkmode, lpszdiraddress.into().abi(), lpnondiraddress)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineParkA<P0>(hcall: u32, dwparkmode: u32, lpszdiraddress: P0, lpnondiraddress: *mut VARSTRING) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineParkA ( hcall : u32 , dwparkmode : u32 , lpszdiraddress : :: windows::core::PCSTR , lpnondiraddress : *mut VARSTRING ) -> i32 );
    lineParkA(hcall, dwparkmode, lpszdiraddress.into().abi(), lpnondiraddress)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineParkW<P0>(hcall: u32, dwparkmode: u32, lpszdiraddress: P0, lpnondiraddress: *mut VARSTRING) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineParkW ( hcall : u32 , dwparkmode : u32 , lpszdiraddress : :: windows::core::PCWSTR , lpnondiraddress : *mut VARSTRING ) -> i32 );
    lineParkW(hcall, dwparkmode, lpszdiraddress.into().abi(), lpnondiraddress)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn linePickup<P0, P1>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0, lpszgroupid: P1) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn linePickup ( hline : u32 , dwaddressid : u32 , lphcall : *mut u32 , lpszdestaddress : :: windows::core::PCSTR , lpszgroupid : :: windows::core::PCSTR ) -> i32 );
    linePickup(hline, dwaddressid, lphcall, lpszdestaddress.into().abi(), lpszgroupid.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn linePickupA<P0, P1>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0, lpszgroupid: P1) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn linePickupA ( hline : u32 , dwaddressid : u32 , lphcall : *mut u32 , lpszdestaddress : :: windows::core::PCSTR , lpszgroupid : :: windows::core::PCSTR ) -> i32 );
    linePickupA(hline, dwaddressid, lphcall, lpszdestaddress.into().abi(), lpszgroupid.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn linePickupW<P0, P1>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0, lpszgroupid: P1) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn linePickupW ( hline : u32 , dwaddressid : u32 , lphcall : *mut u32 , lpszdestaddress : :: windows::core::PCWSTR , lpszgroupid : :: windows::core::PCWSTR ) -> i32 );
    linePickupW(hline, dwaddressid, lphcall, lpszdestaddress.into().abi(), lpszgroupid.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn linePrepareAddToConference(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn linePrepareAddToConference ( hconfcall : u32 , lphconsultcall : *mut u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    linePrepareAddToConference(hconfcall, lphconsultcall, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn linePrepareAddToConferenceA(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn linePrepareAddToConferenceA ( hconfcall : u32 , lphconsultcall : *mut u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    linePrepareAddToConferenceA(hconfcall, lphconsultcall, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn linePrepareAddToConferenceW(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn linePrepareAddToConferenceW ( hconfcall : u32 , lphconsultcall : *mut u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    linePrepareAddToConferenceW(hconfcall, lphconsultcall, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineProxyMessage(hline: u32, hcall: u32, dwmsg: u32, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineProxyMessage ( hline : u32 , hcall : u32 , dwmsg : u32 , dwparam1 : u32 , dwparam2 : u32 , dwparam3 : u32 ) -> i32 );
    lineProxyMessage(hline, hcall, dwmsg, dwparam1, dwparam2, dwparam3)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn lineProxyResponse(hline: u32, lpproxyrequest: *mut LINEPROXYREQUEST, dwresult: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineProxyResponse ( hline : u32 , lpproxyrequest : *mut LINEPROXYREQUEST , dwresult : u32 ) -> i32 );
    lineProxyResponse(hline, lpproxyrequest, dwresult)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineRedirect<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineRedirect ( hcall : u32 , lpszdestaddress : :: windows::core::PCSTR , dwcountrycode : u32 ) -> i32 );
    lineRedirect(hcall, lpszdestaddress.into().abi(), dwcountrycode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineRedirectA<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineRedirectA ( hcall : u32 , lpszdestaddress : :: windows::core::PCSTR , dwcountrycode : u32 ) -> i32 );
    lineRedirectA(hcall, lpszdestaddress.into().abi(), dwcountrycode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineRedirectW<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineRedirectW ( hcall : u32 , lpszdestaddress : :: windows::core::PCWSTR , dwcountrycode : u32 ) -> i32 );
    lineRedirectW(hcall, lpszdestaddress.into().abi(), dwcountrycode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineRegisterRequestRecipient(hlineapp: u32, dwregistrationinstance: u32, dwrequestmode: u32, benable: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineRegisterRequestRecipient ( hlineapp : u32 , dwregistrationinstance : u32 , dwrequestmode : u32 , benable : u32 ) -> i32 );
    lineRegisterRequestRecipient(hlineapp, dwregistrationinstance, dwrequestmode, benable)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineReleaseUserUserInfo(hcall: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineReleaseUserUserInfo ( hcall : u32 ) -> i32 );
    lineReleaseUserUserInfo(hcall)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineRemoveFromConference(hcall: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineRemoveFromConference ( hcall : u32 ) -> i32 );
    lineRemoveFromConference(hcall)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineRemoveProvider<P0>(dwpermanentproviderid: u32, hwndowner: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineRemoveProvider ( dwpermanentproviderid : u32 , hwndowner : super::super::Foundation:: HWND ) -> i32 );
    lineRemoveProvider(dwpermanentproviderid, hwndowner.into())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSecureCall(hcall: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSecureCall ( hcall : u32 ) -> i32 );
    lineSecureCall(hcall)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSendUserUserInfo<P0>(hcall: u32, lpsuseruserinfo: P0, dwsize: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSendUserUserInfo ( hcall : u32 , lpsuseruserinfo : :: windows::core::PCSTR , dwsize : u32 ) -> i32 );
    lineSendUserUserInfo(hcall, lpsuseruserinfo.into().abi(), dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetAgentActivity(hline: u32, dwaddressid: u32, dwactivityid: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetAgentActivity ( hline : u32 , dwaddressid : u32 , dwactivityid : u32 ) -> i32 );
    lineSetAgentActivity(hline, dwaddressid, dwactivityid)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetAgentGroup(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetAgentGroup ( hline : u32 , dwaddressid : u32 , lpagentgrouplist : *mut LINEAGENTGROUPLIST ) -> i32 );
    lineSetAgentGroup(hline, dwaddressid, lpagentgrouplist)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetAgentMeasurementPeriod(hline: u32, hagent: u32, dwmeasurementperiod: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetAgentMeasurementPeriod ( hline : u32 , hagent : u32 , dwmeasurementperiod : u32 ) -> i32 );
    lineSetAgentMeasurementPeriod(hline, hagent, dwmeasurementperiod)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetAgentSessionState(hline: u32, hagentsession: u32, dwagentsessionstate: u32, dwnextagentsessionstate: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetAgentSessionState ( hline : u32 , hagentsession : u32 , dwagentsessionstate : u32 , dwnextagentsessionstate : u32 ) -> i32 );
    lineSetAgentSessionState(hline, hagentsession, dwagentsessionstate, dwnextagentsessionstate)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetAgentState(hline: u32, dwaddressid: u32, dwagentstate: u32, dwnextagentstate: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetAgentState ( hline : u32 , dwaddressid : u32 , dwagentstate : u32 , dwnextagentstate : u32 ) -> i32 );
    lineSetAgentState(hline, dwaddressid, dwagentstate, dwnextagentstate)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetAgentStateEx(hline: u32, hagent: u32, dwagentstate: u32, dwnextagentstate: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetAgentStateEx ( hline : u32 , hagent : u32 , dwagentstate : u32 , dwnextagentstate : u32 ) -> i32 );
    lineSetAgentStateEx(hline, hagent, dwagentstate, dwnextagentstate)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetAppPriority<P0, P1>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: P1, dwpriority: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetAppPriority ( lpszappfilename : :: windows::core::PCSTR , dwmediamode : u32 , lpextensionid : *mut LINEEXTENSIONID , dwrequestmode : u32 , lpszextensionname : :: windows::core::PCSTR , dwpriority : u32 ) -> i32 );
    lineSetAppPriority(lpszappfilename.into().abi(), dwmediamode, lpextensionid, dwrequestmode, lpszextensionname.into().abi(), dwpriority)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetAppPriorityA<P0, P1>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: P1, dwpriority: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetAppPriorityA ( lpszappfilename : :: windows::core::PCSTR , dwmediamode : u32 , lpextensionid : *mut LINEEXTENSIONID , dwrequestmode : u32 , lpszextensionname : :: windows::core::PCSTR , dwpriority : u32 ) -> i32 );
    lineSetAppPriorityA(lpszappfilename.into().abi(), dwmediamode, lpextensionid, dwrequestmode, lpszextensionname.into().abi(), dwpriority)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetAppPriorityW<P0, P1>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: P1, dwpriority: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetAppPriorityW ( lpszappfilename : :: windows::core::PCWSTR , dwmediamode : u32 , lpextensionid : *mut LINEEXTENSIONID , dwrequestmode : u32 , lpszextensionname : :: windows::core::PCWSTR , dwpriority : u32 ) -> i32 );
    lineSetAppPriorityW(lpszappfilename.into().abi(), dwmediamode, lpextensionid, dwrequestmode, lpszextensionname.into().abi(), dwpriority)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetAppSpecific(hcall: u32, dwappspecific: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetAppSpecific ( hcall : u32 , dwappspecific : u32 ) -> i32 );
    lineSetAppSpecific(hcall, dwappspecific)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetCallData(hcall: u32, lpcalldata: *mut ::core::ffi::c_void, dwsize: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetCallData ( hcall : u32 , lpcalldata : *mut ::core::ffi::c_void , dwsize : u32 ) -> i32 );
    lineSetCallData(hcall, lpcalldata, dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetCallParams(hcall: u32, dwbearermode: u32, dwminrate: u32, dwmaxrate: u32, lpdialparams: *const LINEDIALPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetCallParams ( hcall : u32 , dwbearermode : u32 , dwminrate : u32 , dwmaxrate : u32 , lpdialparams : *const LINEDIALPARAMS ) -> i32 );
    lineSetCallParams(hcall, dwbearermode, dwminrate, dwmaxrate, lpdialparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetCallPrivilege(hcall: u32, dwcallprivilege: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetCallPrivilege ( hcall : u32 , dwcallprivilege : u32 ) -> i32 );
    lineSetCallPrivilege(hcall, dwcallprivilege)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetCallQualityOfService(hcall: u32, lpsendingflowspec: *mut ::core::ffi::c_void, dwsendingflowspecsize: u32, lpreceivingflowspec: *mut ::core::ffi::c_void, dwreceivingflowspecsize: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetCallQualityOfService ( hcall : u32 , lpsendingflowspec : *mut ::core::ffi::c_void , dwsendingflowspecsize : u32 , lpreceivingflowspec : *mut ::core::ffi::c_void , dwreceivingflowspecsize : u32 ) -> i32 );
    lineSetCallQualityOfService(hcall, lpsendingflowspec, dwsendingflowspecsize, lpreceivingflowspec, dwreceivingflowspecsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetCallTreatment(hcall: u32, dwtreatment: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetCallTreatment ( hcall : u32 , dwtreatment : u32 ) -> i32 );
    lineSetCallTreatment(hcall, dwtreatment)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetCurrentLocation(hlineapp: u32, dwlocation: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetCurrentLocation ( hlineapp : u32 , dwlocation : u32 ) -> i32 );
    lineSetCurrentLocation(hlineapp, dwlocation)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetDevConfig<P0>(dwdeviceid: u32, lpdeviceconfig: *const ::core::ffi::c_void, dwsize: u32, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetDevConfig ( dwdeviceid : u32 , lpdeviceconfig : *const ::core::ffi::c_void , dwsize : u32 , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    lineSetDevConfig(dwdeviceid, lpdeviceconfig, dwsize, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetDevConfigA<P0>(dwdeviceid: u32, lpdeviceconfig: *const ::core::ffi::c_void, dwsize: u32, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetDevConfigA ( dwdeviceid : u32 , lpdeviceconfig : *const ::core::ffi::c_void , dwsize : u32 , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    lineSetDevConfigA(dwdeviceid, lpdeviceconfig, dwsize, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetDevConfigW<P0>(dwdeviceid: u32, lpdeviceconfig: *const ::core::ffi::c_void, dwsize: u32, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetDevConfigW ( dwdeviceid : u32 , lpdeviceconfig : *const ::core::ffi::c_void , dwsize : u32 , lpszdeviceclass : :: windows::core::PCWSTR ) -> i32 );
    lineSetDevConfigW(dwdeviceid, lpdeviceconfig, dwsize, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetLineDevStatus(hline: u32, dwstatustochange: u32, fstatus: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetLineDevStatus ( hline : u32 , dwstatustochange : u32 , fstatus : u32 ) -> i32 );
    lineSetLineDevStatus(hline, dwstatustochange, fstatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetMediaControl(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdigitlist: *const LINEMEDIACONTROLDIGIT, dwdigitnumentries: u32, lpmedialist: *const LINEMEDIACONTROLMEDIA, dwmedianumentries: u32, lptonelist: *const LINEMEDIACONTROLTONE, dwtonenumentries: u32, lpcallstatelist: *const LINEMEDIACONTROLCALLSTATE, dwcallstatenumentries: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetMediaControl ( hline : u32 , dwaddressid : u32 , hcall : u32 , dwselect : u32 , lpdigitlist : *const LINEMEDIACONTROLDIGIT , dwdigitnumentries : u32 , lpmedialist : *const LINEMEDIACONTROLMEDIA , dwmedianumentries : u32 , lptonelist : *const LINEMEDIACONTROLTONE , dwtonenumentries : u32 , lpcallstatelist : *const LINEMEDIACONTROLCALLSTATE , dwcallstatenumentries : u32 ) -> i32 );
    lineSetMediaControl(hline, dwaddressid, hcall, dwselect, lpdigitlist, dwdigitnumentries, lpmedialist, dwmedianumentries, lptonelist, dwtonenumentries, lpcallstatelist, dwcallstatenumentries)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetMediaMode(hcall: u32, dwmediamodes: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetMediaMode ( hcall : u32 , dwmediamodes : u32 ) -> i32 );
    lineSetMediaMode(hcall, dwmediamodes)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetNumRings(hline: u32, dwaddressid: u32, dwnumrings: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetNumRings ( hline : u32 , dwaddressid : u32 , dwnumrings : u32 ) -> i32 );
    lineSetNumRings(hline, dwaddressid, dwnumrings)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetQueueMeasurementPeriod(hline: u32, dwqueueid: u32, dwmeasurementperiod: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetQueueMeasurementPeriod ( hline : u32 , dwqueueid : u32 , dwmeasurementperiod : u32 ) -> i32 );
    lineSetQueueMeasurementPeriod(hline, dwqueueid, dwmeasurementperiod)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetStatusMessages(hline: u32, dwlinestates: u32, dwaddressstates: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetStatusMessages ( hline : u32 , dwlinestates : u32 , dwaddressstates : u32 ) -> i32 );
    lineSetStatusMessages(hline, dwlinestates, dwaddressstates)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetTerminal(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, dwterminalmodes: u32, dwterminalid: u32, benable: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetTerminal ( hline : u32 , dwaddressid : u32 , hcall : u32 , dwselect : u32 , dwterminalmodes : u32 , dwterminalid : u32 , benable : u32 ) -> i32 );
    lineSetTerminal(hline, dwaddressid, hcall, dwselect, dwterminalmodes, dwterminalid, benable)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetTollList<P0>(hlineapp: u32, dwdeviceid: u32, lpszaddressin: P0, dwtolllistoption: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetTollList ( hlineapp : u32 , dwdeviceid : u32 , lpszaddressin : :: windows::core::PCSTR , dwtolllistoption : u32 ) -> i32 );
    lineSetTollList(hlineapp, dwdeviceid, lpszaddressin.into().abi(), dwtolllistoption)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetTollListA<P0>(hlineapp: u32, dwdeviceid: u32, lpszaddressin: P0, dwtolllistoption: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetTollListA ( hlineapp : u32 , dwdeviceid : u32 , lpszaddressin : :: windows::core::PCSTR , dwtolllistoption : u32 ) -> i32 );
    lineSetTollListA(hlineapp, dwdeviceid, lpszaddressin.into().abi(), dwtolllistoption)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetTollListW<P0>(hlineapp: u32, dwdeviceid: u32, lpszaddressinw: P0, dwtolllistoption: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetTollListW ( hlineapp : u32 , dwdeviceid : u32 , lpszaddressinw : :: windows::core::PCWSTR , dwtolllistoption : u32 ) -> i32 );
    lineSetTollListW(hlineapp, dwdeviceid, lpszaddressinw.into().abi(), dwtolllistoption)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetupConference(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetupConference ( hcall : u32 , hline : u32 , lphconfcall : *mut u32 , lphconsultcall : *mut u32 , dwnumparties : u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineSetupConference(hcall, hline, lphconfcall, lphconsultcall, dwnumparties, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetupConferenceA(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetupConferenceA ( hcall : u32 , hline : u32 , lphconfcall : *mut u32 , lphconsultcall : *mut u32 , dwnumparties : u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineSetupConferenceA(hcall, hline, lphconfcall, lphconsultcall, dwnumparties, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetupConferenceW(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetupConferenceW ( hcall : u32 , hline : u32 , lphconfcall : *mut u32 , lphconsultcall : *mut u32 , dwnumparties : u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineSetupConferenceW(hcall, hline, lphconfcall, lphconsultcall, dwnumparties, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetupTransfer(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetupTransfer ( hcall : u32 , lphconsultcall : *mut u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineSetupTransfer(hcall, lphconsultcall, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetupTransferA(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetupTransferA ( hcall : u32 , lphconsultcall : *mut u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineSetupTransferA(hcall, lphconsultcall, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSetupTransferW(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSetupTransferW ( hcall : u32 , lphconsultcall : *mut u32 , lpcallparams : *const LINECALLPARAMS ) -> i32 );
    lineSetupTransferW(hcall, lphconsultcall, lpcallparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineShutdown(hlineapp: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineShutdown ( hlineapp : u32 ) -> i32 );
    lineShutdown(hlineapp)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineSwapHold(hactivecall: u32, hheldcall: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineSwapHold ( hactivecall : u32 , hheldcall : u32 ) -> i32 );
    lineSwapHold(hactivecall, hheldcall)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineTranslateAddress<P0>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: P0, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineTranslateAddress ( hlineapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , lpszaddressin : :: windows::core::PCSTR , dwcard : u32 , dwtranslateoptions : u32 , lptranslateoutput : *mut LINETRANSLATEOUTPUT ) -> i32 );
    lineTranslateAddress(hlineapp, dwdeviceid, dwapiversion, lpszaddressin.into().abi(), dwcard, dwtranslateoptions, lptranslateoutput)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineTranslateAddressA<P0>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: P0, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineTranslateAddressA ( hlineapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , lpszaddressin : :: windows::core::PCSTR , dwcard : u32 , dwtranslateoptions : u32 , lptranslateoutput : *mut LINETRANSLATEOUTPUT ) -> i32 );
    lineTranslateAddressA(hlineapp, dwdeviceid, dwapiversion, lpszaddressin.into().abi(), dwcard, dwtranslateoptions, lptranslateoutput)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineTranslateAddressW<P0>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: P0, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineTranslateAddressW ( hlineapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , lpszaddressin : :: windows::core::PCWSTR , dwcard : u32 , dwtranslateoptions : u32 , lptranslateoutput : *mut LINETRANSLATEOUTPUT ) -> i32 );
    lineTranslateAddressW(hlineapp, dwdeviceid, dwapiversion, lpszaddressin.into().abi(), dwcard, dwtranslateoptions, lptranslateoutput)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineTranslateDialog<P0, P1>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: P0, lpszaddressin: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineTranslateDialog ( hlineapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , hwndowner : super::super::Foundation:: HWND , lpszaddressin : :: windows::core::PCSTR ) -> i32 );
    lineTranslateDialog(hlineapp, dwdeviceid, dwapiversion, hwndowner.into(), lpszaddressin.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineTranslateDialogA<P0, P1>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: P0, lpszaddressin: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineTranslateDialogA ( hlineapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , hwndowner : super::super::Foundation:: HWND , lpszaddressin : :: windows::core::PCSTR ) -> i32 );
    lineTranslateDialogA(hlineapp, dwdeviceid, dwapiversion, hwndowner.into(), lpszaddressin.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn lineTranslateDialogW<P0, P1>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: P0, lpszaddressin: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineTranslateDialogW ( hlineapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , hwndowner : super::super::Foundation:: HWND , lpszaddressin : :: windows::core::PCWSTR ) -> i32 );
    lineTranslateDialogW(hlineapp, dwdeviceid, dwapiversion, hwndowner.into(), lpszaddressin.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineUncompleteCall(hline: u32, dwcompletionid: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineUncompleteCall ( hline : u32 , dwcompletionid : u32 ) -> i32 );
    lineUncompleteCall(hline, dwcompletionid)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineUnhold(hcall: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn lineUnhold ( hcall : u32 ) -> i32 );
    lineUnhold(hcall)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineUnpark<P0>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineUnpark ( hline : u32 , dwaddressid : u32 , lphcall : *mut u32 , lpszdestaddress : :: windows::core::PCSTR ) -> i32 );
    lineUnpark(hline, dwaddressid, lphcall, lpszdestaddress.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineUnparkA<P0>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineUnparkA ( hline : u32 , dwaddressid : u32 , lphcall : *mut u32 , lpszdestaddress : :: windows::core::PCSTR ) -> i32 );
    lineUnparkA(hline, dwaddressid, lphcall, lpszdestaddress.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn lineUnparkW<P0>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn lineUnparkW ( hline : u32 , dwaddressid : u32 , lphcall : *mut u32 , lpszdestaddress : :: windows::core::PCWSTR ) -> i32 );
    lineUnparkW(hline, dwaddressid, lphcall, lpszdestaddress.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneClose(hphone: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneClose ( hphone : u32 ) -> i32 );
    phoneClose(hphone)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn phoneConfigDialog<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneConfigDialog ( dwdeviceid : u32 , hwndowner : super::super::Foundation:: HWND , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    phoneConfigDialog(dwdeviceid, hwndowner.into(), lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn phoneConfigDialogA<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneConfigDialogA ( dwdeviceid : u32 , hwndowner : super::super::Foundation:: HWND , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    phoneConfigDialogA(dwdeviceid, hwndowner.into(), lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn phoneConfigDialogW<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneConfigDialogW ( dwdeviceid : u32 , hwndowner : super::super::Foundation:: HWND , lpszdeviceclass : :: windows::core::PCWSTR ) -> i32 );
    phoneConfigDialogW(dwdeviceid, hwndowner.into(), lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneDevSpecific(hphone: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneDevSpecific ( hphone : u32 , lpparams : *mut ::core::ffi::c_void , dwsize : u32 ) -> i32 );
    phoneDevSpecific(hphone, lpparams, dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetButtonInfo(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetButtonInfo ( hphone : u32 , dwbuttonlampid : u32 , lpbuttoninfo : *mut PHONEBUTTONINFO ) -> i32 );
    phoneGetButtonInfo(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetButtonInfoA(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetButtonInfoA ( hphone : u32 , dwbuttonlampid : u32 , lpbuttoninfo : *mut PHONEBUTTONINFO ) -> i32 );
    phoneGetButtonInfoA(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetButtonInfoW(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetButtonInfoW ( hphone : u32 , dwbuttonlampid : u32 , lpbuttoninfo : *mut PHONEBUTTONINFO ) -> i32 );
    phoneGetButtonInfoW(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetData(hphone: u32, dwdataid: u32, lpdata: *mut ::core::ffi::c_void, dwsize: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetData ( hphone : u32 , dwdataid : u32 , lpdata : *mut ::core::ffi::c_void , dwsize : u32 ) -> i32 );
    phoneGetData(hphone, dwdataid, lpdata, dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetDevCaps(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetDevCaps ( hphoneapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , dwextversion : u32 , lpphonecaps : *mut PHONECAPS ) -> i32 );
    phoneGetDevCaps(hphoneapp, dwdeviceid, dwapiversion, dwextversion, lpphonecaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetDevCapsA(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetDevCapsA ( hphoneapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , dwextversion : u32 , lpphonecaps : *mut PHONECAPS ) -> i32 );
    phoneGetDevCapsA(hphoneapp, dwdeviceid, dwapiversion, dwextversion, lpphonecaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetDevCapsW(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetDevCapsW ( hphoneapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , dwextversion : u32 , lpphonecaps : *mut PHONECAPS ) -> i32 );
    phoneGetDevCapsW(hphoneapp, dwdeviceid, dwapiversion, dwextversion, lpphonecaps)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetDisplay(hphone: u32, lpdisplay: *mut VARSTRING) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetDisplay ( hphone : u32 , lpdisplay : *mut VARSTRING ) -> i32 );
    phoneGetDisplay(hphone, lpdisplay)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetGain(hphone: u32, dwhookswitchdev: u32, lpdwgain: *mut u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetGain ( hphone : u32 , dwhookswitchdev : u32 , lpdwgain : *mut u32 ) -> i32 );
    phoneGetGain(hphone, dwhookswitchdev, lpdwgain)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetHookSwitch(hphone: u32, lpdwhookswitchdevs: *mut u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetHookSwitch ( hphone : u32 , lpdwhookswitchdevs : *mut u32 ) -> i32 );
    phoneGetHookSwitch(hphone, lpdwhookswitchdevs)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetID<P0>(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetID ( hphone : u32 , lpdeviceid : *mut VARSTRING , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    phoneGetID(hphone, lpdeviceid, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetIDA<P0>(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetIDA ( hphone : u32 , lpdeviceid : *mut VARSTRING , lpszdeviceclass : :: windows::core::PCSTR ) -> i32 );
    phoneGetIDA(hphone, lpdeviceid, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetIDW<P0>(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetIDW ( hphone : u32 , lpdeviceid : *mut VARSTRING , lpszdeviceclass : :: windows::core::PCWSTR ) -> i32 );
    phoneGetIDW(hphone, lpdeviceid, lpszdeviceclass.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetIcon<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut isize) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetIcon ( dwdeviceid : u32 , lpszdeviceclass : :: windows::core::PCSTR , lphicon : *mut isize ) -> i32 );
    phoneGetIcon(dwdeviceid, lpszdeviceclass.into().abi(), lphicon)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetIconA<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut isize) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetIconA ( dwdeviceid : u32 , lpszdeviceclass : :: windows::core::PCSTR , lphicon : *mut isize ) -> i32 );
    phoneGetIconA(dwdeviceid, lpszdeviceclass.into().abi(), lphicon)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetIconW<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut isize) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetIconW ( dwdeviceid : u32 , lpszdeviceclass : :: windows::core::PCWSTR , lphicon : *mut isize ) -> i32 );
    phoneGetIconW(dwdeviceid, lpszdeviceclass.into().abi(), lphicon)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetLamp(hphone: u32, dwbuttonlampid: u32, lpdwlampmode: *mut u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetLamp ( hphone : u32 , dwbuttonlampid : u32 , lpdwlampmode : *mut u32 ) -> i32 );
    phoneGetLamp(hphone, dwbuttonlampid, lpdwlampmode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetMessage(hphoneapp: u32, lpmessage: *mut PHONEMESSAGE, dwtimeout: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetMessage ( hphoneapp : u32 , lpmessage : *mut PHONEMESSAGE , dwtimeout : u32 ) -> i32 );
    phoneGetMessage(hphoneapp, lpmessage, dwtimeout)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetRing(hphone: u32, lpdwringmode: *mut u32, lpdwvolume: *mut u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetRing ( hphone : u32 , lpdwringmode : *mut u32 , lpdwvolume : *mut u32 ) -> i32 );
    phoneGetRing(hphone, lpdwringmode, lpdwvolume)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetStatus(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetStatus ( hphone : u32 , lpphonestatus : *mut PHONESTATUS ) -> i32 );
    phoneGetStatus(hphone, lpphonestatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetStatusA(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetStatusA ( hphone : u32 , lpphonestatus : *mut PHONESTATUS ) -> i32 );
    phoneGetStatusA(hphone, lpphonestatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetStatusMessages(hphone: u32, lpdwphonestates: *mut u32, lpdwbuttonmodes: *mut u32, lpdwbuttonstates: *mut u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetStatusMessages ( hphone : u32 , lpdwphonestates : *mut u32 , lpdwbuttonmodes : *mut u32 , lpdwbuttonstates : *mut u32 ) -> i32 );
    phoneGetStatusMessages(hphone, lpdwphonestates, lpdwbuttonmodes, lpdwbuttonstates)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetStatusW(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetStatusW ( hphone : u32 , lpphonestatus : *mut PHONESTATUS ) -> i32 );
    phoneGetStatusW(hphone, lpphonestatus)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneGetVolume(hphone: u32, dwhookswitchdev: u32, lpdwvolume: *mut u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneGetVolume ( hphone : u32 , dwhookswitchdev : u32 , lpdwvolume : *mut u32 ) -> i32 );
    phoneGetVolume(hphone, dwhookswitchdev, lpdwvolume)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn phoneInitialize<P0, P1>(lphphoneapp: *mut u32, hinstance: P0, lpfncallback: PHONECALLBACK, lpszappname: P1, lpdwnumdevs: *mut u32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneInitialize ( lphphoneapp : *mut u32 , hinstance : super::super::Foundation:: HINSTANCE , lpfncallback : PHONECALLBACK , lpszappname : :: windows::core::PCSTR , lpdwnumdevs : *mut u32 ) -> i32 );
    phoneInitialize(lphphoneapp, hinstance.into(), lpfncallback, lpszappname.into().abi(), lpdwnumdevs)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn phoneInitializeExA<P0, P1>(lphphoneapp: *mut u32, hinstance: P0, lpfncallback: PHONECALLBACK, lpszfriendlyappname: P1, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lpphoneinitializeexparams: *mut PHONEINITIALIZEEXPARAMS) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneInitializeExA ( lphphoneapp : *mut u32 , hinstance : super::super::Foundation:: HINSTANCE , lpfncallback : PHONECALLBACK , lpszfriendlyappname : :: windows::core::PCSTR , lpdwnumdevs : *mut u32 , lpdwapiversion : *mut u32 , lpphoneinitializeexparams : *mut PHONEINITIALIZEEXPARAMS ) -> i32 );
    phoneInitializeExA(lphphoneapp, hinstance.into(), lpfncallback, lpszfriendlyappname.into().abi(), lpdwnumdevs, lpdwapiversion, lpphoneinitializeexparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn phoneInitializeExW<P0, P1>(lphphoneapp: *mut u32, hinstance: P0, lpfncallback: PHONECALLBACK, lpszfriendlyappname: P1, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lpphoneinitializeexparams: *mut PHONEINITIALIZEEXPARAMS) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneInitializeExW ( lphphoneapp : *mut u32 , hinstance : super::super::Foundation:: HINSTANCE , lpfncallback : PHONECALLBACK , lpszfriendlyappname : :: windows::core::PCWSTR , lpdwnumdevs : *mut u32 , lpdwapiversion : *mut u32 , lpphoneinitializeexparams : *mut PHONEINITIALIZEEXPARAMS ) -> i32 );
    phoneInitializeExW(lphphoneapp, hinstance.into(), lpfncallback, lpszfriendlyappname.into().abi(), lpdwnumdevs, lpdwapiversion, lpphoneinitializeexparams)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneNegotiateAPIVersion(hphoneapp: u32, dwdeviceid: u32, dwapilowversion: u32, dwapihighversion: u32, lpdwapiversion: *mut u32, lpextensionid: *mut PHONEEXTENSIONID) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneNegotiateAPIVersion ( hphoneapp : u32 , dwdeviceid : u32 , dwapilowversion : u32 , dwapihighversion : u32 , lpdwapiversion : *mut u32 , lpextensionid : *mut PHONEEXTENSIONID ) -> i32 );
    phoneNegotiateAPIVersion(hphoneapp, dwdeviceid, dwapilowversion, dwapihighversion, lpdwapiversion, lpextensionid)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneNegotiateExtVersion(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextlowversion: u32, dwexthighversion: u32, lpdwextversion: *mut u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneNegotiateExtVersion ( hphoneapp : u32 , dwdeviceid : u32 , dwapiversion : u32 , dwextlowversion : u32 , dwexthighversion : u32 , lpdwextversion : *mut u32 ) -> i32 );
    phoneNegotiateExtVersion(hphoneapp, dwdeviceid, dwapiversion, dwextlowversion, dwexthighversion, lpdwextversion)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneOpen(hphoneapp: u32, dwdeviceid: u32, lphphone: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivilege: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneOpen ( hphoneapp : u32 , dwdeviceid : u32 , lphphone : *mut u32 , dwapiversion : u32 , dwextversion : u32 , dwcallbackinstance : usize , dwprivilege : u32 ) -> i32 );
    phoneOpen(hphoneapp, dwdeviceid, lphphone, dwapiversion, dwextversion, dwcallbackinstance, dwprivilege)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetButtonInfo(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetButtonInfo ( hphone : u32 , dwbuttonlampid : u32 , lpbuttoninfo : *const PHONEBUTTONINFO ) -> i32 );
    phoneSetButtonInfo(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetButtonInfoA(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetButtonInfoA ( hphone : u32 , dwbuttonlampid : u32 , lpbuttoninfo : *const PHONEBUTTONINFO ) -> i32 );
    phoneSetButtonInfoA(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetButtonInfoW(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetButtonInfoW ( hphone : u32 , dwbuttonlampid : u32 , lpbuttoninfo : *const PHONEBUTTONINFO ) -> i32 );
    phoneSetButtonInfoW(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetData(hphone: u32, dwdataid: u32, lpdata: *const ::core::ffi::c_void, dwsize: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetData ( hphone : u32 , dwdataid : u32 , lpdata : *const ::core::ffi::c_void , dwsize : u32 ) -> i32 );
    phoneSetData(hphone, dwdataid, lpdata, dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetDisplay<P0>(hphone: u32, dwrow: u32, dwcolumn: u32, lpsdisplay: P0, dwsize: u32) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetDisplay ( hphone : u32 , dwrow : u32 , dwcolumn : u32 , lpsdisplay : :: windows::core::PCSTR , dwsize : u32 ) -> i32 );
    phoneSetDisplay(hphone, dwrow, dwcolumn, lpsdisplay.into().abi(), dwsize)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetGain(hphone: u32, dwhookswitchdev: u32, dwgain: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetGain ( hphone : u32 , dwhookswitchdev : u32 , dwgain : u32 ) -> i32 );
    phoneSetGain(hphone, dwhookswitchdev, dwgain)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetHookSwitch(hphone: u32, dwhookswitchdevs: u32, dwhookswitchmode: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetHookSwitch ( hphone : u32 , dwhookswitchdevs : u32 , dwhookswitchmode : u32 ) -> i32 );
    phoneSetHookSwitch(hphone, dwhookswitchdevs, dwhookswitchmode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetLamp(hphone: u32, dwbuttonlampid: u32, dwlampmode: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetLamp ( hphone : u32 , dwbuttonlampid : u32 , dwlampmode : u32 ) -> i32 );
    phoneSetLamp(hphone, dwbuttonlampid, dwlampmode)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetRing(hphone: u32, dwringmode: u32, dwvolume: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetRing ( hphone : u32 , dwringmode : u32 , dwvolume : u32 ) -> i32 );
    phoneSetRing(hphone, dwringmode, dwvolume)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetStatusMessages(hphone: u32, dwphonestates: u32, dwbuttonmodes: u32, dwbuttonstates: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetStatusMessages ( hphone : u32 , dwphonestates : u32 , dwbuttonmodes : u32 , dwbuttonstates : u32 ) -> i32 );
    phoneSetStatusMessages(hphone, dwphonestates, dwbuttonmodes, dwbuttonstates)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneSetVolume(hphone: u32, dwhookswitchdev: u32, dwvolume: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneSetVolume ( hphone : u32 , dwhookswitchdev : u32 , dwvolume : u32 ) -> i32 );
    phoneSetVolume(hphone, dwhookswitchdev, dwvolume)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn phoneShutdown(hphoneapp: u32) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn phoneShutdown ( hphoneapp : u32 ) -> i32 );
    phoneShutdown(hphoneapp)
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn tapiGetLocationInfo(lpszcountrycode: &mut [u8; 8], lpszcitycode: &mut [u8; 8]) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn tapiGetLocationInfo ( lpszcountrycode : :: windows::core::PSTR , lpszcitycode : :: windows::core::PSTR ) -> i32 );
    tapiGetLocationInfo(::core::mem::transmute(lpszcountrycode.as_ptr()), ::core::mem::transmute(lpszcitycode.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn tapiGetLocationInfoA(lpszcountrycode: &mut [u8; 8], lpszcitycode: &mut [u8; 8]) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn tapiGetLocationInfoA ( lpszcountrycode : :: windows::core::PSTR , lpszcitycode : :: windows::core::PSTR ) -> i32 );
    tapiGetLocationInfoA(::core::mem::transmute(lpszcountrycode.as_ptr()), ::core::mem::transmute(lpszcitycode.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn tapiGetLocationInfoW(lpszcountrycodew: &mut [u16; 8], lpszcitycodew: &mut [u16; 8]) -> i32 {
    ::windows::core::link ! ( "tapi32.dll""system" fn tapiGetLocationInfoW ( lpszcountrycodew : :: windows::core::PWSTR , lpszcitycodew : :: windows::core::PWSTR ) -> i32 );
    tapiGetLocationInfoW(::core::mem::transmute(lpszcountrycodew.as_ptr()), ::core::mem::transmute(lpszcitycodew.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn tapiRequestDrop<P0, P1>(hwnd: P0, wrequestid: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn tapiRequestDrop ( hwnd : super::super::Foundation:: HWND , wrequestid : super::super::Foundation:: WPARAM ) -> i32 );
    tapiRequestDrop(hwnd.into(), wrequestid.into())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn tapiRequestMakeCall<P0, P1, P2, P3>(lpszdestaddress: P0, lpszappname: P1, lpszcalledparty: P2, lpszcomment: P3) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn tapiRequestMakeCall ( lpszdestaddress : :: windows::core::PCSTR , lpszappname : :: windows::core::PCSTR , lpszcalledparty : :: windows::core::PCSTR , lpszcomment : :: windows::core::PCSTR ) -> i32 );
    tapiRequestMakeCall(lpszdestaddress.into().abi(), lpszappname.into().abi(), lpszcalledparty.into().abi(), lpszcomment.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn tapiRequestMakeCallA<P0, P1, P2, P3>(lpszdestaddress: P0, lpszappname: P1, lpszcalledparty: P2, lpszcomment: P3) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn tapiRequestMakeCallA ( lpszdestaddress : :: windows::core::PCSTR , lpszappname : :: windows::core::PCSTR , lpszcalledparty : :: windows::core::PCSTR , lpszcomment : :: windows::core::PCSTR ) -> i32 );
    tapiRequestMakeCallA(lpszdestaddress.into().abi(), lpszappname.into().abi(), lpszcalledparty.into().abi(), lpszcomment.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[inline]
pub unsafe fn tapiRequestMakeCallW<P0, P1, P2, P3>(lpszdestaddress: P0, lpszappname: P1, lpszcalledparty: P2, lpszcomment: P3) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn tapiRequestMakeCallW ( lpszdestaddress : :: windows::core::PCWSTR , lpszappname : :: windows::core::PCWSTR , lpszcalledparty : :: windows::core::PCWSTR , lpszcomment : :: windows::core::PCWSTR ) -> i32 );
    tapiRequestMakeCallW(lpszdestaddress.into().abi(), lpszappname.into().abi(), lpszcalledparty.into().abi(), lpszcomment.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn tapiRequestMediaCall<P0, P1, P2, P3, P4, P5, P6, P7>(hwnd: P0, wrequestid: P1, lpszdeviceclass: P2, lpdeviceid: P3, dwsize: u32, dwsecure: u32, lpszdestaddress: P4, lpszappname: P5, lpszcalledparty: P6, lpszcomment: P7) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P6: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P7: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn tapiRequestMediaCall ( hwnd : super::super::Foundation:: HWND , wrequestid : super::super::Foundation:: WPARAM , lpszdeviceclass : :: windows::core::PCSTR , lpdeviceid : :: windows::core::PCSTR , dwsize : u32 , dwsecure : u32 , lpszdestaddress : :: windows::core::PCSTR , lpszappname : :: windows::core::PCSTR , lpszcalledparty : :: windows::core::PCSTR , lpszcomment : :: windows::core::PCSTR ) -> i32 );
    tapiRequestMediaCall(hwnd.into(), wrequestid.into(), lpszdeviceclass.into().abi(), lpdeviceid.into().abi(), dwsize, dwsecure, lpszdestaddress.into().abi(), lpszappname.into().abi(), lpszcalledparty.into().abi(), lpszcomment.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn tapiRequestMediaCallA<P0, P1, P2, P3, P4, P5, P6, P7>(hwnd: P0, wrequestid: P1, lpszdeviceclass: P2, lpdeviceid: P3, dwsize: u32, dwsecure: u32, lpszdestaddress: P4, lpszappname: P5, lpszcalledparty: P6, lpszcomment: P7) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P6: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P7: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn tapiRequestMediaCallA ( hwnd : super::super::Foundation:: HWND , wrequestid : super::super::Foundation:: WPARAM , lpszdeviceclass : :: windows::core::PCSTR , lpdeviceid : :: windows::core::PCSTR , dwsize : u32 , dwsecure : u32 , lpszdestaddress : :: windows::core::PCSTR , lpszappname : :: windows::core::PCSTR , lpszcalledparty : :: windows::core::PCSTR , lpszcomment : :: windows::core::PCSTR ) -> i32 );
    tapiRequestMediaCallA(hwnd.into(), wrequestid.into(), lpszdeviceclass.into().abi(), lpdeviceid.into().abi(), dwsize, dwsecure, lpszdestaddress.into().abi(), lpszappname.into().abi(), lpszcalledparty.into().abi(), lpszcomment.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn tapiRequestMediaCallW<P0, P1, P2, P3, P4, P5, P6, P7>(hwnd: P0, wrequestid: P1, lpszdeviceclass: P2, lpdeviceid: P3, dwsize: u32, dwsecure: u32, lpszdestaddress: P4, lpszappname: P5, lpszcalledparty: P6, lpszcomment: P7) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P6: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P7: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "tapi32.dll""system" fn tapiRequestMediaCallW ( hwnd : super::super::Foundation:: HWND , wrequestid : super::super::Foundation:: WPARAM , lpszdeviceclass : :: windows::core::PCWSTR , lpdeviceid : :: windows::core::PCWSTR , dwsize : u32 , dwsecure : u32 , lpszdestaddress : :: windows::core::PCWSTR , lpszappname : :: windows::core::PCWSTR , lpszcalledparty : :: windows::core::PCWSTR , lpszcomment : :: windows::core::PCWSTR ) -> i32 );
    tapiRequestMediaCallW(hwnd.into(), wrequestid.into(), lpszdeviceclass.into().abi(), lpdeviceid.into().abi(), dwsize, dwsecure, lpszdestaddress.into().abi(), lpszappname.into().abi(), lpszcalledparty.into().abi(), lpszcomment.into().abi())
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumACDGroup(::windows::core::IUnknown);
impl IEnumACDGroup {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITACDGroup>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), pceltfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumACDGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumACDGroup, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumACDGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumACDGroup {
    type Vtable = IEnumACDGroup_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumACDGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5afc3157_4bcc_11d1_bf80_00805fc147d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumACDGroup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumAddress(::windows::core::IUnknown);
impl IEnumAddress {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<ITAddress>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumAddress, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumAddress {
    type Vtable = IEnumAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1666fca1_9363_11d0_835c_00aa003ccabd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumAddress_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumAgent(::windows::core::IUnknown);
impl IEnumAgent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITAgent>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), pceltfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumAgent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumAgent, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumAgent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumAgent {
    type Vtable = IEnumAgent_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumAgent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5afc314d_4bcc_11d1_bf80_00805fc147d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumAgent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumAgentHandler(::windows::core::IUnknown);
impl IEnumAgentHandler {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITAgentHandler>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), pceltfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumAgentHandler> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumAgentHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumAgentHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumAgentHandler {
    type Vtable = IEnumAgentHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumAgentHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x587e8c28_9802_11d1_a0a4_00805fc147d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumAgentHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumAgentSession(::windows::core::IUnknown);
impl IEnumAgentSession {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITAgentSession>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), pceltfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumAgentSession> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumAgentSession, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumAgentSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumAgentSession {
    type Vtable = IEnumAgentSession_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumAgentSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5afc314e_4bcc_11d1_bf80_00805fc147d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumAgentSession_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumBstr(::windows::core::IUnknown);
impl IEnumBstr {
    pub unsafe fn Next(&self, ppstrings: &mut [::windows::core::BSTR], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ppstrings.len() as _, ::core::mem::transmute(ppstrings.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBstr> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumBstr, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumBstr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumBstr {
    type Vtable = IEnumBstr_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumBstr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35372049_0bc6_11d2_a033_00c04fb6809f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBstr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppstrings: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumCall(::windows::core::IUnknown);
impl IEnumCall {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITCallInfo>, pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumCall> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumCall, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumCall {
    type Vtable = IEnumCall_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumCall {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae269cf6_935e_11d0_835c_00aa003ccabd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCall_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumCallHub(::windows::core::IUnknown);
impl IEnumCallHub {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<ITCallHub>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumCallHub> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumCallHub, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumCallHub {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumCallHub {
    type Vtable = IEnumCallHub_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumCallHub {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3c15450_5b92_11d1_8f4e_00c04fb6809f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCallHub_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumCallingCard(::windows::core::IUnknown);
impl IEnumCallingCard {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITCallingCard>, pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumCallingCard> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumCallingCard, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumCallingCard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumCallingCard {
    type Vtable = IEnumCallingCard_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumCallingCard {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c4d8f02_8ddb_11d1_a09e_00805fc147d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCallingCard_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumDialableAddrs(::windows::core::IUnknown);
impl IEnumDialableAddrs {
    pub unsafe fn Next(&self, ppelements: &mut [::windows::core::BSTR], pcfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pcfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDialableAddrs> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumDialableAddrs, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumDialableAddrs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumDialableAddrs {
    type Vtable = IEnumDialableAddrs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumDialableAddrs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34621d70_6cff_11d1_aff7_00c04fc31fee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDialableAddrs_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumDirectory(::windows::core::IUnknown);
impl IEnumDirectory {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<ITDirectory>], pcfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pcfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDirectory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumDirectory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumDirectory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumDirectory {
    type Vtable = IEnumDirectory_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumDirectory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34621d6d_6cff_11d1_aff7_00c04fc31fee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDirectory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumDirectoryObject(::windows::core::IUnknown);
impl IEnumDirectoryObject {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, pval: &mut [::core::option::Option<ITDirectoryObject>], pcfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), pval.len() as _, ::core::mem::transmute(pval.as_ptr()), ::core::mem::transmute(pcfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDirectoryObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumDirectoryObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumDirectoryObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumDirectoryObject {
    type Vtable = IEnumDirectoryObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumDirectoryObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06c9b64a_306d_11d1_9774_00c04fd91ac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDirectoryObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pval: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumLocation(::windows::core::IUnknown);
impl IEnumLocation {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITLocationInfo>, pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumLocation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumLocation, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumLocation {
    type Vtable = IEnumLocation_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumLocation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c4d8f01_8ddb_11d1_a09e_00805fc147d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumLocation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumMcastScope(::windows::core::IUnknown);
impl IEnumMcastScope {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppscopes: *mut ::core::option::Option<IMcastScope>, pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppscopes), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumMcastScope> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumMcastScope, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumMcastScope {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumMcastScope {
    type Vtable = IEnumMcastScope_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumMcastScope {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0daf09_a289_11d1_8697_006008b0e5d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMcastScope_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppscopes: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumPhone(::windows::core::IUnknown);
impl IEnumPhone {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<ITPhone>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumPhone> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumPhone, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumPhone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumPhone {
    type Vtable = IEnumPhone_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumPhone {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf15b7669_4780_4595_8c89_fb369c8cf7aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPhone_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumPluggableSuperclassInfo(::windows::core::IUnknown);
impl IEnumPluggableSuperclassInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<ITPluggableTerminalSuperclassInfo>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumPluggableSuperclassInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumPluggableSuperclassInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumPluggableSuperclassInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumPluggableSuperclassInfo {
    type Vtable = IEnumPluggableSuperclassInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumPluggableSuperclassInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9586a80_89e6_4cff_931d_478d5751f4c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPluggableSuperclassInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumPluggableTerminalClassInfo(::windows::core::IUnknown);
impl IEnumPluggableTerminalClassInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [::core::option::Option<ITPluggableTerminalClassInfo>], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ppelements.len() as _, ::core::mem::transmute(ppelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumPluggableTerminalClassInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumPluggableTerminalClassInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumPluggableTerminalClassInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumPluggableTerminalClassInfo {
    type Vtable = IEnumPluggableTerminalClassInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumPluggableTerminalClassInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4567450c_dbee_4e3f_aaf5_37bf9ebf5e29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPluggableTerminalClassInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumQueue(::windows::core::IUnknown);
impl IEnumQueue {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITQueue>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), pceltfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumQueue, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumQueue {
    type Vtable = IEnumQueue_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumQueue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5afc3158_4bcc_11d1_bf80_00805fc147d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumQueue_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumStream(::windows::core::IUnknown);
impl IEnumStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITStream>, pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumStream, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumStream {
    type Vtable = IEnumStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee3bd606_3868_11d2_a045_00c04fb6809f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumSubStream(::windows::core::IUnknown);
impl IEnumSubStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITSubStream>, pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSubStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumSubStream, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumSubStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumSubStream {
    type Vtable = IEnumSubStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumSubStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee3bd609_3868_11d2_a045_00c04fb6809f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSubStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumTerminal(::windows::core::IUnknown);
impl IEnumTerminal {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITTerminal>, pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), celt, ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumTerminal, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumTerminal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumTerminal {
    type Vtable = IEnumTerminal_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumTerminal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae269cf4_935e_11d0_835c_00aa003ccabd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTerminal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct IEnumTerminalClass(::windows::core::IUnknown);
impl IEnumTerminalClass {
    pub unsafe fn Next(&self, pelements: &mut [::windows::core::GUID], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), pelements.len() as _, ::core::mem::transmute(pelements.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTerminalClass> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumTerminalClass, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumTerminalClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IEnumTerminalClass {
    type Vtable = IEnumTerminalClass_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumTerminalClass {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae269cf5_935e_11d0_835c_00aa003ccabd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTerminalClass_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pelements: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMcastAddressAllocation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMcastAddressAllocation {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Scopes(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Scopes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateScopes(&self) -> ::windows::core::Result<IEnumMcastScope> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateScopes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RequestAddress<P0>(&self, pscope: P0, leasestarttime: f64, leasestoptime: f64, numaddresses: i32) -> ::windows::core::Result<IMcastLeaseInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMcastScope>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAddress)(::windows::core::Vtable::as_raw(self), pscope.into().abi(), leasestarttime, leasestoptime, numaddresses, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RenewAddress<P0>(&self, lreserved: i32, prenewrequest: P0) -> ::windows::core::Result<IMcastLeaseInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMcastLeaseInfo>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RenewAddress)(::windows::core::Vtable::as_raw(self), lreserved, prenewrequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReleaseAddress<P0>(&self, preleaserequest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMcastLeaseInfo>>,
    {
        (::windows::core::Vtable::vtable(self).ReleaseAddress)(::windows::core::Vtable::as_raw(self), preleaserequest.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateLeaseInfo<P0, P1>(&self, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const ::windows::core::PCWSTR, prequestid: P0, pserveraddress: P1) -> ::windows::core::Result<IMcastLeaseInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLeaseInfo)(::windows::core::Vtable::as_raw(self), leasestarttime, leasestoptime, dwnumaddresses, ppaddresses, prequestid.into().abi(), pserveraddress.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateLeaseInfoFromVariant(&self, leasestarttime: f64, leasestoptime: f64, vaddresses: super::super::System::Com::VARIANT, prequestid: &::windows::core::BSTR, pserveraddress: &::windows::core::BSTR) -> ::windows::core::Result<IMcastLeaseInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLeaseInfoFromVariant)(::windows::core::Vtable::as_raw(self), leasestarttime, leasestoptime, ::core::mem::transmute(vaddresses), ::core::mem::transmute_copy(prequestid), ::core::mem::transmute_copy(pserveraddress), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IMcastAddressAllocation, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMcastAddressAllocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IMcastAddressAllocation {
    type Vtable = IMcastAddressAllocation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMcastAddressAllocation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0daef1_a289_11d1_8697_006008b0e5d2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMcastAddressAllocation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Scopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Scopes: usize,
    pub EnumerateScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenummcastscope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RequestAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscope: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, numaddresses: i32, ppleaseresponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RequestAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RenewAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lreserved: i32, prenewrequest: *mut ::core::ffi::c_void, pprenewresponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RenewAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReleaseAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preleaserequest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReleaseAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateLeaseInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const ::windows::core::PCWSTR, prequestid: ::windows::core::PCWSTR, pserveraddress: ::windows::core::PCWSTR, ppreleaserequest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateLeaseInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateLeaseInfoFromVariant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, vaddresses: super::super::System::Com::VARIANT, prequestid: *mut ::core::ffi::c_void, pserveraddress: *mut ::core::ffi::c_void, ppreleaserequest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateLeaseInfoFromVariant: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMcastLeaseInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMcastLeaseInfo {
    pub unsafe fn RequestID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LeaseStartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LeaseStartTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLeaseStartTime(&self, time: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLeaseStartTime)(::windows::core::Vtable::as_raw(self), time).ok()
    }
    pub unsafe fn LeaseStopTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LeaseStopTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLeaseStopTime(&self, time: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLeaseStopTime)(::windows::core::Vtable::as_raw(self), time).ok()
    }
    pub unsafe fn AddressCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddressCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ServerAddress(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServerAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TTL(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TTL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Addresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Addresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateAddresses(&self) -> ::windows::core::Result<IEnumBstr> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IMcastLeaseInfo, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMcastLeaseInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IMcastLeaseInfo {
    type Vtable = IMcastLeaseInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMcastLeaseInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0daefd_a289_11d1_8697_006008b0e5d2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMcastLeaseInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub RequestID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprequestid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LeaseStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT,
    pub SetLeaseStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT,
    pub LeaseStopTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT,
    pub SetLeaseStopTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT,
    pub AddressCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub ServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Addresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Addresses: usize,
    pub EnumerateAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumaddresses: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMcastScope(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMcastScope {
    pub unsafe fn ScopeID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ScopeID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ServerID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServerID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InterfaceID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ScopeDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ScopeDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TTL(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TTL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IMcastScope, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMcastScope {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IMcastScope {
    type Vtable = IMcastScope_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMcastScope {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0daef4_a289_11d1_8697_006008b0e5d2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMcastScope_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ScopeID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT,
    pub ServerID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT,
    pub ScopeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITACDGroup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITACDGroup {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateQueues(&self) -> ::windows::core::Result<IEnumQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateQueues)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Queues(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Queues)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITACDGroup, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITACDGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITACDGroup {
    type Vtable = ITACDGroup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITACDGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5afc3148_4bcc_11d1_bf80_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITACDGroup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumerateQueues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Queues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Queues: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITACDGroupEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITACDGroupEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Group(&self) -> ::windows::core::Result<ITACDGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Group)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<ACDGROUP_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITACDGroupEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITACDGroupEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITACDGroupEvent {
    type Vtable = ITACDGroupEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITACDGroupEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x297f3032_bd11_11d1_a0a7_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITACDGroupEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Group: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut ACDGROUP_EVENT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct ITAMMediaFormat(::windows::core::IUnknown);
impl ITAMMediaFormat {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn MediaFormat(&self) -> ::windows::core::Result<*mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MediaFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetMediaFormat(&self, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMediaFormat)(::windows::core::Vtable::as_raw(self), pmt).ok()
    }
}
::windows::core::interface_hierarchy!(ITAMMediaFormat, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITAMMediaFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITAMMediaFormat {
    type Vtable = ITAMMediaFormat_Vtbl;
}
unsafe impl ::windows::core::Interface for ITAMMediaFormat {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0364eb00_4a77_11d1_a671_006097c9a2e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITAMMediaFormat_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub MediaFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    MediaFormat: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub SetMediaFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    SetMediaFormat: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITASRTerminalEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITASRTerminalEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Terminal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITASRTerminalEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITASRTerminalEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITASRTerminalEvent {
    type Vtable = ITASRTerminalEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITASRTerminalEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee016a02_4fa9_467c_933f_5a15b12377d7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITASRTerminalEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAddress(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAddress {
    pub unsafe fn State(&self) -> ::windows::core::Result<ADDRESS_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddressName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddressName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ServiceProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ServiceProviderName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TAPIObject(&self) -> ::windows::core::Result<ITTAPI> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TAPIObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateCall(&self, pdestaddress: &::windows::core::BSTR, laddresstype: i32, lmediatypes: i32) -> ::windows::core::Result<ITBasicCallControl> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCall)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdestaddress), laddresstype, lmediatypes, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Calls(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Calls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateCalls(&self) -> ::windows::core::Result<IEnumCall> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateCalls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DialableAddress(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DialableAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateForwardInfoObject(&self) -> ::windows::core::Result<ITForwardInformation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateForwardInfoObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Forward<P0, P1>(&self, pforwardinfo: P0, pcall: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITForwardInformation>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITBasicCallControl>>,
    {
        (::windows::core::Vtable::vtable(self).Forward)(::windows::core::Vtable::as_raw(self), pforwardinfo.into().abi(), pcall.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentForwardInfo(&self) -> ::windows::core::Result<ITForwardInformation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentForwardInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageWaiting<P0>(&self, fmessagewaiting: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetMessageWaiting)(::windows::core::Vtable::as_raw(self), fmessagewaiting.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageWaiting(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MessageWaiting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDoNotDisturb<P0>(&self, fdonotdisturb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetDoNotDisturb)(::windows::core::Vtable::as_raw(self), fdonotdisturb.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoNotDisturb(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DoNotDisturb)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAddress, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAddress {
    type Vtable = ITAddress_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1efc386_9355_11d0_835c_00aa003ccabd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddressstate: *mut ADDRESS_STATE) -> ::windows::core::HRESULT,
    pub AddressName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ServiceProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TAPIObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptapiobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TAPIObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestaddress: *mut ::core::ffi::c_void, laddresstype: i32, lmediatypes: i32, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateCall: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Calls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Calls: usize,
    pub EnumerateCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DialableAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdialableaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateForwardInfoObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateForwardInfoObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Forward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pforwardinfo: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Forward: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentForwardInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentForwardInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMessageWaiting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmessagewaiting: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMessageWaiting: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MessageWaiting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfmessagewaiting: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MessageWaiting: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDoNotDisturb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdonotdisturb: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDoNotDisturb: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DoNotDisturb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfdonotdisturb: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoNotDisturb: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAddress2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAddress2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Phones(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Phones)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumeratePhones(&self) -> ::windows::core::Result<IEnumPhone> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumeratePhones)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPhoneFromTerminal<P0>(&self, pterminal: P0) -> ::windows::core::Result<ITPhone>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITTerminal>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPhoneFromTerminal)(::windows::core::Vtable::as_raw(self), pterminal.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PreferredPhones(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PreferredPhones)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumeratePreferredPhones(&self) -> ::windows::core::Result<IEnumPhone> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumeratePreferredPhones)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_EventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_EventFilter)(::windows::core::Vtable::as_raw(self), tapievent, lsubevent, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn put_EventFilter<P0>(&self, tapievent: TAPI_EVENT, lsubevent: i32, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).put_EventFilter)(::windows::core::Vtable::as_raw(self), tapievent, lsubevent, benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSpecific<P0>(&self, pcall: P0, pparams: *const u8, dwsize: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITCallInfo>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceSpecific)(::windows::core::Vtable::as_raw(self), pcall.into().abi(), pparams, dwsize).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeviceSpecificVariant<P0>(&self, pcall: P0, vardevspecificbytearray: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITCallInfo>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceSpecificVariant)(::windows::core::Vtable::as_raw(self), pcall.into().abi(), ::core::mem::transmute(vardevspecificbytearray)).ok()
    }
    pub unsafe fn NegotiateExtVersion(&self, llowversion: i32, lhighversion: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NegotiateExtVersion)(::windows::core::Vtable::as_raw(self), llowversion, lhighversion, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAddress2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ITAddress);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAddress2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAddress2 {
    type Vtable = ITAddress2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAddress2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0ae5d9b_be51_46c9_b0f7_dfa8a22a8bc4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAddress2_Vtbl {
    pub base__: ITAddress_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Phones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Phones: usize,
    pub EnumeratePhones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumphone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPhoneFromTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPhoneFromTerminal: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PreferredPhones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PreferredPhones: usize,
    pub EnumeratePreferredPhones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumphone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_EventFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_EventFilter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_EventFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_EventFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceSpecific: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeviceSpecificVariant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void, vardevspecificbytearray: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeviceSpecificVariant: usize,
    pub NegotiateExtVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAddressCapabilities(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAddressCapabilities {
    pub unsafe fn get_AddressCapability(&self, addresscap: ADDRESS_CAPABILITY) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_AddressCapability)(::windows::core::Vtable::as_raw(self), addresscap, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_AddressCapabilityString(&self, addresscapstring: ADDRESS_CAPABILITY_STRING) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_AddressCapabilityString)(::windows::core::Vtable::as_raw(self), addresscapstring, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CallTreatments(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallTreatments)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateCallTreatments(&self) -> ::windows::core::Result<IEnumBstr> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateCallTreatments)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CompletionMessages(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CompletionMessages)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateCompletionMessages(&self) -> ::windows::core::Result<IEnumBstr> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateCompletionMessages)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeviceClasses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DeviceClasses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateDeviceClasses(&self) -> ::windows::core::Result<IEnumBstr> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateDeviceClasses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAddressCapabilities, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAddressCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAddressCapabilities {
    type Vtable = ITAddressCapabilities_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAddressCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8df232f5_821b_11d1_bb5c_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAddressCapabilities_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_AddressCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, addresscap: ADDRESS_CAPABILITY, plcapability: *mut i32) -> ::windows::core::HRESULT,
    pub get_AddressCapabilityString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, addresscapstring: ADDRESS_CAPABILITY_STRING, ppcapabilitystring: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CallTreatments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CallTreatments: usize,
    pub EnumerateCallTreatments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumcalltreatment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CompletionMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CompletionMessages: usize,
    pub EnumerateCompletionMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumcompletionmessage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeviceClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeviceClasses: usize,
    pub EnumerateDeviceClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdeviceclass: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAddressDeviceSpecificEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAddressDeviceSpecificEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<ITAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Address)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn lParam1(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).lParam1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn lParam2(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).lParam2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn lParam3(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).lParam3)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAddressDeviceSpecificEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAddressDeviceSpecificEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAddressDeviceSpecificEvent {
    type Vtable = ITAddressDeviceSpecificEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAddressDeviceSpecificEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3acb216b_40bd_487a_8672_5ce77bd7e3a3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAddressDeviceSpecificEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub lParam1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT,
    pub lParam2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT,
    pub lParam3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAddressEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAddressEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<ITAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Address)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<ADDRESS_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Terminal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAddressEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAddressEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAddressEvent {
    type Vtable = ITAddressEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAddressEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x831ce2d1_83b5_11d1_bb5c_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAddressEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut ADDRESS_EVENT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAddressTranslation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAddressTranslation {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TranslateAddress(&self, paddresstotranslate: &::windows::core::BSTR, lcard: i32, ltranslateoptions: i32) -> ::windows::core::Result<ITAddressTranslationInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TranslateAddress)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(paddresstotranslate), lcard, ltranslateoptions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TranslateDialog(&self, hwndowner: isize, paddressin: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TranslateDialog)(::windows::core::Vtable::as_raw(self), hwndowner, ::core::mem::transmute_copy(paddressin)).ok()
    }
    pub unsafe fn EnumerateLocations(&self) -> ::windows::core::Result<IEnumLocation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateLocations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Locations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Locations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateCallingCards(&self) -> ::windows::core::Result<IEnumCallingCard> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateCallingCards)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CallingCards(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallingCards)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAddressTranslation, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAddressTranslation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAddressTranslation {
    type Vtable = ITAddressTranslation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAddressTranslation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c4d8f03_8ddb_11d1_a09e_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAddressTranslation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub TranslateAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddresstotranslate: *mut ::core::ffi::c_void, lcard: i32, ltranslateoptions: i32, pptranslated: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TranslateAddress: usize,
    pub TranslateDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndowner: isize, paddressin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumerateLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumlocation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Locations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Locations: usize,
    pub EnumerateCallingCards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumcallingcard: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CallingCards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CallingCards: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAddressTranslationInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAddressTranslationInfo {
    pub unsafe fn DialableString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DialableString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayableString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DisplayableString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCountryCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCountryCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DestinationCountryCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DestinationCountryCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TranslationResults(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TranslationResults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAddressTranslationInfo, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAddressTranslationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAddressTranslationInfo {
    type Vtable = ITAddressTranslationInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAddressTranslationInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafc15945_8d40_11d1_a09e_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAddressTranslationInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DialableString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdialablestring: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayableString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdisplayablestring: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentCountryCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT,
    pub DestinationCountryCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT,
    pub TranslationResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plresults: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAgent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAgent {
    pub unsafe fn EnumerateAgentSessions(&self) -> ::windows::core::Result<IEnumAgentSession> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateAgentSessions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSession<P0, P1>(&self, pacdgroup: P0, paddress: P1) -> ::windows::core::Result<ITAgentSession>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITACDGroup>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITAddress>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSession)(::windows::core::Vtable::as_raw(self), pacdgroup.into().abi(), paddress.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSessionWithPIN<P0, P1>(&self, pacdgroup: P0, paddress: P1, ppin: &::windows::core::BSTR) -> ::windows::core::Result<ITAgentSession>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITACDGroup>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITAddress>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSessionWithPIN)(::windows::core::Vtable::as_raw(self), pacdgroup.into().abi(), paddress.into().abi(), ::core::mem::transmute_copy(ppin), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn User(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).User)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetState(&self, agentstate: AGENT_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetState)(::windows::core::Vtable::as_raw(self), agentstate).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<AGENT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMeasurementPeriod(&self, lperiod: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMeasurementPeriod)(::windows::core::Vtable::as_raw(self), lperiod).ok()
    }
    pub unsafe fn MeasurementPeriod(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MeasurementPeriod)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OverallCallRate(&self) -> ::windows::core::Result<super::super::System::Com::CY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OverallCallRate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NumberOfACDCalls(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NumberOfACDCalls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NumberOfIncomingCalls(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NumberOfIncomingCalls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NumberOfOutgoingCalls(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NumberOfOutgoingCalls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalACDTalkTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalACDTalkTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalACDCallTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalACDCallTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalWrapUpTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalWrapUpTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AgentSessions(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AgentSessions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAgent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAgent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAgent {
    type Vtable = ITAgent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAgent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5770ece5_4b27_11d1_bf80_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAgent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EnumerateAgentSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumagentsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pacdgroup: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, ppagentsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSession: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSessionWithPIN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pacdgroup: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, ppin: *mut ::core::ffi::c_void, ppagentsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSessionWithPIN: usize,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, agentstate: AGENT_STATE) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagentstate: *mut AGENT_STATE) -> ::windows::core::HRESULT,
    pub SetMeasurementPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT,
    pub MeasurementPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OverallCallRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OverallCallRate: usize,
    pub NumberOfACDCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfIncomingCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfOutgoingCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub TotalACDTalkTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT,
    pub TotalACDCallTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT,
    pub TotalWrapUpTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AgentSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AgentSessions: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAgentEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAgentEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Agent(&self) -> ::windows::core::Result<ITAgent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Agent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<AGENT_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAgentEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAgentEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAgentEvent {
    type Vtable = ITAgentEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAgentEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5afc314a_4bcc_11d1_bf80_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAgentEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Agent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Agent: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_EVENT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAgentHandler(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAgentHandler {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAgent(&self) -> ::windows::core::Result<ITAgent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAgent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAgentWithID(&self, pid: &::windows::core::BSTR, ppin: &::windows::core::BSTR) -> ::windows::core::Result<ITAgent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAgentWithID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pid), ::core::mem::transmute_copy(ppin), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateACDGroups(&self) -> ::windows::core::Result<IEnumACDGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateACDGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateUsableAddresses(&self) -> ::windows::core::Result<IEnumAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateUsableAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ACDGroups(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ACDGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn UsableAddresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UsableAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAgentHandler, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAgentHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAgentHandler {
    type Vtable = ITAgentHandler_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAgentHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x587e8c22_9802_11d1_a0a4_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAgentHandler_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAgent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAgent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAgentWithID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut ::core::ffi::c_void, ppin: *mut ::core::ffi::c_void, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAgentWithID: usize,
    pub EnumerateACDGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumacdgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumerateUsableAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ACDGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ACDGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UsableAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UsableAddresses: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAgentHandlerEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAgentHandlerEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AgentHandler(&self) -> ::windows::core::Result<ITAgentHandler> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AgentHandler)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<AGENTHANDLER_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAgentHandlerEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAgentHandlerEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAgentHandlerEvent {
    type Vtable = ITAgentHandlerEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAgentHandlerEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x297f3034_bd11_11d1_a0a7_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAgentHandlerEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AgentHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppagenthandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AgentHandler: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut AGENTHANDLER_EVENT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAgentSession(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAgentSession {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Agent(&self) -> ::windows::core::Result<ITAgent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Agent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<ITAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Address)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ACDGroup(&self) -> ::windows::core::Result<ITACDGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ACDGroup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetState(&self, sessionstate: AGENT_SESSION_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetState)(::windows::core::Vtable::as_raw(self), sessionstate).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<AGENT_SESSION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionStartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SessionStartTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionDuration(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SessionDuration)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NumberOfCalls(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NumberOfCalls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalTalkTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalTalkTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AverageTalkTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AverageTalkTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalCallTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalCallTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AverageCallTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AverageCallTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalWrapUpTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalWrapUpTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AverageWrapUpTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AverageWrapUpTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ACDCallRate(&self) -> ::windows::core::Result<super::super::System::Com::CY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ACDCallRate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LongestTimeToAnswer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LongestTimeToAnswer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AverageTimeToAnswer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AverageTimeToAnswer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAgentSession, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAgentSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAgentSession {
    type Vtable = ITAgentSession_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAgentSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5afc3147_4bcc_11d1_bf80_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAgentSession_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Agent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Agent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ACDGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppacdgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ACDGroup: usize,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionstate: AGENT_SESSION_STATE) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessionstate: *mut AGENT_SESSION_STATE) -> ::windows::core::HRESULT,
    pub SessionStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatesessionstart: *mut f64) -> ::windows::core::HRESULT,
    pub SessionDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub TotalTalkTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT,
    pub AverageTalkTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT,
    pub TotalCallTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT,
    pub AverageCallTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT,
    pub TotalWrapUpTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT,
    pub AverageWrapUpTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ACDCallRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ACDCallRate: usize,
    pub LongestTimeToAnswer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT,
    pub AverageTimeToAnswer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAgentSessionEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAgentSessionEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Session(&self) -> ::windows::core::Result<ITAgentSession> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Session)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<AGENT_SESSION_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAgentSessionEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAgentSessionEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAgentSessionEvent {
    type Vtable = ITAgentSessionEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAgentSessionEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5afc314b_4bcc_11d1_bf80_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAgentSessionEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Session: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_SESSION_EVENT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct ITAllocatorProperties(::windows::core::IUnknown);
impl ITAllocatorProperties {
    #[doc = "*Required features: `\"Win32_Media_DirectShow\"`*"]
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn SetAllocatorProperties(&self, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAllocatorProperties)(::windows::core::Vtable::as_raw(self), pallocproperties).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DirectShow\"`*"]
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn GetAllocatorProperties(&self) -> ::windows::core::Result<super::super::Media::DirectShow::ALLOCATOR_PROPERTIES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAllocatorProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateBuffers<P0>(&self, ballocbuffers: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAllocateBuffers)(::windows::core::Vtable::as_raw(self), ballocbuffers.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateBuffers(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAllocateBuffers)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBufferSize(&self, buffersize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBufferSize)(::windows::core::Vtable::as_raw(self), buffersize).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBufferSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITAllocatorProperties, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITAllocatorProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITAllocatorProperties {
    type Vtable = ITAllocatorProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for ITAllocatorProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1bc3c90_bcfe_11d1_9745_00c04fd91ac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITAllocatorProperties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub SetAllocatorProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    SetAllocatorProperties: usize,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub GetAllocatorProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pallocproperties: *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    GetAllocatorProperties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ballocbuffers: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateBuffers: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pballocbuffers: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateBuffers: usize,
    pub SetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::HRESULT,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffersize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITAutomatedPhoneControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITAutomatedPhoneControl {
    pub unsafe fn StartTone(&self, tone: PHONE_TONE, lduration: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StartTone)(::windows::core::Vtable::as_raw(self), tone, lduration).ok()
    }
    pub unsafe fn StopTone(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StopTone)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Tone(&self) -> ::windows::core::Result<PHONE_TONE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Tone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StartRinger(&self, lringmode: i32, lduration: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StartRinger)(::windows::core::Vtable::as_raw(self), lringmode, lduration).ok()
    }
    pub unsafe fn StopRinger(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StopRinger)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Ringer(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Ringer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPhoneHandlingEnabled<P0>(&self, fenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetPhoneHandlingEnabled)(::windows::core::Vtable::as_raw(self), fenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PhoneHandlingEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PhoneHandlingEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAutoEndOfNumberTimeout(&self, ltimeout: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAutoEndOfNumberTimeout)(::windows::core::Vtable::as_raw(self), ltimeout).ok()
    }
    pub unsafe fn AutoEndOfNumberTimeout(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoEndOfNumberTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoDialtone<P0>(&self, fenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAutoDialtone)(::windows::core::Vtable::as_raw(self), fenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoDialtone(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoDialtone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoStopTonesOnOnHook<P0>(&self, fenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAutoStopTonesOnOnHook)(::windows::core::Vtable::as_raw(self), fenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoStopTonesOnOnHook(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoStopTonesOnOnHook)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoStopRingOnOffHook<P0>(&self, fenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAutoStopRingOnOffHook)(::windows::core::Vtable::as_raw(self), fenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoStopRingOnOffHook(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoStopRingOnOffHook)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoKeypadTones<P0>(&self, fenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAutoKeypadTones)(::windows::core::Vtable::as_raw(self), fenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoKeypadTones(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoKeypadTones)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAutoKeypadTonesMinimumDuration(&self, lduration: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAutoKeypadTonesMinimumDuration)(::windows::core::Vtable::as_raw(self), lduration).ok()
    }
    pub unsafe fn AutoKeypadTonesMinimumDuration(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoKeypadTonesMinimumDuration)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoVolumeControl<P0>(&self, fenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAutoVolumeControl)(::windows::core::Vtable::as_raw(self), fenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoVolumeControl(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoVolumeControl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAutoVolumeControlStep(&self, lstepsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAutoVolumeControlStep)(::windows::core::Vtable::as_raw(self), lstepsize).ok()
    }
    pub unsafe fn AutoVolumeControlStep(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoVolumeControlStep)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAutoVolumeControlRepeatDelay(&self, ldelay: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAutoVolumeControlRepeatDelay)(::windows::core::Vtable::as_raw(self), ldelay).ok()
    }
    pub unsafe fn AutoVolumeControlRepeatDelay(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoVolumeControlRepeatDelay)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAutoVolumeControlRepeatPeriod(&self, lperiod: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAutoVolumeControlRepeatPeriod)(::windows::core::Vtable::as_raw(self), lperiod).ok()
    }
    pub unsafe fn AutoVolumeControlRepeatPeriod(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoVolumeControlRepeatPeriod)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SelectCall<P0, P1>(&self, pcall: P0, fselectdefaultterminals: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITCallInfo>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SelectCall)(::windows::core::Vtable::as_raw(self), pcall.into().abi(), fselectdefaultterminals.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnselectCall<P0>(&self, pcall: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITCallInfo>>,
    {
        (::windows::core::Vtable::vtable(self).UnselectCall)(::windows::core::Vtable::as_raw(self), pcall.into().abi()).ok()
    }
    pub unsafe fn EnumerateSelectedCalls(&self) -> ::windows::core::Result<IEnumCall> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateSelectedCalls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SelectedCalls(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SelectedCalls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITAutomatedPhoneControl, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITAutomatedPhoneControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITAutomatedPhoneControl {
    type Vtable = ITAutomatedPhoneControl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITAutomatedPhoneControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee1af0e_6159_4a61_b79b_6a4ba3fc9dfc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITAutomatedPhoneControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartTone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tone: PHONE_TONE, lduration: i32) -> ::windows::core::HRESULT,
    pub StopTone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Tone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptone: *mut PHONE_TONE) -> ::windows::core::HRESULT,
    pub StartRinger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lringmode: i32, lduration: i32) -> ::windows::core::HRESULT,
    pub StopRinger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Ringer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfringing: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Ringer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPhoneHandlingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPhoneHandlingEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PhoneHandlingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PhoneHandlingEnabled: usize,
    pub SetAutoEndOfNumberTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32) -> ::windows::core::HRESULT,
    pub AutoEndOfNumberTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltimeout: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoDialtone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoDialtone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AutoDialtone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AutoDialtone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoStopTonesOnOnHook: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoStopTonesOnOnHook: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AutoStopTonesOnOnHook: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AutoStopTonesOnOnHook: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoStopRingOnOffHook: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoStopRingOnOffHook: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AutoStopRingOnOffHook: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AutoStopRingOnOffHook: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoKeypadTones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoKeypadTones: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AutoKeypadTones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AutoKeypadTones: usize,
    pub SetAutoKeypadTonesMinimumDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT,
    pub AutoKeypadTonesMinimumDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoVolumeControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoVolumeControl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AutoVolumeControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AutoVolumeControl: usize,
    pub SetAutoVolumeControlStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstepsize: i32) -> ::windows::core::HRESULT,
    pub AutoVolumeControlStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstepsize: *mut i32) -> ::windows::core::HRESULT,
    pub SetAutoVolumeControlRepeatDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldelay: i32) -> ::windows::core::HRESULT,
    pub AutoVolumeControlRepeatDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldelay: *mut i32) -> ::windows::core::HRESULT,
    pub SetAutoVolumeControlRepeatPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT,
    pub AutoVolumeControlRepeatPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SelectCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void, fselectdefaultterminals: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SelectCall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectCall: usize,
    pub EnumerateSelectedCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelectedCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelectedCalls: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITBasicAudioTerminal(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITBasicAudioTerminal {
    pub unsafe fn SetVolume(&self, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVolume)(::windows::core::Vtable::as_raw(self), lvolume).ok()
    }
    pub unsafe fn Volume(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Volume)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBalance(&self, lbalance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBalance)(::windows::core::Vtable::as_raw(self), lbalance).ok()
    }
    pub unsafe fn Balance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Balance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITBasicAudioTerminal, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITBasicAudioTerminal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITBasicAudioTerminal {
    type Vtable = ITBasicAudioTerminal_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITBasicAudioTerminal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1efc38d_9355_11d0_835c_00aa003ccabd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITBasicAudioTerminal_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT,
    pub SetBalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT,
    pub Balance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITBasicCallControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITBasicCallControl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<P0>(&self, fsync: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Connect)(::windows::core::Vtable::as_raw(self), fsync.into()).ok()
    }
    pub unsafe fn Answer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Answer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self, code: DISCONNECT_CODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Disconnect)(::windows::core::Vtable::as_raw(self), code).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Hold<P0>(&self, fhold: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Hold)(::windows::core::Vtable::as_raw(self), fhold.into()).ok()
    }
    pub unsafe fn HandoffDirect(&self, papplicationname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).HandoffDirect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(papplicationname)).ok()
    }
    pub unsafe fn HandoffIndirect(&self, lmediatype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).HandoffIndirect)(::windows::core::Vtable::as_raw(self), lmediatype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Conference<P0, P1>(&self, pcall: P0, fsync: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITBasicCallControl>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Conference)(::windows::core::Vtable::as_raw(self), pcall.into().abi(), fsync.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Transfer<P0, P1>(&self, pcall: P0, fsync: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITBasicCallControl>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Transfer)(::windows::core::Vtable::as_raw(self), pcall.into().abi(), fsync.into()).ok()
    }
    pub unsafe fn BlindTransfer(&self, pdestaddress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).BlindTransfer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdestaddress)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SwapHold<P0>(&self, pcall: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITBasicCallControl>>,
    {
        (::windows::core::Vtable::vtable(self).SwapHold)(::windows::core::Vtable::as_raw(self), pcall.into().abi()).ok()
    }
    pub unsafe fn ParkDirect(&self, pparkaddress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ParkDirect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pparkaddress)).ok()
    }
    pub unsafe fn ParkIndirect(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ParkIndirect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unpark(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unpark)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetQOS(&self, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetQOS)(::windows::core::Vtable::as_raw(self), lmediatype, servicelevel).ok()
    }
    pub unsafe fn Pickup(&self, pgroupid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pickup)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pgroupid)).ok()
    }
    pub unsafe fn Dial(&self, pdestaddress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Dial)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdestaddress)).ok()
    }
    pub unsafe fn Finish(&self, finishmode: FINISH_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish)(::windows::core::Vtable::as_raw(self), finishmode).ok()
    }
    pub unsafe fn RemoveFromConference(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveFromConference)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITBasicCallControl, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITBasicCallControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITBasicCallControl {
    type Vtable = ITBasicCallControl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITBasicCallControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1efc389_9355_11d0_835c_00aa003ccabd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITBasicCallControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    pub Answer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, code: DISCONNECT_CODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Hold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fhold: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Hold: usize,
    pub HandoffDirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papplicationname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HandoffIndirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Conference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Conference: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Transfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Transfer: usize,
    pub BlindTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestaddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SwapHold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SwapHold: usize,
    pub ParkDirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparkaddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ParkIndirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnondiraddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unpark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetQOS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::HRESULT,
    pub Pickup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroupid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Dial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestaddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finishmode: FINISH_MODE) -> ::windows::core::HRESULT,
    pub RemoveFromConference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITBasicCallControl2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITBasicCallControl2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RequestTerminal(&self, bstrterminalclassguid: &::windows::core::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestTerminal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrterminalclassguid), lmediatype, direction, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectTerminalOnCall<P0>(&self, pterminal: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITTerminal>>,
    {
        (::windows::core::Vtable::vtable(self).SelectTerminalOnCall)(::windows::core::Vtable::as_raw(self), pterminal.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnselectTerminalOnCall<P0>(&self, pterminal: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITTerminal>>,
    {
        (::windows::core::Vtable::vtable(self).UnselectTerminalOnCall)(::windows::core::Vtable::as_raw(self), pterminal.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITBasicCallControl2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ITBasicCallControl);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITBasicCallControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITBasicCallControl2 {
    type Vtable = ITBasicCallControl2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITBasicCallControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x161a4a56_1e99_4b3f_a46a_168f38a5ee4c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITBasicCallControl2_Vtbl {
    pub base__: ITBasicCallControl_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RequestTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrterminalclassguid: *mut ::core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RequestTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectTerminalOnCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectTerminalOnCall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectTerminalOnCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectTerminalOnCall: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCallHub(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCallHub {
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EnumerateCalls(&self) -> ::windows::core::Result<IEnumCall> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateCalls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Calls(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Calls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NumCalls(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NumCalls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<CALLHUB_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCallHub, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCallHub {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCallHub {
    type Vtable = ITCallHub_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCallHub {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3c1544e_5b92_11d1_8f4e_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCallHub_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumerateCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Calls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcalls: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Calls: usize,
    pub NumCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut CALLHUB_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCallHubEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCallHubEvent {
    pub unsafe fn Event(&self) -> ::windows::core::Result<CALLHUB_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CallHub(&self) -> ::windows::core::Result<ITCallHub> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallHub)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCallHubEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCallHubEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCallHubEvent {
    type Vtable = ITCallHubEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCallHubEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3c15451_5b92_11d1_8f4e_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCallHubEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut CALLHUB_EVENT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CallHub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallhub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CallHub: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCallInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<ITAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Address)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallState(&self) -> ::windows::core::Result<CALL_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Privilege(&self) -> ::windows::core::Result<CALL_PRIVILEGE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Privilege)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CallHub(&self) -> ::windows::core::Result<ITCallHub> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallHub)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_CallInfoLong(&self, callinfolong: CALLINFO_LONG) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_CallInfoLong)(::windows::core::Vtable::as_raw(self), callinfolong, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_CallInfoLong(&self, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_CallInfoLong)(::windows::core::Vtable::as_raw(self), callinfolong, lcallinfolongval).ok()
    }
    pub unsafe fn get_CallInfoString(&self, callinfostring: CALLINFO_STRING) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_CallInfoString)(::windows::core::Vtable::as_raw(self), callinfostring, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_CallInfoString(&self, callinfostring: CALLINFO_STRING, pcallinfostring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_CallInfoString)(::windows::core::Vtable::as_raw(self), callinfostring, ::core::mem::transmute_copy(pcallinfostring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_CallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_CallInfoBuffer)(::windows::core::Vtable::as_raw(self), callinfobuffer, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn put_CallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_CallInfoBuffer)(::windows::core::Vtable::as_raw(self), callinfobuffer, ::core::mem::transmute(pcallinfobuffer)).ok()
    }
    pub unsafe fn GetCallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCallInfoBuffer)(::windows::core::Vtable::as_raw(self), callinfobuffer, pdwsize, ppcallinfobuffer).ok()
    }
    pub unsafe fn SetCallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCallInfoBuffer)(::windows::core::Vtable::as_raw(self), callinfobuffer, pcallinfobuffer.len() as _, ::core::mem::transmute(pcallinfobuffer.as_ptr())).ok()
    }
    pub unsafe fn ReleaseUserUserInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReleaseUserUserInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCallInfo, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCallInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCallInfo {
    type Vtable = ITCallInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCallInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x350f85d1_1227_11d3_83d4_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCallInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    pub CallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT,
    pub Privilege: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprivilege: *mut CALL_PRIVILEGE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CallHub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallhub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CallHub: usize,
    pub get_CallInfoLong: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, plcallinfolongval: *mut i32) -> ::windows::core::HRESULT,
    pub put_CallInfoLong: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::HRESULT,
    pub get_CallInfoString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, ppcallinfostring: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub put_CallInfoString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, pcallinfostring: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_CallInfoBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, ppcallinfobuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_CallInfoBuffer: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_CallInfoBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_CallInfoBuffer: usize,
    pub GetCallInfoBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetCallInfoBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows::core::HRESULT,
    pub ReleaseUserUserInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCallInfo2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfo2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_EventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_EventFilter)(::windows::core::Vtable::as_raw(self), tapievent, lsubevent, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn put_EventFilter<P0>(&self, tapievent: TAPI_EVENT, lsubevent: i32, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).put_EventFilter)(::windows::core::Vtable::as_raw(self), tapievent, lsubevent, benable.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCallInfo2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ITCallInfo);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCallInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCallInfo2 {
    type Vtable = ITCallInfo2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCallInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94d70ca6_7ab0_4daa_81ca_b8f8643faec1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCallInfo2_Vtbl {
    pub base__: ITCallInfo_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_EventFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_EventFilter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_EventFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_EventFilter: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCallInfoChangeEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfoChangeEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cause(&self) -> ::windows::core::Result<CALLINFOCHANGE_CAUSE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cause)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallbackInstance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallbackInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCallInfoChangeEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCallInfoChangeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCallInfoChangeEvent {
    type Vtable = ITCallInfoChangeEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCallInfoChangeEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d4b65f9_e51c_11d1_a02f_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCallInfoChangeEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Cause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcic: *mut CALLINFOCHANGE_CAUSE) -> ::windows::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCallMediaEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCallMediaEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<CALL_MEDIA_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Terminal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Stream(&self) -> ::windows::core::Result<ITStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Stream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cause(&self) -> ::windows::core::Result<CALL_MEDIA_EVENT_CAUSE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cause)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCallMediaEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCallMediaEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCallMediaEvent {
    type Vtable = ITCallMediaEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCallMediaEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff36b87f_ec3a_11d0_8ee4_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCallMediaEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallmediaevent: *mut CALL_MEDIA_EVENT) -> ::windows::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Stream: usize,
    pub Cause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcause: *mut CALL_MEDIA_EVENT_CAUSE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCallNotificationEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCallNotificationEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<CALL_NOTIFICATION_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallbackInstance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallbackInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCallNotificationEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCallNotificationEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCallNotificationEvent {
    type Vtable = ITCallNotificationEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCallNotificationEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x895801df_3dd6_11d1_8f30_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCallNotificationEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallnotificationevent: *mut CALL_NOTIFICATION_EVENT) -> ::windows::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCallStateEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCallStateEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<CALL_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cause(&self) -> ::windows::core::Result<CALL_STATE_EVENT_CAUSE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cause)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallbackInstance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallbackInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCallStateEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCallStateEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCallStateEvent {
    type Vtable = ITCallStateEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCallStateEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62f47097_95c9_11d0_835d_00aa003ccabd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCallStateEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT,
    pub Cause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcec: *mut CALL_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCallingCard(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCallingCard {
    pub unsafe fn PermanentCardID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PermanentCardID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NumberOfDigits(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NumberOfDigits)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Options(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Options)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CardName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CardName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SameAreaDialingRule(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SameAreaDialingRule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LongDistanceDialingRule(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LongDistanceDialingRule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InternationalDialingRule(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InternationalDialingRule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCallingCard, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCallingCard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCallingCard {
    type Vtable = ITCallingCard_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCallingCard {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c4d8f00_8ddb_11d1_a09e_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCallingCard_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub PermanentCardID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldigits: *mut i32) -> ::windows::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT,
    pub CardName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcardname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SameAreaDialingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LongDistanceDialingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InternationalDialingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCollection {
    type Vtable = ITCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ec5acf2_9c02_11d0_8362_00aa003ccabd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCollection2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCollection2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add(&self, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), index, pvariant).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), index).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCollection2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ITCollection);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCollection2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCollection2 {
    type Vtable = ITCollection2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCollection2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6dddda5_a6d3_48ff_8737_d32fc4d95477);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCollection2_Vtbl {
    pub base__: ITCollection_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITCustomTone(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITCustomTone {
    pub unsafe fn Frequency(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Frequency)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFrequency(&self, lfrequency: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFrequency)(::windows::core::Vtable::as_raw(self), lfrequency).ok()
    }
    pub unsafe fn CadenceOn(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CadenceOn)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCadenceOn(&self, cadenceon: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCadenceOn)(::windows::core::Vtable::as_raw(self), cadenceon).ok()
    }
    pub unsafe fn CadenceOff(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CadenceOff)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCadenceOff(&self, lcadenceoff: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCadenceOff)(::windows::core::Vtable::as_raw(self), lcadenceoff).ok()
    }
    pub unsafe fn Volume(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Volume)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVolume(&self, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVolume)(::windows::core::Vtable::as_raw(self), lvolume).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITCustomTone, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITCustomTone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITCustomTone {
    type Vtable = ITCustomTone_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITCustomTone {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x357ad764_b3c6_4b2a_8fa5_0722827a9254);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITCustomTone_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Frequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plfrequency: *mut i32) -> ::windows::core::HRESULT,
    pub SetFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfrequency: i32) -> ::windows::core::HRESULT,
    pub CadenceOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcadenceon: *mut i32) -> ::windows::core::HRESULT,
    pub SetCadenceOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cadenceon: i32) -> ::windows::core::HRESULT,
    pub CadenceOff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcadenceoff: *mut i32) -> ::windows::core::HRESULT,
    pub SetCadenceOff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcadenceoff: i32) -> ::windows::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITDetectTone(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITDetectTone {
    pub unsafe fn AppSpecific(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AppSpecific)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAppSpecific)(::windows::core::Vtable::as_raw(self), lappspecific).ok()
    }
    pub unsafe fn Duration(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Duration)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDuration(&self, lduration: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDuration)(::windows::core::Vtable::as_raw(self), lduration).ok()
    }
    pub unsafe fn get_Frequency(&self, index: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Frequency)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_Frequency(&self, index: i32, lfrequency: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_Frequency)(::windows::core::Vtable::as_raw(self), index, lfrequency).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITDetectTone, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITDetectTone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITDetectTone {
    type Vtable = ITDetectTone_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITDetectTone {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x961f79bd_3097_49df_a1d6_909b77e89ca0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITDetectTone_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT,
    pub get_Frequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, plfrequency: *mut i32) -> ::windows::core::HRESULT,
    pub put_Frequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, lfrequency: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITDigitDetectionEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITDigitDetectionEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Digit(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Digit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DigitMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DigitMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TickCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TickCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallbackInstance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallbackInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITDigitDetectionEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITDigitDetectionEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITDigitDetectionEvent {
    type Vtable = ITDigitDetectionEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITDigitDetectionEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80d3bfac_57d9_11d2_a04a_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITDigitDetectionEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Digit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucdigit: *mut u8) -> ::windows::core::HRESULT,
    pub DigitMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdigitmode: *mut i32) -> ::windows::core::HRESULT,
    pub TickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITDigitGenerationEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITDigitGenerationEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GenerationTermination(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GenerationTermination)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TickCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TickCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallbackInstance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallbackInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITDigitGenerationEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITDigitGenerationEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITDigitGenerationEvent {
    type Vtable = ITDigitGenerationEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITDigitGenerationEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80d3bfad_57d9_11d2_a04a_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITDigitGenerationEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub GenerationTermination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plgenerationtermination: *mut i32) -> ::windows::core::HRESULT,
    pub TickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITDigitsGatheredEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITDigitsGatheredEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Digits(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Digits)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GatherTermination(&self) -> ::windows::core::Result<TAPI_GATHERTERM> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GatherTermination)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TickCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TickCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallbackInstance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallbackInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITDigitsGatheredEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITDigitsGatheredEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITDigitsGatheredEvent {
    type Vtable = ITDigitsGatheredEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITDigitsGatheredEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe52ec4c1_cba3_441a_9e6a_93cb909e9724);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITDigitsGatheredEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Digits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdigits: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GatherTermination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgathertermination: *mut TAPI_GATHERTERM) -> ::windows::core::HRESULT,
    pub TickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITDirectory(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITDirectory {
    pub unsafe fn DirectoryType(&self) -> ::windows::core::Result<DIRECTORY_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DirectoryType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDynamic(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsDynamic)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DefaultObjectTTL(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DefaultObjectTTL)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDefaultObjectTTL(&self, ttl: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultObjectTTL)(::windows::core::Vtable::as_raw(self), ttl).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableAutoRefresh<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).EnableAutoRefresh)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<P0>(&self, fsecure: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Connect)(::windows::core::Vtable::as_raw(self), fsecure.into()).ok()
    }
    pub unsafe fn Bind(&self, pdomainname: &::windows::core::BSTR, pusername: &::windows::core::BSTR, ppassword: &::windows::core::BSTR, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Bind)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdomainname), ::core::mem::transmute_copy(pusername), ::core::mem::transmute_copy(ppassword), lflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDirectoryObject<P0>(&self, pdirectoryobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITDirectoryObject>>,
    {
        (::windows::core::Vtable::vtable(self).AddDirectoryObject)(::windows::core::Vtable::as_raw(self), pdirectoryobject.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModifyDirectoryObject<P0>(&self, pdirectoryobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITDirectoryObject>>,
    {
        (::windows::core::Vtable::vtable(self).ModifyDirectoryObject)(::windows::core::Vtable::as_raw(self), pdirectoryobject.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RefreshDirectoryObject<P0>(&self, pdirectoryobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITDirectoryObject>>,
    {
        (::windows::core::Vtable::vtable(self).RefreshDirectoryObject)(::windows::core::Vtable::as_raw(self), pdirectoryobject.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteDirectoryObject<P0>(&self, pdirectoryobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITDirectoryObject>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteDirectoryObject)(::windows::core::Vtable::as_raw(self), pdirectoryobject.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_DirectoryObjects(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_DirectoryObjects)(::windows::core::Vtable::as_raw(self), directoryobjecttype, ::core::mem::transmute_copy(pname), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateDirectoryObjects(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &::windows::core::BSTR) -> ::windows::core::Result<IEnumDirectoryObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateDirectoryObjects)(::windows::core::Vtable::as_raw(self), directoryobjecttype, ::core::mem::transmute_copy(pname), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITDirectory, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITDirectory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITDirectory {
    type Vtable = ITDirectory_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITDirectory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34621d6c_6cff_11d1_aff7_00c04fc31fee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITDirectory_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DirectoryType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectorytype: *mut DIRECTORY_TYPE) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDynamic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfdynamic: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDynamic: usize,
    pub DefaultObjectTTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT,
    pub SetDefaultObjectTTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ttl: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableAutoRefresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableAutoRefresh: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsecure: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdomainname: *mut ::core::ffi::c_void, pusername: *mut ::core::ffi::c_void, ppassword: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddDirectoryObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddDirectoryObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ModifyDirectoryObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModifyDirectoryObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RefreshDirectoryObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RefreshDirectoryObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteDirectoryObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteDirectoryObject: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_DirectoryObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_DirectoryObjects: usize,
    pub EnumerateDirectoryObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: *mut ::core::ffi::c_void, ppenumobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITDirectoryObject(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObject {
    pub unsafe fn ObjectType(&self) -> ::windows::core::Result<DIRECTORY_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ObjectType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, pname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_DialableAddrs(&self, dwaddresstype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_DialableAddrs)(::windows::core::Vtable::as_raw(self), dwaddresstype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateDialableAddrs(&self, dwaddresstype: u32) -> ::windows::core::Result<IEnumDialableAddrs> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateDialableAddrs)(::windows::core::Vtable::as_raw(self), dwaddresstype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityDescriptor(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SecurityDescriptor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<P0>(&self, psecdes: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).SetSecurityDescriptor)(::windows::core::Vtable::as_raw(self), psecdes.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITDirectoryObject, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITDirectoryObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITDirectoryObject {
    type Vtable = ITDirectoryObject_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITDirectoryObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34621d6e_6cff_11d1_aff7_00c04fc31fee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITDirectoryObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjecttype: *mut DIRECTORY_OBJECT_TYPE) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_DialableAddrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaddresstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_DialableAddrs: usize,
    pub EnumerateDialableAddrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaddresstype: u32, ppenumdialableaddrs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecdes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecdes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityDescriptor: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITDirectoryObjectConference(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObjectConference {
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Protocol)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Originator(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Originator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOriginator(&self, poriginator: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOriginator)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(poriginator)).ok()
    }
    pub unsafe fn AdvertisingScope(&self) -> ::windows::core::Result<RND_ADVERTISING_SCOPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AdvertisingScope)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAdvertisingScope(&self, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAdvertisingScope)(::windows::core::Vtable::as_raw(self), advertisingscope).ok()
    }
    pub unsafe fn Url(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Url)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUrl(&self, purl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(purl)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, pdescription: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEncrypted(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsEncrypted)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsEncrypted<P0>(&self, fencrypted: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetIsEncrypted)(::windows::core::Vtable::as_raw(self), fencrypted.into()).ok()
    }
    pub unsafe fn StartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StartTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStartTime(&self, date: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStartTime)(::windows::core::Vtable::as_raw(self), date).ok()
    }
    pub unsafe fn StopTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StopTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStopTime(&self, date: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStopTime)(::windows::core::Vtable::as_raw(self), date).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITDirectoryObjectConference, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITDirectoryObjectConference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITDirectoryObjectConference {
    type Vtable = ITDirectoryObjectConference_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITDirectoryObjectConference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1029e5d_cb5b_11d0_8d59_00c04fd91ac0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITDirectoryObjectConference_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprotocol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Originator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporiginator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOriginator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poriginator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AdvertisingScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padvertisingscope: *mut RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT,
    pub SetAdvertisingScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT,
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppurl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, purl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEncrypted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfencrypted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEncrypted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsEncrypted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fencrypted: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsEncrypted: usize,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT,
    pub StopTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub SetStopTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITDirectoryObjectUser(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObjectUser {
    pub unsafe fn IPPhonePrimary(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IPPhonePrimary)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetIPPhonePrimary(&self, pname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetIPPhonePrimary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pname)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITDirectoryObjectUser, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITDirectoryObjectUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITDirectoryObjectUser {
    type Vtable = ITDirectoryObjectUser_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITDirectoryObjectUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34621d6f_6cff_11d1_aff7_00c04fc31fee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITDirectoryObjectUser_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IPPhonePrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIPPhonePrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITDispatchMapper(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITDispatchMapper {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDispatchInterface<P0>(&self, piid: &::windows::core::BSTR, pinterfacetomap: P0) -> ::windows::core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QueryDispatchInterface)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(piid), pinterfacetomap.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITDispatchMapper, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITDispatchMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITDispatchMapper {
    type Vtable = ITDispatchMapper_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITDispatchMapper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9225295_c759_11d1_a02b_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITDispatchMapper_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDispatchInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::core::ffi::c_void, pinterfacetomap: *mut ::core::ffi::c_void, ppreturnedinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDispatchInterface: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITFileTerminalEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITFileTerminalEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Terminal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Track(&self) -> ::windows::core::Result<ITFileTrack> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Track)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<TERMINAL_MEDIA_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cause(&self) -> ::windows::core::Result<FT_STATE_EVENT_CAUSE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cause)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITFileTerminalEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITFileTerminalEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITFileTerminalEvent {
    type Vtable = ITFileTerminalEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITFileTerminalEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4a7fbac_8c17_4427_9f55_9f589ac8af00);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITFileTerminalEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Track: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptrackterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Track: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT,
    pub Cause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcause: *mut FT_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITFileTrack(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITFileTrack {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn Format(&self) -> ::windows::core::Result<*mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Format)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetFormat(&self, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFormat)(::windows::core::Vtable::as_raw(self), pmt).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ControllingTerminal(&self) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ControllingTerminal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioFormatForScripting(&self) -> ::windows::core::Result<ITScriptableAudioFormat> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AudioFormatForScripting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAudioFormatForScripting<P0>(&self, paudioformat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITScriptableAudioFormat>>,
    {
        (::windows::core::Vtable::vtable(self).SetAudioFormatForScripting)(::windows::core::Vtable::as_raw(self), paudioformat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EmptyAudioFormatForScripting(&self) -> ::windows::core::Result<ITScriptableAudioFormat> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EmptyAudioFormatForScripting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITFileTrack, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITFileTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITFileTrack {
    type Vtable = ITFileTrack_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITFileTrack {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31ca6ea9_c08a_4bea_8811_8e9c1ba3ea3a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITFileTrack_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    Format: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    SetFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ControllingTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontrollingterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ControllingTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioFormatForScripting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaudioformat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioFormatForScripting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAudioFormatForScripting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paudioformat: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAudioFormatForScripting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EmptyAudioFormatForScripting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaudioformat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EmptyAudioFormatForScripting: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITForwardInformation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITForwardInformation {
    pub unsafe fn SetNumRingsNoAnswer(&self, lnumrings: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNumRingsNoAnswer)(::windows::core::Vtable::as_raw(self), lnumrings).ok()
    }
    pub unsafe fn NumRingsNoAnswer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NumRingsNoAnswer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetForwardType(&self, forwardtype: i32, pdestaddress: &::windows::core::BSTR, pcalleraddress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetForwardType)(::windows::core::Vtable::as_raw(self), forwardtype, ::core::mem::transmute_copy(pdestaddress), ::core::mem::transmute_copy(pcalleraddress)).ok()
    }
    pub unsafe fn get_ForwardTypeDestination(&self, forwardtype: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_ForwardTypeDestination)(::windows::core::Vtable::as_raw(self), forwardtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_ForwardTypeCaller(&self, forwardtype: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_ForwardTypeCaller)(::windows::core::Vtable::as_raw(self), forwardtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetForwardType(&self, forwardtype: i32, ppdestinationaddress: *mut ::windows::core::BSTR, ppcalleraddress: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetForwardType)(::windows::core::Vtable::as_raw(self), forwardtype, ::core::mem::transmute(ppdestinationaddress), ::core::mem::transmute(ppcalleraddress)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Clear)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITForwardInformation, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITForwardInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITForwardInformation {
    type Vtable = ITForwardInformation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITForwardInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x449f659e_88a3_11d1_bb5d_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITForwardInformation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetNumRingsNoAnswer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumrings: i32) -> ::windows::core::HRESULT,
    pub NumRingsNoAnswer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plnumrings: *mut i32) -> ::windows::core::HRESULT,
    pub SetForwardType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: *mut ::core::ffi::c_void, pcalleraddress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_ForwardTypeDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_ForwardTypeCaller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forwardtype: i32, ppcalleraddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetForwardType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut *mut ::core::ffi::c_void, ppcalleraddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITForwardInformation2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITForwardInformation2 {
    pub unsafe fn SetForwardType2(&self, forwardtype: i32, pdestaddress: &::windows::core::BSTR, destaddresstype: i32, pcalleraddress: &::windows::core::BSTR, calleraddresstype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetForwardType2)(::windows::core::Vtable::as_raw(self), forwardtype, ::core::mem::transmute_copy(pdestaddress), destaddresstype, ::core::mem::transmute_copy(pcalleraddress), calleraddresstype).ok()
    }
    pub unsafe fn GetForwardType2(&self, forwardtype: i32, ppdestinationaddress: *mut ::windows::core::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut ::windows::core::BSTR, pcalleraddresstype: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetForwardType2)(::windows::core::Vtable::as_raw(self), forwardtype, ::core::mem::transmute(ppdestinationaddress), pdestaddresstype, ::core::mem::transmute(ppcalleraddress), pcalleraddresstype).ok()
    }
    pub unsafe fn get_ForwardTypeDestinationAddressType(&self, forwardtype: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_ForwardTypeDestinationAddressType)(::windows::core::Vtable::as_raw(self), forwardtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_ForwardTypeCallerAddressType(&self, forwardtype: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_ForwardTypeCallerAddressType)(::windows::core::Vtable::as_raw(self), forwardtype, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITForwardInformation2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ITForwardInformation);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITForwardInformation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITForwardInformation2 {
    type Vtable = ITForwardInformation2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITForwardInformation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5229b4ed_b260_4382_8e1a_5df3a8a4ccc0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITForwardInformation2_Vtbl {
    pub base__: ITForwardInformation_Vtbl,
    pub SetForwardType2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: *mut ::core::ffi::c_void, destaddresstype: i32, pcalleraddress: *mut ::core::ffi::c_void, calleraddresstype: i32) -> ::windows::core::HRESULT,
    pub GetForwardType2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut *mut ::core::ffi::c_void, pdestaddresstype: *mut i32, ppcalleraddress: *mut *mut ::core::ffi::c_void, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT,
    pub get_ForwardTypeDestinationAddressType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddresstype: *mut i32) -> ::windows::core::HRESULT,
    pub get_ForwardTypeCallerAddressType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forwardtype: i32, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITILSConfig(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITILSConfig {
    pub unsafe fn Port(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Port)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPort(&self, port: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPort)(::windows::core::Vtable::as_raw(self), port).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITILSConfig, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITILSConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITILSConfig {
    type Vtable = ITILSConfig_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITILSConfig {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34621d72_6cff_11d1_aff7_00c04fc31fee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITILSConfig_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Port: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pport: *mut i32) -> ::windows::core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, port: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct ITLegacyAddressMediaControl(::windows::core::IUnknown);
impl ITLegacyAddressMediaControl {
    pub unsafe fn GetID(&self, pdeviceclass: &::windows::core::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdeviceclass), pdwsize, ppdeviceid).ok()
    }
    pub unsafe fn GetDevConfig(&self, pdeviceclass: &::windows::core::BSTR, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDevConfig)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdeviceclass), pdwsize, ppdeviceconfig).ok()
    }
    pub unsafe fn SetDevConfig(&self, pdeviceclass: &::windows::core::BSTR, pdeviceconfig: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDevConfig)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdeviceclass), pdeviceconfig.len() as _, ::core::mem::transmute(pdeviceconfig.as_ptr())).ok()
    }
}
::windows::core::interface_hierarchy!(ITLegacyAddressMediaControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITLegacyAddressMediaControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITLegacyAddressMediaControl {
    type Vtable = ITLegacyAddressMediaControl_Vtbl;
}
unsafe impl ::windows::core::Interface for ITLegacyAddressMediaControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab493640_4c0b_11d2_a046_00c04fb6809f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITLegacyAddressMediaControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdeviceclass: *mut ::core::ffi::c_void, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetDevConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdeviceclass: *mut ::core::ffi::c_void, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetDevConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdeviceclass: *mut ::core::ffi::c_void, dwsize: u32, pdeviceconfig: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct ITLegacyAddressMediaControl2(::windows::core::IUnknown);
impl ITLegacyAddressMediaControl2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigDialog<P0>(&self, hwndowner: P0, pdeviceclass: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).ConfigDialog)(::windows::core::Vtable::as_raw(self), hwndowner.into(), ::core::mem::transmute_copy(pdeviceclass)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigDialogEdit<P0>(&self, hwndowner: P0, pdeviceclass: &::windows::core::BSTR, pdeviceconfigin: &[u8], pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).ConfigDialogEdit)(::windows::core::Vtable::as_raw(self), hwndowner.into(), ::core::mem::transmute_copy(pdeviceclass), pdeviceconfigin.len() as _, ::core::mem::transmute(pdeviceconfigin.as_ptr()), pdwsizeout, ppdeviceconfigout).ok()
    }
}
::windows::core::interface_hierarchy!(ITLegacyAddressMediaControl2, ::windows::core::IUnknown, ITLegacyAddressMediaControl);
impl ::core::clone::Clone for ITLegacyAddressMediaControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITLegacyAddressMediaControl2 {
    type Vtable = ITLegacyAddressMediaControl2_Vtbl;
}
unsafe impl ::windows::core::Interface for ITLegacyAddressMediaControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0ee512b_a531_409e_9dd9_4099fe86c738);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITLegacyAddressMediaControl2_Vtbl {
    pub base__: ITLegacyAddressMediaControl_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigDialog: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigDialogEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: *mut ::core::ffi::c_void, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigDialogEdit: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITLegacyCallMediaControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyCallMediaControl {
    pub unsafe fn DetectDigits(&self, digitmode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DetectDigits)(::windows::core::Vtable::as_raw(self), digitmode).ok()
    }
    pub unsafe fn GenerateDigits(&self, pdigits: &::windows::core::BSTR, digitmode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GenerateDigits)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdigits), digitmode).ok()
    }
    pub unsafe fn GetID(&self, pdeviceclass: &::windows::core::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdeviceclass), pdwsize, ppdeviceid).ok()
    }
    pub unsafe fn SetMediaType(&self, lmediatype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMediaType)(::windows::core::Vtable::as_raw(self), lmediatype).ok()
    }
    pub unsafe fn MonitorMedia(&self, lmediatype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MonitorMedia)(::windows::core::Vtable::as_raw(self), lmediatype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITLegacyCallMediaControl, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITLegacyCallMediaControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITLegacyCallMediaControl {
    type Vtable = ITLegacyCallMediaControl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITLegacyCallMediaControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd624582f_cc23_4436_b8a5_47c625c8045d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITLegacyCallMediaControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DetectDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digitmode: i32) -> ::windows::core::HRESULT,
    pub GenerateDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdigits: *mut ::core::ffi::c_void, digitmode: i32) -> ::windows::core::HRESULT,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdeviceclass: *mut ::core::ffi::c_void, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT,
    pub MonitorMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITLegacyCallMediaControl2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyCallMediaControl2 {
    pub unsafe fn GenerateDigits2(&self, pdigits: &::windows::core::BSTR, digitmode: i32, lduration: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GenerateDigits2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdigits), digitmode, lduration).ok()
    }
    pub unsafe fn GatherDigits(&self, digitmode: i32, lnumdigits: i32, pterminationdigits: &::windows::core::BSTR, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GatherDigits)(::windows::core::Vtable::as_raw(self), digitmode, lnumdigits, ::core::mem::transmute_copy(pterminationdigits), lfirstdigittimeout, linterdigittimeout).ok()
    }
    pub unsafe fn DetectTones(&self, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DetectTones)(::windows::core::Vtable::as_raw(self), ptonelist, lnumtones).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectTonesByCollection<P0>(&self, pdetecttonecollection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITCollection2>>,
    {
        (::windows::core::Vtable::vtable(self).DetectTonesByCollection)(::windows::core::Vtable::as_raw(self), pdetecttonecollection.into().abi()).ok()
    }
    pub unsafe fn GenerateTone(&self, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GenerateTone)(::windows::core::Vtable::as_raw(self), tonemode, lduration).ok()
    }
    pub unsafe fn GenerateCustomTones(&self, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GenerateCustomTones)(::windows::core::Vtable::as_raw(self), ptonelist, lnumtones, lduration).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GenerateCustomTonesByCollection<P0>(&self, pcustomtonecollection: P0, lduration: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITCollection2>>,
    {
        (::windows::core::Vtable::vtable(self).GenerateCustomTonesByCollection)(::windows::core::Vtable::as_raw(self), pcustomtonecollection.into().abi(), lduration).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDetectToneObject(&self) -> ::windows::core::Result<ITDetectTone> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDetectToneObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateCustomToneObject(&self) -> ::windows::core::Result<ITCustomTone> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCustomToneObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetIDAsVariant(&self, bstrdeviceclass: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIDAsVariant)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrdeviceclass), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITLegacyCallMediaControl2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ITLegacyCallMediaControl);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITLegacyCallMediaControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITLegacyCallMediaControl2 {
    type Vtable = ITLegacyCallMediaControl2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITLegacyCallMediaControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57ca332d_7bc2_44f1_a60c_936fe8d7ce73);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITLegacyCallMediaControl2_Vtbl {
    pub base__: ITLegacyCallMediaControl_Vtbl,
    pub GenerateDigits2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdigits: *mut ::core::ffi::c_void, digitmode: i32, lduration: i32) -> ::windows::core::HRESULT,
    pub GatherDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digitmode: i32, lnumdigits: i32, pterminationdigits: *mut ::core::ffi::c_void, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows::core::HRESULT,
    pub DetectTones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DetectTonesByCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdetecttonecollection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DetectTonesByCollection: usize,
    pub GenerateTone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows::core::HRESULT,
    pub GenerateCustomTones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GenerateCustomTonesByCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcustomtonecollection: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GenerateCustomTonesByCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDetectToneObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdetecttone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDetectToneObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateCustomToneObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcustomtone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateCustomToneObject: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetIDAsVariant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceclass: *mut ::core::ffi::c_void, pvardeviceid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetIDAsVariant: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITLegacyWaveSupport(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyWaveSupport {
    pub unsafe fn IsFullDuplex(&self) -> ::windows::core::Result<FULLDUPLEX_SUPPORT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsFullDuplex)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITLegacyWaveSupport, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITLegacyWaveSupport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITLegacyWaveSupport {
    type Vtable = ITLegacyWaveSupport_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITLegacyWaveSupport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x207823ea_e252_11d2_b77e_0080c7135381);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITLegacyWaveSupport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IsFullDuplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupport: *mut FULLDUPLEX_SUPPORT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITLocationInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITLocationInfo {
    pub unsafe fn PermanentLocationID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PermanentLocationID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CountryCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CountryCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CountryID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CountryID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Options(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Options)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PreferredCardID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PreferredCardID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LocationName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocationName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CityCode(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CityCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LocalAccessCode(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LocalAccessCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LongDistanceAccessCode(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LongDistanceAccessCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TollPrefixList(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TollPrefixList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CancelCallWaitingCode(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CancelCallWaitingCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITLocationInfo, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITLocationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITLocationInfo {
    type Vtable = ITLocationInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITLocationInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c4d8eff_8ddb_11d1_a09e_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITLocationInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub PermanentLocationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllocationid: *mut i32) -> ::windows::core::HRESULT,
    pub CountryCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows::core::HRESULT,
    pub CountryID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcountryid: *mut i32) -> ::windows::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT,
    pub PreferredCardID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT,
    pub LocationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplocationname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CityCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocalAccessCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LongDistanceAccessCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TollPrefixList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptolllist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelCallWaitingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct ITMSPAddress(::windows::core::IUnknown);
impl ITMSPAddress {
    pub unsafe fn Initialize(&self, hevent: *const i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), hevent).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CreateMSPCall<P0>(&self, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateMSPCall)(::windows::core::Vtable::as_raw(self), hcall, dwreserved, dwmediatype, pouterunknown.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShutdownMSPCall<P0>(&self, pstreamcontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).ShutdownMSPCall)(::windows::core::Vtable::as_raw(self), pstreamcontrol.into().abi()).ok()
    }
    pub unsafe fn ReceiveTSPData<P0>(&self, pmspcall: P0, pbuffer: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).ReceiveTSPData)(::windows::core::Vtable::as_raw(self), pmspcall.into().abi(), ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _).ok()
    }
    pub unsafe fn GetEvent(&self, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetEvent)(::windows::core::Vtable::as_raw(self), pdwsize, peventbuffer).ok()
    }
}
::windows::core::interface_hierarchy!(ITMSPAddress, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITMSPAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITMSPAddress {
    type Vtable = ITMSPAddress_Vtbl;
}
unsafe impl ::windows::core::Interface for ITMSPAddress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee3bd600_3868_11d2_a045_00c04fb6809f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITMSPAddress_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: *const i32) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateMSPCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: *mut ::core::ffi::c_void, ppstreamcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShutdownMSPCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstreamcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReceiveTSPData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmspcall: *mut ::core::ffi::c_void, pbuffer: *const u8, dwsize: u32) -> ::windows::core::HRESULT,
    pub GetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITMediaControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITMediaControl {
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn MediaState(&self) -> ::windows::core::Result<TERMINAL_MEDIA_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MediaState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITMediaControl, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITMediaControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITMediaControl {
    type Vtable = ITMediaControl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITMediaControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc445dde8_5199_4bc7_9807_5ffb92e42e09);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITMediaControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MediaState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminalmediastate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITMediaPlayback(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITMediaPlayback {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPlayList(&self, playlistvariant: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPlayList)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(playlistvariant)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PlayList(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PlayList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITMediaPlayback, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITMediaPlayback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITMediaPlayback {
    type Vtable = ITMediaPlayback_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITMediaPlayback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x627e8ae6_ae4c_4a69_bb63_2ad625404b77);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITMediaPlayback_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPlayList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, playlistvariant: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPlayList: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PlayList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplaylistvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PlayList: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITMediaRecord(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITMediaRecord {
    pub unsafe fn SetFileName(&self, bstrfilename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfilename)).ok()
    }
    pub unsafe fn FileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITMediaRecord, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITMediaRecord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITMediaRecord {
    type Vtable = ITMediaRecord_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITMediaRecord {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5dd4592_5476_4cc1_9d4d_fad3eefe7db2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITMediaRecord_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITMediaSupport(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITMediaSupport {
    pub unsafe fn MediaTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MediaTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryMediaType(&self, lmediatype: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QueryMediaType)(::windows::core::Vtable::as_raw(self), lmediatype, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITMediaSupport, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITMediaSupport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITMediaSupport {
    type Vtable = ITMediaSupport_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITMediaSupport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1efc384_9355_11d0_835c_00aa003ccabd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITMediaSupport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MediaTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32, pfsupport: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryMediaType: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITMultiTrackTerminal(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITMultiTrackTerminal {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TrackTerminals(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TrackTerminals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateTrackTerminals(&self) -> ::windows::core::Result<IEnumTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateTrackTerminals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTrackTerminal(&self, mediatype: i32, terminaldirection: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTrackTerminal)(::windows::core::Vtable::as_raw(self), mediatype, terminaldirection, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MediaTypesInUse(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MediaTypesInUse)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DirectionsInUse(&self) -> ::windows::core::Result<TERMINAL_DIRECTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DirectionsInUse)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveTrackTerminal<P0>(&self, ptrackterminaltoremove: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITTerminal>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveTrackTerminal)(::windows::core::Vtable::as_raw(self), ptrackterminaltoremove.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITMultiTrackTerminal, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITMultiTrackTerminal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITMultiTrackTerminal {
    type Vtable = ITMultiTrackTerminal_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITMultiTrackTerminal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe040091_ade8_4072_95c9_bf7de8c54b44);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITMultiTrackTerminal_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TrackTerminals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TrackTerminals: usize,
    pub EnumerateTrackTerminals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTrackTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: i32, terminaldirection: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTrackTerminal: usize,
    pub MediaTypesInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatypesinuse: *mut i32) -> ::windows::core::HRESULT,
    pub DirectionsInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldirectionsinused: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveTrackTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptrackterminaltoremove: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveTrackTerminal: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITPhone(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITPhone {
    pub unsafe fn Open(&self, privilege: PHONE_PRIVILEGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Open)(::windows::core::Vtable::as_raw(self), privilege).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Addresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Addresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateAddresses(&self) -> ::windows::core::Result<IEnumAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_PhoneCapsLong(&self, pclcap: PHONECAPS_LONG) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_PhoneCapsLong)(::windows::core::Vtable::as_raw(self), pclcap, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_PhoneCapsString(&self, pcscap: PHONECAPS_STRING) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_PhoneCapsString)(::windows::core::Vtable::as_raw(self), pcscap, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Terminals<P0>(&self, paddress: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITAddress>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Terminals)(::windows::core::Vtable::as_raw(self), paddress.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateTerminals<P0>(&self, paddress: P0) -> ::windows::core::Result<IEnumTerminal>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITAddress>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateTerminals)(::windows::core::Vtable::as_raw(self), paddress.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_ButtonMode(&self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_ButtonMode)(::windows::core::Vtable::as_raw(self), lbuttonid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_ButtonMode(&self, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_ButtonMode)(::windows::core::Vtable::as_raw(self), lbuttonid, buttonmode).ok()
    }
    pub unsafe fn get_ButtonFunction(&self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_FUNCTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_ButtonFunction)(::windows::core::Vtable::as_raw(self), lbuttonid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_ButtonFunction(&self, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_ButtonFunction)(::windows::core::Vtable::as_raw(self), lbuttonid, buttonfunction).ok()
    }
    pub unsafe fn get_ButtonText(&self, lbuttonid: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_ButtonText)(::windows::core::Vtable::as_raw(self), lbuttonid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_ButtonText(&self, lbuttonid: i32, bstrbuttontext: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_ButtonText)(::windows::core::Vtable::as_raw(self), lbuttonid, ::core::mem::transmute_copy(bstrbuttontext)).ok()
    }
    pub unsafe fn get_ButtonState(&self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_ButtonState)(::windows::core::Vtable::as_raw(self), lbuttonid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_HookSwitchState(&self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE) -> ::windows::core::Result<PHONE_HOOK_SWITCH_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_HookSwitchState)(::windows::core::Vtable::as_raw(self), hookswitchdevice, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_HookSwitchState(&self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_HookSwitchState)(::windows::core::Vtable::as_raw(self), hookswitchdevice, hookswitchstate).ok()
    }
    pub unsafe fn SetRingMode(&self, lringmode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRingMode)(::windows::core::Vtable::as_raw(self), lringmode).ok()
    }
    pub unsafe fn RingMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RingMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRingVolume(&self, lringvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRingVolume)(::windows::core::Vtable::as_raw(self), lringvolume).ok()
    }
    pub unsafe fn RingVolume(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RingVolume)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Privilege(&self) -> ::windows::core::Result<PHONE_PRIVILEGE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Privilege)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPhoneCapsBuffer(&self, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPhoneCapsBuffer)(::windows::core::Vtable::as_raw(self), pcbcaps, pdwsize, ppphonecapsbuffer).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_PhoneCapsBuffer(&self, pcbcaps: PHONECAPS_BUFFER) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_PhoneCapsBuffer)(::windows::core::Vtable::as_raw(self), pcbcaps, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_LampMode(&self, llampid: i32) -> ::windows::core::Result<PHONE_LAMP_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_LampMode)(::windows::core::Vtable::as_raw(self), llampid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_LampMode(&self, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_LampMode)(::windows::core::Vtable::as_raw(self), llampid, lampmode).ok()
    }
    pub unsafe fn Display(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Display)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDisplay(&self, lrow: i32, lcolumn: i32, bstrdisplay: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDisplay)(::windows::core::Vtable::as_raw(self), lrow, lcolumn, ::core::mem::transmute_copy(bstrdisplay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PreferredAddresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PreferredAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumeratePreferredAddresses(&self) -> ::windows::core::Result<IEnumAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumeratePreferredAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DeviceSpecific(&self, pparams: *const u8, dwsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeviceSpecific)(::windows::core::Vtable::as_raw(self), pparams, dwsize).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeviceSpecificVariant(&self, vardevspecificbytearray: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeviceSpecificVariant)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(vardevspecificbytearray)).ok()
    }
    pub unsafe fn NegotiateExtVersion(&self, llowversion: i32, lhighversion: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NegotiateExtVersion)(::windows::core::Vtable::as_raw(self), llowversion, lhighversion, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITPhone, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITPhone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITPhone {
    type Vtable = ITPhone_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITPhone {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09d48db4_10cc_4388_9de7_a8465618975a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITPhone_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, privilege: PHONE_PRIVILEGE) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Addresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Addresses: usize,
    pub EnumerateAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_PhoneCapsLong: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclcap: PHONECAPS_LONG, plcapability: *mut i32) -> ::windows::core::HRESULT,
    pub get_PhoneCapsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcscap: PHONECAPS_STRING, ppcapability: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Terminals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Terminals: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateTerminals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateTerminals: usize,
    pub get_ButtonMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonmode: *mut PHONE_BUTTON_MODE) -> ::windows::core::HRESULT,
    pub put_ButtonMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows::core::HRESULT,
    pub get_ButtonFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonfunction: *mut PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT,
    pub put_ButtonFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT,
    pub get_ButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbuttonid: i32, ppbuttontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub put_ButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbuttonid: i32, bstrbuttontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_ButtonState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT,
    pub get_HookSwitchState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, phookswitchstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT,
    pub put_HookSwitchState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT,
    pub SetRingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lringmode: i32) -> ::windows::core::HRESULT,
    pub RingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT,
    pub SetRingVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lringvolume: i32) -> ::windows::core::HRESULT,
    pub RingVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plringvolume: *mut i32) -> ::windows::core::HRESULT,
    pub Privilege: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprivilege: *mut PHONE_PRIVILEGE) -> ::windows::core::HRESULT,
    pub GetPhoneCapsBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_PhoneCapsBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pvarbuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_PhoneCapsBuffer: usize,
    pub get_LampMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llampid: i32, plampmode: *mut PHONE_LAMP_MODE) -> ::windows::core::HRESULT,
    pub put_LampMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows::core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisplay: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lrow: i32, lcolumn: i32, bstrdisplay: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PreferredAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PreferredAddresses: usize,
    pub EnumeratePreferredAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeviceSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeviceSpecificVariant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardevspecificbytearray: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeviceSpecificVariant: usize,
    pub NegotiateExtVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITPhoneDeviceSpecificEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITPhoneDeviceSpecificEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Phone(&self) -> ::windows::core::Result<ITPhone> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Phone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn lParam1(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).lParam1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn lParam2(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).lParam2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn lParam3(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).lParam3)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITPhoneDeviceSpecificEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITPhoneDeviceSpecificEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITPhoneDeviceSpecificEvent {
    type Vtable = ITPhoneDeviceSpecificEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITPhoneDeviceSpecificEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63ffb2a6_872b_4cd3_a501_326e8fb40af7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITPhoneDeviceSpecificEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Phone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Phone: usize,
    pub lParam1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT,
    pub lParam2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT,
    pub lParam3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITPhoneEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITPhoneEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Phone(&self) -> ::windows::core::Result<ITPhone> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Phone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<PHONE_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ButtonState(&self) -> ::windows::core::Result<PHONE_BUTTON_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ButtonState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HookSwitchState(&self) -> ::windows::core::Result<PHONE_HOOK_SWITCH_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HookSwitchState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HookSwitchDevice(&self) -> ::windows::core::Result<PHONE_HOOK_SWITCH_DEVICE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HookSwitchDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RingMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RingMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ButtonLampId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ButtonLampId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NumberGathered(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NumberGathered)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITPhoneEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITPhoneEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITPhoneEvent {
    type Vtable = ITPhoneEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITPhoneEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f942dd8_64ed_4aaf_a77d_b23db0837ead);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITPhoneEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Phone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Phone: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut PHONE_EVENT) -> ::windows::core::HRESULT,
    pub ButtonState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT,
    pub HookSwitchState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT,
    pub HookSwitchDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut PHONE_HOOK_SWITCH_DEVICE) -> ::windows::core::HRESULT,
    pub RingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT,
    pub ButtonLampId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbuttonlampid: *mut i32) -> ::windows::core::HRESULT,
    pub NumberGathered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnumber: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITPluggableTerminalClassInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalClassInfo {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Company(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Company)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Version)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TerminalClass(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TerminalClass)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Direction(&self) -> ::windows::core::Result<TERMINAL_DIRECTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Direction)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MediaTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MediaTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITPluggableTerminalClassInfo, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITPluggableTerminalClassInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITPluggableTerminalClassInfo {
    type Vtable = ITPluggableTerminalClassInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITPluggableTerminalClassInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41757f4a_cf09_4b34_bc96_0a79d2390076);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITPluggableTerminalClassInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Company: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompany: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TerminalClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminalclass: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT,
    pub MediaTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct ITPluggableTerminalEventSink(::windows::core::IUnknown);
impl ITPluggableTerminalEventSink {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FireEvent(&self, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FireEvent)(::windows::core::Vtable::as_raw(self), pmspeventinfo).ok()
    }
}
::windows::core::interface_hierarchy!(ITPluggableTerminalEventSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITPluggableTerminalEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITPluggableTerminalEventSink {
    type Vtable = ITPluggableTerminalEventSink_Vtbl;
}
unsafe impl ::windows::core::Interface for ITPluggableTerminalEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e0887be_ba1a_492e_bd10_4020ec5e33e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITPluggableTerminalEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub FireEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireEvent: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct ITPluggableTerminalEventSinkRegistration(::windows::core::IUnknown);
impl ITPluggableTerminalEventSinkRegistration {
    pub unsafe fn RegisterSink<P0>(&self, peventsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITPluggableTerminalEventSink>>,
    {
        (::windows::core::Vtable::vtable(self).RegisterSink)(::windows::core::Vtable::as_raw(self), peventsink.into().abi()).ok()
    }
    pub unsafe fn UnregisterSink(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterSink)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ITPluggableTerminalEventSinkRegistration, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITPluggableTerminalEventSinkRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITPluggableTerminalEventSinkRegistration {
    type Vtable = ITPluggableTerminalEventSinkRegistration_Vtbl;
}
unsafe impl ::windows::core::Interface for ITPluggableTerminalEventSinkRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7115709_a216_4957_a759_060ab32a90d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITPluggableTerminalEventSinkRegistration_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITPluggableTerminalSuperclassInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalSuperclassInfo {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITPluggableTerminalSuperclassInfo, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITPluggableTerminalSuperclassInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITPluggableTerminalSuperclassInfo {
    type Vtable = ITPluggableTerminalSuperclassInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITPluggableTerminalSuperclassInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d54e42c_4625_4359_a6f7_631999107e05);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITPluggableTerminalSuperclassInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITPrivateEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITPrivateEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<ITAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Address)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CallHub(&self) -> ::windows::core::Result<ITCallHub> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallHub)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EventCode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EventCode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EventInterface(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EventInterface)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITPrivateEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITPrivateEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITPrivateEvent {
    type Vtable = ITPrivateEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITPrivateEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e269cd0_10d4_4121_9c22_9c85d625650d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITPrivateEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CallHub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallhub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CallHub: usize,
    pub EventCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pleventcode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EventInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EventInterface: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITQOSEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITQOSEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<QOS_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MediaType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MediaType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITQOSEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITQOSEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITQOSEvent {
    type Vtable = ITQOSEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITQOSEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfa3357c_ad77_11d1_bb68_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITQOSEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqosevent: *mut QOS_EVENT) -> ::windows::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITQueue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITQueue {
    pub unsafe fn SetMeasurementPeriod(&self, lperiod: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMeasurementPeriod)(::windows::core::Vtable::as_raw(self), lperiod).ok()
    }
    pub unsafe fn MeasurementPeriod(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MeasurementPeriod)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalCallsQueued(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalCallsQueued)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCallsQueued(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCallsQueued)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalCallsAbandoned(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalCallsAbandoned)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalCallsFlowedIn(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalCallsFlowedIn)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalCallsFlowedOut(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalCallsFlowedOut)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LongestEverWaitTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LongestEverWaitTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLongestWaitTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentLongestWaitTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AverageWaitTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AverageWaitTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FinalDisposition(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FinalDisposition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITQueue, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITQueue {
    type Vtable = ITQueue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITQueue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5afc3149_4bcc_11d1_bf80_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetMeasurementPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT,
    pub MeasurementPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT,
    pub TotalCallsQueued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentCallsQueued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub TotalCallsAbandoned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub TotalCallsFlowedIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub TotalCallsFlowedOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub LongestEverWaitTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentLongestWaitTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT,
    pub AverageWaitTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT,
    pub FinalDisposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITQueueEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITQueueEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Queue(&self) -> ::windows::core::Result<ITQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Queue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<ACDQUEUE_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITQueueEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITQueueEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITQueueEvent {
    type Vtable = ITQueueEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITQueueEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x297f3033_bd11_11d1_a0a7_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITQueueEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Queue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Queue: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut ACDQUEUE_EVENT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITRendezvous(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITRendezvous {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DefaultDirectories(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DefaultDirectories)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateDefaultDirectories(&self) -> ::windows::core::Result<IEnumDirectory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateDefaultDirectories)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDirectory(&self, directorytype: DIRECTORY_TYPE, pname: &::windows::core::BSTR) -> ::windows::core::Result<ITDirectory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDirectory)(::windows::core::Vtable::as_raw(self), directorytype, ::core::mem::transmute_copy(pname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDirectoryObject(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &::windows::core::BSTR) -> ::windows::core::Result<ITDirectoryObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDirectoryObject)(::windows::core::Vtable::as_raw(self), directoryobjecttype, ::core::mem::transmute_copy(pname), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITRendezvous, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITRendezvous {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITRendezvous {
    type Vtable = ITRendezvous_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITRendezvous {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34621d6b_6cff_11d1_aff7_00c04fc31fee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITRendezvous_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DefaultDirectories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DefaultDirectories: usize,
    pub EnumerateDefaultDirectories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdirectory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, directorytype: DIRECTORY_TYPE, pname: *mut ::core::ffi::c_void, ppdir: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDirectory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDirectoryObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: *mut ::core::ffi::c_void, ppdirectoryobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDirectoryObject: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITRequest(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITRequest {
    pub unsafe fn MakeCall(&self, pdestaddress: &::windows::core::BSTR, pappname: &::windows::core::BSTR, pcalledparty: &::windows::core::BSTR, pcomment: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MakeCall)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdestaddress), ::core::mem::transmute_copy(pappname), ::core::mem::transmute_copy(pcalledparty), ::core::mem::transmute_copy(pcomment)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITRequest, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITRequest {
    type Vtable = ITRequest_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac48ffdf_f8c4_11d1_a030_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITRequest_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MakeCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestaddress: *mut ::core::ffi::c_void, pappname: *mut ::core::ffi::c_void, pcalledparty: *mut ::core::ffi::c_void, pcomment: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITRequestEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITRequestEvent {
    pub unsafe fn RegistrationInstance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegistrationInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RequestMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DestAddress(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DestAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AppName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AppName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CalledParty(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CalledParty)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Comment(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Comment)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITRequestEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITRequestEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITRequestEvent {
    type Vtable = ITRequestEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITRequestEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac48ffde_f8c4_11d1_a030_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITRequestEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub RegistrationInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plregistrationinstance: *mut i32) -> ::windows::core::HRESULT,
    pub RequestMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plrequestmode: *mut i32) -> ::windows::core::HRESULT,
    pub DestAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppappname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CalledParty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcalledparty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcomment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITScriptableAudioFormat(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITScriptableAudioFormat {
    pub unsafe fn Channels(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Channels)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetChannels(&self, nnewval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetChannels)(::windows::core::Vtable::as_raw(self), nnewval).ok()
    }
    pub unsafe fn SamplesPerSec(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SamplesPerSec)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSamplesPerSec(&self, nnewval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSamplesPerSec)(::windows::core::Vtable::as_raw(self), nnewval).ok()
    }
    pub unsafe fn AvgBytesPerSec(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AvgBytesPerSec)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAvgBytesPerSec(&self, nnewval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAvgBytesPerSec)(::windows::core::Vtable::as_raw(self), nnewval).ok()
    }
    pub unsafe fn BlockAlign(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BlockAlign)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBlockAlign(&self, nnewval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBlockAlign)(::windows::core::Vtable::as_raw(self), nnewval).ok()
    }
    pub unsafe fn BitsPerSample(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BitsPerSample)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBitsPerSample(&self, nnewval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBitsPerSample)(::windows::core::Vtable::as_raw(self), nnewval).ok()
    }
    pub unsafe fn FormatTag(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FormatTag)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFormatTag(&self, nnewval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFormatTag)(::windows::core::Vtable::as_raw(self), nnewval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITScriptableAudioFormat, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITScriptableAudioFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITScriptableAudioFormat {
    type Vtable = ITScriptableAudioFormat_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITScriptableAudioFormat {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb87658bd_3c59_4f64_be74_aede3e86a81e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITScriptableAudioFormat_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Channels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT,
    pub SamplesPerSec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetSamplesPerSec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT,
    pub AvgBytesPerSec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAvgBytesPerSec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT,
    pub BlockAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetBlockAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT,
    pub BitsPerSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetBitsPerSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT,
    pub FormatTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetFormatTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITStaticAudioTerminal(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITStaticAudioTerminal {
    pub unsafe fn WaveId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WaveId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITStaticAudioTerminal, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITStaticAudioTerminal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITStaticAudioTerminal {
    type Vtable = ITStaticAudioTerminal_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITStaticAudioTerminal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa86b7871_d14c_48e6_922e_a8d15f984800);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITStaticAudioTerminal_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub WaveId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plwaveid: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITStream {
    pub unsafe fn MediaType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MediaType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Direction(&self) -> ::windows::core::Result<TERMINAL_DIRECTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Direction)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StartStream(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StartStream)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PauseStream(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PauseStream)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn StopStream(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StopStream)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectTerminal<P0>(&self, pterminal: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITTerminal>>,
    {
        (::windows::core::Vtable::vtable(self).SelectTerminal)(::windows::core::Vtable::as_raw(self), pterminal.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnselectTerminal<P0>(&self, pterminal: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITTerminal>>,
    {
        (::windows::core::Vtable::vtable(self).UnselectTerminal)(::windows::core::Vtable::as_raw(self), pterminal.into().abi()).ok()
    }
    pub unsafe fn EnumerateTerminals(&self) -> ::windows::core::Result<IEnumTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateTerminals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Terminals(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Terminals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITStream, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITStream {
    type Vtable = ITStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee3bd605_3868_11d2_a045_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITStream_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptd: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PauseStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectTerminal: usize,
    pub EnumerateTerminals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Terminals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Terminals: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITStreamControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITStreamControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream(&self, lmediatype: i32, td: TERMINAL_DIRECTION) -> ::windows::core::Result<ITStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStream)(::windows::core::Vtable::as_raw(self), lmediatype, td, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveStream<P0>(&self, pstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITStream>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveStream)(::windows::core::Vtable::as_raw(self), pstream.into().abi()).ok()
    }
    pub unsafe fn EnumerateStreams(&self) -> ::windows::core::Result<IEnumStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateStreams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Streams(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Streams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITStreamControl, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITStreamControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITStreamControl {
    type Vtable = ITStreamControl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITStreamControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee3bd604_3868_11d2_a045_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITStreamControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32, td: TERMINAL_DIRECTION, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveStream: usize,
    pub EnumerateStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Streams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Streams: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITSubStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITSubStream {
    pub unsafe fn StartSubStream(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StartSubStream)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PauseSubStream(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PauseSubStream)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn StopSubStream(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StopSubStream)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectTerminal<P0>(&self, pterminal: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITTerminal>>,
    {
        (::windows::core::Vtable::vtable(self).SelectTerminal)(::windows::core::Vtable::as_raw(self), pterminal.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnselectTerminal<P0>(&self, pterminal: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITTerminal>>,
    {
        (::windows::core::Vtable::vtable(self).UnselectTerminal)(::windows::core::Vtable::as_raw(self), pterminal.into().abi()).ok()
    }
    pub unsafe fn EnumerateTerminals(&self) -> ::windows::core::Result<IEnumTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateTerminals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Terminals(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Terminals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Stream(&self) -> ::windows::core::Result<ITStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Stream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITSubStream, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITSubStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITSubStream {
    type Vtable = ITSubStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITSubStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee3bd608_3868_11d2_a045_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITSubStream_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartSubStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PauseSubStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopSubStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectTerminal: usize,
    pub EnumerateTerminals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Terminals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Terminals: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Stream: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITSubStreamControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITSubStreamControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSubStream(&self) -> ::windows::core::Result<ITSubStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSubStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveSubStream<P0>(&self, psubstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITSubStream>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveSubStream)(::windows::core::Vtable::as_raw(self), psubstream.into().abi()).ok()
    }
    pub unsafe fn EnumerateSubStreams(&self) -> ::windows::core::Result<IEnumSubStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateSubStreams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SubStreams(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SubStreams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITSubStreamControl, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITSubStreamControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITSubStreamControl {
    type Vtable = ITSubStreamControl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITSubStreamControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee3bd607_3868_11d2_a045_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITSubStreamControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSubStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSubStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveSubStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveSubStream: usize,
    pub EnumerateSubStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumsubstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SubStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SubStreams: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITTAPI(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPI {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Addresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Addresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateAddresses(&self) -> ::windows::core::Result<IEnumAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RegisterCallNotifications<P0, P1, P2>(&self, paddress: P0, fmonitor: P1, fowner: P2, lmediatypes: i32, lcallbackinstance: i32) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITAddress>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterCallNotifications)(::windows::core::Vtable::as_raw(self), paddress.into().abi(), fmonitor.into(), fowner.into(), lmediatypes, lcallbackinstance, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterNotifications(&self, lregister: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterNotifications)(::windows::core::Vtable::as_raw(self), lregister).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CallHubs(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallHubs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateCallHubs(&self) -> ::windows::core::Result<IEnumCallHub> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateCallHubs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCallHubTracking<P0>(&self, paddresses: super::super::System::Com::VARIANT, btracking: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetCallHubTracking)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(paddresses), btracking.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumeratePrivateTAPIObjects(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumeratePrivateTAPIObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PrivateTAPIObjects(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrivateTAPIObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterRequestRecipient<P0>(&self, lregistrationinstance: i32, lrequestmode: i32, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).RegisterRequestRecipient)(::windows::core::Vtable::as_raw(self), lregistrationinstance, lrequestmode, fenable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAssistedTelephonyPriority<P0>(&self, pappfilename: &::windows::core::BSTR, fpriority: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAssistedTelephonyPriority)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pappfilename), fpriority.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplicationPriority<P0>(&self, pappfilename: &::windows::core::BSTR, lmediatype: i32, fpriority: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetApplicationPriority)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pappfilename), lmediatype, fpriority.into()).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfiltermask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEventFilter)(::windows::core::Vtable::as_raw(self), lfiltermask).ok()
    }
    pub unsafe fn EventFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EventFilter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITTAPI, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITTAPI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITTAPI {
    type Vtable = ITTAPI_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITTAPI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1efc382_9355_11d0_835c_00aa003ccabd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITTAPI_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Addresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Addresses: usize,
    pub EnumerateAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RegisterCallNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, fmonitor: super::super::Foundation::VARIANT_BOOL, fowner: super::super::Foundation::VARIANT_BOOL, lmediatypes: i32, lcallbackinstance: i32, plregister: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RegisterCallNotifications: usize,
    pub UnregisterNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lregister: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CallHubs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CallHubs: usize,
    pub EnumerateCallHubs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumcallhub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCallHubTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddresses: super::super::System::Com::VARIANT, btracking: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCallHubTracking: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumeratePrivateTAPIObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumeratePrivateTAPIObjects: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PrivateTAPIObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PrivateTAPIObjects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterRequestRecipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lregistrationinstance: i32, lrequestmode: i32, fenable: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterRequestRecipient: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAssistedTelephonyPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappfilename: *mut ::core::ffi::c_void, fpriority: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAssistedTelephonyPriority: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetApplicationPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappfilename: *mut ::core::ffi::c_void, lmediatype: i32, fpriority: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetApplicationPriority: usize,
    pub SetEventFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfiltermask: i32) -> ::windows::core::HRESULT,
    pub EventFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plfiltermask: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITTAPI2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPI2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Phones(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Phones)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumeratePhones(&self) -> ::windows::core::Result<IEnumPhone> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumeratePhones)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEmptyCollectionObject(&self) -> ::windows::core::Result<ITCollection2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEmptyCollectionObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITTAPI2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ITTAPI);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITTAPI2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITTAPI2 {
    type Vtable = ITTAPI2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITTAPI2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54fbdc8c_d90f_4dad_9695_b373097f094b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITTAPI2_Vtbl {
    pub base__: ITTAPI_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Phones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Phones: usize,
    pub EnumeratePhones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumphone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEmptyCollectionObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEmptyCollectionObject: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITTAPICallCenter(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPICallCenter {
    pub unsafe fn EnumerateAgentHandlers(&self) -> ::windows::core::Result<IEnumAgentHandler> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateAgentHandlers)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AgentHandlers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AgentHandlers)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITTAPICallCenter, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITTAPICallCenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITTAPICallCenter {
    type Vtable = ITTAPICallCenter_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITTAPICallCenter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5afc3154_4bcc_11d1_bf80_00805fc147d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITTAPICallCenter_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EnumerateAgentHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumhandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AgentHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AgentHandlers: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITTAPIDispatchEventNotification(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIDispatchEventNotification {}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITTAPIDispatchEventNotification, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITTAPIDispatchEventNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITTAPIDispatchEventNotification {
    type Vtable = ITTAPIDispatchEventNotification_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITTAPIDispatchEventNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f34325b_7e62_11d2_9457_00c04f8ec888);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITTAPIDispatchEventNotification_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct ITTAPIEventNotification(::windows::core::IUnknown);
impl ITTAPIEventNotification {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Event<P0>(&self, tapievent: TAPI_EVENT, pevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), tapievent, pevent.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ITTAPIEventNotification, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITTAPIEventNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITTAPIEventNotification {
    type Vtable = ITTAPIEventNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for ITTAPIEventNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeddb9426_3b91_11d1_8f30_00c04fb6809f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITTAPIEventNotification_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, pevent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Event: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITTAPIObjectEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIObjectEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TAPIObject(&self) -> ::windows::core::Result<ITTAPI> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TAPIObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<TAPIOBJECT_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<ITAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Address)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallbackInstance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallbackInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITTAPIObjectEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITTAPIObjectEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITTAPIObjectEvent {
    type Vtable = ITTAPIObjectEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITTAPIObjectEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4854d48_937a_11d1_bb58_00c04fb6809f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITTAPIObjectEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub TAPIObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptapiobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TAPIObject: usize,
    pub Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevent: *mut TAPIOBJECT_EVENT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITTAPIObjectEvent2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIObjectEvent2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Phone(&self) -> ::windows::core::Result<ITPhone> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Phone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITTAPIObjectEvent2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ITTAPIObjectEvent);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITTAPIObjectEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITTAPIObjectEvent2 {
    type Vtable = ITTAPIObjectEvent2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITTAPIObjectEvent2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x359dda6e_68ce_4383_bf0b_169133c41b46);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITTAPIObjectEvent2_Vtbl {
    pub base__: ITTAPIObjectEvent_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Phone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Phone: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITTTSTerminalEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITTTSTerminalEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Terminal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITTTSTerminalEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITTTSTerminalEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITTTSTerminalEvent {
    type Vtable = ITTTSTerminalEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITTTSTerminalEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd964788f_95a5_461d_ab0c_b9900a6c2713);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITTTSTerminalEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITTerminal(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITTerminal {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<TERMINAL_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TerminalType(&self) -> ::windows::core::Result<TERMINAL_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TerminalType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TerminalClass(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TerminalClass)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MediaType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MediaType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Direction(&self) -> ::windows::core::Result<TERMINAL_DIRECTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Direction)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITTerminal, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITTerminal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITTerminal {
    type Vtable = ITTerminal_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITTerminal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1efc38a_9355_11d0_835c_00aa003ccabd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITTerminal_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminalstate: *mut TERMINAL_STATE) -> ::windows::core::HRESULT,
    pub TerminalType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut TERMINAL_TYPE) -> ::windows::core::HRESULT,
    pub TerminalClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppterminalclass: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITTerminalSupport(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITTerminalSupport {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StaticTerminals(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StaticTerminals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateStaticTerminals(&self) -> ::windows::core::Result<IEnumTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateStaticTerminals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DynamicTerminalClasses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DynamicTerminalClasses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateDynamicTerminalClasses(&self) -> ::windows::core::Result<IEnumTerminalClass> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumerateDynamicTerminalClasses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTerminal(&self, pterminalclass: &::windows::core::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTerminal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pterminalclass), lmediatype, direction, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDefaultStaticTerminal(&self, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDefaultStaticTerminal)(::windows::core::Vtable::as_raw(self), lmediatype, direction, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITTerminalSupport, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITTerminalSupport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITTerminalSupport {
    type Vtable = ITTerminalSupport_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITTerminalSupport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1efc385_9355_11d0_835c_00aa003ccabd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITTerminalSupport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StaticTerminals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StaticTerminals: usize,
    pub EnumerateStaticTerminals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppterminalenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DynamicTerminalClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DynamicTerminalClasses: usize,
    pub EnumerateDynamicTerminalClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppterminalclassenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pterminalclass: *mut ::core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDefaultStaticTerminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDefaultStaticTerminal: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITTerminalSupport2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITTerminalSupport2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PluggableSuperclasses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PluggableSuperclasses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumeratePluggableSuperclasses(&self) -> ::windows::core::Result<IEnumPluggableSuperclassInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumeratePluggableSuperclasses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_PluggableTerminalClasses(&self, bstrterminalsuperclass: &::windows::core::BSTR, lmediatype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_PluggableTerminalClasses)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrterminalsuperclass), lmediatype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumeratePluggableTerminalClasses(&self, iidterminalsuperclass: ::windows::core::GUID, lmediatype: i32) -> ::windows::core::Result<IEnumPluggableTerminalClassInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumeratePluggableTerminalClasses)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(iidterminalsuperclass), lmediatype, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITTerminalSupport2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ITTerminalSupport);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITTerminalSupport2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITTerminalSupport2 {
    type Vtable = ITTerminalSupport2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITTerminalSupport2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3eb39bc_1b1f_4e99_a0c0_56305c4dd591);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITTerminalSupport2_Vtbl {
    pub base__: ITTerminalSupport_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PluggableSuperclasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PluggableSuperclasses: usize,
    pub EnumeratePluggableSuperclasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsuperclassenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_PluggableTerminalClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrterminalsuperclass: *mut ::core::ffi::c_void, lmediatype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_PluggableTerminalClasses: usize,
    pub EnumeratePluggableTerminalClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iidterminalsuperclass: ::windows::core::GUID, lmediatype: i32, ppclassenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITToneDetectionEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITToneDetectionEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AppSpecific)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TickCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TickCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallbackInstance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CallbackInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITToneDetectionEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITToneDetectionEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITToneDetectionEvent {
    type Vtable = ITToneDetectionEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITToneDetectionEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x407e0faf_d047_4753_b0c6_8e060373fecd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITToneDetectionEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT,
    pub TickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITToneTerminalEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITToneTerminalEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Terminal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> ::windows::core::Result<ITCallInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Call)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(ITToneTerminalEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITToneTerminalEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for ITToneTerminalEvent {
    type Vtable = ITToneTerminalEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITToneTerminalEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6f56009_611f_4945_bbd2_2d0ce5612056);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITToneTerminalEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
pub struct ITnef(::windows::core::IUnknown);
impl ITnef {
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub unsafe fn AddProps(&self, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddProps)(::windows::core::Vtable::as_raw(self), ulflags, ulelemid, lpvdata, lpproplist).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub unsafe fn ExtractProps(&self, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ExtractProps)(::windows::core::Vtable::as_raw(self), ulflags, lpproplist, lpproblems).ok()
    }
    pub unsafe fn Finish(&self, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Finish)(::windows::core::Vtable::as_raw(self), ulflags, lpkey, lpproblems).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub unsafe fn OpenTaggedBody<P0>(&self, lpmessage: P0, ulflags: u32) -> ::windows::core::Result<super::super::System::Com::IStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::AddressBook::IMessage>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenTaggedBody)(::windows::core::Vtable::as_raw(self), lpmessage.into().abi(), ulflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProps)(::windows::core::Vtable::as_raw(self), ulflags, ulelemid, cvalues, lpprops).ok()
    }
    #[doc = "*Required features: `\"Win32_System_AddressBook\"`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub unsafe fn EncodeRecips<P0>(&self, ulflags: u32, lprecipienttable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::AddressBook::IMAPITable>>,
    {
        (::windows::core::Vtable::vtable(self).EncodeRecips)(::windows::core::Vtable::as_raw(self), ulflags, lprecipienttable.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub unsafe fn FinishComponent(&self, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FinishComponent)(::windows::core::Vtable::as_raw(self), ulflags, ulcomponentid, lpcustomproplist, lpcustomprops, lpproplist, lpproblems).ok()
    }
}
::windows::core::interface_hierarchy!(ITnef, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITnef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITnef {
    type Vtable = ITnef_Vtbl;
}
unsafe impl ::windows::core::Interface for ITnef {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITnef_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_AddressBook")]
    pub AddProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_AddressBook"))]
    AddProps: usize,
    #[cfg(feature = "Win32_System_AddressBook")]
    pub ExtractProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_AddressBook"))]
    ExtractProps: usize,
    pub Finish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub OpenTaggedBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpmessage: *mut ::core::ffi::c_void, ulflags: u32, lppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com")))]
    OpenTaggedBody: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub SetProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com")))]
    SetProps: usize,
    #[cfg(feature = "Win32_System_AddressBook")]
    pub EncodeRecips: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, lprecipienttable: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_AddressBook"))]
    EncodeRecips: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub FinishComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com")))]
    FinishComponent: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DISPIDMASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DispatchMapper: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9225296_c759_11d1_a02b_00c04fb6809f);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const GETTNEFSTREAMCODEPAGE: ::windows::core::PCSTR = ::windows::s!("GetTnefStreamCodePage");
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPADDRESS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPADDRESSCAPABILITIES: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPADDRESSTRANSLATION: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPAGGREGATEDMSPADDRESSOBJ: u32 = 393216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPAGGREGATEDMSPCALLOBJ: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPAPC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPBASICCALLCONTROL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPCALLINFO: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPDIRECTORY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPDIROBJCONFERENCE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPDIROBJECT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPDIROBJUSER: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPFILETRACK: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPILSCONFIG: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPLEGACYADDRESSMEDIACONTROL: u32 = 327680u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPLEGACYCALLMEDIACONTROL: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPMEDIACONTROL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPMEDIAPLAYBACK: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPMEDIARECORD: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPMEDIASUPPORT: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPMULTITRACK: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPPHONE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPTAPI: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const IDISPTAPICALLCENTER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const INITIALIZE_NEGOTIATION: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const INTERFACEMASK: u32 = 16711680u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LAST_LINEMEDIAMODE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LAST_LINEREQUESTMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_ACCEPTTOALERT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_ACDGROUP: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_AUTORECONNECT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_BLOCKIDDEFAULT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_BLOCKIDOVERRIDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_COMPLETIONID: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_CONFDROP: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_CONFERENCEHELD: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_CONFERENCEMAKE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_DESTOFFHOOK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_DIALED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_FWDBUSYNAADDR: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_FWDCONSULT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_FWDINTEXTADDR: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_FWDNUMRINGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_FWDSTATUSVALID: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_HOLDMAKESNEW: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_NOEXTERNALCALLS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_NOINTERNALCALLS: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_NOPSTNADDRESSTRANSLATION: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_ORIGOFFHOOK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_PARTIALDIAL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_PICKUPCALLWAIT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_PICKUPGROUPID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_PREDICTIVEDIALER: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_QUEUE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_ROUTEPOINT: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_SECURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_SETCALLINGID: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_SETUPCONFNULL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_TRANSFERHELD: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRCAPFLAGS_TRANSFERMAKE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSMODE_ADDRESSID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSMODE_DIALABLEADDR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSHARING_BRIDGEDEXCL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSHARING_BRIDGEDNEW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSHARING_BRIDGEDSHARED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSHARING_MONITORED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSHARING_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_CAPSCHANGE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_DEVSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_FORWARD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_INUSEMANY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_INUSEONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_INUSEZERO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_NUMCALLS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSSTATE_TERMINALS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSTYPE_DOMAINNAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSTYPE_EMAILNAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSTYPE_IPADDRESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSTYPE_PHONENUMBER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRESSTYPE_SDP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_FORWARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_FORWARDDND: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_FORWARDFWD: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_MAKECALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_PICKUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_PICKUPDIRECT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_PICKUPGROUP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_PICKUPHELD: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_PICKUPWAITING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_SETMEDIACONTROL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_SETTERMINAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_SETUPCONF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_UNCOMPLETECALL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEADDRFEATURE_UNPARK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_AGENTSPECIFIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_GETAGENTACTIVITYLIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_GETAGENTGROUP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_SETAGENTACTIVITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_SETAGENTGROUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTFEATURE_SETAGENTSTATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_BUSYONCALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_BUSYWRAPUP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_ENDED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_NOTREADY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_READY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATE_RELEASED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATUS_NEWSESSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATUS_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSESSIONSTATUS_UPDATEINFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_BUSYACD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_BUSYINCOMING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_BUSYOUTGOING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_NOTREADY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_READY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_RELEASED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATEEX_UNKNOWN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_BUSYACD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_BUSYINCOMING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_BUSYOTHER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_BUSYOUTBOUND: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_LOGGEDOFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_NOTREADY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_READY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_UNAVAIL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_UNKNOWN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATE_WORKINGAFTERCALL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUSEX_NEWAGENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUSEX_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUSEX_UPDATEINFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_ACTIVITY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_ACTIVITYLIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_CAPSCHANGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_GROUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_GROUPLIST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_NEXTSTATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_VALIDNEXTSTATES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEAGENTSTATUS_VALIDSTATES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEANSWERMODE_DROP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEANSWERMODE_HOLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEANSWERMODE_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_ALTSPEECHDATA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_DATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_MULTIUSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_NONCALLSIGNALING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_PASSTHROUGH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_RESTRICTEDDATA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_SPEECH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBEARERMODE_VOICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBUSYMODE_STATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBUSYMODE_TRUNK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBUSYMODE_UNAVAIL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEBUSYMODE_UNKNOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLCOND_BUSY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLCOND_NOANSWER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLMODE_CALLBACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLMODE_CAMPON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLMODE_INTRUDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLCOMPLMODE_MESSAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_COMPLCALLBACK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_COMPLCAMPON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_COMPLINTRUDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_COMPLMESSAGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_NOHOLDCONFERENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_ONESTEPTRANSFER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_PARKDIRECT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_PARKNONDIRECT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_TRANSFERCONF: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE2_TRANSFERNORM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_ACCEPT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_ADDTOCONF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_ANSWER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_BLINDTRANSFER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_COMPLETECALL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_COMPLETETRANSF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_DIAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_DROP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_GATHERDIGITS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_GENERATEDIGITS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_GENERATETONE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_HOLD: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_MONITORDIGITS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_MONITORMEDIA: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_MONITORTONES: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_PARK: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_PREPAREADDCONF: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_REDIRECT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_RELEASEUSERUSERINFO: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_REMOVEFROMCONF: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SECURECALL: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SENDUSERUSER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETCALLDATA: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETCALLPARAMS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETMEDIACONTROL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETQOS: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETTERMINAL: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETTREATMENT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETUPCONF: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SETUPTRANSFER: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_SWAPHOLD: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLFEATURE_UNHOLD: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLHUBTRACKING_ALLCALLS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLHUBTRACKING_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLHUBTRACKING_PROVIDERLEVEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_APPSPECIFIC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_BEARERMODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CALLDATA: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CALLEDID: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CALLERID: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CALLID: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CHARGINGINFO: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_COMPLETIONID: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_CONNECTEDID: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_DEVSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_DIALPARAMS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_DISPLAY: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_HIGHLEVELCOMP: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_LOWLEVELCOMP: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_MEDIAMODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_MONITORMODES: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_NUMMONITORS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_NUMOWNERDECR: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_NUMOWNERINCR: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_ORIGIN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_QOS: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_RATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_REASON: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_REDIRECTINGID: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_REDIRECTIONID: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_RELATEDCALLID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_TERMINAL: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_TREATMENT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_TRUNK: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLINFOSTATE_USERUSERINFO: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_CONFERENCE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_EXTERNAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_INBOUND: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_INTERNAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_OUTBOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_UNAVAIL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLORIGIN_UNKNOWN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_BLOCKID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_DESTOFFHOOK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_IDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_NOHOLDCONFERENCE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_ONESTEPTRANSFER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_ORIGOFFHOOK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_PREDICTIVEDIAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARAMFLAGS_SECURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_BLOCKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_OUTOFAREA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_PARTIAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_UNAVAIL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPARTYID_UNKNOWN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPRIVILEGE_MONITOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPRIVILEGE_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLPRIVILEGE_OWNER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_CALLCOMPLETION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_CAMPEDON: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_DIRECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_FWDBUSY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_FWDNOANSWER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_FWDUNCOND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_INTRUDE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_PARKED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_PICKUP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_REDIRECT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_REMINDER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_ROUTEREQUEST: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_TRANSFER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_UNAVAIL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_UNKNOWN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLREASON_UNPARK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSELECT_ADDRESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSELECT_CALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSELECT_CALLID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSELECT_DEVICEID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSELECT_LINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_ACCEPTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_BUSY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_CONFERENCED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_CONNECTED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_DIALING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_DIALTONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_DISCONNECTED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_IDLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_OFFERING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_ONHOLD: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_ONHOLDPENDCONF: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_ONHOLDPENDTRANSFER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_PROCEEDING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_RINGBACK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_SPECIALINFO: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLSTATE_UNKNOWN: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLTREATMENT_BUSY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLTREATMENT_MUSIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLTREATMENT_RINGBACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECALLTREATMENT_SILENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECARDOPTION_HIDDEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECARDOPTION_PREDEFINED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECONNECTEDMODE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECONNECTEDMODE_ACTIVEHELD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECONNECTEDMODE_CONFIRMED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECONNECTEDMODE_INACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINECONNECTEDMODE_INACTIVEHELD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_CALLHUB: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_CALLHUBTRACKING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_CLOSEDROP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_CROSSADDRCONF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_DIALBILLING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_DIALDIALTONE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_DIALQUIET: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_HIGHLEVCOMP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_LOCAL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_LOWLEVCOMP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_MEDIACONTROL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_MSP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_MULTIPLEADDR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVCAPFLAGS_PRIVATEOBJECTS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_BATTERY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_CAPSCHANGE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_CLOSE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_COMPLCANCEL: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_CONFIGCHANGE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_CONNECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_DEVSPECIFIC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_DISCONNECTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_INSERVICE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_LOCK: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_MAINTENANCE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_MSGWAITOFF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_MSGWAITON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_NUMCALLS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_NUMCOMPLETIONS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_OPEN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_OUTOFSERVICE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_REINIT: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_REMOVED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_RINGING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_ROAMMODE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_SIGNAL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_TERMINALS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATE_TRANSLATECHANGE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATUSFLAGS_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATUSFLAGS_INSERVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATUSFLAGS_LOCKED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDEVSTATUSFLAGS_MSGWAIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_EXTERNAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_INTERNAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_SPECIAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_UNAVAIL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIALTONEMODE_UNKNOWN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIGITMODE_DTMF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIGITMODE_DTMFEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDIGITMODE_PULSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_BADADDRESS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_BLOCKED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_BUSY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_CANCELLED: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_CONGESTION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_DESTINATIONBARRED: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_DONOTDISTURB: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_FDNRESTRICT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_FORWARDED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_INCOMPATIBLE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_NOANSWER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_NODIALTONE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_NUMBERCHANGED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_OUTOFORDER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_PICKUP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_QOSUNAVAIL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_REJECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_TEMPFAILURE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_UNAVAIL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEDISCONNECTMODE_UNREACHABLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEEQOSINFO_ADMISSIONFAILURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEEQOSINFO_GENERICERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEEQOSINFO_NOQOS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEEQOSINFO_POLICYFAILURE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_ADDRESSBLOCKED: u32 = 2147483731u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_ALLOCATED: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_BADDEVICEID: u32 = 2147483650u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_BEARERMODEUNAVAIL: u32 = 2147483651u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_BILLINGREJECTED: u32 = 2147483732u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_CALLUNAVAIL: u32 = 2147483653u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_COMPLETIONOVERRUN: u32 = 2147483654u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_CONFERENCEFULL: u32 = 2147483655u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DIALBILLING: u32 = 2147483656u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DIALDIALTONE: u32 = 2147483657u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DIALPROMPT: u32 = 2147483658u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DIALQUIET: u32 = 2147483659u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DIALVOICEDETECT: u32 = 2147483740u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_DISCONNECTED: u32 = 2147483744u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INCOMPATIBLEAPIVERSION: u32 = 2147483660u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INCOMPATIBLEEXTVERSION: u32 = 2147483661u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INIFILECORRUPT: u32 = 2147483662u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INUSE: u32 = 2147483663u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALADDRESS: u32 = 2147483664u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALADDRESSID: u32 = 2147483665u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALADDRESSMODE: u32 = 2147483666u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALADDRESSSTATE: u32 = 2147483667u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALADDRESSTYPE: u32 = 2147483742u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAGENTACTIVITY: u32 = 2147483739u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAGENTGROUP: u32 = 2147483736u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAGENTID: u32 = 2147483735u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAGENTSESSIONSTATE: u32 = 2147483743u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAGENTSTATE: u32 = 2147483738u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAPPHANDLE: u32 = 2147483668u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALAPPNAME: u32 = 2147483669u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALBEARERMODE: u32 = 2147483670u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLCOMPLMODE: u32 = 2147483671u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLHANDLE: u32 = 2147483672u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLPARAMS: u32 = 2147483673u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLPRIVILEGE: u32 = 2147483674u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLSELECT: u32 = 2147483675u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLSTATE: u32 = 2147483676u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCALLSTATELIST: u32 = 2147483677u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCARD: u32 = 2147483678u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCOMPLETIONID: u32 = 2147483679u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCONFCALLHANDLE: u32 = 2147483680u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCONSULTCALLHANDLE: u32 = 2147483681u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALCOUNTRYCODE: u32 = 2147483682u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDEVICECLASS: u32 = 2147483683u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDEVICEHANDLE: u32 = 2147483684u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDIALPARAMS: u32 = 2147483685u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDIGITLIST: u32 = 2147483686u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDIGITMODE: u32 = 2147483687u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALDIGITS: u32 = 2147483688u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALEXTVERSION: u32 = 2147483689u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALFEATURE: u32 = 2147483733u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALGROUPID: u32 = 2147483690u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALLINEHANDLE: u32 = 2147483691u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALLINESTATE: u32 = 2147483692u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALLOCATION: u32 = 2147483693u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALMEDIALIST: u32 = 2147483694u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALMEDIAMODE: u32 = 2147483695u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALMESSAGEID: u32 = 2147483696u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPARAM: u32 = 2147483698u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPARKID: u32 = 2147483699u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPARKMODE: u32 = 2147483700u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPASSWORD: u32 = 2147483737u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPOINTER: u32 = 2147483701u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALPRIVSELECT: u32 = 2147483702u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALRATE: u32 = 2147483703u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALREQUESTMODE: u32 = 2147483704u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTERMINALID: u32 = 2147483705u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTERMINALMODE: u32 = 2147483706u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTIMEOUT: u32 = 2147483707u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTONE: u32 = 2147483708u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTONELIST: u32 = 2147483709u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTONEMODE: u32 = 2147483710u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_INVALTRANSFERMODE: u32 = 2147483711u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_LINEMAPPERFAILED: u32 = 2147483712u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOCONFERENCE: u32 = 2147483713u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NODEVICE: u32 = 2147483714u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NODRIVER: u32 = 2147483715u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOMEM: u32 = 2147483716u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOMULTIPLEINSTANCE: u32 = 2147483734u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOREQUEST: u32 = 2147483717u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOTOWNER: u32 = 2147483718u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_NOTREGISTERED: u32 = 2147483719u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_OPERATIONFAILED: u32 = 2147483720u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_OPERATIONUNAVAIL: u32 = 2147483721u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_RATEUNAVAIL: u32 = 2147483722u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_REINIT: u32 = 2147483730u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_REQUESTOVERRUN: u32 = 2147483724u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_RESOURCEUNAVAIL: u32 = 2147483723u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_SERVICE_NOT_RUNNING: u32 = 2147483745u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_STRUCTURETOOSMALL: u32 = 2147483725u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_TARGETNOTFOUND: u32 = 2147483726u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_TARGETSELF: u32 = 2147483727u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_UNINITIALIZED: u32 = 2147483728u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_USERCANCELLED: u32 = 2147483741u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEERR_USERUSERINFOTOOBIG: u32 = 2147483729u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_DEVSPECIFIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_DEVSPECIFICFEAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_FORWARD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_FORWARDDND: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_FORWARDFWD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_MAKECALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_SETDEVSTATUS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_SETMEDIACONTROL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFEATURE_SETTERMINAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYEXTERNAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYINTERNAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYNA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYNAEXTERNAL: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYNAINTERNAL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYNASPECIFIC: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_BUSYSPECIFIC: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_NOANSW: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_NOANSWEXTERNAL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_NOANSWINTERNAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_NOANSWSPECIFIC: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNAVAIL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNCOND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNCONDEXTERNAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNCONDINTERNAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNCONDSPECIFIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEFORWARDMODE_UNKNOWN: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGATHERTERM_BUFFERFULL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGATHERTERM_CANCEL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGATHERTERM_FIRSTTIMEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGATHERTERM_INTERTIMEOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGATHERTERM_TERMDIGIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGENERATETERM_CANCEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGENERATETERM_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGROUPSTATUS_GROUPREMOVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEGROUPSTATUS_NEWGROUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEINITIALIZEEXOPTION_CALLHUBTRACKING: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEINITIALIZEEXOPTION_USECOMPLETIONPORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEINITIALIZEEXOPTION_USEEVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEINITIALIZEEXOPTION_USEHIDDENWINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINELOCATIONOPTION_PULSEDIAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMAPPER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_PAUSE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_RATEDOWN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_RATENORMAL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_RATEUP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_RESET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_RESUME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_START: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_VOLUMEDOWN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_VOLUMENORMAL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIACONTROL_VOLUMEUP: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_ADSI: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_AUTOMATEDVOICE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_DATAMODEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_DIGITALDATA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_G3FAX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_G4FAX: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_INTERACTIVEVOICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_MIXED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_TDD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_TELETEX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_TELEX: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_VIDEO: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_VIDEOTEX: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEMEDIAMODE_VOICEVIEW: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEOFFERINGMODE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEOFFERINGMODE_INACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEOPENOPTION_PROXY: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEOPENOPTION_SINGLEADDRESS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPARKMODE_DIRECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPARKMODE_NONDIRECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_AGENTSPECIFIC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_CREATEAGENT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_CREATEAGENTSESSION: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTACTIVITYLIST: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTCAPS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTGROUPLIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTINFO: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTSESSIONINFO: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTSESSIONLIST: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETAGENTSTATUS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETGROUPLIST: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETQUEUEINFO: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_GETQUEUELIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTACTIVITY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTGROUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTMEASUREMENTPERIOD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTSESSIONSTATE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTSTATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETAGENTSTATEEX: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYREQUEST_SETQUEUEMEASUREMENTPERIOD: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYSTATUS_ALLOPENFORACD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYSTATUS_CLOSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEPROXYSTATUS_OPEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQOSREQUESTTYPE_SERVICELEVEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQOSSERVICELEVEL_BESTEFFORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQOSSERVICELEVEL_IFAVAILABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQOSSERVICELEVEL_NEEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQUEUESTATUS_NEWQUEUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQUEUESTATUS_QUEUEREMOVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEQUEUESTATUS_UPDATEINFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREMOVEFROMCONF_ANY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREMOVEFROMCONF_LAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREMOVEFROMCONF_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREQUESTMODE_DROP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREQUESTMODE_MAKECALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEREQUESTMODE_MEDIACALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEROAMMODE_HOME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEROAMMODE_ROAMA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEROAMMODE_ROAMB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEROAMMODE_UNAVAIL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINEROAMMODE_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINESPECIALINFO_CUSTIRREG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINESPECIALINFO_NOCIRCUIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINESPECIALINFO_REORDER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINESPECIALINFO_UNAVAIL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINESPECIALINFO_UNKNOWN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMDEV_HEADSET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMDEV_PHONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMDEV_SPEAKER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_BUTTONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_DISPLAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_HOOKSWITCH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_LAMPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_MEDIABIDIRECT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_MEDIAFROMLINE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_MEDIATOLINE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMMODE_RINGER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMSHARING_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMSHARING_SHAREDCONF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETERMSHARING_SHAREDEXCL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETOLLLISTOPTION_ADD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETOLLLISTOPTION_REMOVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETONEMODE_BEEP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETONEMODE_BILLING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETONEMODE_BUSY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETONEMODE_CUSTOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETONEMODE_RINGBACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSFERMODE_CONFERENCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSFERMODE_TRANSFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATEOPTION_CANCELCALLWAITING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATEOPTION_CARDOVERRIDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATEOPTION_FORCELD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATEOPTION_FORCELOCAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_CANONICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_DIALBILLING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_DIALDIALTONE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_DIALPROMPT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_DIALQUIET: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_INTERNATIONAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_INTOLLLIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_LOCAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_LONGDISTANCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_NOTINTOLLLIST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_NOTRANSLATION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETRANSLATERESULT_VOICEDETECT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINETSPIOPTION_NONREENTRANT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_ADDRESSSTATE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_AGENTSESSIONSTATUS: i32 = 27i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_AGENTSPECIFIC: i32 = 21i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_AGENTSTATUS: i32 = 22i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_AGENTSTATUSEX: i32 = 29i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_APPNEWCALL: i32 = 23i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_APPNEWCALLHUB: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_CALLHUBCLOSE: i32 = 33i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_CALLINFO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_CALLSTATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_CLOSE: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_CREATE: i32 = 19i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_DEVSPECIFIC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_DEVSPECIFICEX: i32 = 34i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_DEVSPECIFICFEATURE: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_GATHERDIGITS: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_GENERATE: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_GROUPSTATUS: i32 = 30i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_LINEDEVSTATE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_MONITORDIGITS: i32 = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_MONITORMEDIA: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_MONITORTONE: i32 = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_PROXYREQUEST: i32 = 24i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_PROXYSTATUS: i32 = 31i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_QUEUESTATUS: i32 = 28i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_REMOVE: i32 = 25i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_REPLY: i32 = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LINE_REQUEST: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const McastAddressAllocation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0daef2_a289_11d1_8697_006008b0e5d2);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const OPENTNEFSTREAM: ::windows::core::PCSTR = ::windows::s!("OpenTnefStream");
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const OPENTNEFSTREAMEX: ::windows::core::PCSTR = ::windows::s!("OpenTnefStreamEx");
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_ABBREVDIAL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_BRIDGEDAPP: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_BUSY: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_CALLAPP: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_CALLID: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_CAMPON: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_CONFERENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_CONNECT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_COVER: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DATAOFF: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DATAON: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DATETIME: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DIRECTORY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DISCONNECT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DONOTDISTURB: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_DROP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_FLASH: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_FORWARD: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_HOLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_INTERCOM: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_LASTNUM: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_MSGINDICATOR: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_MSGWAITOFF: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_MSGWAITON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_MUTE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_NIGHTSRV: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_NONE: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_PARK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_PICKUP: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_QUEUECALL: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_RECALL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_REDIRECT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_REJECT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_REPDIAL: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_RINGAGAIN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SAVEREPEAT: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SELECTRING: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SEND: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SENDCALLS: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SETREPDIAL: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SPEAKEROFF: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SPEAKERON: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_STATIONSPEED: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_SYSTEMSPEED: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_TRANSFER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_VOLUMEDOWN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONFUNCTION_VOLUMEUP: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_CALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_DISPLAY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_DUMMY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_FEATURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_KEYPAD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONMODE_LOCAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONSTATE_DOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONSTATE_UNAVAIL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONSTATE_UNKNOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEBUTTONSTATE_UP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_ALLOCATED: u32 = 2415919105u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_BADDEVICEID: u32 = 2415919106u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_DISCONNECTED: u32 = 2415919140u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INCOMPATIBLEAPIVERSION: u32 = 2415919107u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INCOMPATIBLEEXTVERSION: u32 = 2415919108u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INIFILECORRUPT: u32 = 2415919109u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INUSE: u32 = 2415919110u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALAPPHANDLE: u32 = 2415919111u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALAPPNAME: u32 = 2415919112u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALBUTTONLAMPID: u32 = 2415919113u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALBUTTONMODE: u32 = 2415919114u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALBUTTONSTATE: u32 = 2415919115u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALDATAID: u32 = 2415919116u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALDEVICECLASS: u32 = 2415919117u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALEXTVERSION: u32 = 2415919118u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALHOOKSWITCHDEV: u32 = 2415919119u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALHOOKSWITCHMODE: u32 = 2415919120u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALLAMPMODE: u32 = 2415919121u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALPARAM: u32 = 2415919122u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALPHONEHANDLE: u32 = 2415919123u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALPHONESTATE: u32 = 2415919124u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALPOINTER: u32 = 2415919125u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALPRIVILEGE: u32 = 2415919126u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_INVALRINGMODE: u32 = 2415919127u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_NODEVICE: u32 = 2415919128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_NODRIVER: u32 = 2415919129u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_NOMEM: u32 = 2415919130u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_NOTOWNER: u32 = 2415919131u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_OPERATIONFAILED: u32 = 2415919132u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_OPERATIONUNAVAIL: u32 = 2415919133u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_REINIT: u32 = 2415919139u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_REQUESTOVERRUN: u32 = 2415919136u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_RESOURCEUNAVAIL: u32 = 2415919135u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_SERVICE_NOT_RUNNING: u32 = 2415919141u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_STRUCTURETOOSMALL: u32 = 2415919137u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEERR_UNINITIALIZED: u32 = 2415919138u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GENERICPHONE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETBUTTONINFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETDATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETDISPLAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETGAINHANDSET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETGAINHEADSET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETGAINSPEAKER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETHOOKSWITCHHANDSET: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETHOOKSWITCHHEADSET: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETHOOKSWITCHSPEAKER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETLAMP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETRING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETVOLUMEHANDSET: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETVOLUMEHEADSET: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_GETVOLUMESPEAKER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETBUTTONINFO: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETDATA: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETDISPLAY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETGAINHANDSET: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETGAINHEADSET: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETGAINSPEAKER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETHOOKSWITCHHANDSET: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETHOOKSWITCHHEADSET: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETHOOKSWITCHSPEAKER: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETLAMP: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETRING: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETVOLUMEHANDSET: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETVOLUMEHEADSET: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEFEATURE_SETVOLUMESPEAKER: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHDEV_HANDSET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHDEV_HEADSET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHDEV_SPEAKER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHMODE_MIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHMODE_MICSPEAKER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHMODE_ONHOOK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHMODE_SPEAKER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEHOOKSWITCHMODE_UNKNOWN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEINITIALIZEEXOPTION_USECOMPLETIONPORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEINITIALIZEEXOPTION_USEEVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEINITIALIZEEXOPTION_USEHIDDENWINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_BROKENFLUTTER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_DUMMY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_FLASH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_FLUTTER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_OFF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_STEADY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_UNKNOWN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONELAMPMODE_WINK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEPRIVILEGE_MONITOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONEPRIVILEGE_OWNER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_CAPSCHANGE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_CONNECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_DEVSPECIFIC: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_DISCONNECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_DISPLAY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HANDSETGAIN: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HANDSETHOOKSWITCH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HANDSETVOLUME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HEADSETGAIN: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HEADSETHOOKSWITCH: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_HEADSETVOLUME: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_LAMP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_MONITORS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_OWNER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_REINIT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_REMOVED: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_RESUME: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_RINGMODE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_RINGVOLUME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_SPEAKERGAIN: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_SPEAKERHOOKSWITCH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_SPEAKERVOLUME: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATE_SUSPEND: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATUSFLAGS_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONESTATUSFLAGS_SUSPENDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_BUTTON: i32 = 14i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_CLOSE: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_CREATE: i32 = 20i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_DEVSPECIFIC: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_REMOVE: i32 = 26i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_REPLY: i32 = 17i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHONE_STATE: i32 = 18i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_ADDRESS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_CALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_CALLID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_LINE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PRIVATEOBJECT_PHONE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RENDBIND_AUTHENTICATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RENDBIND_DEFAULTCREDENTIALS: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RENDBIND_DEFAULTDOMAINNAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RENDBIND_DEFAULTPASSWORD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RENDBIND_DEFAULTUSERNAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const Rendezvous: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1029e5b_cb5b_11d0_8d59_00c04fd91ac0);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RequestMakeCall: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac48ffe0_f8c4_11d1_a030_00c04fb6809f);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRINGFORMAT_ASCII: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRINGFORMAT_BINARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRINGFORMAT_DBCS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRINGFORMAT_UNICODE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_CONFIGURED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_INITIAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_PAUSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_RUNNING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_STOPPED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const STRM_TERMINALSELECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21d6d48e_a88b_11d0_83dd_00aa003ccabd);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_CONNECTED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DESTBUSY: i32 = -11i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DESTNOANSWER: i32 = -12i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DESTUNAVAIL: i32 = -13i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DEVICECLASSUNAVAIL: i32 = -8i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DEVICEIDUNAVAIL: i32 = -9i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DEVICEINUSE: i32 = -10i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_DROPPED: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_INVALDESTADDRESS: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_INVALDEVICECLASS: i32 = -6i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_INVALDEVICEID: i32 = -7i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_INVALPOINTER: i32 = -18i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_INVALWINDOWHANDLE: i32 = -5i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_MMCWRITELOCKED: i32 = -20i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_NOREQUESTRECIPIENT: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_NOTADMIN: i32 = -19i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_PROVIDERALREADYINSTALLED: i32 = -21i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_REQUESTCANCELLED: i32 = -17i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_REQUESTFAILED: i32 = -16i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_REQUESTQUEUEFULL: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_SCP_ALREADY_EXISTS: i32 = -22i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_SCP_DOES_NOT_EXIST: i32 = -23i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_UNKNOWNREQUESTID: i32 = -15i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIERR_UNKNOWNWINHANDLE: i32 = -14i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXAPPNAMESIZE: i32 = 40i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXCALLEDPARTYSIZE: i32 = 40i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXCOMMENTSIZE: i32 = 80i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXDESTADDRESSSIZE: i32 = 80i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXDEVICECLASSSIZE: i32 = 40i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMAXDEVICEIDSIZE: i32 = 40i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMEDIATYPE_AUDIO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMEDIATYPE_DATAMODEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMEDIATYPE_G3FAX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMEDIATYPE_MULTITRACK: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPIMEDIATYPE_VIDEO: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_CURRENT_VERSION: u32 = 131074u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_ADDRESSBLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221462i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_ALLOCATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221498i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_BILLINGREJECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221461i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_GROUP_REMOVED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221435i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTACTIVITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221428i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTGROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221431i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221432i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTSTATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221429i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_INVALPASSWORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221430i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_NO_AGENT_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221433i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLCENTER_QUEUE_REMOVED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221434i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLNOTSELECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221420i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CALLUNAVAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221497i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_COMPLETIONOVERRUN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221496i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_CONFERENCEFULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221495i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_DESTBUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221452i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_DESTNOANSWER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221451i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_DESTUNAVAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221450i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_DIALMODIFIERNOTSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221494i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_DROPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221455i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INUSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221493i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221492i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALADDRESSSTATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221491i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALADDRESSTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221423i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALBUTTONLAMPID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221459i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALBUTTONSTATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221458i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCALLPARAMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221490i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCALLPRIVILEGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221489i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCALLSTATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221488i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCARD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221487i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCOMPLETIONID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221486i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALCOUNTRYCODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221485i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALDATAID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221457i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALDEVICECLASS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221484i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALDIALPARAMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221483i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALDIGITS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221482i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALFEATURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221460i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALGROUPID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221481i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALHOOKSWITCHDEV: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221456i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDDIRECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221446i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDMEDIATYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221500i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDSTREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221437i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDSTREAMSTATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221417i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDTERMINAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221445i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALIDTERMINALCLASS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221444i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221474i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALLOCATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221480i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALMESSAGEID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221479i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALMODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221473i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALPARKID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221478i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALPRIVILEGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221447i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALRATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221477i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALTIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221476i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_INVALTONE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221475i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_MAXSTREAMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221442i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_MAXTERMINALS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221438i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOCONFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221472i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NODEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221471i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NODRIVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221443i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOEVENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221424i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221418i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOITEMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221502i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOREQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221470i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOREQUESTRECIPIENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221454i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTENOUGHMEMORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221503i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTERMINALSELECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221441i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTOWNER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221469i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221468i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTSTOPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221439i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOTSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221501i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221415i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_OPERATIONFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221499i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_PEER_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221425i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_PHONENOTOPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221421i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REGISTRY_SETTING_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221427i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REINIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221463i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REQUESTCANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221448i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REQUESTFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221449i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REQUESTOVERRUN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221467i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_REQUESTQUEUEFULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221453i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_RESOURCEUNAVAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221422i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_SERVICE_NOT_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221414i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_TARGETNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221466i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_TARGETSELF: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221465i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_TERMINALINUSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221440i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_TERMINAL_PEER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221426i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221436i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_USERUSERINFOTOOBIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221464i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_WRONGEVENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221419i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_E_WRONG_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221416i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TAPI_REPLY: u32 = 1123u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEACCEPT: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEADDTOCONFERENCE: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEANSWER: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEBLINDTRANSFER: u32 = 503u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECLOSE: u32 = 504u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECLOSECALL: u32 = 505u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECLOSEMSPINSTANCE: u32 = 609u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECOMPLETECALL: u32 = 506u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECOMPLETETRANSFER: u32 = 507u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECONDITIONALMEDIADETECTION: u32 = 508u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECONFIGDIALOG: u32 = 509u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECONFIGDIALOGEDIT: u32 = 601u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINECREATEMSPINSTANCE: u32 = 608u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDEVSPECIFIC: u32 = 510u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDEVSPECIFICFEATURE: u32 = 511u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDIAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDROP: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDROPNOOWNER: u32 = 597u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEDROPONCLOSE: u32 = 596u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEFORWARD: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGATHERDIGITS: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGENERATEDIGITS: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGENERATETONE: u32 = 517u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETADDRESSCAPS: u32 = 518u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETADDRESSID: u32 = 519u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETADDRESSSTATUS: u32 = 520u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETCALLADDRESSID: u32 = 521u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETCALLHUBTRACKING: u32 = 604u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETCALLID: u32 = 603u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETCALLINFO: u32 = 522u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETCALLSTATUS: u32 = 523u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETDEVCAPS: u32 = 524u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETDEVCONFIG: u32 = 525u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETEXTENSIONID: u32 = 526u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETICON: u32 = 527u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETID: u32 = 528u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETLINEDEVSTATUS: u32 = 529u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEGETNUMADDRESSIDS: u32 = 530u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEHOLD: u32 = 531u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEMAKECALL: u32 = 532u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEMONITORDIGITS: u32 = 533u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEMONITORMEDIA: u32 = 534u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEMONITORTONES: u32 = 535u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEMSPIDENTIFY: u32 = 607u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINENEGOTIATEEXTVERSION: u32 = 536u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINENEGOTIATETSPIVERSION: u32 = 537u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEOPEN: u32 = 538u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEPARK: u32 = 539u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEPICKUP: u32 = 540u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEPREPAREADDTOCONFERENCE: u32 = 541u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINERECEIVEMSPDATA: u32 = 606u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEREDIRECT: u32 = 542u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINERELEASEUSERUSERINFO: u32 = 602u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEREMOVEFROMCONFERENCE: u32 = 543u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESECURECALL: u32 = 544u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESELECTEXTVERSION: u32 = 545u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESENDUSERUSERINFO: u32 = 546u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETAPPSPECIFIC: u32 = 547u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETCALLHUBTRACKING: u32 = 605u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETCALLPARAMS: u32 = 548u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETCURRENTLOCATION: u32 = 600u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETDEFAULTMEDIADETECTION: u32 = 549u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETDEVCONFIG: u32 = 550u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETMEDIACONTROL: u32 = 551u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETMEDIAMODE: u32 = 552u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETSTATUSMESSAGES: u32 = 553u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETTERMINAL: u32 = 554u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETUPCONFERENCE: u32 = 555u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESETUPTRANSFER: u32 = 556u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINESWAPHOLD: u32 = 557u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEUNCOMPLETECALL: u32 = 558u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEUNHOLD: u32 = 559u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_LINEUNPARK: u32 = 560u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_MESSAGE_BASE: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONECLOSE: u32 = 561u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONECONFIGDIALOG: u32 = 562u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEDEVSPECIFIC: u32 = 563u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETBUTTONINFO: u32 = 564u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETDATA: u32 = 565u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETDEVCAPS: u32 = 566u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETDISPLAY: u32 = 567u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETEXTENSIONID: u32 = 568u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETGAIN: u32 = 569u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETHOOKSWITCH: u32 = 570u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETICON: u32 = 571u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETID: u32 = 572u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETLAMP: u32 = 573u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETRING: u32 = 574u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETSTATUS: u32 = 575u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEGETVOLUME: u32 = 576u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONENEGOTIATEEXTVERSION: u32 = 577u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONENEGOTIATETSPIVERSION: u32 = 578u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONEOPEN: u32 = 579u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESELECTEXTVERSION: u32 = 580u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETBUTTONINFO: u32 = 581u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETDATA: u32 = 582u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETDISPLAY: u32 = 583u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETGAIN: u32 = 584u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETHOOKSWITCH: u32 = 585u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETLAMP: u32 = 586u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETRING: u32 = 587u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETSTATUSMESSAGES: u32 = 588u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PHONESETVOLUME: u32 = 589u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROC_BASE: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERCONFIG: u32 = 590u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERCREATELINEDEVICE: u32 = 598u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERCREATEPHONEDEVICE: u32 = 599u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERENUMDEVICES: u32 = 595u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERINIT: u32 = 591u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERINSTALL: u32 = 592u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERREMOVE: u32 = 593u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TSPI_PROVIDERSHUTDOWN: u32 = 594u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TUISPIDLL_OBJECT_DIALOGINSTANCE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TUISPIDLL_OBJECT_LINEID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TUISPIDLL_OBJECT_PHONEID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TUISPIDLL_OBJECT_PROVIDERID: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const atypFile: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const atypMax: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const atypNull: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const atypOle: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const atypPicture: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const cbDisplayName: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const cbEmailName: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const cbMaxIdData: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const cbSeverName: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const cbTYPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const prioHigh: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const prioLow: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const prioNorm: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACDGROUP_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACDGE_NEW_GROUP: ACDGROUP_EVENT = ACDGROUP_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACDGE_GROUP_REMOVED: ACDGROUP_EVENT = ACDGROUP_EVENT(1i32);
impl ::core::marker::Copy for ACDGROUP_EVENT {}
impl ::core::clone::Clone for ACDGROUP_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACDGROUP_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACDQUEUE_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACDQE_NEW_QUEUE: ACDQUEUE_EVENT = ACDQUEUE_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACDQE_QUEUE_REMOVED: ACDQUEUE_EVENT = ACDQUEUE_EVENT(1i32);
impl ::core::marker::Copy for ACDQUEUE_EVENT {}
impl ::core::clone::Clone for ACDQUEUE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACDQUEUE_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADDRESS_CAPABILITY(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_ADDRESSTYPES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_BEARERMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXACTIVECALLS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXONHOLDCALLS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXONHOLDPENDINGCALLS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXNUMCONFERENCE: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXNUMTRANSCONF: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MONITORDIGITSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(7i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATEDIGITSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATETONEMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(9i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATETONEMAXNUMFREQ: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(10i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MONITORTONEMAXNUMFREQ: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(11i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MONITORTONEMAXNUMENTRIES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(12i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_DEVCAPFLAGS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(13i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_ANSWERMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(14i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_LINEFEATURES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(15i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_SETTABLEDEVSTATUS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(16i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_PARKSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(17i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLERIDSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(18i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLEDIDSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(19i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CONNECTEDIDSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(20i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_REDIRECTIONIDSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(21i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_REDIRECTINGIDSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(22i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_ADDRESSCAPFLAGS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(23i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLFEATURES1: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(24i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLFEATURES2: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(25i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_REMOVEFROMCONFCAPS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(26i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_REMOVEFROMCONFSTATE: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(27i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_TRANSFERMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(28i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_ADDRESSFEATURES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(29i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_PREDICTIVEAUTOTRANSFERSTATES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(30i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXCALLDATASIZE: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(31i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_LINEID: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(32i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_ADDRESSID: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(33i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_FORWARDMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(34i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXFORWARDENTRIES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(35i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXSPECIFICENTRIES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(36i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MINFWDNUMRINGS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(37i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXFWDNUMRINGS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(38i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_MAXCALLCOMPLETIONS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(39i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLCOMPLETIONCONDITIONS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(40i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_CALLCOMPLETIONMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(41i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_PERMANENTDEVICEID: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(42i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GATHERDIGITSMINTIMEOUT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(43i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GATHERDIGITSMAXTIMEOUT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(44i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATEDIGITMINDURATION: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(45i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATEDIGITMAXDURATION: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(46i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AC_GENERATEDIGITDEFAULTDURATION: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(47i32);
impl ::core::marker::Copy for ADDRESS_CAPABILITY {}
impl ::core::clone::Clone for ADDRESS_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADDRESS_CAPABILITY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADDRESS_CAPABILITY_STRING(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_PROTOCOL: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_ADDRESSDEVICESPECIFIC: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_LINEDEVICESPECIFIC: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_PROVIDERSPECIFIC: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_SWITCHSPECIFIC: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ACS_PERMANENTDEVICEGUID: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(5i32);
impl ::core::marker::Copy for ADDRESS_CAPABILITY_STRING {}
impl ::core::clone::Clone for ADDRESS_CAPABILITY_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADDRESS_CAPABILITY_STRING {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADDRESS_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_STATE: ADDRESS_EVENT = ADDRESS_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_CAPSCHANGE: ADDRESS_EVENT = ADDRESS_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_RINGING: ADDRESS_EVENT = ADDRESS_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_CONFIGCHANGE: ADDRESS_EVENT = ADDRESS_EVENT(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_FORWARD: ADDRESS_EVENT = ADDRESS_EVENT(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_NEWTERMINAL: ADDRESS_EVENT = ADDRESS_EVENT(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_REMOVETERMINAL: ADDRESS_EVENT = ADDRESS_EVENT(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_MSGWAITON: ADDRESS_EVENT = ADDRESS_EVENT(7i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_MSGWAITOFF: ADDRESS_EVENT = ADDRESS_EVENT(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_LASTITEM: ADDRESS_EVENT = ADDRESS_EVENT(8i32);
impl ::core::marker::Copy for ADDRESS_EVENT {}
impl ::core::clone::Clone for ADDRESS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADDRESS_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADDRESS_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_INSERVICE: ADDRESS_STATE = ADDRESS_STATE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_OUTOFSERVICE: ADDRESS_STATE = ADDRESS_STATE(1i32);
impl ::core::marker::Copy for ADDRESS_STATE {}
impl ::core::clone::Clone for ADDRESS_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADDRESS_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AGENTHANDLER_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AHE_NEW_AGENTHANDLER: AGENTHANDLER_EVENT = AGENTHANDLER_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AHE_AGENTHANDLER_REMOVED: AGENTHANDLER_EVENT = AGENTHANDLER_EVENT(1i32);
impl ::core::marker::Copy for AGENTHANDLER_EVENT {}
impl ::core::clone::Clone for AGENTHANDLER_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AGENTHANDLER_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AGENT_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_NOT_READY: AGENT_EVENT = AGENT_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_READY: AGENT_EVENT = AGENT_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_BUSY_ACD: AGENT_EVENT = AGENT_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_BUSY_INCOMING: AGENT_EVENT = AGENT_EVENT(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_BUSY_OUTGOING: AGENT_EVENT = AGENT_EVENT(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AE_UNKNOWN: AGENT_EVENT = AGENT_EVENT(5i32);
impl ::core::marker::Copy for AGENT_EVENT {}
impl ::core::clone::Clone for AGENT_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AGENT_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AGENT_SESSION_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_NEW_SESSION: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_NOT_READY: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_READY: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_BUSY: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_WRAPUP: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASE_END: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(5i32);
impl ::core::marker::Copy for AGENT_SESSION_EVENT {}
impl ::core::clone::Clone for AGENT_SESSION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AGENT_SESSION_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AGENT_SESSION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASST_NOT_READY: AGENT_SESSION_STATE = AGENT_SESSION_STATE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASST_READY: AGENT_SESSION_STATE = AGENT_SESSION_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASST_BUSY_ON_CALL: AGENT_SESSION_STATE = AGENT_SESSION_STATE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASST_BUSY_WRAPUP: AGENT_SESSION_STATE = AGENT_SESSION_STATE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ASST_SESSION_ENDED: AGENT_SESSION_STATE = AGENT_SESSION_STATE(4i32);
impl ::core::marker::Copy for AGENT_SESSION_STATE {}
impl ::core::clone::Clone for AGENT_SESSION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AGENT_SESSION_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AGENT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_NOT_READY: AGENT_STATE = AGENT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_READY: AGENT_STATE = AGENT_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_BUSY_ACD: AGENT_STATE = AGENT_STATE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_BUSY_INCOMING: AGENT_STATE = AGENT_STATE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_BUSY_OUTGOING: AGENT_STATE = AGENT_STATE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const AS_UNKNOWN: AGENT_STATE = AGENT_STATE(5i32);
impl ::core::marker::Copy for AGENT_STATE {}
impl ::core::clone::Clone for AGENT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AGENT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLHUB_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHE_CALLJOIN: CALLHUB_EVENT = CALLHUB_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHE_CALLLEAVE: CALLHUB_EVENT = CALLHUB_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHE_CALLHUBNEW: CALLHUB_EVENT = CALLHUB_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHE_CALLHUBIDLE: CALLHUB_EVENT = CALLHUB_EVENT(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHE_LASTITEM: CALLHUB_EVENT = CALLHUB_EVENT(3i32);
impl ::core::marker::Copy for CALLHUB_EVENT {}
impl ::core::clone::Clone for CALLHUB_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALLHUB_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLHUB_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHS_ACTIVE: CALLHUB_STATE = CALLHUB_STATE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CHS_IDLE: CALLHUB_STATE = CALLHUB_STATE(1i32);
impl ::core::marker::Copy for CALLHUB_STATE {}
impl ::core::clone::Clone for CALLHUB_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALLHUB_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLINFOCHANGE_CAUSE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_OTHER: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_DEVSPECIFIC: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_BEARERMODE: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_RATE: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_APPSPECIFIC: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CALLID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_RELATEDCALLID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_ORIGIN: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(7i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_REASON: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_COMPLETIONID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(9i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_NUMOWNERINCR: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(10i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_NUMOWNERDECR: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(11i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_NUMMONITORS: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(12i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_TRUNK: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(13i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CALLERID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(14i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CALLEDID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(15i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CONNECTEDID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(16i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_REDIRECTIONID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(17i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_REDIRECTINGID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(18i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_USERUSERINFO: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(19i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_HIGHLEVELCOMP: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(20i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_LOWLEVELCOMP: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(21i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CHARGINGINFO: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(22i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_TREATMENT: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(23i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_CALLDATA: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(24i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_PRIVILEGE: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(25i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_MEDIATYPE: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(26i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIC_LASTITEM: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(26i32);
impl ::core::marker::Copy for CALLINFOCHANGE_CAUSE {}
impl ::core::clone::Clone for CALLINFOCHANGE_CAUSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALLINFOCHANGE_CAUSE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLINFO_BUFFER(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_USERUSERINFO: CALLINFO_BUFFER = CALLINFO_BUFFER(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_DEVSPECIFICBUFFER: CALLINFO_BUFFER = CALLINFO_BUFFER(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_CALLDATABUFFER: CALLINFO_BUFFER = CALLINFO_BUFFER(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_CHARGINGINFOBUFFER: CALLINFO_BUFFER = CALLINFO_BUFFER(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_HIGHLEVELCOMPATIBILITYBUFFER: CALLINFO_BUFFER = CALLINFO_BUFFER(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIB_LOWLEVELCOMPATIBILITYBUFFER: CALLINFO_BUFFER = CALLINFO_BUFFER(5i32);
impl ::core::marker::Copy for CALLINFO_BUFFER {}
impl ::core::clone::Clone for CALLINFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALLINFO_BUFFER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLINFO_LONG(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_MEDIATYPESAVAILABLE: CALLINFO_LONG = CALLINFO_LONG(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_BEARERMODE: CALLINFO_LONG = CALLINFO_LONG(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CALLERIDADDRESSTYPE: CALLINFO_LONG = CALLINFO_LONG(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CALLEDIDADDRESSTYPE: CALLINFO_LONG = CALLINFO_LONG(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CONNECTEDIDADDRESSTYPE: CALLINFO_LONG = CALLINFO_LONG(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_REDIRECTIONIDADDRESSTYPE: CALLINFO_LONG = CALLINFO_LONG(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_REDIRECTINGIDADDRESSTYPE: CALLINFO_LONG = CALLINFO_LONG(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_ORIGIN: CALLINFO_LONG = CALLINFO_LONG(7i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_REASON: CALLINFO_LONG = CALLINFO_LONG(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_APPSPECIFIC: CALLINFO_LONG = CALLINFO_LONG(9i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CALLPARAMSFLAGS: CALLINFO_LONG = CALLINFO_LONG(10i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CALLTREATMENT: CALLINFO_LONG = CALLINFO_LONG(11i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_MINRATE: CALLINFO_LONG = CALLINFO_LONG(12i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_MAXRATE: CALLINFO_LONG = CALLINFO_LONG(13i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_COUNTRYCODE: CALLINFO_LONG = CALLINFO_LONG(14i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_CALLID: CALLINFO_LONG = CALLINFO_LONG(15i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_RELATEDCALLID: CALLINFO_LONG = CALLINFO_LONG(16i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_COMPLETIONID: CALLINFO_LONG = CALLINFO_LONG(17i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_NUMBEROFOWNERS: CALLINFO_LONG = CALLINFO_LONG(18i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_NUMBEROFMONITORS: CALLINFO_LONG = CALLINFO_LONG(19i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_TRUNK: CALLINFO_LONG = CALLINFO_LONG(20i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_RATE: CALLINFO_LONG = CALLINFO_LONG(21i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_GENERATEDIGITDURATION: CALLINFO_LONG = CALLINFO_LONG(22i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_MONITORDIGITMODES: CALLINFO_LONG = CALLINFO_LONG(23i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIL_MONITORMEDIAMODES: CALLINFO_LONG = CALLINFO_LONG(24i32);
impl ::core::marker::Copy for CALLINFO_LONG {}
impl ::core::clone::Clone for CALLINFO_LONG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALLINFO_LONG {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLINFO_STRING(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLERIDNAME: CALLINFO_STRING = CALLINFO_STRING(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLERIDNUMBER: CALLINFO_STRING = CALLINFO_STRING(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLEDIDNAME: CALLINFO_STRING = CALLINFO_STRING(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLEDIDNUMBER: CALLINFO_STRING = CALLINFO_STRING(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CONNECTEDIDNAME: CALLINFO_STRING = CALLINFO_STRING(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CONNECTEDIDNUMBER: CALLINFO_STRING = CALLINFO_STRING(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_REDIRECTIONIDNAME: CALLINFO_STRING = CALLINFO_STRING(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_REDIRECTIONIDNUMBER: CALLINFO_STRING = CALLINFO_STRING(7i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_REDIRECTINGIDNAME: CALLINFO_STRING = CALLINFO_STRING(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_REDIRECTINGIDNUMBER: CALLINFO_STRING = CALLINFO_STRING(9i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLEDPARTYFRIENDLYNAME: CALLINFO_STRING = CALLINFO_STRING(10i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_COMMENT: CALLINFO_STRING = CALLINFO_STRING(11i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_DISPLAYABLEADDRESS: CALLINFO_STRING = CALLINFO_STRING(12i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CIS_CALLINGPARTYID: CALLINFO_STRING = CALLINFO_STRING(13i32);
impl ::core::marker::Copy for CALLINFO_STRING {}
impl ::core::clone::Clone for CALLINFO_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALLINFO_STRING {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALL_MEDIA_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_NEW_STREAM: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_STREAM_FAIL: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_TERMINAL_FAIL: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_STREAM_NOT_USED: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_STREAM_ACTIVE: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_STREAM_INACTIVE: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CME_LASTITEM: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(5i32);
impl ::core::marker::Copy for CALL_MEDIA_EVENT {}
impl ::core::clone::Clone for CALL_MEDIA_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALL_MEDIA_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALL_MEDIA_EVENT_CAUSE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_UNKNOWN: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_BAD_DEVICE: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_CONNECT_FAIL: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_LOCAL_REQUEST: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_REMOTE_REQUEST: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_MEDIA_TIMEOUT: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_MEDIA_RECOVERED: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CMC_QUALITY_OF_SERVICE: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(7i32);
impl ::core::marker::Copy for CALL_MEDIA_EVENT_CAUSE {}
impl ::core::clone::Clone for CALL_MEDIA_EVENT_CAUSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALL_MEDIA_EVENT_CAUSE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALL_NOTIFICATION_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CNE_OWNER: CALL_NOTIFICATION_EVENT = CALL_NOTIFICATION_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CNE_MONITOR: CALL_NOTIFICATION_EVENT = CALL_NOTIFICATION_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CNE_LASTITEM: CALL_NOTIFICATION_EVENT = CALL_NOTIFICATION_EVENT(1i32);
impl ::core::marker::Copy for CALL_NOTIFICATION_EVENT {}
impl ::core::clone::Clone for CALL_NOTIFICATION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALL_NOTIFICATION_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALL_PRIVILEGE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CP_OWNER: CALL_PRIVILEGE = CALL_PRIVILEGE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CP_MONITOR: CALL_PRIVILEGE = CALL_PRIVILEGE(1i32);
impl ::core::marker::Copy for CALL_PRIVILEGE {}
impl ::core::clone::Clone for CALL_PRIVILEGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALL_PRIVILEGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALL_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_IDLE: CALL_STATE = CALL_STATE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_INPROGRESS: CALL_STATE = CALL_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_CONNECTED: CALL_STATE = CALL_STATE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_DISCONNECTED: CALL_STATE = CALL_STATE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_OFFERING: CALL_STATE = CALL_STATE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_HOLD: CALL_STATE = CALL_STATE(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_QUEUED: CALL_STATE = CALL_STATE(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CS_LASTITEM: CALL_STATE = CALL_STATE(6i32);
impl ::core::marker::Copy for CALL_STATE {}
impl ::core::clone::Clone for CALL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALL_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALL_STATE_EVENT_CAUSE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_NONE: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_NORMAL: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_BUSY: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_BADADDRESS: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_NOANSWER: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_CANCELLED: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_REJECTED: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_FAILED: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(7i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CEC_DISCONNECT_BLOCKED: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(8i32);
impl ::core::marker::Copy for CALL_STATE_EVENT_CAUSE {}
impl ::core::clone::Clone for CALL_STATE_EVENT_CAUSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CALL_STATE_EVENT_CAUSE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTORY_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const OT_CONFERENCE: DIRECTORY_OBJECT_TYPE = DIRECTORY_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const OT_USER: DIRECTORY_OBJECT_TYPE = DIRECTORY_OBJECT_TYPE(2i32);
impl ::core::marker::Copy for DIRECTORY_OBJECT_TYPE {}
impl ::core::clone::Clone for DIRECTORY_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DIRECTORY_OBJECT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTORY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DT_NTDS: DIRECTORY_TYPE = DIRECTORY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DT_ILS: DIRECTORY_TYPE = DIRECTORY_TYPE(2i32);
impl ::core::marker::Copy for DIRECTORY_TYPE {}
impl ::core::clone::Clone for DIRECTORY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DIRECTORY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISCONNECT_CODE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DC_NORMAL: DISCONNECT_CODE = DISCONNECT_CODE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DC_NOANSWER: DISCONNECT_CODE = DISCONNECT_CODE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const DC_REJECTED: DISCONNECT_CODE = DISCONNECT_CODE(2i32);
impl ::core::marker::Copy for DISCONNECT_CODE {}
impl ::core::clone::Clone for DISCONNECT_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DISCONNECT_CODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FINISH_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FM_ASTRANSFER: FINISH_MODE = FINISH_MODE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FM_ASCONFERENCE: FINISH_MODE = FINISH_MODE(1i32);
impl ::core::marker::Copy for FINISH_MODE {}
impl ::core::clone::Clone for FINISH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FINISH_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FT_STATE_EVENT_CAUSE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FTEC_NORMAL: FT_STATE_EVENT_CAUSE = FT_STATE_EVENT_CAUSE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FTEC_END_OF_FILE: FT_STATE_EVENT_CAUSE = FT_STATE_EVENT_CAUSE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FTEC_READ_ERROR: FT_STATE_EVENT_CAUSE = FT_STATE_EVENT_CAUSE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FTEC_WRITE_ERROR: FT_STATE_EVENT_CAUSE = FT_STATE_EVENT_CAUSE(3i32);
impl ::core::marker::Copy for FT_STATE_EVENT_CAUSE {}
impl ::core::clone::Clone for FT_STATE_EVENT_CAUSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FT_STATE_EVENT_CAUSE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FULLDUPLEX_SUPPORT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FDS_SUPPORTED: FULLDUPLEX_SUPPORT = FULLDUPLEX_SUPPORT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FDS_NOTSUPPORTED: FULLDUPLEX_SUPPORT = FULLDUPLEX_SUPPORT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const FDS_UNKNOWN: FULLDUPLEX_SUPPORT = FULLDUPLEX_SUPPORT(2i32);
impl ::core::marker::Copy for FULLDUPLEX_SUPPORT {}
impl ::core::clone::Clone for FULLDUPLEX_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FULLDUPLEX_SUPPORT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSP_ADDRESS_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ADDRESS_TERMINAL_AVAILABLE: MSP_ADDRESS_EVENT = MSP_ADDRESS_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ADDRESS_TERMINAL_UNAVAILABLE: MSP_ADDRESS_EVENT = MSP_ADDRESS_EVENT(1i32);
impl ::core::marker::Copy for MSP_ADDRESS_EVENT {}
impl ::core::clone::Clone for MSP_ADDRESS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSP_ADDRESS_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSP_CALL_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_NEW_STREAM: MSP_CALL_EVENT = MSP_CALL_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_STREAM_FAIL: MSP_CALL_EVENT = MSP_CALL_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_TERMINAL_FAIL: MSP_CALL_EVENT = MSP_CALL_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_STREAM_NOT_USED: MSP_CALL_EVENT = MSP_CALL_EVENT(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_STREAM_ACTIVE: MSP_CALL_EVENT = MSP_CALL_EVENT(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_STREAM_INACTIVE: MSP_CALL_EVENT = MSP_CALL_EVENT(5i32);
impl ::core::marker::Copy for MSP_CALL_EVENT {}
impl ::core::clone::Clone for MSP_CALL_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSP_CALL_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSP_CALL_EVENT_CAUSE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_UNKNOWN: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_BAD_DEVICE: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_CONNECT_FAIL: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_LOCAL_REQUEST: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_REMOTE_REQUEST: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_MEDIA_TIMEOUT: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_MEDIA_RECOVERED: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const CALL_CAUSE_QUALITY_OF_SERVICE: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(7i32);
impl ::core::marker::Copy for MSP_CALL_EVENT_CAUSE {}
impl ::core::clone::Clone for MSP_CALL_EVENT_CAUSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSP_CALL_EVENT_CAUSE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSP_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_ADDRESS_EVENT: MSP_EVENT = MSP_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_CALL_EVENT: MSP_EVENT = MSP_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_TSP_DATA: MSP_EVENT = MSP_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_PRIVATE_EVENT: MSP_EVENT = MSP_EVENT(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_ASR_TERMINAL_EVENT: MSP_EVENT = MSP_EVENT(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_TTS_TERMINAL_EVENT: MSP_EVENT = MSP_EVENT(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_FILE_TERMINAL_EVENT: MSP_EVENT = MSP_EVENT(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const ME_TONE_TERMINAL_EVENT: MSP_EVENT = MSP_EVENT(7i32);
impl ::core::marker::Copy for MSP_EVENT {}
impl ::core::clone::Clone for MSP_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSP_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONECAPS_BUFFER(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCB_DEVSPECIFICBUFFER: PHONECAPS_BUFFER = PHONECAPS_BUFFER(0i32);
impl ::core::marker::Copy for PHONECAPS_BUFFER {}
impl ::core::clone::Clone for PHONECAPS_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONECAPS_BUFFER {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONECAPS_LONG(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_HOOKSWITCHES: PHONECAPS_LONG = PHONECAPS_LONG(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_HANDSETHOOKSWITCHMODES: PHONECAPS_LONG = PHONECAPS_LONG(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_HEADSETHOOKSWITCHMODES: PHONECAPS_LONG = PHONECAPS_LONG(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_SPEAKERPHONEHOOKSWITCHMODES: PHONECAPS_LONG = PHONECAPS_LONG(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_DISPLAYNUMROWS: PHONECAPS_LONG = PHONECAPS_LONG(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_DISPLAYNUMCOLUMNS: PHONECAPS_LONG = PHONECAPS_LONG(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_NUMRINGMODES: PHONECAPS_LONG = PHONECAPS_LONG(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_NUMBUTTONLAMPS: PHONECAPS_LONG = PHONECAPS_LONG(7i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCL_GENERICPHONE: PHONECAPS_LONG = PHONECAPS_LONG(8i32);
impl ::core::marker::Copy for PHONECAPS_LONG {}
impl ::core::clone::Clone for PHONECAPS_LONG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONECAPS_LONG {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONECAPS_STRING(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCS_PHONENAME: PHONECAPS_STRING = PHONECAPS_STRING(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCS_PHONEINFO: PHONECAPS_STRING = PHONECAPS_STRING(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PCS_PROVIDERINFO: PHONECAPS_STRING = PHONECAPS_STRING(2i32);
impl ::core::marker::Copy for PHONECAPS_STRING {}
impl ::core::clone::Clone for PHONECAPS_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONECAPS_STRING {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONE_BUTTON_FUNCTION(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_UNKNOWN: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_CONFERENCE: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_TRANSFER: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DROP: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_HOLD: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_RECALL: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DISCONNECT: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_CONNECT: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(7i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_MSGWAITON: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_MSGWAITOFF: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(9i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SELECTRING: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(10i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_ABBREVDIAL: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(11i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_FORWARD: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(12i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_PICKUP: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(13i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_RINGAGAIN: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(14i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_PARK: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(15i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_REJECT: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(16i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_REDIRECT: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(17i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_MUTE: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(18i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_VOLUMEUP: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(19i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_VOLUMEDOWN: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(20i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SPEAKERON: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(21i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SPEAKEROFF: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(22i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_FLASH: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(23i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DATAON: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(24i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DATAOFF: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(25i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DONOTDISTURB: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(26i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_INTERCOM: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(27i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_BRIDGEDAPP: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(28i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_BUSY: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(29i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_CALLAPP: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(30i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DATETIME: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(31i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_DIRECTORY: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(32i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_COVER: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(33i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_CALLID: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(34i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_LASTNUM: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(35i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_NIGHTSRV: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(36i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SENDCALLS: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(37i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_MSGINDICATOR: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(38i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_REPDIAL: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(39i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SETREPDIAL: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(40i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SYSTEMSPEED: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(41i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_STATIONSPEED: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(42i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_CAMPON: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(43i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SAVEREPEAT: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(44i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_QUEUECALL: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(45i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_NONE: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(46i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBF_SEND: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(47i32);
impl ::core::marker::Copy for PHONE_BUTTON_FUNCTION {}
impl ::core::clone::Clone for PHONE_BUTTON_FUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONE_BUTTON_FUNCTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONE_BUTTON_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_DUMMY: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_CALL: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_FEATURE: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_KEYPAD: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_LOCAL: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBM_DISPLAY: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(5i32);
impl ::core::marker::Copy for PHONE_BUTTON_MODE {}
impl ::core::clone::Clone for PHONE_BUTTON_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONE_BUTTON_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONE_BUTTON_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBS_UP: PHONE_BUTTON_STATE = PHONE_BUTTON_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBS_DOWN: PHONE_BUTTON_STATE = PHONE_BUTTON_STATE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBS_UNKNOWN: PHONE_BUTTON_STATE = PHONE_BUTTON_STATE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PBS_UNAVAIL: PHONE_BUTTON_STATE = PHONE_BUTTON_STATE(8i32);
impl ::core::marker::Copy for PHONE_BUTTON_STATE {}
impl ::core::clone::Clone for PHONE_BUTTON_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONE_BUTTON_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONE_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_DISPLAY: PHONE_EVENT = PHONE_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_LAMPMODE: PHONE_EVENT = PHONE_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_RINGMODE: PHONE_EVENT = PHONE_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_RINGVOLUME: PHONE_EVENT = PHONE_EVENT(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_HOOKSWITCH: PHONE_EVENT = PHONE_EVENT(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_CAPSCHANGE: PHONE_EVENT = PHONE_EVENT(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_BUTTON: PHONE_EVENT = PHONE_EVENT(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_CLOSE: PHONE_EVENT = PHONE_EVENT(7i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_NUMBERGATHERED: PHONE_EVENT = PHONE_EVENT(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_DIALING: PHONE_EVENT = PHONE_EVENT(9i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_ANSWER: PHONE_EVENT = PHONE_EVENT(10i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_DISCONNECT: PHONE_EVENT = PHONE_EVENT(11i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PE_LASTITEM: PHONE_EVENT = PHONE_EVENT(11i32);
impl ::core::marker::Copy for PHONE_EVENT {}
impl ::core::clone::Clone for PHONE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONE_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONE_HOOK_SWITCH_DEVICE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSD_HANDSET: PHONE_HOOK_SWITCH_DEVICE = PHONE_HOOK_SWITCH_DEVICE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSD_SPEAKERPHONE: PHONE_HOOK_SWITCH_DEVICE = PHONE_HOOK_SWITCH_DEVICE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSD_HEADSET: PHONE_HOOK_SWITCH_DEVICE = PHONE_HOOK_SWITCH_DEVICE(4i32);
impl ::core::marker::Copy for PHONE_HOOK_SWITCH_DEVICE {}
impl ::core::clone::Clone for PHONE_HOOK_SWITCH_DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONE_HOOK_SWITCH_DEVICE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONE_HOOK_SWITCH_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSS_ONHOOK: PHONE_HOOK_SWITCH_STATE = PHONE_HOOK_SWITCH_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSS_OFFHOOK_MIC_ONLY: PHONE_HOOK_SWITCH_STATE = PHONE_HOOK_SWITCH_STATE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSS_OFFHOOK_SPEAKER_ONLY: PHONE_HOOK_SWITCH_STATE = PHONE_HOOK_SWITCH_STATE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PHSS_OFFHOOK: PHONE_HOOK_SWITCH_STATE = PHONE_HOOK_SWITCH_STATE(8i32);
impl ::core::marker::Copy for PHONE_HOOK_SWITCH_STATE {}
impl ::core::clone::Clone for PHONE_HOOK_SWITCH_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONE_HOOK_SWITCH_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONE_LAMP_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_DUMMY: PHONE_LAMP_MODE = PHONE_LAMP_MODE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_OFF: PHONE_LAMP_MODE = PHONE_LAMP_MODE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_STEADY: PHONE_LAMP_MODE = PHONE_LAMP_MODE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_WINK: PHONE_LAMP_MODE = PHONE_LAMP_MODE(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_FLASH: PHONE_LAMP_MODE = PHONE_LAMP_MODE(16i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_FLUTTER: PHONE_LAMP_MODE = PHONE_LAMP_MODE(32i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_BROKENFLUTTER: PHONE_LAMP_MODE = PHONE_LAMP_MODE(64i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const LM_UNKNOWN: PHONE_LAMP_MODE = PHONE_LAMP_MODE(128i32);
impl ::core::marker::Copy for PHONE_LAMP_MODE {}
impl ::core::clone::Clone for PHONE_LAMP_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONE_LAMP_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONE_PRIVILEGE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PP_OWNER: PHONE_PRIVILEGE = PHONE_PRIVILEGE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PP_MONITOR: PHONE_PRIVILEGE = PHONE_PRIVILEGE(1i32);
impl ::core::marker::Copy for PHONE_PRIVILEGE {}
impl ::core::clone::Clone for PHONE_PRIVILEGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONE_PRIVILEGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONE_TONE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADZERO: PHONE_TONE = PHONE_TONE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADONE: PHONE_TONE = PHONE_TONE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADTWO: PHONE_TONE = PHONE_TONE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADTHREE: PHONE_TONE = PHONE_TONE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADFOUR: PHONE_TONE = PHONE_TONE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADFIVE: PHONE_TONE = PHONE_TONE(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADSIX: PHONE_TONE = PHONE_TONE(6i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADSEVEN: PHONE_TONE = PHONE_TONE(7i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADEIGHT: PHONE_TONE = PHONE_TONE(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADNINE: PHONE_TONE = PHONE_TONE(9i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADSTAR: PHONE_TONE = PHONE_TONE(10i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADPOUND: PHONE_TONE = PHONE_TONE(11i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADA: PHONE_TONE = PHONE_TONE(12i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADB: PHONE_TONE = PHONE_TONE(13i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADC: PHONE_TONE = PHONE_TONE(14i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_KEYPADD: PHONE_TONE = PHONE_TONE(15i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_NORMALDIALTONE: PHONE_TONE = PHONE_TONE(16i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_EXTERNALDIALTONE: PHONE_TONE = PHONE_TONE(17i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_BUSY: PHONE_TONE = PHONE_TONE(18i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_RINGBACK: PHONE_TONE = PHONE_TONE(19i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_ERRORTONE: PHONE_TONE = PHONE_TONE(20i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const PT_SILENCE: PHONE_TONE = PHONE_TONE(21i32);
impl ::core::marker::Copy for PHONE_TONE {}
impl ::core::clone::Clone for PHONE_TONE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONE_TONE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QOS_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QE_NOQOS: QOS_EVENT = QOS_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QE_ADMISSIONFAILURE: QOS_EVENT = QOS_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QE_POLICYFAILURE: QOS_EVENT = QOS_EVENT(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QE_GENERICERROR: QOS_EVENT = QOS_EVENT(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QE_LASTITEM: QOS_EVENT = QOS_EVENT(4i32);
impl ::core::marker::Copy for QOS_EVENT {}
impl ::core::clone::Clone for QOS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for QOS_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QOS_SERVICE_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QSL_NEEDED: QOS_SERVICE_LEVEL = QOS_SERVICE_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QSL_IF_AVAILABLE: QOS_SERVICE_LEVEL = QOS_SERVICE_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const QSL_BEST_EFFORT: QOS_SERVICE_LEVEL = QOS_SERVICE_LEVEL(3i32);
impl ::core::marker::Copy for QOS_SERVICE_LEVEL {}
impl ::core::clone::Clone for QOS_SERVICE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for QOS_SERVICE_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RND_ADVERTISING_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RAS_LOCAL: RND_ADVERTISING_SCOPE = RND_ADVERTISING_SCOPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RAS_SITE: RND_ADVERTISING_SCOPE = RND_ADVERTISING_SCOPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RAS_REGION: RND_ADVERTISING_SCOPE = RND_ADVERTISING_SCOPE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const RAS_WORLD: RND_ADVERTISING_SCOPE = RND_ADVERTISING_SCOPE(4i32);
impl ::core::marker::Copy for RND_ADVERTISING_SCOPE {}
impl ::core::clone::Clone for RND_ADVERTISING_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RND_ADVERTISING_SCOPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TAPIOBJECT_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ADDRESSCREATE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ADDRESSREMOVE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_REINIT: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_TRANSLATECHANGE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ADDRESSCLOSE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_PHONECREATE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_PHONEREMOVE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(6i32);
impl ::core::marker::Copy for TAPIOBJECT_EVENT {}
impl ::core::clone::Clone for TAPIOBJECT_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TAPIOBJECT_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TAPI_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_TAPIOBJECT: TAPI_EVENT = TAPI_EVENT(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ADDRESS: TAPI_EVENT = TAPI_EVENT(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_CALLNOTIFICATION: TAPI_EVENT = TAPI_EVENT(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_CALLSTATE: TAPI_EVENT = TAPI_EVENT(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_CALLMEDIA: TAPI_EVENT = TAPI_EVENT(16i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_CALLHUB: TAPI_EVENT = TAPI_EVENT(32i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_CALLINFOCHANGE: TAPI_EVENT = TAPI_EVENT(64i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_PRIVATE: TAPI_EVENT = TAPI_EVENT(128i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_REQUEST: TAPI_EVENT = TAPI_EVENT(256i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_AGENT: TAPI_EVENT = TAPI_EVENT(512i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_AGENTSESSION: TAPI_EVENT = TAPI_EVENT(1024i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_QOSEVENT: TAPI_EVENT = TAPI_EVENT(2048i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_AGENTHANDLER: TAPI_EVENT = TAPI_EVENT(4096i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ACDGROUP: TAPI_EVENT = TAPI_EVENT(8192i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_QUEUE: TAPI_EVENT = TAPI_EVENT(16384i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_DIGITEVENT: TAPI_EVENT = TAPI_EVENT(32768i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_GENERATEEVENT: TAPI_EVENT = TAPI_EVENT(65536i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ASRTERMINAL: TAPI_EVENT = TAPI_EVENT(131072i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_TTSTERMINAL: TAPI_EVENT = TAPI_EVENT(262144i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_FILETERMINAL: TAPI_EVENT = TAPI_EVENT(524288i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_TONETERMINAL: TAPI_EVENT = TAPI_EVENT(1048576i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_PHONEEVENT: TAPI_EVENT = TAPI_EVENT(2097152i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_TONEEVENT: TAPI_EVENT = TAPI_EVENT(4194304i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_GATHERDIGITS: TAPI_EVENT = TAPI_EVENT(8388608i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_ADDRESSDEVSPECIFIC: TAPI_EVENT = TAPI_EVENT(16777216i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TE_PHONEDEVSPECIFIC: TAPI_EVENT = TAPI_EVENT(33554432i32);
impl ::core::marker::Copy for TAPI_EVENT {}
impl ::core::clone::Clone for TAPI_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TAPI_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TAPI_GATHERTERM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TGT_BUFFERFULL: TAPI_GATHERTERM = TAPI_GATHERTERM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TGT_TERMDIGIT: TAPI_GATHERTERM = TAPI_GATHERTERM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TGT_FIRSTTIMEOUT: TAPI_GATHERTERM = TAPI_GATHERTERM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TGT_INTERTIMEOUT: TAPI_GATHERTERM = TAPI_GATHERTERM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TGT_CANCEL: TAPI_GATHERTERM = TAPI_GATHERTERM(16i32);
impl ::core::marker::Copy for TAPI_GATHERTERM {}
impl ::core::clone::Clone for TAPI_GATHERTERM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TAPI_GATHERTERM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TAPI_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_NONE: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_TAPI: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_ADDRESS: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_TERMINAL: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_CALL: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_CALLHUB: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TOT_PHONE: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(6i32);
impl ::core::marker::Copy for TAPI_OBJECT_TYPE {}
impl ::core::clone::Clone for TAPI_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TAPI_OBJECT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TAPI_TONEMODE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TTM_RINGBACK: TAPI_TONEMODE = TAPI_TONEMODE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TTM_BUSY: TAPI_TONEMODE = TAPI_TONEMODE(4i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TTM_BEEP: TAPI_TONEMODE = TAPI_TONEMODE(8i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TTM_BILLING: TAPI_TONEMODE = TAPI_TONEMODE(16i32);
impl ::core::marker::Copy for TAPI_TONEMODE {}
impl ::core::clone::Clone for TAPI_TONEMODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TAPI_TONEMODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TERMINAL_DIRECTION(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TD_CAPTURE: TERMINAL_DIRECTION = TERMINAL_DIRECTION(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TD_RENDER: TERMINAL_DIRECTION = TERMINAL_DIRECTION(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TD_BIDIRECTIONAL: TERMINAL_DIRECTION = TERMINAL_DIRECTION(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TD_MULTITRACK_MIXED: TERMINAL_DIRECTION = TERMINAL_DIRECTION(3i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TD_NONE: TERMINAL_DIRECTION = TERMINAL_DIRECTION(4i32);
impl ::core::marker::Copy for TERMINAL_DIRECTION {}
impl ::core::clone::Clone for TERMINAL_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TERMINAL_DIRECTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TERMINAL_MEDIA_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TMS_IDLE: TERMINAL_MEDIA_STATE = TERMINAL_MEDIA_STATE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TMS_ACTIVE: TERMINAL_MEDIA_STATE = TERMINAL_MEDIA_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TMS_PAUSED: TERMINAL_MEDIA_STATE = TERMINAL_MEDIA_STATE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TMS_LASTITEM: TERMINAL_MEDIA_STATE = TERMINAL_MEDIA_STATE(2i32);
impl ::core::marker::Copy for TERMINAL_MEDIA_STATE {}
impl ::core::clone::Clone for TERMINAL_MEDIA_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TERMINAL_MEDIA_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TERMINAL_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TS_INUSE: TERMINAL_STATE = TERMINAL_STATE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TS_NOTINUSE: TERMINAL_STATE = TERMINAL_STATE(1i32);
impl ::core::marker::Copy for TERMINAL_STATE {}
impl ::core::clone::Clone for TERMINAL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TERMINAL_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TERMINAL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TT_STATIC: TERMINAL_TYPE = TERMINAL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub const TT_DYNAMIC: TERMINAL_TYPE = TERMINAL_TYPE(1i32);
impl ::core::marker::Copy for TERMINAL_TYPE {}
impl ::core::clone::Clone for TERMINAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TERMINAL_TYPE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADDRALIAS {
    pub rgchName: [super::super::Foundation::CHAR; 41],
    pub rgchEName: [super::super::Foundation::CHAR; 11],
    pub rgchSrvr: [super::super::Foundation::CHAR; 12],
    pub dibDetail: u32,
    pub r#type: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADDRALIAS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADDRALIAS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ADDRALIAS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct DTR {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wDayOfWeek: u16,
}
impl ::core::marker::Copy for DTR {}
impl ::core::clone::Clone for DTR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DTR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HDRVCALL__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDRVCALL__ {}
impl ::core::clone::Clone for HDRVCALL__ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HDRVCALL__ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HDRVDIALOGINSTANCE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDRVDIALOGINSTANCE__ {}
impl ::core::clone::Clone for HDRVDIALOGINSTANCE__ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HDRVDIALOGINSTANCE__ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HDRVLINE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDRVLINE__ {}
impl ::core::clone::Clone for HDRVLINE__ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HDRVLINE__ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HDRVMSPLINE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDRVMSPLINE__ {}
impl ::core::clone::Clone for HDRVMSPLINE__ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HDRVMSPLINE__ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HDRVPHONE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDRVPHONE__ {}
impl ::core::clone::Clone for HDRVPHONE__ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HDRVPHONE__ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HPROVIDER__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HPROVIDER__ {}
impl ::core::clone::Clone for HPROVIDER__ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HPROVIDER__ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HTAPICALL__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HTAPICALL__ {}
impl ::core::clone::Clone for HTAPICALL__ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HTAPICALL__ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HTAPILINE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HTAPILINE__ {}
impl ::core::clone::Clone for HTAPILINE__ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HTAPILINE__ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct HTAPIPHONE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HTAPIPHONE__ {}
impl ::core::clone::Clone for HTAPIPHONE__ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HTAPIPHONE__ {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEADDRESSCAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwLineDeviceID: u32,
    pub dwAddressSize: u32,
    pub dwAddressOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwAddressSharing: u32,
    pub dwAddressStates: u32,
    pub dwCallInfoStates: u32,
    pub dwCallerIDFlags: u32,
    pub dwCalledIDFlags: u32,
    pub dwConnectedIDFlags: u32,
    pub dwRedirectionIDFlags: u32,
    pub dwRedirectingIDFlags: u32,
    pub dwCallStates: u32,
    pub dwDialToneModes: u32,
    pub dwBusyModes: u32,
    pub dwSpecialInfo: u32,
    pub dwDisconnectModes: u32,
    pub dwMaxNumActiveCalls: u32,
    pub dwMaxNumOnHoldCalls: u32,
    pub dwMaxNumOnHoldPendingCalls: u32,
    pub dwMaxNumConference: u32,
    pub dwMaxNumTransConf: u32,
    pub dwAddrCapFlags: u32,
    pub dwCallFeatures: u32,
    pub dwRemoveFromConfCaps: u32,
    pub dwRemoveFromConfState: u32,
    pub dwTransferModes: u32,
    pub dwParkModes: u32,
    pub dwForwardModes: u32,
    pub dwMaxForwardEntries: u32,
    pub dwMaxSpecificEntries: u32,
    pub dwMinFwdNumRings: u32,
    pub dwMaxFwdNumRings: u32,
    pub dwMaxCallCompletions: u32,
    pub dwCallCompletionConds: u32,
    pub dwCallCompletionModes: u32,
    pub dwNumCompletionMessages: u32,
    pub dwCompletionMsgTextEntrySize: u32,
    pub dwCompletionMsgTextSize: u32,
    pub dwCompletionMsgTextOffset: u32,
    pub dwAddressFeatures: u32,
    pub dwPredictiveAutoTransferStates: u32,
    pub dwNumCallTreatments: u32,
    pub dwCallTreatmentListSize: u32,
    pub dwCallTreatmentListOffset: u32,
    pub dwDeviceClassesSize: u32,
    pub dwDeviceClassesOffset: u32,
    pub dwMaxCallDataSize: u32,
    pub dwCallFeatures2: u32,
    pub dwMaxNoAnswerTimeout: u32,
    pub dwConnectedModes: u32,
    pub dwOfferingModes: u32,
    pub dwAvailableMediaModes: u32,
}
impl ::core::marker::Copy for LINEADDRESSCAPS {}
impl ::core::clone::Clone for LINEADDRESSCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEADDRESSCAPS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEADDRESSSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumInUse: u32,
    pub dwNumActiveCalls: u32,
    pub dwNumOnHoldCalls: u32,
    pub dwNumOnHoldPendCalls: u32,
    pub dwAddressFeatures: u32,
    pub dwNumRingsNoAnswer: u32,
    pub dwForwardNumEntries: u32,
    pub dwForwardSize: u32,
    pub dwForwardOffset: u32,
    pub dwTerminalModesSize: u32,
    pub dwTerminalModesOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
}
impl ::core::marker::Copy for LINEADDRESSSTATUS {}
impl ::core::clone::Clone for LINEADDRESSSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEADDRESSSTATUS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTACTIVITYENTRY {
    pub dwID: u32,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTACTIVITYENTRY {}
impl ::core::clone::Clone for LINEAGENTACTIVITYENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTACTIVITYENTRY {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTACTIVITYLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTACTIVITYLIST {}
impl ::core::clone::Clone for LINEAGENTACTIVITYLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTACTIVITYLIST {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTCAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwAgentHandlerInfoSize: u32,
    pub dwAgentHandlerInfoOffset: u32,
    pub dwCapsVersion: u32,
    pub dwFeatures: u32,
    pub dwStates: u32,
    pub dwNextStates: u32,
    pub dwMaxNumGroupEntries: u32,
    pub dwAgentStatusMessages: u32,
    pub dwNumAgentExtensionIDs: u32,
    pub dwAgentExtensionIDListSize: u32,
    pub dwAgentExtensionIDListOffset: u32,
    pub ProxyGUID: ::windows::core::GUID,
}
impl ::core::marker::Copy for LINEAGENTCAPS {}
impl ::core::clone::Clone for LINEAGENTCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTCAPS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTENTRY {
    pub hAgent: u32,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
    pub dwIDSize: u32,
    pub dwIDOffset: u32,
    pub dwPINSize: u32,
    pub dwPINOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTENTRY {}
impl ::core::clone::Clone for LINEAGENTENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTENTRY {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTGROUPENTRY {
    pub GroupID: LINEAGENTGROUPENTRY_0,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTGROUPENTRY {}
impl ::core::clone::Clone for LINEAGENTGROUPENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTGROUPENTRY {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTGROUPENTRY_0 {
    pub dwGroupID1: u32,
    pub dwGroupID2: u32,
    pub dwGroupID3: u32,
    pub dwGroupID4: u32,
}
impl ::core::marker::Copy for LINEAGENTGROUPENTRY_0 {}
impl ::core::clone::Clone for LINEAGENTGROUPENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTGROUPENTRY_0 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTGROUPLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTGROUPLIST {}
impl ::core::clone::Clone for LINEAGENTGROUPLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTGROUPLIST {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEAGENTINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwAgentState: u32,
    pub dwNextAgentState: u32,
    pub dwMeasurementPeriod: u32,
    pub cyOverallCallRate: super::super::System::Com::CY,
    pub dwNumberOfACDCalls: u32,
    pub dwNumberOfIncomingCalls: u32,
    pub dwNumberOfOutgoingCalls: u32,
    pub dwTotalACDTalkTime: u32,
    pub dwTotalACDCallTime: u32,
    pub dwTotalACDWrapUpTime: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEAGENTINFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEAGENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEAGENTINFO {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTLIST {}
impl ::core::clone::Clone for LINEAGENTLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTLIST {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTSESSIONENTRY {
    pub hAgentSession: u32,
    pub hAgent: u32,
    pub GroupID: ::windows::core::GUID,
    pub dwWorkingAddressID: u32,
}
impl ::core::marker::Copy for LINEAGENTSESSIONENTRY {}
impl ::core::clone::Clone for LINEAGENTSESSIONENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTSESSIONENTRY {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEAGENTSESSIONINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwAgentSessionState: u32,
    pub dwNextAgentSessionState: u32,
    pub dateSessionStartTime: f64,
    pub dwSessionDuration: u32,
    pub dwNumberOfCalls: u32,
    pub dwTotalTalkTime: u32,
    pub dwAverageTalkTime: u32,
    pub dwTotalCallTime: u32,
    pub dwAverageCallTime: u32,
    pub dwTotalWrapUpTime: u32,
    pub dwAverageWrapUpTime: u32,
    pub cyACDCallRate: super::super::System::Com::CY,
    pub dwLongestTimeToAnswer: u32,
    pub dwAverageTimeToAnswer: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEAGENTSESSIONINFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEAGENTSESSIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEAGENTSESSIONINFO {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTSESSIONLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEAGENTSESSIONLIST {}
impl ::core::clone::Clone for LINEAGENTSESSIONLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTSESSIONLIST {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAGENTSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwGroupListSize: u32,
    pub dwGroupListOffset: u32,
    pub dwState: u32,
    pub dwNextState: u32,
    pub dwActivityID: u32,
    pub dwActivitySize: u32,
    pub dwActivityOffset: u32,
    pub dwAgentFeatures: u32,
    pub dwValidStates: u32,
    pub dwValidNextStates: u32,
}
impl ::core::marker::Copy for LINEAGENTSTATUS {}
impl ::core::clone::Clone for LINEAGENTSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAGENTSTATUS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEAPPINFO {
    pub dwMachineNameSize: u32,
    pub dwMachineNameOffset: u32,
    pub dwUserNameSize: u32,
    pub dwUserNameOffset: u32,
    pub dwModuleFilenameSize: u32,
    pub dwModuleFilenameOffset: u32,
    pub dwFriendlyNameSize: u32,
    pub dwFriendlyNameOffset: u32,
    pub dwMediaModes: u32,
    pub dwAddressID: u32,
}
impl ::core::marker::Copy for LINEAPPINFO {}
impl ::core::clone::Clone for LINEAPPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEAPPINFO {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECALLINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub hLine: u32,
    pub dwLineDeviceID: u32,
    pub dwAddressID: u32,
    pub dwBearerMode: u32,
    pub dwRate: u32,
    pub dwMediaMode: u32,
    pub dwAppSpecific: u32,
    pub dwCallID: u32,
    pub dwRelatedCallID: u32,
    pub dwCallParamFlags: u32,
    pub dwCallStates: u32,
    pub dwMonitorDigitModes: u32,
    pub dwMonitorMediaModes: u32,
    pub DialParams: LINEDIALPARAMS,
    pub dwOrigin: u32,
    pub dwReason: u32,
    pub dwCompletionID: u32,
    pub dwNumOwners: u32,
    pub dwNumMonitors: u32,
    pub dwCountryCode: u32,
    pub dwTrunk: u32,
    pub dwCallerIDFlags: u32,
    pub dwCallerIDSize: u32,
    pub dwCallerIDOffset: u32,
    pub dwCallerIDNameSize: u32,
    pub dwCallerIDNameOffset: u32,
    pub dwCalledIDFlags: u32,
    pub dwCalledIDSize: u32,
    pub dwCalledIDOffset: u32,
    pub dwCalledIDNameSize: u32,
    pub dwCalledIDNameOffset: u32,
    pub dwConnectedIDFlags: u32,
    pub dwConnectedIDSize: u32,
    pub dwConnectedIDOffset: u32,
    pub dwConnectedIDNameSize: u32,
    pub dwConnectedIDNameOffset: u32,
    pub dwRedirectionIDFlags: u32,
    pub dwRedirectionIDSize: u32,
    pub dwRedirectionIDOffset: u32,
    pub dwRedirectionIDNameSize: u32,
    pub dwRedirectionIDNameOffset: u32,
    pub dwRedirectingIDFlags: u32,
    pub dwRedirectingIDSize: u32,
    pub dwRedirectingIDOffset: u32,
    pub dwRedirectingIDNameSize: u32,
    pub dwRedirectingIDNameOffset: u32,
    pub dwAppNameSize: u32,
    pub dwAppNameOffset: u32,
    pub dwDisplayableAddressSize: u32,
    pub dwDisplayableAddressOffset: u32,
    pub dwCalledPartySize: u32,
    pub dwCalledPartyOffset: u32,
    pub dwCommentSize: u32,
    pub dwCommentOffset: u32,
    pub dwDisplaySize: u32,
    pub dwDisplayOffset: u32,
    pub dwUserUserInfoSize: u32,
    pub dwUserUserInfoOffset: u32,
    pub dwHighLevelCompSize: u32,
    pub dwHighLevelCompOffset: u32,
    pub dwLowLevelCompSize: u32,
    pub dwLowLevelCompOffset: u32,
    pub dwChargingInfoSize: u32,
    pub dwChargingInfoOffset: u32,
    pub dwTerminalModesSize: u32,
    pub dwTerminalModesOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwCallTreatment: u32,
    pub dwCallDataSize: u32,
    pub dwCallDataOffset: u32,
    pub dwSendingFlowspecSize: u32,
    pub dwSendingFlowspecOffset: u32,
    pub dwReceivingFlowspecSize: u32,
    pub dwReceivingFlowspecOffset: u32,
}
impl ::core::marker::Copy for LINECALLINFO {}
impl ::core::clone::Clone for LINECALLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINECALLINFO {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECALLLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwCallsNumEntries: u32,
    pub dwCallsSize: u32,
    pub dwCallsOffset: u32,
}
impl ::core::marker::Copy for LINECALLLIST {}
impl ::core::clone::Clone for LINECALLLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINECALLLIST {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECALLPARAMS {
    pub dwTotalSize: u32,
    pub dwBearerMode: u32,
    pub dwMinRate: u32,
    pub dwMaxRate: u32,
    pub dwMediaMode: u32,
    pub dwCallParamFlags: u32,
    pub dwAddressMode: u32,
    pub dwAddressID: u32,
    pub DialParams: LINEDIALPARAMS,
    pub dwOrigAddressSize: u32,
    pub dwOrigAddressOffset: u32,
    pub dwDisplayableAddressSize: u32,
    pub dwDisplayableAddressOffset: u32,
    pub dwCalledPartySize: u32,
    pub dwCalledPartyOffset: u32,
    pub dwCommentSize: u32,
    pub dwCommentOffset: u32,
    pub dwUserUserInfoSize: u32,
    pub dwUserUserInfoOffset: u32,
    pub dwHighLevelCompSize: u32,
    pub dwHighLevelCompOffset: u32,
    pub dwLowLevelCompSize: u32,
    pub dwLowLevelCompOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwPredictiveAutoTransferStates: u32,
    pub dwTargetAddressSize: u32,
    pub dwTargetAddressOffset: u32,
    pub dwSendingFlowspecSize: u32,
    pub dwSendingFlowspecOffset: u32,
    pub dwReceivingFlowspecSize: u32,
    pub dwReceivingFlowspecOffset: u32,
    pub dwDeviceClassSize: u32,
    pub dwDeviceClassOffset: u32,
    pub dwDeviceConfigSize: u32,
    pub dwDeviceConfigOffset: u32,
    pub dwCallDataSize: u32,
    pub dwCallDataOffset: u32,
    pub dwNoAnswerTimeout: u32,
    pub dwCallingPartyIDSize: u32,
    pub dwCallingPartyIDOffset: u32,
}
impl ::core::marker::Copy for LINECALLPARAMS {}
impl ::core::clone::Clone for LINECALLPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINECALLPARAMS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LINECALLSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwCallState: u32,
    pub dwCallStateMode: u32,
    pub dwCallPrivilege: u32,
    pub dwCallFeatures: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwCallFeatures2: u32,
    pub tStateEntryTime: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINECALLSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINECALLSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LINECALLSTATUS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECALLTREATMENTENTRY {
    pub dwCallTreatmentID: u32,
    pub dwCallTreatmentNameSize: u32,
    pub dwCallTreatmentNameOffset: u32,
}
impl ::core::marker::Copy for LINECALLTREATMENTENTRY {}
impl ::core::clone::Clone for LINECALLTREATMENTENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINECALLTREATMENTENTRY {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECARDENTRY {
    pub dwPermanentCardID: u32,
    pub dwCardNameSize: u32,
    pub dwCardNameOffset: u32,
    pub dwCardNumberDigits: u32,
    pub dwSameAreaRuleSize: u32,
    pub dwSameAreaRuleOffset: u32,
    pub dwLongDistanceRuleSize: u32,
    pub dwLongDistanceRuleOffset: u32,
    pub dwInternationalRuleSize: u32,
    pub dwInternationalRuleOffset: u32,
    pub dwOptions: u32,
}
impl ::core::marker::Copy for LINECARDENTRY {}
impl ::core::clone::Clone for LINECARDENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINECARDENTRY {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECOUNTRYENTRY {
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub dwNextCountryID: u32,
    pub dwCountryNameSize: u32,
    pub dwCountryNameOffset: u32,
    pub dwSameAreaRuleSize: u32,
    pub dwSameAreaRuleOffset: u32,
    pub dwLongDistanceRuleSize: u32,
    pub dwLongDistanceRuleOffset: u32,
    pub dwInternationalRuleSize: u32,
    pub dwInternationalRuleOffset: u32,
}
impl ::core::marker::Copy for LINECOUNTRYENTRY {}
impl ::core::clone::Clone for LINECOUNTRYENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINECOUNTRYENTRY {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINECOUNTRYLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumCountries: u32,
    pub dwCountryListSize: u32,
    pub dwCountryListOffset: u32,
}
impl ::core::marker::Copy for LINECOUNTRYLIST {}
impl ::core::clone::Clone for LINECOUNTRYLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINECOUNTRYLIST {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEDEVCAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwProviderInfoSize: u32,
    pub dwProviderInfoOffset: u32,
    pub dwSwitchInfoSize: u32,
    pub dwSwitchInfoOffset: u32,
    pub dwPermanentLineID: u32,
    pub dwLineNameSize: u32,
    pub dwLineNameOffset: u32,
    pub dwStringFormat: u32,
    pub dwAddressModes: u32,
    pub dwNumAddresses: u32,
    pub dwBearerModes: u32,
    pub dwMaxRate: u32,
    pub dwMediaModes: u32,
    pub dwGenerateToneModes: u32,
    pub dwGenerateToneMaxNumFreq: u32,
    pub dwGenerateDigitModes: u32,
    pub dwMonitorToneMaxNumFreq: u32,
    pub dwMonitorToneMaxNumEntries: u32,
    pub dwMonitorDigitModes: u32,
    pub dwGatherDigitsMinTimeout: u32,
    pub dwGatherDigitsMaxTimeout: u32,
    pub dwMedCtlDigitMaxListSize: u32,
    pub dwMedCtlMediaMaxListSize: u32,
    pub dwMedCtlToneMaxListSize: u32,
    pub dwMedCtlCallStateMaxListSize: u32,
    pub dwDevCapFlags: u32,
    pub dwMaxNumActiveCalls: u32,
    pub dwAnswerMode: u32,
    pub dwRingModes: u32,
    pub dwLineStates: u32,
    pub dwUUIAcceptSize: u32,
    pub dwUUIAnswerSize: u32,
    pub dwUUIMakeCallSize: u32,
    pub dwUUIDropSize: u32,
    pub dwUUISendUserUserInfoSize: u32,
    pub dwUUICallInfoSize: u32,
    pub MinDialParams: LINEDIALPARAMS,
    pub MaxDialParams: LINEDIALPARAMS,
    pub DefaultDialParams: LINEDIALPARAMS,
    pub dwNumTerminals: u32,
    pub dwTerminalCapsSize: u32,
    pub dwTerminalCapsOffset: u32,
    pub dwTerminalTextEntrySize: u32,
    pub dwTerminalTextSize: u32,
    pub dwTerminalTextOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwLineFeatures: u32,
    pub dwSettableDevStatus: u32,
    pub dwDeviceClassesSize: u32,
    pub dwDeviceClassesOffset: u32,
    pub PermanentLineGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for LINEDEVCAPS {}
impl ::core::clone::Clone for LINEDEVCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEDEVCAPS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEDEVSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumOpens: u32,
    pub dwOpenMediaModes: u32,
    pub dwNumActiveCalls: u32,
    pub dwNumOnHoldCalls: u32,
    pub dwNumOnHoldPendCalls: u32,
    pub dwLineFeatures: u32,
    pub dwNumCallCompletions: u32,
    pub dwRingMode: u32,
    pub dwSignalLevel: u32,
    pub dwBatteryLevel: u32,
    pub dwRoamMode: u32,
    pub dwDevStatusFlags: u32,
    pub dwTerminalModesSize: u32,
    pub dwTerminalModesOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwAvailableMediaModes: u32,
    pub dwAppInfoSize: u32,
    pub dwAppInfoOffset: u32,
}
impl ::core::marker::Copy for LINEDEVSTATUS {}
impl ::core::clone::Clone for LINEDEVSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEDEVSTATUS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEDIALPARAMS {
    pub dwDialPause: u32,
    pub dwDialSpeed: u32,
    pub dwDigitDuration: u32,
    pub dwWaitForDialtone: u32,
}
impl ::core::marker::Copy for LINEDIALPARAMS {}
impl ::core::clone::Clone for LINEDIALPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEDIALPARAMS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEEXTENSIONID {
    pub dwExtensionID0: u32,
    pub dwExtensionID1: u32,
    pub dwExtensionID2: u32,
    pub dwExtensionID3: u32,
}
impl ::core::marker::Copy for LINEEXTENSIONID {}
impl ::core::clone::Clone for LINEEXTENSIONID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEEXTENSIONID {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEFORWARD {
    pub dwForwardMode: u32,
    pub dwCallerAddressSize: u32,
    pub dwCallerAddressOffset: u32,
    pub dwDestCountryCode: u32,
    pub dwDestAddressSize: u32,
    pub dwDestAddressOffset: u32,
}
impl ::core::marker::Copy for LINEFORWARD {}
impl ::core::clone::Clone for LINEFORWARD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEFORWARD {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEFORWARDLIST {
    pub dwTotalSize: u32,
    pub dwNumEntries: u32,
    pub ForwardList: [LINEFORWARD; 1],
}
impl ::core::marker::Copy for LINEFORWARDLIST {}
impl ::core::clone::Clone for LINEFORWARDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEFORWARDLIST {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEGENERATETONE {
    pub dwFrequency: u32,
    pub dwCadenceOn: u32,
    pub dwCadenceOff: u32,
    pub dwVolume: u32,
}
impl ::core::marker::Copy for LINEGENERATETONE {}
impl ::core::clone::Clone for LINEGENERATETONE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEGENERATETONE {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LINEINITIALIZEEXPARAMS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwOptions: u32,
    pub Handles: LINEINITIALIZEEXPARAMS_0,
    pub dwCompletionKey: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINEINITIALIZEEXPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINEINITIALIZEEXPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LINEINITIALIZEEXPARAMS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union LINEINITIALIZEEXPARAMS_0 {
    pub hEvent: super::super::Foundation::HANDLE,
    pub hCompletionPort: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINEINITIALIZEEXPARAMS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINEINITIALIZEEXPARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LINEINITIALIZEEXPARAMS_0 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINELOCATIONENTRY {
    pub dwPermanentLocationID: u32,
    pub dwLocationNameSize: u32,
    pub dwLocationNameOffset: u32,
    pub dwCountryCode: u32,
    pub dwCityCodeSize: u32,
    pub dwCityCodeOffset: u32,
    pub dwPreferredCardID: u32,
    pub dwLocalAccessCodeSize: u32,
    pub dwLocalAccessCodeOffset: u32,
    pub dwLongDistanceAccessCodeSize: u32,
    pub dwLongDistanceAccessCodeOffset: u32,
    pub dwTollPrefixListSize: u32,
    pub dwTollPrefixListOffset: u32,
    pub dwCountryID: u32,
    pub dwOptions: u32,
    pub dwCancelCallWaitingSize: u32,
    pub dwCancelCallWaitingOffset: u32,
}
impl ::core::marker::Copy for LINELOCATIONENTRY {}
impl ::core::clone::Clone for LINELOCATIONENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINELOCATIONENTRY {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMEDIACONTROLCALLSTATE {
    pub dwCallStates: u32,
    pub dwMediaControl: u32,
}
impl ::core::marker::Copy for LINEMEDIACONTROLCALLSTATE {}
impl ::core::clone::Clone for LINEMEDIACONTROLCALLSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEMEDIACONTROLCALLSTATE {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMEDIACONTROLDIGIT {
    pub dwDigit: u32,
    pub dwDigitModes: u32,
    pub dwMediaControl: u32,
}
impl ::core::marker::Copy for LINEMEDIACONTROLDIGIT {}
impl ::core::clone::Clone for LINEMEDIACONTROLDIGIT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEMEDIACONTROLDIGIT {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMEDIACONTROLMEDIA {
    pub dwMediaModes: u32,
    pub dwDuration: u32,
    pub dwMediaControl: u32,
}
impl ::core::marker::Copy for LINEMEDIACONTROLMEDIA {}
impl ::core::clone::Clone for LINEMEDIACONTROLMEDIA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEMEDIACONTROLMEDIA {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMEDIACONTROLTONE {
    pub dwAppSpecific: u32,
    pub dwDuration: u32,
    pub dwFrequency1: u32,
    pub dwFrequency2: u32,
    pub dwFrequency3: u32,
    pub dwMediaControl: u32,
}
impl ::core::marker::Copy for LINEMEDIACONTROLTONE {}
impl ::core::clone::Clone for LINEMEDIACONTROLTONE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEMEDIACONTROLTONE {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMESSAGE {
    pub hDevice: u32,
    pub dwMessageID: u32,
    pub dwCallbackInstance: usize,
    pub dwParam1: usize,
    pub dwParam2: usize,
    pub dwParam3: usize,
}
impl ::core::marker::Copy for LINEMESSAGE {}
impl ::core::clone::Clone for LINEMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEMESSAGE {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEMONITORTONE {
    pub dwAppSpecific: u32,
    pub dwDuration: u32,
    pub dwFrequency1: u32,
    pub dwFrequency2: u32,
    pub dwFrequency3: u32,
}
impl ::core::marker::Copy for LINEMONITORTONE {}
impl ::core::clone::Clone for LINEMONITORTONE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEMONITORTONE {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEPROVIDERENTRY {
    pub dwPermanentProviderID: u32,
    pub dwProviderFilenameSize: u32,
    pub dwProviderFilenameOffset: u32,
}
impl ::core::marker::Copy for LINEPROVIDERENTRY {}
impl ::core::clone::Clone for LINEPROVIDERENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEPROVIDERENTRY {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEPROVIDERLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumProviders: u32,
    pub dwProviderListSize: u32,
    pub dwProviderListOffset: u32,
}
impl ::core::marker::Copy for LINEPROVIDERLIST {}
impl ::core::clone::Clone for LINEPROVIDERLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEPROVIDERLIST {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST {
    pub dwSize: u32,
    pub dwClientMachineNameSize: u32,
    pub dwClientMachineNameOffset: u32,
    pub dwClientUserNameSize: u32,
    pub dwClientUserNameOffset: u32,
    pub dwClientAppAPIVersion: u32,
    pub dwRequestType: u32,
    pub Anonymous: LINEPROXYREQUEST_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union LINEPROXYREQUEST_0 {
    pub SetAgentGroup: LINEPROXYREQUEST_0_14,
    pub SetAgentState: LINEPROXYREQUEST_0_18,
    pub SetAgentActivity: LINEPROXYREQUEST_0_13,
    pub GetAgentCaps: LINEPROXYREQUEST_0_4,
    pub GetAgentStatus: LINEPROXYREQUEST_0_9,
    pub AgentSpecific: LINEPROXYREQUEST_0_0,
    pub GetAgentActivityList: LINEPROXYREQUEST_0_3,
    pub GetAgentGroupList: LINEPROXYREQUEST_0_5,
    pub CreateAgent: LINEPROXYREQUEST_0_2,
    pub SetAgentStateEx: LINEPROXYREQUEST_0_17,
    pub SetAgentMeasurementPeriod: LINEPROXYREQUEST_0_15,
    pub GetAgentInfo: LINEPROXYREQUEST_0_6,
    pub CreateAgentSession: LINEPROXYREQUEST_0_1,
    pub GetAgentSessionList: LINEPROXYREQUEST_0_8,
    pub GetAgentSessionInfo: LINEPROXYREQUEST_0_7,
    pub SetAgentSessionState: LINEPROXYREQUEST_0_16,
    pub GetQueueList: LINEPROXYREQUEST_0_12,
    pub SetQueueMeasurementPeriod: LINEPROXYREQUEST_0_19,
    pub GetQueueInfo: LINEPROXYREQUEST_0_11,
    pub GetGroupList: LINEPROXYREQUEST_0_10,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_0 {
    pub dwAddressID: u32,
    pub dwAgentExtensionIDIndex: u32,
    pub dwSize: u32,
    pub Params: [u8; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_0 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_1 {
    pub hAgentSession: u32,
    pub dwAgentPINSize: u32,
    pub dwAgentPINOffset: u32,
    pub hAgent: u32,
    pub GroupID: ::windows::core::GUID,
    pub dwWorkingAddressID: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_1 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_1 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_2 {
    pub hAgent: u32,
    pub dwAgentIDSize: u32,
    pub dwAgentIDOffset: u32,
    pub dwAgentPINSize: u32,
    pub dwAgentPINOffset: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_2 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_3 {
    pub dwAddressID: u32,
    pub ActivityList: LINEAGENTACTIVITYLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_3 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_4 {
    pub dwAddressID: u32,
    pub AgentCaps: LINEAGENTCAPS,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_4 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_5 {
    pub dwAddressID: u32,
    pub GroupList: LINEAGENTGROUPLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_5 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_5 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_6 {
    pub hAgent: u32,
    pub AgentInfo: LINEAGENTINFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_6 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_6 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_7 {
    pub hAgentSession: u32,
    pub SessionInfo: LINEAGENTSESSIONINFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_7 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_7 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_8 {
    pub hAgent: u32,
    pub SessionList: LINEAGENTSESSIONLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_8 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_8 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_9 {
    pub dwAddressID: u32,
    pub AgentStatus: LINEAGENTSTATUS,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_9 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_9 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_10 {
    pub GroupList: LINEAGENTGROUPLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_10 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_10 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_11 {
    pub dwQueueID: u32,
    pub QueueInfo: LINEQUEUEINFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_11 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_11 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_12 {
    pub GroupID: ::windows::core::GUID,
    pub QueueList: LINEQUEUELIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_12 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_12 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_12 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_13 {
    pub dwAddressID: u32,
    pub dwActivityID: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_13 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_13 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_13 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_14 {
    pub dwAddressID: u32,
    pub GroupList: LINEAGENTGROUPLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_14 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_14 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_14 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_15 {
    pub hAgent: u32,
    pub dwMeasurementPeriod: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_15 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_15 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_15 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_16 {
    pub hAgentSession: u32,
    pub dwAgentSessionState: u32,
    pub dwNextAgentSessionState: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_16 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_16 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_17 {
    pub hAgent: u32,
    pub dwAgentState: u32,
    pub dwNextAgentState: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_17 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_17 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_17 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_18 {
    pub dwAddressID: u32,
    pub dwAgentState: u32,
    pub dwNextAgentState: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_18 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_18 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_18 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct LINEPROXYREQUEST_0_19 {
    pub dwQueueID: u32,
    pub dwMeasurementPeriod: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for LINEPROXYREQUEST_0_19 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for LINEPROXYREQUEST_0_19 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for LINEPROXYREQUEST_0_19 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEPROXYREQUESTLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEPROXYREQUESTLIST {}
impl ::core::clone::Clone for LINEPROXYREQUESTLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEPROXYREQUESTLIST {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEQUEUEENTRY {
    pub dwQueueID: u32,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
}
impl ::core::marker::Copy for LINEQUEUEENTRY {}
impl ::core::clone::Clone for LINEQUEUEENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEQUEUEENTRY {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEQUEUEINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwMeasurementPeriod: u32,
    pub dwTotalCallsQueued: u32,
    pub dwCurrentCallsQueued: u32,
    pub dwTotalCallsAbandoned: u32,
    pub dwTotalCallsFlowedIn: u32,
    pub dwTotalCallsFlowedOut: u32,
    pub dwLongestEverWaitTime: u32,
    pub dwCurrentLongestWaitTime: u32,
    pub dwAverageWaitTime: u32,
    pub dwFinalDisposition: u32,
}
impl ::core::marker::Copy for LINEQUEUEINFO {}
impl ::core::clone::Clone for LINEQUEUEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEQUEUEINFO {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEQUEUELIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl ::core::marker::Copy for LINEQUEUELIST {}
impl ::core::clone::Clone for LINEQUEUELIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEQUEUELIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LINEREQMAKECALL {
    pub szDestAddress: [super::super::Foundation::CHAR; 80],
    pub szAppName: [super::super::Foundation::CHAR; 40],
    pub szCalledParty: [super::super::Foundation::CHAR; 40],
    pub szComment: [super::super::Foundation::CHAR; 80],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINEREQMAKECALL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINEREQMAKECALL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LINEREQMAKECALL {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINEREQMAKECALLW {
    pub szDestAddress: [u16; 80],
    pub szAppName: [u16; 40],
    pub szCalledParty: [u16; 40],
    pub szComment: [u16; 80],
}
impl ::core::marker::Copy for LINEREQMAKECALLW {}
impl ::core::clone::Clone for LINEREQMAKECALLW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINEREQMAKECALLW {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LINEREQMEDIACALL {
    pub hWnd: super::super::Foundation::HWND,
    pub wRequestID: super::super::Foundation::WPARAM,
    pub szDeviceClass: [super::super::Foundation::CHAR; 40],
    pub ucDeviceID: [u8; 40],
    pub dwSize: u32,
    pub dwSecure: u32,
    pub szDestAddress: [super::super::Foundation::CHAR; 80],
    pub szAppName: [super::super::Foundation::CHAR; 40],
    pub szCalledParty: [super::super::Foundation::CHAR; 40],
    pub szComment: [super::super::Foundation::CHAR; 80],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINEREQMEDIACALL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINEREQMEDIACALL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LINEREQMEDIACALL {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LINEREQMEDIACALLW {
    pub hWnd: super::super::Foundation::HWND,
    pub wRequestID: super::super::Foundation::WPARAM,
    pub szDeviceClass: [u16; 40],
    pub ucDeviceID: [u8; 40],
    pub dwSize: u32,
    pub dwSecure: u32,
    pub szDestAddress: [u16; 80],
    pub szAppName: [u16; 40],
    pub szCalledParty: [u16; 40],
    pub szComment: [u16; 80],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINEREQMEDIACALLW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINEREQMEDIACALLW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LINEREQMEDIACALLW {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINETERMCAPS {
    pub dwTermDev: u32,
    pub dwTermModes: u32,
    pub dwTermSharing: u32,
}
impl ::core::marker::Copy for LINETERMCAPS {}
impl ::core::clone::Clone for LINETERMCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINETERMCAPS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINETRANSLATECAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumLocations: u32,
    pub dwLocationListSize: u32,
    pub dwLocationListOffset: u32,
    pub dwCurrentLocationID: u32,
    pub dwNumCards: u32,
    pub dwCardListSize: u32,
    pub dwCardListOffset: u32,
    pub dwCurrentPreferredCardID: u32,
}
impl ::core::marker::Copy for LINETRANSLATECAPS {}
impl ::core::clone::Clone for LINETRANSLATECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINETRANSLATECAPS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct LINETRANSLATEOUTPUT {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwDialableStringSize: u32,
    pub dwDialableStringOffset: u32,
    pub dwDisplayableStringSize: u32,
    pub dwDisplayableStringOffset: u32,
    pub dwCurrentCountry: u32,
    pub dwDestCountry: u32,
    pub dwTranslateResults: u32,
}
impl ::core::marker::Copy for LINETRANSLATEOUTPUT {}
impl ::core::clone::Clone for LINETRANSLATEOUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINETRANSLATEOUTPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO {
    pub dwSize: u32,
    pub Event: MSP_EVENT,
    pub hCall: *mut i32,
    pub Anonymous: MSP_EVENT_INFO_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MSP_EVENT_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union MSP_EVENT_INFO_0 {
    pub MSP_ADDRESS_EVENT_INFO: ::std::mem::ManuallyDrop<MSP_EVENT_INFO_0_0>,
    pub MSP_CALL_EVENT_INFO: ::std::mem::ManuallyDrop<MSP_EVENT_INFO_0_2>,
    pub MSP_TSP_DATA: MSP_EVENT_INFO_0_6,
    pub MSP_PRIVATE_EVENT_INFO: ::std::mem::ManuallyDrop<MSP_EVENT_INFO_0_4>,
    pub MSP_FILE_TERMINAL_EVENT_INFO: ::std::mem::ManuallyDrop<MSP_EVENT_INFO_0_3>,
    pub MSP_ASR_TERMINAL_EVENT_INFO: ::std::mem::ManuallyDrop<MSP_EVENT_INFO_0_1>,
    pub MSP_TTS_TERMINAL_EVENT_INFO: ::std::mem::ManuallyDrop<MSP_EVENT_INFO_0_7>,
    pub MSP_TONE_TERMINAL_EVENT_INFO: ::std::mem::ManuallyDrop<MSP_EVENT_INFO_0_5>,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MSP_EVENT_INFO_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_0 {
    pub Type: MSP_ADDRESS_EVENT,
    pub pTerminal: ::windows::core::ManuallyDrop<ITTerminal>,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MSP_EVENT_INFO_0_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_1 {
    pub pASRTerminal: ::windows::core::ManuallyDrop<ITTerminal>,
    pub hrErrorCode: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MSP_EVENT_INFO_0_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_2 {
    pub Type: MSP_CALL_EVENT,
    pub Cause: MSP_CALL_EVENT_CAUSE,
    pub pStream: ::windows::core::ManuallyDrop<ITStream>,
    pub pTerminal: ::windows::core::ManuallyDrop<ITTerminal>,
    pub hrError: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_2 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MSP_EVENT_INFO_0_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_3 {
    pub pParentFileTerminal: ::windows::core::ManuallyDrop<ITTerminal>,
    pub pFileTrack: ::windows::core::ManuallyDrop<ITFileTrack>,
    pub TerminalMediaState: TERMINAL_MEDIA_STATE,
    pub ftecEventCause: FT_STATE_EVENT_CAUSE,
    pub hrErrorCode: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_3 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MSP_EVENT_INFO_0_3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_4 {
    pub pEvent: ::windows::core::ManuallyDrop<super::super::System::Com::IDispatch>,
    pub lEventCode: i32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_4 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MSP_EVENT_INFO_0_4 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_5 {
    pub pToneTerminal: ::windows::core::ManuallyDrop<ITTerminal>,
    pub hrErrorCode: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_5 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MSP_EVENT_INFO_0_5 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_6 {
    pub dwBufferSize: u32,
    pub pBuffer: [u8; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MSP_EVENT_INFO_0_6 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MSP_EVENT_INFO_0_6 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO_0_7 {
    pub pTTSTerminal: ::windows::core::ManuallyDrop<ITTerminal>,
    pub hrErrorCode: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MSP_EVENT_INFO_0_7 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MSP_EVENT_INFO_0_7 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NSID {
    pub dwSize: u32,
    pub uchType: [u8; 16],
    pub xtype: u32,
    pub lTime: i32,
    pub address: NSID_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NSID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NSID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NSID {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NSID_0 {
    pub alias: ADDRALIAS,
    pub rgchInterNet: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NSID_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NSID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NSID_0 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct PHONEBUTTONINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwButtonMode: u32,
    pub dwButtonFunction: u32,
    pub dwButtonTextSize: u32,
    pub dwButtonTextOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwButtonState: u32,
}
impl ::core::marker::Copy for PHONEBUTTONINFO {}
impl ::core::clone::Clone for PHONEBUTTONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONEBUTTONINFO {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct PHONECAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwProviderInfoSize: u32,
    pub dwProviderInfoOffset: u32,
    pub dwPhoneInfoSize: u32,
    pub dwPhoneInfoOffset: u32,
    pub dwPermanentPhoneID: u32,
    pub dwPhoneNameSize: u32,
    pub dwPhoneNameOffset: u32,
    pub dwStringFormat: u32,
    pub dwPhoneStates: u32,
    pub dwHookSwitchDevs: u32,
    pub dwHandsetHookSwitchModes: u32,
    pub dwSpeakerHookSwitchModes: u32,
    pub dwHeadsetHookSwitchModes: u32,
    pub dwVolumeFlags: u32,
    pub dwGainFlags: u32,
    pub dwDisplayNumRows: u32,
    pub dwDisplayNumColumns: u32,
    pub dwNumRingModes: u32,
    pub dwNumButtonLamps: u32,
    pub dwButtonModesSize: u32,
    pub dwButtonModesOffset: u32,
    pub dwButtonFunctionsSize: u32,
    pub dwButtonFunctionsOffset: u32,
    pub dwLampModesSize: u32,
    pub dwLampModesOffset: u32,
    pub dwNumSetData: u32,
    pub dwSetDataSize: u32,
    pub dwSetDataOffset: u32,
    pub dwNumGetData: u32,
    pub dwGetDataSize: u32,
    pub dwGetDataOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwDeviceClassesSize: u32,
    pub dwDeviceClassesOffset: u32,
    pub dwPhoneFeatures: u32,
    pub dwSettableHandsetHookSwitchModes: u32,
    pub dwSettableSpeakerHookSwitchModes: u32,
    pub dwSettableHeadsetHookSwitchModes: u32,
    pub dwMonitoredHandsetHookSwitchModes: u32,
    pub dwMonitoredSpeakerHookSwitchModes: u32,
    pub dwMonitoredHeadsetHookSwitchModes: u32,
    pub PermanentPhoneGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for PHONECAPS {}
impl ::core::clone::Clone for PHONECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONECAPS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct PHONEEXTENSIONID {
    pub dwExtensionID0: u32,
    pub dwExtensionID1: u32,
    pub dwExtensionID2: u32,
    pub dwExtensionID3: u32,
}
impl ::core::marker::Copy for PHONEEXTENSIONID {}
impl ::core::clone::Clone for PHONEEXTENSIONID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONEEXTENSIONID {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PHONEINITIALIZEEXPARAMS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwOptions: u32,
    pub Handles: PHONEINITIALIZEEXPARAMS_0,
    pub dwCompletionKey: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PHONEINITIALIZEEXPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PHONEINITIALIZEEXPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PHONEINITIALIZEEXPARAMS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PHONEINITIALIZEEXPARAMS_0 {
    pub hEvent: super::super::Foundation::HANDLE,
    pub hCompletionPort: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PHONEINITIALIZEEXPARAMS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PHONEINITIALIZEEXPARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PHONEINITIALIZEEXPARAMS_0 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct PHONEMESSAGE {
    pub hDevice: u32,
    pub dwMessageID: u32,
    pub dwCallbackInstance: usize,
    pub dwParam1: usize,
    pub dwParam2: usize,
    pub dwParam3: usize,
}
impl ::core::marker::Copy for PHONEMESSAGE {}
impl ::core::clone::Clone for PHONEMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONEMESSAGE {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct PHONESTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwStatusFlags: u32,
    pub dwNumOwners: u32,
    pub dwNumMonitors: u32,
    pub dwRingMode: u32,
    pub dwRingVolume: u32,
    pub dwHandsetHookSwitchMode: u32,
    pub dwHandsetVolume: u32,
    pub dwHandsetGain: u32,
    pub dwSpeakerHookSwitchMode: u32,
    pub dwSpeakerVolume: u32,
    pub dwSpeakerGain: u32,
    pub dwHeadsetHookSwitchMode: u32,
    pub dwHeadsetVolume: u32,
    pub dwHeadsetGain: u32,
    pub dwDisplaySize: u32,
    pub dwDisplayOffset: u32,
    pub dwLampModesSize: u32,
    pub dwLampModesOffset: u32,
    pub dwOwnerNameSize: u32,
    pub dwOwnerNameOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwPhoneFeatures: u32,
}
impl ::core::marker::Copy for PHONESTATUS {}
impl ::core::clone::Clone for PHONESTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PHONESTATUS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct RENDDATA {
    pub atyp: u16,
    pub ulPosition: u32,
    pub dxWidth: u16,
    pub dyHeight: u16,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for RENDDATA {}
impl ::core::clone::Clone for RENDDATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RENDDATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct STnefProblem {
    pub ulComponent: u32,
    pub ulAttribute: u32,
    pub ulPropTag: u32,
    pub scode: i32,
}
impl ::core::marker::Copy for STnefProblem {}
impl ::core::clone::Clone for STnefProblem {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for STnefProblem {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct STnefProblemArray {
    pub cProblem: u32,
    pub aProblem: [STnefProblem; 1],
}
impl ::core::marker::Copy for STnefProblemArray {}
impl ::core::clone::Clone for STnefProblemArray {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for STnefProblemArray {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct TAPI_CUSTOMTONE {
    pub dwFrequency: u32,
    pub dwCadenceOn: u32,
    pub dwCadenceOff: u32,
    pub dwVolume: u32,
}
impl ::core::marker::Copy for TAPI_CUSTOMTONE {}
impl ::core::clone::Clone for TAPI_CUSTOMTONE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TAPI_CUSTOMTONE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct TAPI_DETECTTONE {
    pub dwAppSpecific: u32,
    pub dwDuration: u32,
    pub dwFrequency1: u32,
    pub dwFrequency2: u32,
    pub dwFrequency3: u32,
}
impl ::core::marker::Copy for TAPI_DETECTTONE {}
impl ::core::clone::Clone for TAPI_DETECTTONE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TAPI_DETECTTONE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct TRP {
    pub trpid: u16,
    pub cbgrtrp: u16,
    pub cch: u16,
    pub cbRgb: u16,
}
impl ::core::marker::Copy for TRP {}
impl ::core::clone::Clone for TRP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct TUISPICREATEDIALOGINSTANCEPARAMS {
    pub dwRequestID: u32,
    pub hdDlgInst: *mut HDRVDIALOGINSTANCE__,
    pub htDlgInst: u32,
    pub lpszUIDLLName: ::windows::core::PCWSTR,
    pub lpParams: *mut ::core::ffi::c_void,
    pub dwSize: u32,
}
impl ::core::marker::Copy for TUISPICREATEDIALOGINSTANCEPARAMS {}
impl ::core::clone::Clone for TUISPICREATEDIALOGINSTANCEPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TUISPICREATEDIALOGINSTANCEPARAMS {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub struct VARSTRING {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwStringFormat: u32,
    pub dwStringSize: u32,
    pub dwStringOffset: u32,
}
impl ::core::marker::Copy for VARSTRING {}
impl ::core::clone::Clone for VARSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VARSTRING {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type ASYNC_COMPLETION = ::core::option::Option<unsafe extern "system" fn(dwrequestid: u32, lresult: i32) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type LINECALLBACK = ::core::option::Option<unsafe extern "system" fn(hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type LINEEVENT = ::core::option::Option<unsafe extern "system" fn(htline: *mut HTAPILINE__, htcall: *mut HTAPICALL__, dwmsg: u32, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type LPGETTNEFSTREAMCODEPAGE = ::core::option::Option<unsafe extern "system" fn(lpstream: ::core::option::Option<super::super::System::Com::IStream>, lpulcodepage: *mut u32, lpulsubcodepage: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub type LPOPENTNEFSTREAM = ::core::option::Option<unsafe extern "system" fn(lpvsupport: *mut ::core::ffi::c_void, lpstream: ::core::option::Option<super::super::System::Com::IStream>, lpszstreamname: *const i8, ulflags: u32, lpmessage: ::core::option::Option<super::super::System::AddressBook::IMessage>, wkeyval: u16, lpptnef: *mut ::core::option::Option<ITnef>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub type LPOPENTNEFSTREAMEX = ::core::option::Option<unsafe extern "system" fn(lpvsupport: *mut ::core::ffi::c_void, lpstream: ::core::option::Option<super::super::System::Com::IStream>, lpszstreamname: *const i8, ulflags: u32, lpmessage: ::core::option::Option<super::super::System::AddressBook::IMessage>, wkeyval: u16, lpadressbook: ::core::option::Option<super::super::System::AddressBook::IAddrBook>, lpptnef: *mut ::core::option::Option<ITnef>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONECALLBACK = ::core::option::Option<unsafe extern "system" fn(hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type PHONEEVENT = ::core::option::Option<unsafe extern "system" fn(htphone: *mut HTAPIPHONE__, dwmsg: u32, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ()>;
#[doc = "*Required features: `\"Win32_Devices_Tapi\"`*"]
pub type TUISPIDLLCALLBACK = ::core::option::Option<unsafe extern "system" fn(dwobjectid: usize, dwobjecttype: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
