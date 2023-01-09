#[cfg(feature = "Win32_Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AbortPrinter<P0>(hprinter: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AbortPrinter ( hprinter : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    AbortPrinter(hprinter.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddFormA<P0>(hprinter: P0, level: u32, pform: *const u8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddFormA ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pform : *const u8 ) -> super::super::Foundation:: BOOL );
    AddFormA(hprinter.into(), level, pform)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddFormW<P0>(hprinter: P0, level: u32, pform: *const u8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddFormW ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pform : *const u8 ) -> super::super::Foundation:: BOOL );
    AddFormW(hprinter.into(), level, pform)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddJobA<P0>(hprinter: P0, level: u32, pdata: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddJobA ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pdata : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    AddJobA(hprinter.into(), level, ::core::mem::transmute(pdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddJobW<P0>(hprinter: P0, level: u32, pdata: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddJobW ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pdata : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    AddJobW(hprinter.into(), level, ::core::mem::transmute(pdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddMonitorA<P0>(pname: P0, level: u32, pmonitors: ::core::option::Option<*const u8>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddMonitorA ( pname : :: windows::core::PCSTR , level : u32 , pmonitors : *const u8 ) -> super::super::Foundation:: BOOL );
    AddMonitorA(pname.into().abi(), level, ::core::mem::transmute(pmonitors.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddMonitorW<P0>(pname: P0, level: u32, pmonitors: ::core::option::Option<*const u8>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddMonitorW ( pname : :: windows::core::PCWSTR , level : u32 , pmonitors : *const u8 ) -> super::super::Foundation:: BOOL );
    AddMonitorW(pname.into().abi(), level, ::core::mem::transmute(pmonitors.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPortA<P0, P1, P2>(pname: P0, hwnd: P1, pmonitorname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPortA ( pname : :: windows::core::PCSTR , hwnd : super::super::Foundation:: HWND , pmonitorname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    AddPortA(pname.into().abi(), hwnd.into(), pmonitorname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPortW<P0, P1, P2>(pname: P0, hwnd: P1, pmonitorname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPortW ( pname : :: windows::core::PCWSTR , hwnd : super::super::Foundation:: HWND , pmonitorname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    AddPortW(pname.into().abi(), hwnd.into(), pmonitorname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrintDeviceObject<P0>(hprinter: P0) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn AddPrintDeviceObject ( hprinter : super::super::Foundation:: HANDLE , phdeviceobject : *mut super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    AddPrintDeviceObject(hprinter.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrintProcessorA<P0, P1, P2, P3>(pname: P0, penvironment: P1, ppathname: P2, pprintprocessorname: P3) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrintProcessorA ( pname : :: windows::core::PCSTR , penvironment : :: windows::core::PCSTR , ppathname : :: windows::core::PCSTR , pprintprocessorname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    AddPrintProcessorA(pname.into().abi(), penvironment.into().abi(), ppathname.into().abi(), pprintprocessorname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrintProcessorW<P0, P1, P2, P3>(pname: P0, penvironment: P1, ppathname: P2, pprintprocessorname: P3) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrintProcessorW ( pname : :: windows::core::PCWSTR , penvironment : :: windows::core::PCWSTR , ppathname : :: windows::core::PCWSTR , pprintprocessorname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    AddPrintProcessorW(pname.into().abi(), penvironment.into().abi(), ppathname.into().abi(), pprintprocessorname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrintProvidorA<P0>(pname: P0, level: u32, pprovidorinfo: *const u8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrintProvidorA ( pname : :: windows::core::PCSTR , level : u32 , pprovidorinfo : *const u8 ) -> super::super::Foundation:: BOOL );
    AddPrintProvidorA(pname.into().abi(), level, pprovidorinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrintProvidorW<P0>(pname: P0, level: u32, pprovidorinfo: *const u8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrintProvidorW ( pname : :: windows::core::PCWSTR , level : u32 , pprovidorinfo : *const u8 ) -> super::super::Foundation:: BOOL );
    AddPrintProvidorW(pname.into().abi(), level, pprovidorinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrinterA<P0>(pname: P0, level: u32, pprinter: *const u8) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrinterA ( pname : :: windows::core::PCSTR , level : u32 , pprinter : *const u8 ) -> super::super::Foundation:: HANDLE );
    AddPrinterA(pname.into().abi(), level, pprinter)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrinterConnection2A<P0, P1>(hwnd: P0, pszname: P1, dwlevel: u32, pconnectioninfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrinterConnection2A ( hwnd : super::super::Foundation:: HWND , pszname : :: windows::core::PCSTR , dwlevel : u32 , pconnectioninfo : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    AddPrinterConnection2A(hwnd.into(), pszname.into().abi(), dwlevel, pconnectioninfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrinterConnection2W<P0, P1>(hwnd: P0, pszname: P1, dwlevel: u32, pconnectioninfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrinterConnection2W ( hwnd : super::super::Foundation:: HWND , pszname : :: windows::core::PCWSTR , dwlevel : u32 , pconnectioninfo : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    AddPrinterConnection2W(hwnd.into(), pszname.into().abi(), dwlevel, pconnectioninfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrinterConnectionA<P0>(pname: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrinterConnectionA ( pname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    AddPrinterConnectionA(pname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrinterConnectionW<P0>(pname: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrinterConnectionW ( pname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    AddPrinterConnectionW(pname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrinterDriverA<P0>(pname: P0, level: u32, pdriverinfo: *const u8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrinterDriverA ( pname : :: windows::core::PCSTR , level : u32 , pdriverinfo : *const u8 ) -> super::super::Foundation:: BOOL );
    AddPrinterDriverA(pname.into().abi(), level, pdriverinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrinterDriverExA<P0>(pname: P0, level: u32, lpbdriverinfo: *const u8, dwfilecopyflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrinterDriverExA ( pname : :: windows::core::PCSTR , level : u32 , lpbdriverinfo : *const u8 , dwfilecopyflags : u32 ) -> super::super::Foundation:: BOOL );
    AddPrinterDriverExA(pname.into().abi(), level, lpbdriverinfo, dwfilecopyflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrinterDriverExW<P0>(pname: P0, level: u32, lpbdriverinfo: *const u8, dwfilecopyflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrinterDriverExW ( pname : :: windows::core::PCWSTR , level : u32 , lpbdriverinfo : *const u8 , dwfilecopyflags : u32 ) -> super::super::Foundation:: BOOL );
    AddPrinterDriverExW(pname.into().abi(), level, lpbdriverinfo, dwfilecopyflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrinterDriverW<P0>(pname: P0, level: u32, pdriverinfo: *const u8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrinterDriverW ( pname : :: windows::core::PCWSTR , level : u32 , pdriverinfo : *const u8 ) -> super::super::Foundation:: BOOL );
    AddPrinterDriverW(pname.into().abi(), level, pdriverinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPrinterW<P0>(pname: P0, level: u32, pprinter: *const u8) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AddPrinterW ( pname : :: windows::core::PCWSTR , level : u32 , pprinter : *const u8 ) -> super::super::Foundation:: HANDLE );
    AddPrinterW(pname.into().abi(), level, pprinter)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn AdvancedDocumentPropertiesA<P0, P1, P2>(hwnd: P0, hprinter: P1, pdevicename: P2, pdevmodeoutput: ::core::option::Option<*mut super::Gdi::DEVMODEA>, pdevmodeinput: ::core::option::Option<*const super::Gdi::DEVMODEA>) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AdvancedDocumentPropertiesA ( hwnd : super::super::Foundation:: HWND , hprinter : super::super::Foundation:: HANDLE , pdevicename : :: windows::core::PCSTR , pdevmodeoutput : *mut super::Gdi:: DEVMODEA , pdevmodeinput : *const super::Gdi:: DEVMODEA ) -> i32 );
    AdvancedDocumentPropertiesA(hwnd.into(), hprinter.into(), pdevicename.into().abi(), ::core::mem::transmute(pdevmodeoutput.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdevmodeinput.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn AdvancedDocumentPropertiesW<P0, P1, P2>(hwnd: P0, hprinter: P1, pdevicename: P2, pdevmodeoutput: ::core::option::Option<*mut super::Gdi::DEVMODEW>, pdevmodeinput: ::core::option::Option<*const super::Gdi::DEVMODEW>) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn AdvancedDocumentPropertiesW ( hwnd : super::super::Foundation:: HWND , hprinter : super::super::Foundation:: HANDLE , pdevicename : :: windows::core::PCWSTR , pdevmodeoutput : *mut super::Gdi:: DEVMODEW , pdevmodeinput : *const super::Gdi:: DEVMODEW ) -> i32 );
    AdvancedDocumentPropertiesW(hwnd.into(), hprinter.into(), pdevicename.into().abi(), ::core::mem::transmute(pdevmodeoutput.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdevmodeinput.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppendPrinterNotifyInfoData(pinfodest: *const PRINTER_NOTIFY_INFO, pdatasrc: ::core::option::Option<*const PRINTER_NOTIFY_INFO_DATA>, fdwflags: u32) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "spoolss.dll""system" fn AppendPrinterNotifyInfoData ( pinfodest : *const PRINTER_NOTIFY_INFO , pdatasrc : *const PRINTER_NOTIFY_INFO_DATA , fdwflags : u32 ) -> super::super::Foundation:: BOOL );
    AppendPrinterNotifyInfoData(pinfodest, ::core::mem::transmute(pdatasrc.unwrap_or(::std::ptr::null())), fdwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallRouterFindFirstPrinterChangeNotification<P0, P1>(hprinterrpc: P0, fdwfilterflags: u32, fdwoptions: u32, hnotify: P1, pprinternotifyoptions: *const PRINTER_NOTIFY_OPTIONS) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn CallRouterFindFirstPrinterChangeNotification ( hprinterrpc : super::super::Foundation:: HANDLE , fdwfilterflags : u32 , fdwoptions : u32 , hnotify : super::super::Foundation:: HANDLE , pprinternotifyoptions : *const PRINTER_NOTIFY_OPTIONS ) -> u32 );
    CallRouterFindFirstPrinterChangeNotification(hprinterrpc.into(), fdwfilterflags, fdwoptions, hnotify.into(), pprinternotifyoptions)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClosePrinter<P0>(hprinter: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn ClosePrinter ( hprinter : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    ClosePrinter(hprinter.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseSpoolFileHandle<P0, P1>(hprinter: P0, hspoolfile: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn CloseSpoolFileHandle ( hprinter : super::super::Foundation:: HANDLE , hspoolfile : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CloseSpoolFileHandle(hprinter.into(), hspoolfile.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitSpoolData<P0, P1>(hprinter: P0, hspoolfile: P1, cbcommit: u32) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn CommitSpoolData ( hprinter : super::super::Foundation:: HANDLE , hspoolfile : super::super::Foundation:: HANDLE , cbcommit : u32 ) -> super::super::Foundation:: HANDLE );
    CommitSpoolData(hprinter.into(), hspoolfile.into(), cbcommit)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommonPropertySheetUIA<P0, P1>(hwndowner: P0, pfnpropsheetui: PFNPROPSHEETUI, lparam: P1, presult: ::core::option::Option<*mut u32>) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    ::windows::core::link ! ( "compstui.dll""system" fn CommonPropertySheetUIA ( hwndowner : super::super::Foundation:: HWND , pfnpropsheetui : PFNPROPSHEETUI , lparam : super::super::Foundation:: LPARAM , presult : *mut u32 ) -> i32 );
    CommonPropertySheetUIA(hwndowner.into(), pfnpropsheetui, lparam.into(), ::core::mem::transmute(presult.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommonPropertySheetUIW<P0, P1>(hwndowner: P0, pfnpropsheetui: PFNPROPSHEETUI, lparam: P1, presult: ::core::option::Option<*mut u32>) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    ::windows::core::link ! ( "compstui.dll""system" fn CommonPropertySheetUIW ( hwndowner : super::super::Foundation:: HWND , pfnpropsheetui : PFNPROPSHEETUI , lparam : super::super::Foundation:: LPARAM , presult : *mut u32 ) -> i32 );
    CommonPropertySheetUIW(hwndowner.into(), pfnpropsheetui, lparam.into(), ::core::mem::transmute(presult.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConfigurePortA<P0, P1, P2>(pname: P0, hwnd: P1, pportname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn ConfigurePortA ( pname : :: windows::core::PCSTR , hwnd : super::super::Foundation:: HWND , pportname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    ConfigurePortA(pname.into().abi(), hwnd.into(), pportname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConfigurePortW<P0, P1, P2>(pname: P0, hwnd: P1, pportname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn ConfigurePortW ( pname : :: windows::core::PCWSTR , hwnd : super::super::Foundation:: HWND , pportname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    ConfigurePortW(pname.into().abi(), hwnd.into(), pportname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConnectToPrinterDlg<P0>(hwnd: P0, flags: u32) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn ConnectToPrinterDlg ( hwnd : super::super::Foundation:: HWND , flags : u32 ) -> super::super::Foundation:: HANDLE );
    ConnectToPrinterDlg(hwnd.into(), flags)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CorePrinterDriverInstalledA<P0, P1>(pszserver: P0, pszenvironment: P1, coredriverguid: ::windows::core::GUID, ftdriverdate: super::super::Foundation::FILETIME, dwldriverversion: u64) -> ::windows::core::Result<super::super::Foundation::BOOL>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn CorePrinterDriverInstalledA ( pszserver : :: windows::core::PCSTR , pszenvironment : :: windows::core::PCSTR , coredriverguid : :: windows::core::GUID , ftdriverdate : super::super::Foundation:: FILETIME , dwldriverversion : u64 , pbdriverinstalled : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CorePrinterDriverInstalledA(pszserver.into().abi(), pszenvironment.into().abi(), ::core::mem::transmute(coredriverguid), ::core::mem::transmute(ftdriverdate), dwldriverversion, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CorePrinterDriverInstalledW<P0, P1>(pszserver: P0, pszenvironment: P1, coredriverguid: ::windows::core::GUID, ftdriverdate: super::super::Foundation::FILETIME, dwldriverversion: u64) -> ::windows::core::Result<super::super::Foundation::BOOL>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn CorePrinterDriverInstalledW ( pszserver : :: windows::core::PCWSTR , pszenvironment : :: windows::core::PCWSTR , coredriverguid : :: windows::core::GUID , ftdriverdate : super::super::Foundation:: FILETIME , dwldriverversion : u64 , pbdriverinstalled : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CorePrinterDriverInstalledW(pszserver.into().abi(), pszenvironment.into().abi(), ::core::mem::transmute(coredriverguid), ::core::mem::transmute(ftdriverdate), dwldriverversion, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn CreatePrintAsyncNotifyChannel<P0, P1>(pszname: P0, pnotificationtype: *const ::windows::core::GUID, euserfilter: PrintAsyncNotifyUserFilter, econversationstyle: PrintAsyncNotifyConversationStyle, pcallback: P1) -> ::windows::core::Result<IPrintAsyncNotifyChannel>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyCallback>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn CreatePrintAsyncNotifyChannel ( pszname : :: windows::core::PCWSTR , pnotificationtype : *const :: windows::core::GUID , euserfilter : PrintAsyncNotifyUserFilter , econversationstyle : PrintAsyncNotifyConversationStyle , pcallback : * mut::core::ffi::c_void , ppiasynchnotification : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreatePrintAsyncNotifyChannel(pszname.into().abi(), pnotificationtype, euserfilter, econversationstyle, pcallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CreatePrinterIC<P0>(hprinter: P0, pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEW>) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn CreatePrinterIC ( hprinter : super::super::Foundation:: HANDLE , pdevmode : *const super::Gdi:: DEVMODEW ) -> super::super::Foundation:: HANDLE );
    CreatePrinterIC(hprinter.into(), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFormA<P0, P1>(hprinter: P0, pformname: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeleteFormA ( hprinter : super::super::Foundation:: HANDLE , pformname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeleteFormA(hprinter.into(), pformname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFormW<P0, P1>(hprinter: P0, pformname: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeleteFormW ( hprinter : super::super::Foundation:: HANDLE , pformname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeleteFormW(hprinter.into(), pformname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteJobNamedProperty<P0, P1>(hprinter: P0, jobid: u32, pszname: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeleteJobNamedProperty ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 , pszname : :: windows::core::PCWSTR ) -> u32 );
    DeleteJobNamedProperty(hprinter.into(), jobid, pszname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteMonitorA<P0, P1, P2>(pname: P0, penvironment: P1, pmonitorname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeleteMonitorA ( pname : :: windows::core::PCSTR , penvironment : :: windows::core::PCSTR , pmonitorname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeleteMonitorA(pname.into().abi(), penvironment.into().abi(), pmonitorname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteMonitorW<P0, P1, P2>(pname: P0, penvironment: P1, pmonitorname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeleteMonitorW ( pname : :: windows::core::PCWSTR , penvironment : :: windows::core::PCWSTR , pmonitorname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeleteMonitorW(pname.into().abi(), penvironment.into().abi(), pmonitorname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePortA<P0, P1, P2>(pname: P0, hwnd: P1, pportname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePortA ( pname : :: windows::core::PCSTR , hwnd : super::super::Foundation:: HWND , pportname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeletePortA(pname.into().abi(), hwnd.into(), pportname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePortW<P0, P1, P2>(pname: P0, hwnd: P1, pportname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePortW ( pname : :: windows::core::PCWSTR , hwnd : super::super::Foundation:: HWND , pportname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeletePortW(pname.into().abi(), hwnd.into(), pportname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrintProcessorA<P0, P1, P2>(pname: P0, penvironment: P1, pprintprocessorname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrintProcessorA ( pname : :: windows::core::PCSTR , penvironment : :: windows::core::PCSTR , pprintprocessorname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeletePrintProcessorA(pname.into().abi(), penvironment.into().abi(), pprintprocessorname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrintProcessorW<P0, P1, P2>(pname: P0, penvironment: P1, pprintprocessorname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrintProcessorW ( pname : :: windows::core::PCWSTR , penvironment : :: windows::core::PCWSTR , pprintprocessorname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeletePrintProcessorW(pname.into().abi(), penvironment.into().abi(), pprintprocessorname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrintProvidorA<P0, P1, P2>(pname: P0, penvironment: P1, pprintprovidorname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrintProvidorA ( pname : :: windows::core::PCSTR , penvironment : :: windows::core::PCSTR , pprintprovidorname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeletePrintProvidorA(pname.into().abi(), penvironment.into().abi(), pprintprovidorname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrintProvidorW<P0, P1, P2>(pname: P0, penvironment: P1, pprintprovidorname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrintProvidorW ( pname : :: windows::core::PCWSTR , penvironment : :: windows::core::PCWSTR , pprintprovidorname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeletePrintProvidorW(pname.into().abi(), penvironment.into().abi(), pprintprovidorname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinter<P0>(hprinter: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinter ( hprinter : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    DeletePrinter(hprinter.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterConnectionA<P0>(pname: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterConnectionA ( pname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeletePrinterConnectionA(pname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterConnectionW<P0>(pname: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterConnectionW ( pname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeletePrinterConnectionW(pname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterDataA<P0, P1>(hprinter: P0, pvaluename: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterDataA ( hprinter : super::super::Foundation:: HANDLE , pvaluename : :: windows::core::PCSTR ) -> u32 );
    DeletePrinterDataA(hprinter.into(), pvaluename.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterDataExA<P0, P1, P2>(hprinter: P0, pkeyname: P1, pvaluename: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterDataExA ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCSTR , pvaluename : :: windows::core::PCSTR ) -> u32 );
    DeletePrinterDataExA(hprinter.into(), pkeyname.into().abi(), pvaluename.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterDataExW<P0, P1, P2>(hprinter: P0, pkeyname: P1, pvaluename: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterDataExW ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCWSTR , pvaluename : :: windows::core::PCWSTR ) -> u32 );
    DeletePrinterDataExW(hprinter.into(), pkeyname.into().abi(), pvaluename.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterDataW<P0, P1>(hprinter: P0, pvaluename: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterDataW ( hprinter : super::super::Foundation:: HANDLE , pvaluename : :: windows::core::PCWSTR ) -> u32 );
    DeletePrinterDataW(hprinter.into(), pvaluename.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterDriverA<P0, P1, P2>(pname: P0, penvironment: P1, pdrivername: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterDriverA ( pname : :: windows::core::PCSTR , penvironment : :: windows::core::PCSTR , pdrivername : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeletePrinterDriverA(pname.into().abi(), penvironment.into().abi(), pdrivername.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterDriverExA<P0, P1, P2>(pname: P0, penvironment: P1, pdrivername: P2, dwdeleteflag: u32, dwversionflag: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterDriverExA ( pname : :: windows::core::PCSTR , penvironment : :: windows::core::PCSTR , pdrivername : :: windows::core::PCSTR , dwdeleteflag : u32 , dwversionflag : u32 ) -> super::super::Foundation:: BOOL );
    DeletePrinterDriverExA(pname.into().abi(), penvironment.into().abi(), pdrivername.into().abi(), dwdeleteflag, dwversionflag)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterDriverExW<P0, P1, P2>(pname: P0, penvironment: P1, pdrivername: P2, dwdeleteflag: u32, dwversionflag: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterDriverExW ( pname : :: windows::core::PCWSTR , penvironment : :: windows::core::PCWSTR , pdrivername : :: windows::core::PCWSTR , dwdeleteflag : u32 , dwversionflag : u32 ) -> super::super::Foundation:: BOOL );
    DeletePrinterDriverExW(pname.into().abi(), penvironment.into().abi(), pdrivername.into().abi(), dwdeleteflag, dwversionflag)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn DeletePrinterDriverPackageA<P0, P1, P2>(pszserver: P0, pszinfpath: P1, pszenvironment: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterDriverPackageA ( pszserver : :: windows::core::PCSTR , pszinfpath : :: windows::core::PCSTR , pszenvironment : :: windows::core::PCSTR ) -> :: windows::core::HRESULT );
    DeletePrinterDriverPackageA(pszserver.into().abi(), pszinfpath.into().abi(), pszenvironment.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn DeletePrinterDriverPackageW<P0, P1, P2>(pszserver: P0, pszinfpath: P1, pszenvironment: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterDriverPackageW ( pszserver : :: windows::core::PCWSTR , pszinfpath : :: windows::core::PCWSTR , pszenvironment : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    DeletePrinterDriverPackageW(pszserver.into().abi(), pszinfpath.into().abi(), pszenvironment.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterDriverW<P0, P1, P2>(pname: P0, penvironment: P1, pdrivername: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterDriverW ( pname : :: windows::core::PCWSTR , penvironment : :: windows::core::PCWSTR , pdrivername : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeletePrinterDriverW(pname.into().abi(), penvironment.into().abi(), pdrivername.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterIC<P0>(hprinteric: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterIC ( hprinteric : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    DeletePrinterIC(hprinteric.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterKeyA<P0, P1>(hprinter: P0, pkeyname: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterKeyA ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCSTR ) -> u32 );
    DeletePrinterKeyA(hprinter.into(), pkeyname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePrinterKeyW<P0, P1>(hprinter: P0, pkeyname: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeletePrinterKeyW ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCWSTR ) -> u32 );
    DeletePrinterKeyW(hprinter.into(), pkeyname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DevQueryPrint<P0>(hprinter: P0, pdevmode: *const super::Gdi::DEVMODEA, presid: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DevQueryPrint ( hprinter : super::super::Foundation:: HANDLE , pdevmode : *const super::Gdi:: DEVMODEA , presid : *mut u32 ) -> super::super::Foundation:: BOOL );
    DevQueryPrint(hprinter.into(), pdevmode, presid)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DevQueryPrintEx(pdqpinfo: *mut DEVQUERYPRINT_INFO) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "winspool.drv""system" fn DevQueryPrintEx ( pdqpinfo : *mut DEVQUERYPRINT_INFO ) -> super::super::Foundation:: BOOL );
    DevQueryPrintEx(pdqpinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DocumentPropertiesA<P0, P1, P2>(hwnd: P0, hprinter: P1, pdevicename: P2, pdevmodeoutput: ::core::option::Option<*mut super::Gdi::DEVMODEA>, pdevmodeinput: ::core::option::Option<*const super::Gdi::DEVMODEA>, fmode: u32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DocumentPropertiesA ( hwnd : super::super::Foundation:: HWND , hprinter : super::super::Foundation:: HANDLE , pdevicename : :: windows::core::PCSTR , pdevmodeoutput : *mut super::Gdi:: DEVMODEA , pdevmodeinput : *const super::Gdi:: DEVMODEA , fmode : u32 ) -> i32 );
    DocumentPropertiesA(hwnd.into(), hprinter.into(), pdevicename.into().abi(), ::core::mem::transmute(pdevmodeoutput.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdevmodeinput.unwrap_or(::std::ptr::null())), fmode)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DocumentPropertiesW<P0, P1, P2>(hwnd: P0, hprinter: P1, pdevicename: P2, pdevmodeoutput: ::core::option::Option<*mut super::Gdi::DEVMODEW>, pdevmodeinput: ::core::option::Option<*const super::Gdi::DEVMODEW>, fmode: u32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DocumentPropertiesW ( hwnd : super::super::Foundation:: HWND , hprinter : super::super::Foundation:: HANDLE , pdevicename : :: windows::core::PCWSTR , pdevmodeoutput : *mut super::Gdi:: DEVMODEW , pdevmodeinput : *const super::Gdi:: DEVMODEW , fmode : u32 ) -> i32 );
    DocumentPropertiesW(hwnd.into(), hprinter.into(), pdevicename.into().abi(), ::core::mem::transmute(pdevmodeoutput.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdevmodeinput.unwrap_or(::std::ptr::null())), fmode)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndDocPrinter<P0>(hprinter: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EndDocPrinter ( hprinter : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    EndDocPrinter(hprinter.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndPagePrinter<P0>(hprinter: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EndPagePrinter ( hprinter : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    EndPagePrinter(hprinter.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumFormsA<P0>(hprinter: P0, level: u32, pform: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumFormsA ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pform : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumFormsA(hprinter.into(), level, ::core::mem::transmute(pform.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pform.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumFormsW<P0>(hprinter: P0, level: u32, pform: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumFormsW ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pform : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumFormsW(hprinter.into(), level, ::core::mem::transmute(pform.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pform.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumJobNamedProperties<P0>(hprinter: P0, jobid: u32, pcproperties: *mut u32, ppproperties: *mut *mut PrintNamedProperty) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumJobNamedProperties ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 , pcproperties : *mut u32 , ppproperties : *mut *mut PrintNamedProperty ) -> u32 );
    EnumJobNamedProperties(hprinter.into(), jobid, pcproperties, ppproperties)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumJobsA<P0>(hprinter: P0, firstjob: u32, nojobs: u32, level: u32, pjob: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumJobsA ( hprinter : super::super::Foundation:: HANDLE , firstjob : u32 , nojobs : u32 , level : u32 , pjob : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumJobsA(hprinter.into(), firstjob, nojobs, level, ::core::mem::transmute(pjob.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pjob.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumJobsW<P0>(hprinter: P0, firstjob: u32, nojobs: u32, level: u32, pjob: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumJobsW ( hprinter : super::super::Foundation:: HANDLE , firstjob : u32 , nojobs : u32 , level : u32 , pjob : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumJobsW(hprinter.into(), firstjob, nojobs, level, ::core::mem::transmute(pjob.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pjob.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumMonitorsA<P0>(pname: P0, level: u32, pmonitor: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumMonitorsA ( pname : :: windows::core::PCSTR , level : u32 , pmonitor : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumMonitorsA(pname.into().abi(), level, ::core::mem::transmute(pmonitor.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pmonitor.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumMonitorsW<P0>(pname: P0, level: u32, pmonitor: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumMonitorsW ( pname : :: windows::core::PCWSTR , level : u32 , pmonitor : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumMonitorsW(pname.into().abi(), level, ::core::mem::transmute(pmonitor.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pmonitor.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPortsA<P0>(pname: P0, level: u32, pport: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPortsA ( pname : :: windows::core::PCSTR , level : u32 , pport : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumPortsA(pname.into().abi(), level, ::core::mem::transmute(pport.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pport.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPortsW<P0>(pname: P0, level: u32, pport: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPortsW ( pname : :: windows::core::PCWSTR , level : u32 , pport : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumPortsW(pname.into().abi(), level, ::core::mem::transmute(pport.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pport.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrintProcessorDatatypesA<P0, P1>(pname: P0, pprintprocessorname: P1, level: u32, pdatatypes: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrintProcessorDatatypesA ( pname : :: windows::core::PCSTR , pprintprocessorname : :: windows::core::PCSTR , level : u32 , pdatatypes : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumPrintProcessorDatatypesA(pname.into().abi(), pprintprocessorname.into().abi(), level, ::core::mem::transmute(pdatatypes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdatatypes.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrintProcessorDatatypesW<P0, P1>(pname: P0, pprintprocessorname: P1, level: u32, pdatatypes: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrintProcessorDatatypesW ( pname : :: windows::core::PCWSTR , pprintprocessorname : :: windows::core::PCWSTR , level : u32 , pdatatypes : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumPrintProcessorDatatypesW(pname.into().abi(), pprintprocessorname.into().abi(), level, ::core::mem::transmute(pdatatypes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdatatypes.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrintProcessorsA<P0, P1>(pname: P0, penvironment: P1, level: u32, pprintprocessorinfo: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrintProcessorsA ( pname : :: windows::core::PCSTR , penvironment : :: windows::core::PCSTR , level : u32 , pprintprocessorinfo : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumPrintProcessorsA(pname.into().abi(), penvironment.into().abi(), level, ::core::mem::transmute(pprintprocessorinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pprintprocessorinfo.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrintProcessorsW<P0, P1>(pname: P0, penvironment: P1, level: u32, pprintprocessorinfo: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrintProcessorsW ( pname : :: windows::core::PCWSTR , penvironment : :: windows::core::PCWSTR , level : u32 , pprintprocessorinfo : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumPrintProcessorsW(pname.into().abi(), penvironment.into().abi(), level, ::core::mem::transmute(pprintprocessorinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pprintprocessorinfo.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrinterDataA<P0>(hprinter: P0, dwindex: u32, pvaluename: &mut [u8], pcbvaluename: *mut u32, ptype: ::core::option::Option<*mut u32>, pdata: ::core::option::Option<&mut [u8]>, pcbdata: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrinterDataA ( hprinter : super::super::Foundation:: HANDLE , dwindex : u32 , pvaluename : :: windows::core::PSTR , cbvaluename : u32 , pcbvaluename : *mut u32 , ptype : *mut u32 , pdata : *mut u8 , cbdata : u32 , pcbdata : *mut u32 ) -> u32 );
    EnumPrinterDataA(hprinter.into(), dwindex, ::core::mem::transmute(pvaluename.as_ptr()), pvaluename.len() as _, pcbvaluename, ::core::mem::transmute(ptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrinterDataExA<P0, P1>(hprinter: P0, pkeyname: P1, penumvalues: ::core::option::Option<&mut [u8]>, pcbenumvalues: *mut u32, pnenumvalues: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrinterDataExA ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCSTR , penumvalues : *mut u8 , cbenumvalues : u32 , pcbenumvalues : *mut u32 , pnenumvalues : *mut u32 ) -> u32 );
    EnumPrinterDataExA(hprinter.into(), pkeyname.into().abi(), ::core::mem::transmute(penumvalues.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), penumvalues.as_deref().map_or(0, |slice| slice.len() as _), pcbenumvalues, pnenumvalues)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrinterDataExW<P0, P1>(hprinter: P0, pkeyname: P1, penumvalues: ::core::option::Option<&mut [u8]>, pcbenumvalues: *mut u32, pnenumvalues: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrinterDataExW ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCWSTR , penumvalues : *mut u8 , cbenumvalues : u32 , pcbenumvalues : *mut u32 , pnenumvalues : *mut u32 ) -> u32 );
    EnumPrinterDataExW(hprinter.into(), pkeyname.into().abi(), ::core::mem::transmute(penumvalues.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), penumvalues.as_deref().map_or(0, |slice| slice.len() as _), pcbenumvalues, pnenumvalues)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrinterDataW<P0>(hprinter: P0, dwindex: u32, pvaluename: ::windows::core::PWSTR, cbvaluename: u32, pcbvaluename: *mut u32, ptype: ::core::option::Option<*mut u32>, pdata: ::core::option::Option<&mut [u8]>, pcbdata: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrinterDataW ( hprinter : super::super::Foundation:: HANDLE , dwindex : u32 , pvaluename : :: windows::core::PWSTR , cbvaluename : u32 , pcbvaluename : *mut u32 , ptype : *mut u32 , pdata : *mut u8 , cbdata : u32 , pcbdata : *mut u32 ) -> u32 );
    EnumPrinterDataW(hprinter.into(), dwindex, ::core::mem::transmute(pvaluename), cbvaluename, pcbvaluename, ::core::mem::transmute(ptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrinterDriversA<P0, P1>(pname: P0, penvironment: P1, level: u32, pdriverinfo: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrinterDriversA ( pname : :: windows::core::PCSTR , penvironment : :: windows::core::PCSTR , level : u32 , pdriverinfo : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumPrinterDriversA(pname.into().abi(), penvironment.into().abi(), level, ::core::mem::transmute(pdriverinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrinterDriversW<P0, P1>(pname: P0, penvironment: P1, level: u32, pdriverinfo: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrinterDriversW ( pname : :: windows::core::PCWSTR , penvironment : :: windows::core::PCWSTR , level : u32 , pdriverinfo : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumPrinterDriversW(pname.into().abi(), penvironment.into().abi(), level, ::core::mem::transmute(pdriverinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrinterKeyA<P0, P1>(hprinter: P0, pkeyname: P1, psubkey: ::core::option::Option<&mut [u8]>, pcbsubkey: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrinterKeyA ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCSTR , psubkey : :: windows::core::PSTR , cbsubkey : u32 , pcbsubkey : *mut u32 ) -> u32 );
    EnumPrinterKeyA(hprinter.into(), pkeyname.into().abi(), ::core::mem::transmute(psubkey.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), psubkey.as_deref().map_or(0, |slice| slice.len() as _), pcbsubkey)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrinterKeyW<P0, P1>(hprinter: P0, pkeyname: P1, psubkey: ::windows::core::PWSTR, cbsubkey: u32, pcbsubkey: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrinterKeyW ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCWSTR , psubkey : :: windows::core::PWSTR , cbsubkey : u32 , pcbsubkey : *mut u32 ) -> u32 );
    EnumPrinterKeyW(hprinter.into(), pkeyname.into().abi(), ::core::mem::transmute(psubkey), cbsubkey, pcbsubkey)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrintersA<P0>(flags: u32, name: P0, level: u32, pprinterenum: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrintersA ( flags : u32 , name : :: windows::core::PCSTR , level : u32 , pprinterenum : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumPrintersA(flags, name.into().abi(), level, ::core::mem::transmute(pprinterenum.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pprinterenum.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumPrintersW<P0>(flags: u32, name: P0, level: u32, pprinterenum: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32, pcreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn EnumPrintersW ( flags : u32 , name : :: windows::core::PCWSTR , level : u32 , pprinterenum : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 , pcreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumPrintersW(flags, name.into().abi(), level, ::core::mem::transmute(pprinterenum.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pprinterenum.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded, pcreturned)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ExtDeviceMode<P0, P1, P2, P3, P4>(hwnd: P0, hinst: P1, pdevmodeoutput: ::core::option::Option<*mut super::Gdi::DEVMODEA>, pdevicename: P2, pport: P3, pdevmodeinput: ::core::option::Option<*const super::Gdi::DEVMODEA>, pprofile: P4, fmode: u32) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn ExtDeviceMode ( hwnd : super::super::Foundation:: HWND , hinst : super::super::Foundation:: HANDLE , pdevmodeoutput : *mut super::Gdi:: DEVMODEA , pdevicename : :: windows::core::PCSTR , pport : :: windows::core::PCSTR , pdevmodeinput : *const super::Gdi:: DEVMODEA , pprofile : :: windows::core::PCSTR , fmode : u32 ) -> i32 );
    ExtDeviceMode(hwnd.into(), hinst.into(), ::core::mem::transmute(pdevmodeoutput.unwrap_or(::std::ptr::null_mut())), pdevicename.into().abi(), pport.into().abi(), ::core::mem::transmute(pdevmodeinput.unwrap_or(::std::ptr::null())), pprofile.into().abi(), fmode)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindClosePrinterChangeNotification<P0>(hchange: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn FindClosePrinterChangeNotification ( hchange : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    FindClosePrinterChangeNotification(hchange.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstPrinterChangeNotification<P0>(hprinter: P0, fdwfilter: u32, fdwoptions: u32, pprinternotifyoptions: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn FindFirstPrinterChangeNotification ( hprinter : super::super::Foundation:: HANDLE , fdwfilter : u32 , fdwoptions : u32 , pprinternotifyoptions : *const ::core::ffi::c_void ) -> super::super::Foundation:: HANDLE );
    FindFirstPrinterChangeNotification(hprinter.into(), fdwfilter, fdwoptions, ::core::mem::transmute(pprinternotifyoptions.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextPrinterChangeNotification<P0>(hchange: P0, pdwchange: ::core::option::Option<*mut u32>, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>, ppprinternotifyinfo: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn FindNextPrinterChangeNotification ( hchange : super::super::Foundation:: HANDLE , pdwchange : *mut u32 , pvreserved : *const ::core::ffi::c_void , ppprinternotifyinfo : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FindNextPrinterChangeNotification(hchange.into(), ::core::mem::transmute(pdwchange.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppprinternotifyinfo.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushPrinter<P0>(hprinter: P0, pbuf: ::core::option::Option<*const ::core::ffi::c_void>, cbbuf: u32, pcwritten: *mut u32, csleep: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn FlushPrinter ( hprinter : super::super::Foundation:: HANDLE , pbuf : *const ::core::ffi::c_void , cbbuf : u32 , pcwritten : *mut u32 , csleep : u32 ) -> super::super::Foundation:: BOOL );
    FlushPrinter(hprinter.into(), ::core::mem::transmute(pbuf.unwrap_or(::std::ptr::null())), cbbuf, pcwritten, csleep)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn FreePrintNamedPropertyArray(ppproperties: ::core::option::Option<&mut [*mut PrintNamedProperty]>) {
    ::windows::core::link ! ( "winspool.drv""system" fn FreePrintNamedPropertyArray ( cproperties : u32 , ppproperties : *mut *mut PrintNamedProperty ) -> ( ) );
    FreePrintNamedPropertyArray(ppproperties.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppproperties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn FreePrintPropertyValue(pvalue: *mut PrintPropertyValue) {
    ::windows::core::link ! ( "winspool.drv""system" fn FreePrintPropertyValue ( pvalue : *mut PrintPropertyValue ) -> ( ) );
    FreePrintPropertyValue(pvalue)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreePrinterNotifyInfo(pprinternotifyinfo: *const PRINTER_NOTIFY_INFO) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "winspool.drv""system" fn FreePrinterNotifyInfo ( pprinternotifyinfo : *const PRINTER_NOTIFY_INFO ) -> super::super::Foundation:: BOOL );
    FreePrinterNotifyInfo(pprinternotifyinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiDeleteSpoolFileHandle<P0>(spoolfilehandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiDeleteSpoolFileHandle ( spoolfilehandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    GdiDeleteSpoolFileHandle(spoolfilehandle.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiEndDocEMF<P0>(spoolfilehandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiEndDocEMF ( spoolfilehandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    GdiEndDocEMF(spoolfilehandle.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiEndPageEMF<P0>(spoolfilehandle: P0, dwoptimization: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiEndPageEMF ( spoolfilehandle : super::super::Foundation:: HANDLE , dwoptimization : u32 ) -> super::super::Foundation:: BOOL );
    GdiEndPageEMF(spoolfilehandle.into(), dwoptimization)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GdiGetDC<P0>(spoolfilehandle: P0) -> super::Gdi::HDC
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiGetDC ( spoolfilehandle : super::super::Foundation:: HANDLE ) -> super::Gdi:: HDC );
    GdiGetDC(spoolfilehandle.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GdiGetDevmodeForPage<P0>(spoolfilehandle: P0, dwpagenumber: u32, pcurrdm: *mut *mut super::Gdi::DEVMODEW, plastdm: *mut *mut super::Gdi::DEVMODEW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiGetDevmodeForPage ( spoolfilehandle : super::super::Foundation:: HANDLE , dwpagenumber : u32 , pcurrdm : *mut *mut super::Gdi:: DEVMODEW , plastdm : *mut *mut super::Gdi:: DEVMODEW ) -> super::super::Foundation:: BOOL );
    GdiGetDevmodeForPage(spoolfilehandle.into(), dwpagenumber, pcurrdm, plastdm)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiGetPageCount<P0>(spoolfilehandle: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiGetPageCount ( spoolfilehandle : super::super::Foundation:: HANDLE ) -> u32 );
    GdiGetPageCount(spoolfilehandle.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiGetPageHandle<P0>(spoolfilehandle: P0, page: u32, pdwpagetype: *mut u32) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiGetPageHandle ( spoolfilehandle : super::super::Foundation:: HANDLE , page : u32 , pdwpagetype : *mut u32 ) -> super::super::Foundation:: HANDLE );
    GdiGetPageHandle(spoolfilehandle.into(), page, pdwpagetype)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GdiGetSpoolFileHandle<P0, P1>(pwszprintername: P0, pdevmode: *mut super::Gdi::DEVMODEW, pwszdocname: P1) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiGetSpoolFileHandle ( pwszprintername : :: windows::core::PCWSTR , pdevmode : *mut super::Gdi:: DEVMODEW , pwszdocname : :: windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    GdiGetSpoolFileHandle(pwszprintername.into().abi(), pdevmode, pwszdocname.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiPlayPageEMF<P0, P1>(spoolfilehandle: P0, hemf: P1, prectdocument: *mut super::super::Foundation::RECT, prectborder: *mut super::super::Foundation::RECT, prectclip: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiPlayPageEMF ( spoolfilehandle : super::super::Foundation:: HANDLE , hemf : super::super::Foundation:: HANDLE , prectdocument : *mut super::super::Foundation:: RECT , prectborder : *mut super::super::Foundation:: RECT , prectclip : *mut super::super::Foundation:: RECT ) -> super::super::Foundation:: BOOL );
    GdiPlayPageEMF(spoolfilehandle.into(), hemf.into(), prectdocument, prectborder, prectclip)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GdiResetDCEMF<P0>(spoolfilehandle: P0, pcurrdm: *mut super::Gdi::DEVMODEW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiResetDCEMF ( spoolfilehandle : super::super::Foundation:: HANDLE , pcurrdm : *mut super::Gdi:: DEVMODEW ) -> super::super::Foundation:: BOOL );
    GdiResetDCEMF(spoolfilehandle.into(), pcurrdm)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Xps\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
#[inline]
pub unsafe fn GdiStartDocEMF<P0>(spoolfilehandle: P0, pdocinfo: *mut super::super::Storage::Xps::DOCINFOW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiStartDocEMF ( spoolfilehandle : super::super::Foundation:: HANDLE , pdocinfo : *mut super::super::Storage::Xps:: DOCINFOW ) -> super::super::Foundation:: BOOL );
    GdiStartDocEMF(spoolfilehandle.into(), pdocinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GdiStartPageEMF<P0>(spoolfilehandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn GdiStartPageEMF ( spoolfilehandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    GdiStartPageEMF(spoolfilehandle.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn GenerateCopyFilePaths<P0, P1>(pszprintername: P0, pszdirectory: P1, psplclientinfo: *const u8, dwlevel: u32, pszsourcedir: ::windows::core::PWSTR, pcchsourcedirsize: *mut u32, psztargetdir: ::windows::core::PWSTR, pcchtargetdirsize: *mut u32, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mscms.dll""system" fn GenerateCopyFilePaths ( pszprintername : :: windows::core::PCWSTR , pszdirectory : :: windows::core::PCWSTR , psplclientinfo : *const u8 , dwlevel : u32 , pszsourcedir : :: windows::core::PWSTR , pcchsourcedirsize : *mut u32 , psztargetdir : :: windows::core::PWSTR , pcchtargetdirsize : *mut u32 , dwflags : u32 ) -> u32 );
    GenerateCopyFilePaths(pszprintername.into().abi(), pszdirectory.into().abi(), psplclientinfo, dwlevel, ::core::mem::transmute(pszsourcedir), pcchsourcedirsize, ::core::mem::transmute(psztargetdir), pcchtargetdirsize, dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCPSUIUserData<P0>(hdlg: P0) -> usize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "compstui.dll""system" fn GetCPSUIUserData ( hdlg : super::super::Foundation:: HWND ) -> usize );
    GetCPSUIUserData(hdlg.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCorePrinterDriversA<P0, P1, P2>(pszserver: P0, pszenvironment: P1, pszzcoredriverdependencies: P2, pcoreprinterdrivers: &mut [CORE_PRINTER_DRIVERA]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetCorePrinterDriversA ( pszserver : :: windows::core::PCSTR , pszenvironment : :: windows::core::PCSTR , pszzcoredriverdependencies : :: windows::core::PCSTR , ccoreprinterdrivers : u32 , pcoreprinterdrivers : *mut CORE_PRINTER_DRIVERA ) -> :: windows::core::HRESULT );
    GetCorePrinterDriversA(pszserver.into().abi(), pszenvironment.into().abi(), pszzcoredriverdependencies.into().abi(), pcoreprinterdrivers.len() as _, ::core::mem::transmute(pcoreprinterdrivers.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCorePrinterDriversW<P0, P1, P2>(pszserver: P0, pszenvironment: P1, pszzcoredriverdependencies: P2, pcoreprinterdrivers: &mut [CORE_PRINTER_DRIVERW]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetCorePrinterDriversW ( pszserver : :: windows::core::PCWSTR , pszenvironment : :: windows::core::PCWSTR , pszzcoredriverdependencies : :: windows::core::PCWSTR , ccoreprinterdrivers : u32 , pcoreprinterdrivers : *mut CORE_PRINTER_DRIVERW ) -> :: windows::core::HRESULT );
    GetCorePrinterDriversW(pszserver.into().abi(), pszenvironment.into().abi(), pszzcoredriverdependencies.into().abi(), pcoreprinterdrivers.len() as _, ::core::mem::transmute(pcoreprinterdrivers.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDefaultPrinterA(pszbuffer: ::windows::core::PSTR, pcchbuffer: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "winspool.drv""system" fn GetDefaultPrinterA ( pszbuffer : :: windows::core::PSTR , pcchbuffer : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetDefaultPrinterA(::core::mem::transmute(pszbuffer), pcchbuffer)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDefaultPrinterW(pszbuffer: ::windows::core::PWSTR, pcchbuffer: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "winspool.drv""system" fn GetDefaultPrinterW ( pszbuffer : :: windows::core::PWSTR , pcchbuffer : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetDefaultPrinterW(::core::mem::transmute(pszbuffer), pcchbuffer)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFormA<P0, P1>(hprinter: P0, pformname: P1, level: u32, pform: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetFormA ( hprinter : super::super::Foundation:: HANDLE , pformname : :: windows::core::PCSTR , level : u32 , pform : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetFormA(hprinter.into(), pformname.into().abi(), level, ::core::mem::transmute(pform.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pform.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFormW<P0, P1>(hprinter: P0, pformname: P1, level: u32, pform: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetFormW ( hprinter : super::super::Foundation:: HANDLE , pformname : :: windows::core::PCWSTR , level : u32 , pform : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetFormW(hprinter.into(), pformname.into().abi(), level, ::core::mem::transmute(pform.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pform.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetJobA<P0>(hprinter: P0, jobid: u32, level: u32, pjob: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetJobA ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 , level : u32 , pjob : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetJobA(hprinter.into(), jobid, level, ::core::mem::transmute(pjob.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pjob.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetJobAttributes<P0>(pprintername: P0, pdevmode: *const super::Gdi::DEVMODEW, pattributeinfo: *mut ATTRIBUTE_INFO_3) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn GetJobAttributes ( pprintername : :: windows::core::PCWSTR , pdevmode : *const super::Gdi:: DEVMODEW , pattributeinfo : *mut ATTRIBUTE_INFO_3 ) -> super::super::Foundation:: BOOL );
    GetJobAttributes(pprintername.into().abi(), pdevmode, pattributeinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetJobAttributesEx<P0>(pprintername: P0, pdevmode: *const super::Gdi::DEVMODEW, dwlevel: u32, pattributeinfo: &mut [u8], dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn GetJobAttributesEx ( pprintername : :: windows::core::PCWSTR , pdevmode : *const super::Gdi:: DEVMODEW , dwlevel : u32 , pattributeinfo : *mut u8 , nsize : u32 , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    GetJobAttributesEx(pprintername.into().abi(), pdevmode, dwlevel, ::core::mem::transmute(pattributeinfo.as_ptr()), pattributeinfo.len() as _, dwflags)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetJobNamedPropertyValue<P0, P1>(hprinter: P0, jobid: u32, pszname: P1, pvalue: *mut PrintPropertyValue) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetJobNamedPropertyValue ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 , pszname : :: windows::core::PCWSTR , pvalue : *mut PrintPropertyValue ) -> u32 );
    GetJobNamedPropertyValue(hprinter.into(), jobid, pszname.into().abi(), pvalue)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetJobW<P0>(hprinter: P0, jobid: u32, level: u32, pjob: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetJobW ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 , level : u32 , pjob : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetJobW(hprinter.into(), jobid, level, ::core::mem::transmute(pjob.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pjob.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrintExecutionData(pdata: *mut PRINT_EXECUTION_DATA) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrintExecutionData ( pdata : *mut PRINT_EXECUTION_DATA ) -> super::super::Foundation:: BOOL );
    GetPrintExecutionData(pdata)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrintOutputInfo<P0, P1>(hwnd: P0, pszprinter: P1, phfile: *mut super::super::Foundation::HANDLE, ppszoutputfile: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrintOutputInfo ( hwnd : super::super::Foundation:: HWND , pszprinter : :: windows::core::PCWSTR , phfile : *mut super::super::Foundation:: HANDLE , ppszoutputfile : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    GetPrintOutputInfo(hwnd.into(), pszprinter.into().abi(), phfile, ppszoutputfile).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrintProcessorDirectoryA<P0, P1>(pname: P0, penvironment: P1, level: u32, pprintprocessorinfo: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrintProcessorDirectoryA ( pname : :: windows::core::PCSTR , penvironment : :: windows::core::PCSTR , level : u32 , pprintprocessorinfo : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPrintProcessorDirectoryA(pname.into().abi(), penvironment.into().abi(), level, ::core::mem::transmute(pprintprocessorinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pprintprocessorinfo.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrintProcessorDirectoryW<P0, P1>(pname: P0, penvironment: P1, level: u32, pprintprocessorinfo: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrintProcessorDirectoryW ( pname : :: windows::core::PCWSTR , penvironment : :: windows::core::PCWSTR , level : u32 , pprintprocessorinfo : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPrintProcessorDirectoryW(pname.into().abi(), penvironment.into().abi(), level, ::core::mem::transmute(pprintprocessorinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pprintprocessorinfo.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterA<P0>(hprinter: P0, level: u32, pprinter: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterA ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pprinter : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPrinterA(hprinter.into(), level, ::core::mem::transmute(pprinter.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pprinter.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterDataA<P0, P1>(hprinter: P0, pvaluename: P1, ptype: ::core::option::Option<*mut u32>, pdata: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDataA ( hprinter : super::super::Foundation:: HANDLE , pvaluename : :: windows::core::PCSTR , ptype : *mut u32 , pdata : *mut u8 , nsize : u32 , pcbneeded : *mut u32 ) -> u32 );
    GetPrinterDataA(hprinter.into(), pvaluename.into().abi(), ::core::mem::transmute(ptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterDataExA<P0, P1, P2>(hprinter: P0, pkeyname: P1, pvaluename: P2, ptype: ::core::option::Option<*mut u32>, pdata: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDataExA ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCSTR , pvaluename : :: windows::core::PCSTR , ptype : *mut u32 , pdata : *mut u8 , nsize : u32 , pcbneeded : *mut u32 ) -> u32 );
    GetPrinterDataExA(hprinter.into(), pkeyname.into().abi(), pvaluename.into().abi(), ::core::mem::transmute(ptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterDataExW<P0, P1, P2>(hprinter: P0, pkeyname: P1, pvaluename: P2, ptype: ::core::option::Option<*mut u32>, pdata: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDataExW ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCWSTR , pvaluename : :: windows::core::PCWSTR , ptype : *mut u32 , pdata : *mut u8 , nsize : u32 , pcbneeded : *mut u32 ) -> u32 );
    GetPrinterDataExW(hprinter.into(), pkeyname.into().abi(), pvaluename.into().abi(), ::core::mem::transmute(ptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterDataW<P0, P1>(hprinter: P0, pvaluename: P1, ptype: ::core::option::Option<*mut u32>, pdata: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDataW ( hprinter : super::super::Foundation:: HANDLE , pvaluename : :: windows::core::PCWSTR , ptype : *mut u32 , pdata : *mut u8 , nsize : u32 , pcbneeded : *mut u32 ) -> u32 );
    GetPrinterDataW(hprinter.into(), pvaluename.into().abi(), ::core::mem::transmute(ptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterDriver2A<P0, P1, P2>(hwnd: P0, hprinter: P1, penvironment: P2, level: u32, pdriverinfo: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDriver2A ( hwnd : super::super::Foundation:: HWND , hprinter : super::super::Foundation:: HANDLE , penvironment : :: windows::core::PCSTR , level : u32 , pdriverinfo : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPrinterDriver2A(hwnd.into(), hprinter.into(), penvironment.into().abi(), level, ::core::mem::transmute(pdriverinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterDriver2W<P0, P1, P2>(hwnd: P0, hprinter: P1, penvironment: P2, level: u32, pdriverinfo: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDriver2W ( hwnd : super::super::Foundation:: HWND , hprinter : super::super::Foundation:: HANDLE , penvironment : :: windows::core::PCWSTR , level : u32 , pdriverinfo : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPrinterDriver2W(hwnd.into(), hprinter.into(), penvironment.into().abi(), level, ::core::mem::transmute(pdriverinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterDriverA<P0, P1>(hprinter: P0, penvironment: P1, level: u32, pdriverinfo: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDriverA ( hprinter : super::super::Foundation:: HANDLE , penvironment : :: windows::core::PCSTR , level : u32 , pdriverinfo : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPrinterDriverA(hprinter.into(), penvironment.into().abi(), level, ::core::mem::transmute(pdriverinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterDriverDirectoryA<P0, P1>(pname: P0, penvironment: P1, level: u32, pdriverdirectory: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDriverDirectoryA ( pname : :: windows::core::PCSTR , penvironment : :: windows::core::PCSTR , level : u32 , pdriverdirectory : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPrinterDriverDirectoryA(pname.into().abi(), penvironment.into().abi(), level, ::core::mem::transmute(pdriverdirectory.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriverdirectory.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterDriverDirectoryW<P0, P1>(pname: P0, penvironment: P1, level: u32, pdriverdirectory: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDriverDirectoryW ( pname : :: windows::core::PCWSTR , penvironment : :: windows::core::PCWSTR , level : u32 , pdriverdirectory : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPrinterDriverDirectoryW(pname.into().abi(), penvironment.into().abi(), level, ::core::mem::transmute(pdriverdirectory.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriverdirectory.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn GetPrinterDriverPackagePathA<P0, P1, P2, P3>(pszserver: P0, pszenvironment: P1, pszlanguage: P2, pszpackageid: P3, pszdriverpackagecab: ::core::option::Option<&mut [u8]>, pcchrequiredsize: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDriverPackagePathA ( pszserver : :: windows::core::PCSTR , pszenvironment : :: windows::core::PCSTR , pszlanguage : :: windows::core::PCSTR , pszpackageid : :: windows::core::PCSTR , pszdriverpackagecab : :: windows::core::PSTR , cchdriverpackagecab : u32 , pcchrequiredsize : *mut u32 ) -> :: windows::core::HRESULT );
    GetPrinterDriverPackagePathA(pszserver.into().abi(), pszenvironment.into().abi(), pszlanguage.into().abi(), pszpackageid.into().abi(), ::core::mem::transmute(pszdriverpackagecab.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszdriverpackagecab.as_deref().map_or(0, |slice| slice.len() as _), pcchrequiredsize).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn GetPrinterDriverPackagePathW<P0, P1, P2, P3>(pszserver: P0, pszenvironment: P1, pszlanguage: P2, pszpackageid: P3, pszdriverpackagecab: ::core::option::Option<&mut [u16]>, pcchrequiredsize: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDriverPackagePathW ( pszserver : :: windows::core::PCWSTR , pszenvironment : :: windows::core::PCWSTR , pszlanguage : :: windows::core::PCWSTR , pszpackageid : :: windows::core::PCWSTR , pszdriverpackagecab : :: windows::core::PWSTR , cchdriverpackagecab : u32 , pcchrequiredsize : *mut u32 ) -> :: windows::core::HRESULT );
    GetPrinterDriverPackagePathW(pszserver.into().abi(), pszenvironment.into().abi(), pszlanguage.into().abi(), pszpackageid.into().abi(), ::core::mem::transmute(pszdriverpackagecab.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszdriverpackagecab.as_deref().map_or(0, |slice| slice.len() as _), pcchrequiredsize).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterDriverW<P0, P1>(hprinter: P0, penvironment: P1, level: u32, pdriverinfo: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterDriverW ( hprinter : super::super::Foundation:: HANDLE , penvironment : :: windows::core::PCWSTR , level : u32 , pdriverinfo : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPrinterDriverW(hprinter.into(), penvironment.into().abi(), level, ::core::mem::transmute(pdriverinfo.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriverinfo.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrinterW<P0>(hprinter: P0, level: u32, pprinter: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetPrinterW ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pprinter : *mut u8 , cbbuf : u32 , pcbneeded : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPrinterW(hprinter.into(), level, ::core::mem::transmute(pprinter.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pprinter.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSpoolFileHandle<P0>(hprinter: P0) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn GetSpoolFileHandle ( hprinter : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: HANDLE );
    GetSpoolFileHandle(hprinter.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonatePrinterClient<P0>(htoken: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn ImpersonatePrinterClient ( htoken : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    ImpersonatePrinterClient(htoken.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn InstallPrinterDriverFromPackageA<P0, P1, P2, P3>(pszserver: P0, pszinfpath: P1, pszdrivername: P2, pszenvironment: P3, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn InstallPrinterDriverFromPackageA ( pszserver : :: windows::core::PCSTR , pszinfpath : :: windows::core::PCSTR , pszdrivername : :: windows::core::PCSTR , pszenvironment : :: windows::core::PCSTR , dwflags : u32 ) -> :: windows::core::HRESULT );
    InstallPrinterDriverFromPackageA(pszserver.into().abi(), pszinfpath.into().abi(), pszdrivername.into().abi(), pszenvironment.into().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn InstallPrinterDriverFromPackageW<P0, P1, P2, P3>(pszserver: P0, pszinfpath: P1, pszdrivername: P2, pszenvironment: P3, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn InstallPrinterDriverFromPackageW ( pszserver : :: windows::core::PCWSTR , pszinfpath : :: windows::core::PCWSTR , pszdrivername : :: windows::core::PCWSTR , pszenvironment : :: windows::core::PCWSTR , dwflags : u32 ) -> :: windows::core::HRESULT );
    InstallPrinterDriverFromPackageW(pszserver.into().abi(), pszinfpath.into().abi(), pszdrivername.into().abi(), pszenvironment.into().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn IsValidDevmodeA(pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEA>, devmodesize: usize) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "winspool.drv""system" fn IsValidDevmodeA ( pdevmode : *const super::Gdi:: DEVMODEA , devmodesize : usize ) -> super::super::Foundation:: BOOL );
    IsValidDevmodeA(::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), devmodesize)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn IsValidDevmodeW(pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEW>, devmodesize: usize) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "winspool.drv""system" fn IsValidDevmodeW ( pdevmode : *const super::Gdi:: DEVMODEW , devmodesize : usize ) -> super::super::Foundation:: BOOL );
    IsValidDevmodeW(::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), devmodesize)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OpenPrinter2A<P0>(pprintername: P0, phprinter: *mut super::super::Foundation::HANDLE, pdefault: ::core::option::Option<*const PRINTER_DEFAULTSA>, poptions: ::core::option::Option<*const PRINTER_OPTIONSA>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn OpenPrinter2A ( pprintername : :: windows::core::PCSTR , phprinter : *mut super::super::Foundation:: HANDLE , pdefault : *const PRINTER_DEFAULTSA , poptions : *const PRINTER_OPTIONSA ) -> super::super::Foundation:: BOOL );
    OpenPrinter2A(pprintername.into().abi(), phprinter, ::core::mem::transmute(pdefault.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poptions.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OpenPrinter2W<P0>(pprintername: P0, phprinter: *mut super::super::Foundation::HANDLE, pdefault: ::core::option::Option<*const PRINTER_DEFAULTSW>, poptions: ::core::option::Option<*const PRINTER_OPTIONSW>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn OpenPrinter2W ( pprintername : :: windows::core::PCWSTR , phprinter : *mut super::super::Foundation:: HANDLE , pdefault : *const PRINTER_DEFAULTSW , poptions : *const PRINTER_OPTIONSW ) -> super::super::Foundation:: BOOL );
    OpenPrinter2W(pprintername.into().abi(), phprinter, ::core::mem::transmute(pdefault.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poptions.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OpenPrinterA<P0>(pprintername: P0, phprinter: *mut super::super::Foundation::HANDLE, pdefault: ::core::option::Option<*const PRINTER_DEFAULTSA>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn OpenPrinterA ( pprintername : :: windows::core::PCSTR , phprinter : *mut super::super::Foundation:: HANDLE , pdefault : *const PRINTER_DEFAULTSA ) -> super::super::Foundation:: BOOL );
    OpenPrinterA(pprintername.into().abi(), phprinter, ::core::mem::transmute(pdefault.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OpenPrinterW<P0>(pprintername: P0, phprinter: *mut super::super::Foundation::HANDLE, pdefault: ::core::option::Option<*const PRINTER_DEFAULTSW>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn OpenPrinterW ( pprintername : :: windows::core::PCWSTR , phprinter : *mut super::super::Foundation:: HANDLE , pdefault : *const PRINTER_DEFAULTSW ) -> super::super::Foundation:: BOOL );
    OpenPrinterW(pprintername.into().abi(), phprinter, ::core::mem::transmute(pdefault.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PartialReplyPrinterChangeNotification<P0>(hprinter: P0, pdatasrc: ::core::option::Option<*const PRINTER_NOTIFY_INFO_DATA>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn PartialReplyPrinterChangeNotification ( hprinter : super::super::Foundation:: HANDLE , pdatasrc : *const PRINTER_NOTIFY_INFO_DATA ) -> super::super::Foundation:: BOOL );
    PartialReplyPrinterChangeNotification(hprinter.into(), ::core::mem::transmute(pdatasrc.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlayGdiScriptOnPrinterIC<P0>(hprinteric: P0, pin: &[u8], pout: &mut [u8], ul: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn PlayGdiScriptOnPrinterIC ( hprinteric : super::super::Foundation:: HANDLE , pin : *const u8 , cin : u32 , pout : *mut u8 , cout : u32 , ul : u32 ) -> super::super::Foundation:: BOOL );
    PlayGdiScriptOnPrinterIC(hprinteric.into(), ::core::mem::transmute(pin.as_ptr()), pin.len() as _, ::core::mem::transmute(pout.as_ptr()), pout.len() as _, ul)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrinterMessageBoxA<P0, P1, P2, P3>(hprinter: P0, error: u32, hwnd: P1, ptext: P2, pcaption: P3, dwtype: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn PrinterMessageBoxA ( hprinter : super::super::Foundation:: HANDLE , error : u32 , hwnd : super::super::Foundation:: HWND , ptext : :: windows::core::PCSTR , pcaption : :: windows::core::PCSTR , dwtype : u32 ) -> u32 );
    PrinterMessageBoxA(hprinter.into(), error, hwnd.into(), ptext.into().abi(), pcaption.into().abi(), dwtype)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrinterMessageBoxW<P0, P1, P2, P3>(hprinter: P0, error: u32, hwnd: P1, ptext: P2, pcaption: P3, dwtype: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn PrinterMessageBoxW ( hprinter : super::super::Foundation:: HANDLE , error : u32 , hwnd : super::super::Foundation:: HWND , ptext : :: windows::core::PCWSTR , pcaption : :: windows::core::PCWSTR , dwtype : u32 ) -> u32 );
    PrinterMessageBoxW(hprinter.into(), error, hwnd.into(), ptext.into().abi(), pcaption.into().abi(), dwtype)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrinterProperties<P0, P1>(hwnd: P0, hprinter: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn PrinterProperties ( hwnd : super::super::Foundation:: HWND , hprinter : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    PrinterProperties(hwnd.into(), hprinter.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProvidorFindClosePrinterChangeNotification<P0>(hprinter: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn ProvidorFindClosePrinterChangeNotification ( hprinter : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    ProvidorFindClosePrinterChangeNotification(hprinter.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProvidorFindFirstPrinterChangeNotification<P0, P1>(hprinter: P0, fdwflags: u32, fdwoptions: u32, hnotify: P1, pprinternotifyoptions: ::core::option::Option<*const ::core::ffi::c_void>, pvreserved1: ::core::option::Option<*mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn ProvidorFindFirstPrinterChangeNotification ( hprinter : super::super::Foundation:: HANDLE , fdwflags : u32 , fdwoptions : u32 , hnotify : super::super::Foundation:: HANDLE , pprinternotifyoptions : *const ::core::ffi::c_void , pvreserved1 : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ProvidorFindFirstPrinterChangeNotification(hprinter.into(), fdwflags, fdwoptions, hnotify.into(), ::core::mem::transmute(pprinternotifyoptions.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvreserved1.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadPrinter<P0>(hprinter: P0, pbuf: *mut ::core::ffi::c_void, cbbuf: u32, pnobytesread: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn ReadPrinter ( hprinter : super::super::Foundation:: HANDLE , pbuf : *mut ::core::ffi::c_void , cbbuf : u32 , pnobytesread : *mut u32 ) -> super::super::Foundation:: BOOL );
    ReadPrinter(hprinter.into(), pbuf, cbbuf, pnobytesread)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterForPrintAsyncNotifications<P0, P1>(pszname: P0, pnotificationtype: *const ::windows::core::GUID, euserfilter: PrintAsyncNotifyUserFilter, econversationstyle: PrintAsyncNotifyConversationStyle, pcallback: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyCallback>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn RegisterForPrintAsyncNotifications ( pszname : :: windows::core::PCWSTR , pnotificationtype : *const :: windows::core::GUID , euserfilter : PrintAsyncNotifyUserFilter , econversationstyle : PrintAsyncNotifyConversationStyle , pcallback : * mut::core::ffi::c_void , phnotify : *mut super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    RegisterForPrintAsyncNotifications(pszname.into().abi(), pnotificationtype, euserfilter, econversationstyle, pcallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemovePrintDeviceObject<P0>(hdeviceobject: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn RemovePrintDeviceObject ( hdeviceobject : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    RemovePrintDeviceObject(hdeviceobject.into()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplyPrinterChangeNotification<P0>(hprinter: P0, fdwchangeflags: u32, pdwresult: ::core::option::Option<*mut u32>, pprinternotifyinfo: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn ReplyPrinterChangeNotification ( hprinter : super::super::Foundation:: HANDLE , fdwchangeflags : u32 , pdwresult : *mut u32 , pprinternotifyinfo : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ReplyPrinterChangeNotification(hprinter.into(), fdwchangeflags, ::core::mem::transmute(pdwresult.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pprinternotifyinfo.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplyPrinterChangeNotificationEx<P0>(hnotify: P0, dwcolor: u32, fdwflags: u32, pdwresult: *mut u32, pprinternotifyinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn ReplyPrinterChangeNotificationEx ( hnotify : super::super::Foundation:: HANDLE , dwcolor : u32 , fdwflags : u32 , pdwresult : *mut u32 , pprinternotifyinfo : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ReplyPrinterChangeNotificationEx(hnotify.into(), dwcolor, fdwflags, pdwresult, pprinternotifyinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportJobProcessingProgress<P0>(printerhandle: P0, jobid: u32, joboperation: EPrintXPSJobOperation, jobprogress: EPrintXPSJobProgress) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn ReportJobProcessingProgress ( printerhandle : super::super::Foundation:: HANDLE , jobid : u32 , joboperation : EPrintXPSJobOperation , jobprogress : EPrintXPSJobProgress ) -> :: windows::core::HRESULT );
    ReportJobProcessingProgress(printerhandle.into(), jobid, joboperation, jobprogress).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ResetPrinterA<P0>(hprinter: P0, pdefault: ::core::option::Option<*const PRINTER_DEFAULTSA>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn ResetPrinterA ( hprinter : super::super::Foundation:: HANDLE , pdefault : *const PRINTER_DEFAULTSA ) -> super::super::Foundation:: BOOL );
    ResetPrinterA(hprinter.into(), ::core::mem::transmute(pdefault.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ResetPrinterW<P0>(hprinter: P0, pdefault: ::core::option::Option<*const PRINTER_DEFAULTSW>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn ResetPrinterW ( hprinter : super::super::Foundation:: HANDLE , pdefault : *const PRINTER_DEFAULTSW ) -> super::super::Foundation:: BOOL );
    ResetPrinterW(hprinter.into(), ::core::mem::transmute(pdefault.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RevertToPrinterSelf() -> super::super::Foundation::HANDLE {
    ::windows::core::link ! ( "spoolss.dll""system" fn RevertToPrinterSelf ( ) -> super::super::Foundation:: HANDLE );
    RevertToPrinterSelf()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn RouterAllocBidiMem(numbytes: usize) -> *mut ::core::ffi::c_void {
    ::windows::core::link ! ( "spoolss.dll""system" fn RouterAllocBidiMem ( numbytes : usize ) -> *mut ::core::ffi::c_void );
    RouterAllocBidiMem(numbytes)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterAllocBidiResponseContainer(count: u32) -> *mut BIDI_RESPONSE_CONTAINER {
    ::windows::core::link ! ( "spoolss.dll""system" fn RouterAllocBidiResponseContainer ( count : u32 ) -> *mut BIDI_RESPONSE_CONTAINER );
    RouterAllocBidiResponseContainer(count)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn RouterAllocPrinterNotifyInfo(cprinternotifyinfodata: u32) -> *mut PRINTER_NOTIFY_INFO {
    ::windows::core::link ! ( "spoolss.dll""system" fn RouterAllocPrinterNotifyInfo ( cprinternotifyinfodata : u32 ) -> *mut PRINTER_NOTIFY_INFO );
    RouterAllocPrinterNotifyInfo(cprinternotifyinfodata)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn RouterFreeBidiMem(pmempointer: *const ::core::ffi::c_void) {
    ::windows::core::link ! ( "spoolss.dll""system" fn RouterFreeBidiMem ( pmempointer : *const ::core::ffi::c_void ) -> ( ) );
    RouterFreeBidiMem(pmempointer)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterFreeBidiResponseContainer(pdata: *const BIDI_RESPONSE_CONTAINER) -> u32 {
    ::windows::core::link ! ( "winspool.drv""system" fn RouterFreeBidiResponseContainer ( pdata : *const BIDI_RESPONSE_CONTAINER ) -> u32 );
    RouterFreeBidiResponseContainer(pdata)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterFreePrinterNotifyInfo(pinfo: ::core::option::Option<*const PRINTER_NOTIFY_INFO>) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "spoolss.dll""system" fn RouterFreePrinterNotifyInfo ( pinfo : *const PRINTER_NOTIFY_INFO ) -> super::super::Foundation:: BOOL );
    RouterFreePrinterNotifyInfo(::core::mem::transmute(pinfo.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScheduleJob<P0>(hprinter: P0, jobid: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn ScheduleJob ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 ) -> super::super::Foundation:: BOOL );
    ScheduleJob(hprinter.into(), jobid)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCPSUIUserData<P0>(hdlg: P0, cpsuiuserdata: usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "compstui.dll""system" fn SetCPSUIUserData ( hdlg : super::super::Foundation:: HWND , cpsuiuserdata : usize ) -> super::super::Foundation:: BOOL );
    SetCPSUIUserData(hdlg.into(), cpsuiuserdata)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDefaultPrinterA<P0>(pszprinter: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetDefaultPrinterA ( pszprinter : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetDefaultPrinterA(pszprinter.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDefaultPrinterW<P0>(pszprinter: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetDefaultPrinterW ( pszprinter : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetDefaultPrinterW(pszprinter.into().abi())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFormA<P0, P1>(hprinter: P0, pformname: P1, level: u32, pform: *const u8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetFormA ( hprinter : super::super::Foundation:: HANDLE , pformname : :: windows::core::PCSTR , level : u32 , pform : *const u8 ) -> super::super::Foundation:: BOOL );
    SetFormA(hprinter.into(), pformname.into().abi(), level, pform)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFormW<P0, P1>(hprinter: P0, pformname: P1, level: u32, pform: *const u8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetFormW ( hprinter : super::super::Foundation:: HANDLE , pformname : :: windows::core::PCWSTR , level : u32 , pform : *const u8 ) -> super::super::Foundation:: BOOL );
    SetFormW(hprinter.into(), pformname.into().abi(), level, pform)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetJobA<P0>(hprinter: P0, jobid: u32, level: u32, pjob: ::core::option::Option<*const u8>, command: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetJobA ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 , level : u32 , pjob : *const u8 , command : u32 ) -> super::super::Foundation:: BOOL );
    SetJobA(hprinter.into(), jobid, level, ::core::mem::transmute(pjob.unwrap_or(::std::ptr::null())), command)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetJobNamedProperty<P0>(hprinter: P0, jobid: u32, pproperty: *const PrintNamedProperty) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetJobNamedProperty ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 , pproperty : *const PrintNamedProperty ) -> u32 );
    SetJobNamedProperty(hprinter.into(), jobid, pproperty)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetJobW<P0>(hprinter: P0, jobid: u32, level: u32, pjob: ::core::option::Option<*const u8>, command: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetJobW ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 , level : u32 , pjob : *const u8 , command : u32 ) -> super::super::Foundation:: BOOL );
    SetJobW(hprinter.into(), jobid, level, ::core::mem::transmute(pjob.unwrap_or(::std::ptr::null())), command)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPortA<P0, P1>(pname: P0, pportname: P1, dwlevel: u32, pportinfo: *const u8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetPortA ( pname : :: windows::core::PCSTR , pportname : :: windows::core::PCSTR , dwlevel : u32 , pportinfo : *const u8 ) -> super::super::Foundation:: BOOL );
    SetPortA(pname.into().abi(), pportname.into().abi(), dwlevel, pportinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPortW<P0, P1>(pname: P0, pportname: P1, dwlevel: u32, pportinfo: *const u8) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetPortW ( pname : :: windows::core::PCWSTR , pportname : :: windows::core::PCWSTR , dwlevel : u32 , pportinfo : *const u8 ) -> super::super::Foundation:: BOOL );
    SetPortW(pname.into().abi(), pportname.into().abi(), dwlevel, pportinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPrinterA<P0>(hprinter: P0, level: u32, pprinter: ::core::option::Option<*const u8>, command: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetPrinterA ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pprinter : *const u8 , command : u32 ) -> super::super::Foundation:: BOOL );
    SetPrinterA(hprinter.into(), level, ::core::mem::transmute(pprinter.unwrap_or(::std::ptr::null())), command)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPrinterDataA<P0, P1>(hprinter: P0, pvaluename: P1, r#type: u32, pdata: &[u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetPrinterDataA ( hprinter : super::super::Foundation:: HANDLE , pvaluename : :: windows::core::PCSTR , r#type : u32 , pdata : *const u8 , cbdata : u32 ) -> u32 );
    SetPrinterDataA(hprinter.into(), pvaluename.into().abi(), r#type, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPrinterDataExA<P0, P1, P2>(hprinter: P0, pkeyname: P1, pvaluename: P2, r#type: u32, pdata: &[u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetPrinterDataExA ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCSTR , pvaluename : :: windows::core::PCSTR , r#type : u32 , pdata : *const u8 , cbdata : u32 ) -> u32 );
    SetPrinterDataExA(hprinter.into(), pkeyname.into().abi(), pvaluename.into().abi(), r#type, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPrinterDataExW<P0, P1, P2>(hprinter: P0, pkeyname: P1, pvaluename: P2, r#type: u32, pdata: &[u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetPrinterDataExW ( hprinter : super::super::Foundation:: HANDLE , pkeyname : :: windows::core::PCWSTR , pvaluename : :: windows::core::PCWSTR , r#type : u32 , pdata : *const u8 , cbdata : u32 ) -> u32 );
    SetPrinterDataExW(hprinter.into(), pkeyname.into().abi(), pvaluename.into().abi(), r#type, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPrinterDataW<P0, P1>(hprinter: P0, pvaluename: P1, r#type: u32, pdata: &[u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetPrinterDataW ( hprinter : super::super::Foundation:: HANDLE , pvaluename : :: windows::core::PCWSTR , r#type : u32 , pdata : *const u8 , cbdata : u32 ) -> u32 );
    SetPrinterDataW(hprinter.into(), pvaluename.into().abi(), r#type, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPrinterW<P0>(hprinter: P0, level: u32, pprinter: ::core::option::Option<*const u8>, command: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn SetPrinterW ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pprinter : *const u8 , command : u32 ) -> super::super::Foundation:: BOOL );
    SetPrinterW(hprinter.into(), level, ::core::mem::transmute(pprinter.unwrap_or(::std::ptr::null())), command)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SplIsSessionZero<P0>(hprinter: P0, jobid: u32, pissessionzero: *mut super::super::Foundation::BOOL) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn SplIsSessionZero ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 , pissessionzero : *mut super::super::Foundation:: BOOL ) -> u32 );
    SplIsSessionZero(hprinter.into(), jobid, pissessionzero)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SplPromptUIInUsersSession<P0>(hprinter: P0, jobid: u32, puiparams: *const SHOWUIPARAMS, presponse: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn SplPromptUIInUsersSession ( hprinter : super::super::Foundation:: HANDLE , jobid : u32 , puiparams : *const SHOWUIPARAMS , presponse : *mut u32 ) -> super::super::Foundation:: BOOL );
    SplPromptUIInUsersSession(hprinter.into(), jobid, puiparams, presponse)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SpoolerCopyFileEvent<P0, P1>(pszprintername: P0, pszkey: P1, dwcopyfileevent: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "mscms.dll""system" fn SpoolerCopyFileEvent ( pszprintername : :: windows::core::PCWSTR , pszkey : :: windows::core::PCWSTR , dwcopyfileevent : u32 ) -> super::super::Foundation:: BOOL );
    SpoolerCopyFileEvent(pszprintername.into().abi(), pszkey.into().abi(), dwcopyfileevent)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SpoolerFindClosePrinterChangeNotification<P0>(hprinter: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn SpoolerFindClosePrinterChangeNotification ( hprinter : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    SpoolerFindClosePrinterChangeNotification(hprinter.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SpoolerFindFirstPrinterChangeNotification<P0>(hprinter: P0, fdwfilterflags: u32, fdwoptions: u32, pprinternotifyoptions: *const ::core::ffi::c_void, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>, pnotificationconfig: *const ::core::ffi::c_void, phnotify: ::core::option::Option<*mut super::super::Foundation::HANDLE>, phevent: ::core::option::Option<*mut super::super::Foundation::HANDLE>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn SpoolerFindFirstPrinterChangeNotification ( hprinter : super::super::Foundation:: HANDLE , fdwfilterflags : u32 , fdwoptions : u32 , pprinternotifyoptions : *const ::core::ffi::c_void , pvreserved : *const ::core::ffi::c_void , pnotificationconfig : *const ::core::ffi::c_void , phnotify : *mut super::super::Foundation:: HANDLE , phevent : *mut super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    SpoolerFindFirstPrinterChangeNotification(hprinter.into(), fdwfilterflags, fdwoptions, pprinternotifyoptions, ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())), pnotificationconfig, ::core::mem::transmute(phnotify.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phevent.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SpoolerFindNextPrinterChangeNotification<P0>(hprinter: P0, pfdwchange: *mut u32, pprinternotifyoptions: ::core::option::Option<*const ::core::ffi::c_void>, ppprinternotifyinfo: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn SpoolerFindNextPrinterChangeNotification ( hprinter : super::super::Foundation:: HANDLE , pfdwchange : *mut u32 , pprinternotifyoptions : *const ::core::ffi::c_void , ppprinternotifyinfo : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SpoolerFindNextPrinterChangeNotification(hprinter.into(), pfdwchange, ::core::mem::transmute(pprinternotifyoptions.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppprinternotifyinfo.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[inline]
pub unsafe fn SpoolerFreePrinterNotifyInfo(pinfo: *const PRINTER_NOTIFY_INFO) {
    ::windows::core::link ! ( "spoolss.dll""system" fn SpoolerFreePrinterNotifyInfo ( pinfo : *const PRINTER_NOTIFY_INFO ) -> ( ) );
    SpoolerFreePrinterNotifyInfo(pinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SpoolerRefreshPrinterChangeNotification<P0>(hprinter: P0, dwcolor: u32, poptions: *const PRINTER_NOTIFY_OPTIONS, ppinfo: ::core::option::Option<*mut *mut PRINTER_NOTIFY_INFO>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn SpoolerRefreshPrinterChangeNotification ( hprinter : super::super::Foundation:: HANDLE , dwcolor : u32 , poptions : *const PRINTER_NOTIFY_OPTIONS , ppinfo : *mut *mut PRINTER_NOTIFY_INFO ) -> super::super::Foundation:: BOOL );
    SpoolerRefreshPrinterChangeNotification(hprinter.into(), dwcolor, poptions, ::core::mem::transmute(ppinfo.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartDocPrinterA<P0>(hprinter: P0, level: u32, pdocinfo: *const DOC_INFO_1A) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn StartDocPrinterA ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pdocinfo : *const DOC_INFO_1A ) -> u32 );
    StartDocPrinterA(hprinter.into(), level, pdocinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartDocPrinterW<P0>(hprinter: P0, level: u32, pdocinfo: *const DOC_INFO_1W) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn StartDocPrinterW ( hprinter : super::super::Foundation:: HANDLE , level : u32 , pdocinfo : *const DOC_INFO_1W ) -> u32 );
    StartDocPrinterW(hprinter.into(), level, pdocinfo)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartPagePrinter<P0>(hprinter: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn StartPagePrinter ( hprinter : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    StartPagePrinter(hprinter.into())
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnRegisterForPrintAsyncNotifications<P0>(param0: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn UnRegisterForPrintAsyncNotifications ( param0 : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    UnRegisterForPrintAsyncNotifications(param0.into()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdatePrintDeviceObject<P0, P1>(hprinter: P0, hdeviceobject: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "spoolss.dll""system" fn UpdatePrintDeviceObject ( hprinter : super::super::Foundation:: HANDLE , hdeviceobject : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    UpdatePrintDeviceObject(hprinter.into(), hdeviceobject.into()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UploadPrinterDriverPackageA<P0, P1, P2, P3>(pszserver: P0, pszinfpath: P1, pszenvironment: P2, dwflags: u32, hwnd: P3, pszdestinfpath: ::windows::core::PSTR, pcchdestinfpath: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P3: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn UploadPrinterDriverPackageA ( pszserver : :: windows::core::PCSTR , pszinfpath : :: windows::core::PCSTR , pszenvironment : :: windows::core::PCSTR , dwflags : u32 , hwnd : super::super::Foundation:: HWND , pszdestinfpath : :: windows::core::PSTR , pcchdestinfpath : *mut u32 ) -> :: windows::core::HRESULT );
    UploadPrinterDriverPackageA(pszserver.into().abi(), pszinfpath.into().abi(), pszenvironment.into().abi(), dwflags, hwnd.into(), ::core::mem::transmute(pszdestinfpath), pcchdestinfpath).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UploadPrinterDriverPackageW<P0, P1, P2, P3>(pszserver: P0, pszinfpath: P1, pszenvironment: P2, dwflags: u32, hwnd: P3, pszdestinfpath: ::windows::core::PWSTR, pcchdestinfpath: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn UploadPrinterDriverPackageW ( pszserver : :: windows::core::PCWSTR , pszinfpath : :: windows::core::PCWSTR , pszenvironment : :: windows::core::PCWSTR , dwflags : u32 , hwnd : super::super::Foundation:: HWND , pszdestinfpath : :: windows::core::PWSTR , pcchdestinfpath : *mut u32 ) -> :: windows::core::HRESULT );
    UploadPrinterDriverPackageW(pszserver.into().abi(), pszinfpath.into().abi(), pszenvironment.into().abi(), dwflags, hwnd.into(), ::core::mem::transmute(pszdestinfpath), pcchdestinfpath).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForPrinterChange<P0>(hprinter: P0, flags: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn WaitForPrinterChange ( hprinter : super::super::Foundation:: HANDLE , flags : u32 ) -> u32 );
    WaitForPrinterChange(hprinter.into(), flags)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrinter<P0>(hprinter: P0, pbuf: *const ::core::ffi::c_void, cbbuf: u32, pcwritten: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn WritePrinter ( hprinter : super::super::Foundation:: HANDLE , pbuf : *const ::core::ffi::c_void , cbbuf : u32 , pcwritten : *mut u32 ) -> super::super::Foundation:: BOOL );
    WritePrinter(hprinter.into(), pbuf, cbbuf, pcwritten)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn XcvDataW<P0, P1>(hxcv: P0, pszdataname: P1, pinputdata: ::core::option::Option<&[u8]>, poutputdata: ::core::option::Option<&mut [u8]>, pcboutputneeded: *mut u32, pdwstatus: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn XcvDataW ( hxcv : super::super::Foundation:: HANDLE , pszdataname : :: windows::core::PCWSTR , pinputdata : *const u8 , cbinputdata : u32 , poutputdata : *mut u8 , cboutputdata : u32 , pcboutputneeded : *mut u32 , pdwstatus : *mut u32 ) -> super::super::Foundation:: BOOL );
    XcvDataW(hxcv.into(), pszdataname.into().abi(), ::core::mem::transmute(pinputdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pinputdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(poutputdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), poutputdata.as_deref().map_or(0, |slice| slice.len() as _), pcboutputneeded, ::core::mem::transmute(pdwstatus.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IAsyncGetSendNotificationCookie(::windows::core::IUnknown);
impl IAsyncGetSendNotificationCookie {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FinishAsyncCallWithData<P0, P1>(&self, param0: P0, param1: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyDataObject>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).FinishAsyncCallWithData)(::windows::core::Vtable::as_raw(self), param0.into().abi(), param1.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IAsyncGetSendNotificationCookie, ::windows::core::IUnknown, IPrintAsyncCookie);
impl ::core::clone::Clone for IAsyncGetSendNotificationCookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAsyncGetSendNotificationCookie {
    type Vtable = IAsyncGetSendNotificationCookie_Vtbl;
}
unsafe impl ::windows::core::Interface for IAsyncGetSendNotificationCookie {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncGetSendNotificationCookie_Vtbl {
    pub base__: IPrintAsyncCookie_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub FinishAsyncCallWithData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FinishAsyncCallWithData: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IAsyncGetSrvReferralCookie(::windows::core::IUnknown);
impl IAsyncGetSrvReferralCookie {
    pub unsafe fn FinishAsyncCall(&self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FinishAsyncCall)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    pub unsafe fn CancelAsyncCall(&self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelAsyncCall)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    pub unsafe fn FinishAsyncCallWithData<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).FinishAsyncCallWithData)(::windows::core::Vtable::as_raw(self), param0.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IAsyncGetSrvReferralCookie, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAsyncGetSrvReferralCookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAsyncGetSrvReferralCookie {
    type Vtable = IAsyncGetSrvReferralCookie_Vtbl;
}
unsafe impl ::windows::core::Interface for IAsyncGetSrvReferralCookie {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncGetSrvReferralCookie_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FinishAsyncCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub CancelAsyncCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub FinishAsyncCallWithData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IBidiAsyncNotifyChannel(::windows::core::IUnknown);
impl IBidiAsyncNotifyChannel {
    pub unsafe fn CreateNotificationChannel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CreateNotificationChannel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetPrintName(&self, param0: *const ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPrintName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(param0)).ok()
    }
    pub unsafe fn GetChannelNotificationType(&self, param0: *const ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetChannelNotificationType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(param0)).ok()
    }
    pub unsafe fn AsyncGetNotificationSendResponse<P0, P1>(&self, param0: P0, param1: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyDataObject>>,
        P1: ::std::convert::Into<::windows::core::InParam<IAsyncGetSendNotificationCookie>>,
    {
        (::windows::core::Vtable::vtable(self).AsyncGetNotificationSendResponse)(::windows::core::Vtable::as_raw(self), param0.into().abi(), param1.into().abi()).ok()
    }
    pub unsafe fn AsyncCloseChannel<P0, P1>(&self, param0: P0, param1: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyDataObject>>,
        P1: ::std::convert::Into<::windows::core::InParam<IPrintAsyncCookie>>,
    {
        (::windows::core::Vtable::vtable(self).AsyncCloseChannel)(::windows::core::Vtable::as_raw(self), param0.into().abi(), param1.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IBidiAsyncNotifyChannel, ::windows::core::IUnknown, IPrintAsyncNotifyChannel);
impl ::core::clone::Clone for IBidiAsyncNotifyChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IBidiAsyncNotifyChannel {
    type Vtable = IBidiAsyncNotifyChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for IBidiAsyncNotifyChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x532818f7_921b_4fb2_bff8_2f4fd52ebebf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBidiAsyncNotifyChannel_Vtbl {
    pub base__: IPrintAsyncNotifyChannel_Vtbl,
    pub CreateNotificationChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrintName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetChannelNotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AsyncGetNotificationSendResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AsyncCloseChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IBidiRequest(::windows::core::IUnknown);
impl IBidiRequest {
    pub unsafe fn SetSchema<P0>(&self, pszschema: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSchema)(::windows::core::Vtable::as_raw(self), pszschema.into().abi()).ok()
    }
    pub unsafe fn SetInputData(&self, dwtype: u32, pdata: *const u8, usize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInputData)(::windows::core::Vtable::as_raw(self), dwtype, pdata, usize).ok()
    }
    pub unsafe fn GetResult(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetResult)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOutputData(&self, dwindex: u32, ppszschema: *mut ::windows::core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetOutputData)(::windows::core::Vtable::as_raw(self), dwindex, ppszschema, pdwtype, ppdata, usize).ok()
    }
    pub unsafe fn GetEnumCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEnumCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IBidiRequest, ::windows::core::IUnknown);
impl ::core::clone::Clone for IBidiRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IBidiRequest {
    type Vtable = IBidiRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IBidiRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f348bd7_4b47_4755_8a9d_0f422df3dc89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBidiRequest_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszschema: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetInputData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtype: u32, pdata: *const u8, usize: u32) -> ::windows::core::HRESULT,
    pub GetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetOutputData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppszschema: *mut ::windows::core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> ::windows::core::HRESULT,
    pub GetEnumCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtotal: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IBidiRequestContainer(::windows::core::IUnknown);
impl IBidiRequestContainer {
    pub unsafe fn AddRequest<P0>(&self, prequest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IBidiRequest>>,
    {
        (::windows::core::Vtable::vtable(self).AddRequest)(::windows::core::Vtable::as_raw(self), prequest.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumObject(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEnumObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRequestCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRequestCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IBidiRequestContainer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IBidiRequestContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IBidiRequestContainer {
    type Vtable = IBidiRequestContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for IBidiRequestContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd752f6c0_94a8_4275_a77d_8f1d1a1121ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBidiRequestContainer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEnumObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEnumObject: usize,
    pub GetRequestCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IBidiSpl(::windows::core::IUnknown);
impl IBidiSpl {
    pub unsafe fn BindDevice<P0>(&self, pszdevicename: P0, dwaccess: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).BindDevice)(::windows::core::Vtable::as_raw(self), pszdevicename.into().abi(), dwaccess).ok()
    }
    pub unsafe fn UnbindDevice(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnbindDevice)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SendRecv<P0, P1>(&self, pszaction: P0, prequest: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IBidiRequest>>,
    {
        (::windows::core::Vtable::vtable(self).SendRecv)(::windows::core::Vtable::as_raw(self), pszaction.into().abi(), prequest.into().abi()).ok()
    }
    pub unsafe fn MultiSendRecv<P0, P1>(&self, pszaction: P0, prequestcontainer: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IBidiRequestContainer>>,
    {
        (::windows::core::Vtable::vtable(self).MultiSendRecv)(::windows::core::Vtable::as_raw(self), pszaction.into().abi(), prequestcontainer.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IBidiSpl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IBidiSpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IBidiSpl {
    type Vtable = IBidiSpl_Vtbl;
}
unsafe impl ::windows::core::Interface for IBidiSpl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd580dc0e_de39_4649_baa8_bf0b85a03a97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBidiSpl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BindDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdevicename: ::windows::core::PCWSTR, dwaccess: u32) -> ::windows::core::HRESULT,
    pub UnbindDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendRecv: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaction: ::windows::core::PCWSTR, prequest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MultiSendRecv: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaction: ::windows::core::PCWSTR, prequestcontainer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IBidiSpl2(::windows::core::IUnknown);
impl IBidiSpl2 {
    pub unsafe fn BindDevice<P0>(&self, pszdevicename: P0, dwaccess: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).BindDevice)(::windows::core::Vtable::as_raw(self), pszdevicename.into().abi(), dwaccess).ok()
    }
    pub unsafe fn UnbindDevice(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnbindDevice)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SendRecvXMLString(&self, bstrrequest: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SendRecvXMLString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrequest), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SendRecvXMLStream<P0>(&self, psrequest: P0) -> ::windows::core::Result<super::super::System::Com::IStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SendRecvXMLStream)(::windows::core::Vtable::as_raw(self), psrequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IBidiSpl2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IBidiSpl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IBidiSpl2 {
    type Vtable = IBidiSpl2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBidiSpl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e8f51b8_8273_4906_8e7b_be453ffd2e2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBidiSpl2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BindDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdevicename: ::windows::core::PCWSTR, dwaccess: u32) -> ::windows::core::HRESULT,
    pub UnbindDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendRecvXMLString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrequest: *mut ::core::ffi::c_void, pbstrresponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SendRecvXMLStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrequest: *mut ::core::ffi::c_void, ppsresponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SendRecvXMLStream: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IFixedDocument(::windows::core::IUnknown);
impl IFixedDocument {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPrintTicket(&self) -> ::windows::core::Result<IPartPrintTicket> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrintTicket)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrintTicket<P0>(&self, pprintticket: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPartPrintTicket>>,
    {
        (::windows::core::Vtable::vtable(self).SetPrintTicket)(::windows::core::Vtable::as_raw(self), pprintticket.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IFixedDocument, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFixedDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IFixedDocument {
    type Vtable = IFixedDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for IFixedDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf222ca9f_9968_4db9_81bd_abaebf15f93f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFixedDocument_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprintticket: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IFixedDocumentSequence(::windows::core::IUnknown);
impl IFixedDocumentSequence {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPrintTicket(&self) -> ::windows::core::Result<IPartPrintTicket> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrintTicket)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrintTicket<P0>(&self, pprintticket: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPartPrintTicket>>,
    {
        (::windows::core::Vtable::vtable(self).SetPrintTicket)(::windows::core::Vtable::as_raw(self), pprintticket.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IFixedDocumentSequence, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFixedDocumentSequence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IFixedDocumentSequence {
    type Vtable = IFixedDocumentSequence_Vtbl;
}
unsafe impl ::windows::core::Interface for IFixedDocumentSequence {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8028d181_2c32_4249_8493_1bfb22045574);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFixedDocumentSequence_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprintticket: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IFixedPage(::windows::core::IUnknown);
impl IFixedPage {
    pub unsafe fn GetPrintTicket(&self) -> ::windows::core::Result<IPartPrintTicket> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrintTicket)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPagePart<P0>(&self, uri: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPagePart)(::windows::core::Vtable::as_raw(self), uri.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWriteStream(&self) -> ::windows::core::Result<IPrintWriteStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWriteStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrintTicket<P0>(&self, ppprintticket: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPartPrintTicket>>,
    {
        (::windows::core::Vtable::vtable(self).SetPrintTicket)(::windows::core::Vtable::as_raw(self), ppprintticket.into().abi()).ok()
    }
    pub unsafe fn SetPagePart<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetPagePart)(::windows::core::Vtable::as_raw(self), punk.into().abi()).ok()
    }
    pub unsafe fn DeleteResource<P0>(&self, uri: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteResource)(::windows::core::Vtable::as_raw(self), uri.into().abi()).ok()
    }
    pub unsafe fn GetXpsPartIterator(&self) -> ::windows::core::Result<IXpsPartIterator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetXpsPartIterator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IFixedPage, ::windows::core::IUnknown, IPartBase);
impl ::core::clone::Clone for IFixedPage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IFixedPage {
    type Vtable = IFixedPage_Vtbl;
}
unsafe impl ::windows::core::Interface for IFixedPage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d9f6448_7e95_4cb5_94fb_0180c2883a57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFixedPage_Vtbl {
    pub base__: IPartBase_Vtbl,
    pub GetPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprintticket: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPagePart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::PCWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetWriteStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwritestream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprintticket: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPagePart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetXpsPartIterator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxpspartit: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
#[repr(transparent)]
pub struct IImgCreateErrorInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Ole")]
impl IImgCreateErrorInfo {
    pub unsafe fn AttachToErrorInfo(&self, perrorinfo: *mut ImgErrorInfo) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AttachToErrorInfo)(::windows::core::Vtable::as_raw(self), perrorinfo).ok()
    }
}
#[cfg(feature = "Win32_System_Ole")]
::windows::core::interface_hierarchy!(IImgCreateErrorInfo, ::windows::core::IUnknown, super::super::System::Ole::ICreateErrorInfo);
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for IImgCreateErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Vtable for IImgCreateErrorInfo {
    type Vtable = IImgCreateErrorInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Interface for IImgCreateErrorInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c55a64c_07cd_4fb5_90f7_b753d91f0c9e);
}
#[cfg(feature = "Win32_System_Ole")]
#[repr(C)]
#[doc(hidden)]
pub struct IImgCreateErrorInfo_Vtbl {
    pub base__: super::super::System::Ole::ICreateErrorInfo_Vtbl,
    pub AttachToErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrorinfo: *mut ImgErrorInfo) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IImgErrorInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IImgErrorInfo {
    pub unsafe fn GetDeveloperDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDeveloperDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUserErrorId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUserErrorId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUserParameterCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUserParameterCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUserParameter(&self, cparam: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUserParameter)(::windows::core::Vtable::as_raw(self), cparam, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUserFallback(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUserFallback)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetExceptionId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetExceptionId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DetachErrorInfo(&self, perrorinfo: *mut ImgErrorInfo) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DetachErrorInfo)(::windows::core::Vtable::as_raw(self), perrorinfo).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IImgErrorInfo, ::windows::core::IUnknown, super::super::System::Com::IErrorInfo);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IImgErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IImgErrorInfo {
    type Vtable = IImgErrorInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IImgErrorInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bce4ece_d30e_445a_9423_6829be945ad8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IImgErrorInfo_Vtbl {
    pub base__: super::super::System::Com::IErrorInfo_Vtbl,
    pub GetDeveloperDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUserErrorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrorid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetUserParameterCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcuserparams: *mut u32) -> ::windows::core::HRESULT,
    pub GetUserParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cparam: u32, pbstrparam: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUserFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfallback: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetExceptionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexceptionid: *mut u32) -> ::windows::core::HRESULT,
    pub DetachErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrorinfo: *mut ImgErrorInfo) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IInterFilterCommunicator(::windows::core::IUnknown);
impl IInterFilterCommunicator {
    pub unsafe fn RequestReader(&self, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RequestReader)(::windows::core::Vtable::as_raw(self), ppireader).ok()
    }
    pub unsafe fn RequestWriter(&self, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RequestWriter)(::windows::core::Vtable::as_raw(self), ppiwriter).ok()
    }
}
::windows::core::interface_hierarchy!(IInterFilterCommunicator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IInterFilterCommunicator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IInterFilterCommunicator {
    type Vtable = IInterFilterCommunicator_Vtbl;
}
unsafe impl ::windows::core::Interface for IInterFilterCommunicator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4daf1e69_81fd_462d_940f_8cd3ddf56fca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInterFilterCommunicator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RequestReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPartBase(::windows::core::IUnknown);
impl IPartBase {
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<IPrintReadStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPartCompression(&self) -> ::windows::core::Result<EXpsCompressionOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPartCompression)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPartCompression(&self, compression: EXpsCompressionOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPartCompression)(::windows::core::Vtable::as_raw(self), compression).ok()
    }
}
::windows::core::interface_hierarchy!(IPartBase, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPartBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPartBase {
    type Vtable = IPartBase_Vtbl;
}
unsafe impl ::windows::core::Interface for IPartBase {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36d51e28_369e_43ba_a666_9540c62c3f58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartBase_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPartCompression: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompression: *mut EXpsCompressionOptions) -> ::windows::core::HRESULT,
    pub SetPartCompression: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compression: EXpsCompressionOptions) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPartColorProfile(::windows::core::IUnknown);
impl IPartColorProfile {}
::windows::core::interface_hierarchy!(IPartColorProfile, ::windows::core::IUnknown, IPartBase);
impl ::core::clone::Clone for IPartColorProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPartColorProfile {
    type Vtable = IPartColorProfile_Vtbl;
}
unsafe impl ::windows::core::Interface for IPartColorProfile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63cca95b_7d18_4762_b15e_98658693d24a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartColorProfile_Vtbl {
    pub base__: IPartBase_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPartDiscardControl(::windows::core::IUnknown);
impl IPartDiscardControl {
    pub unsafe fn GetDiscardProperties(&self, urisentinelpage: *mut ::windows::core::BSTR, uriparttodiscard: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDiscardProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(urisentinelpage), ::core::mem::transmute(uriparttodiscard)).ok()
    }
}
::windows::core::interface_hierarchy!(IPartDiscardControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPartDiscardControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPartDiscardControl {
    type Vtable = IPartDiscardControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IPartDiscardControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc350c00_095b_42a5_bf0f_c8780edadb3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartDiscardControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDiscardProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urisentinelpage: *mut *mut ::core::ffi::c_void, uriparttodiscard: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPartFont(::windows::core::IUnknown);
impl IPartFont {
    pub unsafe fn GetFontProperties(&self, pcontenttype: *mut ::windows::core::BSTR, pfontoptions: *mut EXpsFontOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetFontProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pcontenttype), pfontoptions).ok()
    }
    pub unsafe fn SetFontContent<P0>(&self, pcontenttype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetFontContent)(::windows::core::Vtable::as_raw(self), pcontenttype.into().abi()).ok()
    }
    pub unsafe fn SetFontOptions(&self, options: EXpsFontOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFontOptions)(::windows::core::Vtable::as_raw(self), options).ok()
    }
}
::windows::core::interface_hierarchy!(IPartFont, ::windows::core::IUnknown, IPartBase);
impl ::core::clone::Clone for IPartFont {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPartFont {
    type Vtable = IPartFont_Vtbl;
}
unsafe impl ::windows::core::Interface for IPartFont {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe07fe0ab_1124_43d0_a865_e8ffb6a3ea82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartFont_Vtbl {
    pub base__: IPartBase_Vtbl,
    pub GetFontProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontenttype: *mut *mut ::core::ffi::c_void, pfontoptions: *mut EXpsFontOptions) -> ::windows::core::HRESULT,
    pub SetFontContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontenttype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetFontOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: EXpsFontOptions) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPartFont2(::windows::core::IUnknown);
impl IPartFont2 {
    pub unsafe fn GetFontRestriction(&self) -> ::windows::core::Result<EXpsFontRestriction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFontRestriction)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPartFont2, ::windows::core::IUnknown, IPartBase, IPartFont);
impl ::core::clone::Clone for IPartFont2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPartFont2 {
    type Vtable = IPartFont2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPartFont2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x511e025f_d6cb_43be_bf65_63fe88515a39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartFont2_Vtbl {
    pub base__: IPartFont_Vtbl,
    pub GetFontRestriction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prestriction: *mut EXpsFontRestriction) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPartImage(::windows::core::IUnknown);
impl IPartImage {
    pub unsafe fn GetImageProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImageProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetImageContent<P0>(&self, pcontenttype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetImageContent)(::windows::core::Vtable::as_raw(self), pcontenttype.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPartImage, ::windows::core::IUnknown, IPartBase);
impl ::core::clone::Clone for IPartImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPartImage {
    type Vtable = IPartImage_Vtbl;
}
unsafe impl ::windows::core::Interface for IPartImage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x725f2e3c_401a_4705_9de0_fe6f1353b87f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartImage_Vtbl {
    pub base__: IPartBase_Vtbl,
    pub GetImageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontenttype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetImageContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontenttype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPartPrintTicket(::windows::core::IUnknown);
impl IPartPrintTicket {}
::windows::core::interface_hierarchy!(IPartPrintTicket, ::windows::core::IUnknown, IPartBase);
impl ::core::clone::Clone for IPartPrintTicket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPartPrintTicket {
    type Vtable = IPartPrintTicket_Vtbl;
}
unsafe impl ::windows::core::Interface for IPartPrintTicket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a0f50f6_f9a2_41f0_99e7_5ae955be8e9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartPrintTicket_Vtbl {
    pub base__: IPartBase_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPartResourceDictionary(::windows::core::IUnknown);
impl IPartResourceDictionary {}
::windows::core::interface_hierarchy!(IPartResourceDictionary, ::windows::core::IUnknown, IPartBase);
impl ::core::clone::Clone for IPartResourceDictionary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPartResourceDictionary {
    type Vtable = IPartResourceDictionary_Vtbl;
}
unsafe impl ::windows::core::Interface for IPartResourceDictionary {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16cfce6d_e744_4fb3_b474_f1d54f024a01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartResourceDictionary_Vtbl {
    pub base__: IPartBase_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPartThumbnail(::windows::core::IUnknown);
impl IPartThumbnail {
    pub unsafe fn GetThumbnailProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetThumbnailProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetThumbnailContent<P0>(&self, pcontenttype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetThumbnailContent)(::windows::core::Vtable::as_raw(self), pcontenttype.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPartThumbnail, ::windows::core::IUnknown, IPartBase);
impl ::core::clone::Clone for IPartThumbnail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPartThumbnail {
    type Vtable = IPartThumbnail_Vtbl;
}
unsafe impl ::windows::core::Interface for IPartThumbnail {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x027ed1c9_ba39_4cc5_aa55_7ec3a0de171a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartThumbnail_Vtbl {
    pub base__: IPartBase_Vtbl,
    pub GetThumbnailProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontenttype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetThumbnailContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontenttype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintAsyncCookie(::windows::core::IUnknown);
impl IPrintAsyncCookie {
    pub unsafe fn FinishAsyncCall(&self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FinishAsyncCall)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    pub unsafe fn CancelAsyncCall(&self, param0: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelAsyncCall)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintAsyncCookie, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintAsyncCookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintAsyncCookie {
    type Vtable = IPrintAsyncCookie_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintAsyncCookie {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncCookie_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FinishAsyncCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub CancelAsyncCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintAsyncNewChannelCookie(::windows::core::IUnknown);
impl IPrintAsyncNewChannelCookie {
    pub unsafe fn FinishAsyncCallWithData(&self, param0: *const ::core::option::Option<IPrintAsyncNotifyChannel>, param1: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FinishAsyncCallWithData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(param0), param1).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintAsyncNewChannelCookie, ::windows::core::IUnknown, IPrintAsyncCookie);
impl ::core::clone::Clone for IPrintAsyncNewChannelCookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintAsyncNewChannelCookie {
    type Vtable = IPrintAsyncNewChannelCookie_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintAsyncNewChannelCookie {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNewChannelCookie_Vtbl {
    pub base__: IPrintAsyncCookie_Vtbl,
    pub FinishAsyncCallWithData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const *mut ::core::ffi::c_void, param1: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintAsyncNotify(::windows::core::IUnknown);
impl IPrintAsyncNotify {
    pub unsafe fn CreatePrintAsyncNotifyChannel<P0>(&self, param0: u32, param1: *const ::windows::core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: P0) -> ::windows::core::Result<IPrintAsyncNotifyChannel>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePrintAsyncNotifyChannel)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3, param4.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePrintAsyncNotifyRegistration<P0>(&self, param0: *const ::windows::core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: P0) -> ::windows::core::Result<IPrintAsyncNotifyRegistration>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePrintAsyncNotifyRegistration)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPrintAsyncNotify, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintAsyncNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintAsyncNotify {
    type Vtable = IPrintAsyncNotify_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintAsyncNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x532818f7_921b_4fb2_bff8_2f4fd52ebebf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreatePrintAsyncNotifyChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *const ::windows::core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: *mut ::core::ffi::c_void, param5: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePrintAsyncNotifyRegistration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: *mut ::core::ffi::c_void, param4: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintAsyncNotifyCallback(::windows::core::IUnknown);
impl IPrintAsyncNotifyCallback {
    pub unsafe fn OnEventNotify<P0, P1>(&self, pchannel: P0, pdata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyChannel>>,
        P1: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).OnEventNotify)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), pdata.into().abi()).ok()
    }
    pub unsafe fn ChannelClosed<P0, P1>(&self, pchannel: P0, pdata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyChannel>>,
        P1: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).ChannelClosed)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), pdata.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintAsyncNotifyCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintAsyncNotifyCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintAsyncNotifyCallback {
    type Vtable = IPrintAsyncNotifyCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintAsyncNotifyCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7def34c1_9d92_4c99_b3b3_db94a9d4191b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotifyCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnEventNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ChannelClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintAsyncNotifyChannel(::windows::core::IUnknown);
impl IPrintAsyncNotifyChannel {
    pub unsafe fn SendNotification<P0>(&self, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).SendNotification)(::windows::core::Vtable::as_raw(self), pdata.into().abi()).ok()
    }
    pub unsafe fn CloseChannel<P0>(&self, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNotifyDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).CloseChannel)(::windows::core::Vtable::as_raw(self), pdata.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintAsyncNotifyChannel, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintAsyncNotifyChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintAsyncNotifyChannel {
    type Vtable = IPrintAsyncNotifyChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintAsyncNotifyChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a5031b1_1f3f_4db0_a462_4530ed8b0451);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotifyChannel_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SendNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintAsyncNotifyDataObject(::windows::core::IUnknown);
impl IPrintAsyncNotifyDataObject {
    pub unsafe fn AcquireData(&self, ppnotificationdata: ::core::option::Option<*mut *mut u8>, psize: ::core::option::Option<*mut u32>, ppschema: ::core::option::Option<*mut *mut ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AcquireData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppnotificationdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(psize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppschema.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ReleaseData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReleaseData)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintAsyncNotifyDataObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintAsyncNotifyDataObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintAsyncNotifyDataObject {
    type Vtable = IPrintAsyncNotifyDataObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintAsyncNotifyDataObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77cf513e_5d49_4789_9f30_d0822b335c0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotifyDataObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AcquireData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ReleaseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintAsyncNotifyRegistration(::windows::core::IUnknown);
impl IPrintAsyncNotifyRegistration {
    pub unsafe fn RegisterForNotifications(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RegisterForNotifications)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnregisterForNotifications(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnregisterForNotifications)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintAsyncNotifyRegistration, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintAsyncNotifyRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintAsyncNotifyRegistration {
    type Vtable = IPrintAsyncNotifyRegistration_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintAsyncNotifyRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f6f27b6_6f86_4591_9203_64c3bfadedfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotifyRegistration_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterForNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterForNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintAsyncNotifyServerReferral(::windows::core::IUnknown);
impl IPrintAsyncNotifyServerReferral {
    pub unsafe fn GetServerReferral(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetServerReferral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AsyncGetServerReferral<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAsyncGetSrvReferralCookie>>,
    {
        (::windows::core::Vtable::vtable(self).AsyncGetServerReferral)(::windows::core::Vtable::as_raw(self), param0.into().abi()).ok()
    }
    pub unsafe fn SetServerReferral<P0>(&self, prmtserverreferral: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetServerReferral)(::windows::core::Vtable::as_raw(self), prmtserverreferral.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintAsyncNotifyServerReferral, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintAsyncNotifyServerReferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintAsyncNotifyServerReferral {
    type Vtable = IPrintAsyncNotifyServerReferral_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintAsyncNotifyServerReferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintAsyncNotifyServerReferral_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetServerReferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub AsyncGetServerReferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetServerReferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prmtserverreferral: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintBidiAsyncNotifyRegistration(::windows::core::IUnknown);
impl IPrintBidiAsyncNotifyRegistration {
    pub unsafe fn AsyncGetNewChannel<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintAsyncNewChannelCookie>>,
    {
        (::windows::core::Vtable::vtable(self).AsyncGetNewChannel)(::windows::core::Vtable::as_raw(self), param0.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintBidiAsyncNotifyRegistration, ::windows::core::IUnknown, IPrintAsyncNotifyRegistration);
impl ::core::clone::Clone for IPrintBidiAsyncNotifyRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintBidiAsyncNotifyRegistration {
    type Vtable = IPrintBidiAsyncNotifyRegistration_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintBidiAsyncNotifyRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintBidiAsyncNotifyRegistration_Vtbl {
    pub base__: IPrintAsyncNotifyRegistration_Vtbl,
    pub AsyncGetNewChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintClassObjectFactory(::windows::core::IUnknown);
impl IPrintClassObjectFactory {
    pub unsafe fn GetPrintClassObject<P0>(&self, pszprintername: P0, riid: *const ::windows::core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetPrintClassObject)(::windows::core::Vtable::as_raw(self), pszprintername.into().abi(), riid, ppnewobject).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintClassObjectFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintClassObjectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintClassObjectFactory {
    type Vtable = IPrintClassObjectFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintClassObjectFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9af593dd_9b02_48a8_9bad_69ace423f88b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintClassObjectFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPrintClassObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszprintername: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintCoreHelper(::windows::core::IUnknown);
impl IPrintCoreHelper {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetOption<P0>(&self, pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszfeaturerequested: P0) -> ::windows::core::Result<::windows::core::PSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), cbsize, pszfeaturerequested.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetOptions<P0>(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: P0, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetOptions)(::windows::core::Vtable::as_raw(self), pdevmode, cbsize, bresolveconflicts.into(), pfopairs, cpairs, pcpairswritten, pdwresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn EnumConstrainedOptions<P0>(&self, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: P0, pconstrainedoptionlist: *mut *mut *mut ::windows::core::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).EnumConstrainedOptions)(::windows::core::Vtable::as_raw(self), pdevmode, cbsize, pszfeaturekeyword.into().abi(), pconstrainedoptionlist, pdwnumoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn WhyConstrained<P0, P1>(&self, pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszfeaturekeyword: P0, pszoptionkeyword: P1, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WhyConstrained)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), cbsize, pszfeaturekeyword.into().abi(), pszoptionkeyword.into().abi(), ppfoconstraints, pdwnumoptions).ok()
    }
    pub unsafe fn EnumFeatures(&self, pfeaturelist: *mut *mut *mut ::windows::core::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnumFeatures)(::windows::core::Vtable::as_raw(self), pfeaturelist, pdwnumfeatures).ok()
    }
    pub unsafe fn EnumOptions<P0>(&self, pszfeaturekeyword: P0, poptionlist: *mut *mut *mut ::windows::core::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).EnumOptions)(::windows::core::Vtable::as_raw(self), pszfeaturekeyword.into().abi(), poptionlist, pdwnumoptions).ok()
    }
    pub unsafe fn GetFontSubstitution<P0>(&self, psztruetypefontname: P0, ppszdevfontname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetFontSubstitution)(::windows::core::Vtable::as_raw(self), psztruetypefontname.into().abi(), ppszdevfontname).ok()
    }
    pub unsafe fn SetFontSubstitution<P0, P1>(&self, psztruetypefontname: P0, pszdevfontname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetFontSubstitution)(::windows::core::Vtable::as_raw(self), psztruetypefontname.into().abi(), pszdevfontname.into().abi()).ok()
    }
    pub unsafe fn CreateInstanceOfMSXMLObject<P0>(&self, rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).CreateInstanceOfMSXMLObject)(::windows::core::Vtable::as_raw(self), rclsid, punkouter.into().abi(), dwclscontext, riid, ppv).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintCoreHelper, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintCoreHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintCoreHelper {
    type Vtable = IPrintCoreHelper_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintCoreHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa89ec53e_3905_49c6_9c1a_c0a88117fdb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCoreHelper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: ::windows::core::PCSTR, ppszoption: *mut ::windows::core::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetOption: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub SetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    SetOptions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub EnumConstrainedOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: ::windows::core::PCSTR, pconstrainedoptionlist: *mut *mut *mut ::windows::core::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    EnumConstrainedOptions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub WhyConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: ::windows::core::PCSTR, pszoptionkeyword: ::windows::core::PCSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    WhyConstrained: usize,
    pub EnumFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfeaturelist: *mut *mut *mut ::windows::core::PSTR, pdwnumfeatures: *mut u32) -> ::windows::core::HRESULT,
    pub EnumOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfeaturekeyword: ::windows::core::PCSTR, poptionlist: *mut *mut *mut ::windows::core::PSTR, pdwnumoptions: *mut u32) -> ::windows::core::HRESULT,
    pub GetFontSubstitution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztruetypefontname: ::windows::core::PCWSTR, ppszdevfontname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetFontSubstitution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztruetypefontname: ::windows::core::PCWSTR, pszdevfontname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub CreateInstanceOfMSXMLObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintCoreHelperPS(::windows::core::IUnknown);
impl IPrintCoreHelperPS {
    pub unsafe fn GetGlobalAttribute<P0>(&self, pszattribute: P0, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetGlobalAttribute)(::windows::core::Vtable::as_raw(self), pszattribute.into().abi(), pdwdatatype, ppbdata, pcbsize).ok()
    }
    pub unsafe fn GetFeatureAttribute<P0, P1>(&self, pszfeaturekeyword: P0, pszattribute: P1, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetFeatureAttribute)(::windows::core::Vtable::as_raw(self), pszfeaturekeyword.into().abi(), pszattribute.into().abi(), pdwdatatype, ppbdata, pcbsize).ok()
    }
    pub unsafe fn GetOptionAttribute<P0, P1, P2>(&self, pszfeaturekeyword: P0, pszoptionkeyword: P1, pszattribute: P2, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetOptionAttribute)(::windows::core::Vtable::as_raw(self), pszfeaturekeyword.into().abi(), pszoptionkeyword.into().abi(), pszattribute.into().abi(), pdwdatatype, ppbdata, pcbsize).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintCoreHelperPS, ::windows::core::IUnknown, IPrintCoreHelper);
impl ::core::clone::Clone for IPrintCoreHelperPS {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintCoreHelperPS {
    type Vtable = IPrintCoreHelperPS_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintCoreHelperPS {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2c14f6f_95d3_4d63_96cf_6bd9e6c907c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCoreHelperPS_Vtbl {
    pub base__: IPrintCoreHelper_Vtbl,
    pub GetGlobalAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattribute: ::windows::core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetFeatureAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfeaturekeyword: ::windows::core::PCSTR, pszattribute: ::windows::core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetOptionAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfeaturekeyword: ::windows::core::PCSTR, pszoptionkeyword: ::windows::core::PCSTR, pszattribute: ::windows::core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintCoreHelperUni(::windows::core::IUnknown);
impl IPrintCoreHelperUni {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn CreateGDLSnapshot(&self, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CreateGDLSnapshot)(::windows::core::Vtable::as_raw(self), pdevmode, cbsize, dwflags, ::core::mem::transmute(ppsnapshotstream)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDefaultGDLSnapshot(&self, dwflags: u32) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDefaultGDLSnapshot)(::windows::core::Vtable::as_raw(self), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPrintCoreHelperUni, ::windows::core::IUnknown, IPrintCoreHelper);
impl ::core::clone::Clone for IPrintCoreHelperUni {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintCoreHelperUni {
    type Vtable = IPrintCoreHelperUni_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintCoreHelperUni {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e8e51d6_e5ee_4426_817b_958b9444eb79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCoreHelperUni_Vtbl {
    pub base__: IPrintCoreHelper_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub CreateGDLSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    CreateGDLSnapshot: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDefaultGDLSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppsnapshotstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDefaultGDLSnapshot: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintCoreHelperUni2(::windows::core::IUnknown);
impl IPrintCoreHelperUni2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetNamedCommand<P0>(&self, pdevmode: ::core::option::Option<*const super::Gdi::DEVMODEA>, cbsize: u32, pszcommandname: P0, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetNamedCommand)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())), cbsize, pszcommandname.into().abi(), ppcommandbytes, pcbcommandsize).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintCoreHelperUni2, ::windows::core::IUnknown, IPrintCoreHelper, IPrintCoreHelperUni);
impl ::core::clone::Clone for IPrintCoreHelperUni2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintCoreHelperUni2 {
    type Vtable = IPrintCoreHelperUni2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintCoreHelperUni2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c8afdfc_ead0_4d2d_8071_9bf0175a6c3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCoreHelperUni2_Vtbl {
    pub base__: IPrintCoreHelperUni_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetNamedCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: ::windows::core::PCWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetNamedCommand: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintCoreUI2(::windows::core::IUnknown);
impl IPrintCoreUI2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: ::core::option::Option<&[u8]>, pmszfeatureoptionbuf: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetOptions)(::windows::core::Vtable::as_raw(self), poemuiobj, dwflags, ::core::mem::transmute(pmszfeaturesrequested.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pmszfeaturesrequested.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pmszfeatureoptionbuf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pmszfeatureoptionbuf.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOptions(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: &[u8]) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SetOptions)(::windows::core::Vtable::as_raw(self), poemuiobj, dwflags, ::core::mem::transmute(pmszfeatureoptionbuf.as_ptr()), pmszfeatureoptionbuf.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumConstrainedOptions<P0>(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: P0, pmszconstrainedoptionlist: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).EnumConstrainedOptions)(::windows::core::Vtable::as_raw(self), poemuiobj, dwflags, pszfeaturekeyword.into().abi(), ::core::mem::transmute(pmszconstrainedoptionlist.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pmszconstrainedoptionlist.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WhyConstrained<P0, P1>(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: P0, pszoptionkeyword: P1, pmszreasonlist: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WhyConstrained)(::windows::core::Vtable::as_raw(self), poemuiobj, dwflags, pszfeaturekeyword.into().abi(), pszoptionkeyword.into().abi(), ::core::mem::transmute(pmszreasonlist.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pmszreasonlist.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlobalAttribute<P0>(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: P0, pdwdatatype: *mut u32, pbdata: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetGlobalAttribute)(::windows::core::Vtable::as_raw(self), poemuiobj, dwflags, pszattribute.into().abi(), pdwdatatype, ::core::mem::transmute(pbdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pbdata.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFeatureAttribute<P0, P1>(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: P0, pszattribute: P1, pdwdatatype: *mut u32, pbdata: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetFeatureAttribute)(::windows::core::Vtable::as_raw(self), poemuiobj, dwflags, pszfeaturekeyword.into().abi(), pszattribute.into().abi(), pdwdatatype, ::core::mem::transmute(pbdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pbdata.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOptionAttribute<P0, P1, P2>(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: P0, pszoptionkeyword: P1, pszattribute: P2, pdwdatatype: *mut u32, pbdata: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetOptionAttribute)(::windows::core::Vtable::as_raw(self), poemuiobj, dwflags, pszfeaturekeyword.into().abi(), pszoptionkeyword.into().abi(), pszattribute.into().abi(), pdwdatatype, ::core::mem::transmute(pbdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pbdata.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumFeatures(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnumFeatures)(::windows::core::Vtable::as_raw(self), poemuiobj, dwflags, ::core::mem::transmute(pmszfeaturelist.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pmszfeaturelist.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumOptions<P0>(&self, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: P0, pmszoptionlist: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).EnumOptions)(::windows::core::Vtable::as_raw(self), poemuiobj, dwflags, pszfeaturekeyword.into().abi(), ::core::mem::transmute(pmszoptionlist.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pmszoptionlist.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QuerySimulationSupport<P0>(&self, hprinter: P0, dwlevel: u32, pcaps: ::core::option::Option<&mut [u8]>, pcbneeded: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).QuerySimulationSupport)(::windows::core::Vtable::as_raw(self), hprinter.into(), dwlevel, ::core::mem::transmute(pcaps.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcaps.as_deref().map_or(0, |slice| slice.len() as _), pcbneeded).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintCoreUI2, ::windows::core::IUnknown, IPrintOemDriverUI);
impl ::core::clone::Clone for IPrintCoreUI2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintCoreUI2 {
    type Vtable = IPrintCoreUI2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintCoreUI2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x085ccfca_3adf_4c9e_b491_d851a6edc997);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCoreUI2_Vtbl {
    pub base__: IPrintOemDriverUI_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: ::windows::core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOptions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOptions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumConstrainedOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows::core::PCSTR, pmszconstrainedoptionlist: ::windows::core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumConstrainedOptions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WhyConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows::core::PCSTR, pszoptionkeyword: ::windows::core::PCSTR, pmszreasonlist: ::windows::core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WhyConstrained: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGlobalAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: ::windows::core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGlobalAttribute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFeatureAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows::core::PCSTR, pszattribute: ::windows::core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFeatureAttribute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOptionAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows::core::PCSTR, pszoptionkeyword: ::windows::core::PCSTR, pszattribute: ::windows::core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOptionAttribute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: ::windows::core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumFeatures: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows::core::PCSTR, pmszoptionlist: ::windows::core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumOptions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QuerySimulationSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QuerySimulationSupport: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintJob(::windows::core::IUnknown);
impl IPrintJob {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PrintedPages(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrintedPages)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TotalPages(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TotalPages)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<PrintJobStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Status)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SubmissionTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SubmissionTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RequestCancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RequestCancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintJob, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintJob {
    type Vtable = IPrintJob_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintJob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb771dab8_1282_41b7_858c_f206e4d20577);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintJob_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT,
    pub PrintedPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows::core::HRESULT,
    pub TotalPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut PrintJobStatus) -> ::windows::core::HRESULT,
    pub SubmissionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubmissiontime: *mut f64) -> ::windows::core::HRESULT,
    pub RequestCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintJobCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintJobCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, ulindex: u32) -> ::windows::core::Result<IPrintJob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), ulindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintJobCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintJobCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintJobCollection {
    type Vtable = IPrintJobCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintJobCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b82a24_a598_4e87_895f_cdb23a49e9dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintJobCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintOemCommon(::windows::core::IUnknown);
impl IPrintOemCommon {
    pub unsafe fn GetInfo(&self, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetInfo)(::windows::core::Vtable::as_raw(self), dwmode, pbuffer, cbsize, pcbneeded).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn DevMode(&self, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DevMode)(::windows::core::Vtable::as_raw(self), dwmode, poemdmparam).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintOemCommon, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintOemCommon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintOemCommon {
    type Vtable = IPrintOemCommon_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintOemCommon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f42285e_91d5_11d1_8820_00c04fb961ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOemCommon_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub DevMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    DevMode: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintOemDriverUI(::windows::core::IUnknown);
impl IPrintOemDriverUI {
    pub unsafe fn DrvGetDriverSetting<P0>(&self, pci: *mut ::core::ffi::c_void, feature: P0, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DrvGetDriverSetting)(::windows::core::Vtable::as_raw(self), pci, feature.into().abi(), poutput, cbsize, pcbneeded, pdwoptionsreturned).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrvUpgradeRegistrySetting<P0, P1, P2>(&self, hprinter: P0, pfeature: P1, poption: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DrvUpgradeRegistrySetting)(::windows::core::Vtable::as_raw(self), hprinter.into(), pfeature.into().abi(), poption.into().abi()).ok()
    }
    pub unsafe fn DrvUpdateUISetting(&self, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DrvUpdateUISetting)(::windows::core::Vtable::as_raw(self), pci, poptitem, dwpreviousselection, dwmode).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintOemDriverUI, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintOemDriverUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintOemDriverUI {
    type Vtable = IPrintOemDriverUI_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintOemDriverUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92b05d50_78bc_11d1_9480_00a0c90640b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOemDriverUI_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DrvGetDriverSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, feature: ::windows::core::PCSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DrvUpgradeRegistrySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pfeature: ::windows::core::PCSTR, poption: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrvUpgradeRegistrySetting: usize,
    pub DrvUpdateUISetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintOemUI(::windows::core::IUnknown);
impl IPrintOemUI {
    pub unsafe fn PublishDriverInterface<P0>(&self, piunknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).PublishDriverInterface)(::windows::core::Vtable::as_raw(self), piunknown.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn CommonUIProp(&self, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CommonUIProp)(::windows::core::Vtable::as_raw(self), dwmode, poemcuipparam).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentPropertySheets<P0>(&self, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).DocumentPropertySheets)(::windows::core::Vtable::as_raw(self), ppsuiinfo, lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DevicePropertySheets<P0>(&self, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).DevicePropertySheets)(::windows::core::Vtable::as_raw(self), ppsuiinfo, lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn DevQueryPrintEx(&self, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DevQueryPrintEx)(::windows::core::Vtable::as_raw(self), poemuiobj, pdqpinfo, ppublicdm, poemdm).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn DeviceCapabilitiesA<P0, P1>(&self, poemuiobj: *mut OEMUIOBJ, hprinter: P0, pdevicename: P1, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeviceCapabilitiesA)(::windows::core::Vtable::as_raw(self), poemuiobj, hprinter.into(), pdevicename.into().abi(), wcapability, poutput, ppublicdm, poemdm, dwold, dwresult).ok()
    }
    pub unsafe fn UpgradePrinter(&self, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpgradePrinter)(::windows::core::Vtable::as_raw(self), dwlevel, pdriverupgradeinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrinterEvent<P0, P1>(&self, pprintername: P0, idriverevent: i32, dwflags: u32, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).PrinterEvent)(::windows::core::Vtable::as_raw(self), pprintername.into().abi(), idriverevent, dwflags, lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverEvent<P0>(&self, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).DriverEvent)(::windows::core::Vtable::as_raw(self), dwdriverevent, dwlevel, pdriverinfo, lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn QueryColorProfile<P0>(&self, hprinter: P0, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).QueryColorProfile)(::windows::core::Vtable::as_raw(self), hprinter.into(), poemuiobj, ppublicdm, poemdm, ulquerymode, pvprofiledata, pcbprofiledata, pflprofiledata).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FontInstallerDlgProc<P0, P1, P2>(&self, hwnd: P0, usmsg: u32, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).FontInstallerDlgProc)(::windows::core::Vtable::as_raw(self), hwnd.into(), usmsg, wparam.into(), lparam.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateExternalFonts<P0, P1, P2>(&self, hprinter: P0, hheap: P1, pwstrcartridges: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).UpdateExternalFonts)(::windows::core::Vtable::as_raw(self), hprinter.into(), hheap.into(), pwstrcartridges.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintOemUI, ::windows::core::IUnknown, IPrintOemCommon);
impl ::core::clone::Clone for IPrintOemUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintOemUI {
    type Vtable = IPrintOemUI_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintOemUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6a7a9d0_774c_11d1_947f_00a0c90640b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOemUI_Vtbl {
    pub base__: IPrintOemCommon_Vtbl,
    pub PublishDriverInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub CommonUIProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    CommonUIProp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DocumentPropertySheets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DocumentPropertySheets: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DevicePropertySheets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DevicePropertySheets: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub DevQueryPrintEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    DevQueryPrintEx: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub DeviceCapabilitiesA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: ::windows::core::PCWSTR, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    DeviceCapabilitiesA: usize,
    pub UpgradePrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PrinterEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprintername: ::windows::core::PCWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrinterEvent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverEvent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub QueryColorProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    QueryColorProfile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FontInstallerDlgProc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FontInstallerDlgProc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateExternalFonts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateExternalFonts: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintOemUI2(::windows::core::IUnknown);
impl IPrintOemUI2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn QueryJobAttributes<P0>(&self, hprinter: P0, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).QueryJobAttributes)(::windows::core::Vtable::as_raw(self), hprinter.into(), pdevmode, dwlevel, lpattributeinfo).ok()
    }
    pub unsafe fn HideStandardUI(&self, dwmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).HideStandardUI)(::windows::core::Vtable::as_raw(self), dwmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn DocumentEvent<P0, P1>(&self, hprinter: P0, hdc: P1, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<super::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).DocumentEvent)(::windows::core::Vtable::as_raw(self), hprinter.into(), hdc.into(), iesc, cbin, pvin, cbout, pvout, piresult).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintOemUI2, ::windows::core::IUnknown, IPrintOemCommon, IPrintOemUI);
impl ::core::clone::Clone for IPrintOemUI2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintOemUI2 {
    type Vtable = IPrintOemUI2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintOemUI2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x292515f9_b54b_489b_9275_bab56821395e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOemUI2_Vtbl {
    pub base__: IPrintOemUI_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub QueryJobAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    QueryJobAttributes: usize,
    pub HideStandardUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmode: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub DocumentEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    DocumentEvent: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintOemUIMXDC(::windows::core::IUnknown);
impl IPrintOemUIMXDC {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn AdjustImageableArea<P0>(&self, hprinter: P0, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).AdjustImageableArea)(::windows::core::Vtable::as_raw(self), hprinter.into(), cbdevmode, pdevmode, cboemdm, poemdm, prclimageablearea).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn AdjustImageCompression<P0>(&self, hprinter: P0, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).AdjustImageCompression)(::windows::core::Vtable::as_raw(self), hprinter.into(), cbdevmode, pdevmode, cboemdm, poemdm, pcompressionmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn AdjustDPI<P0>(&self, hprinter: P0, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).AdjustDPI)(::windows::core::Vtable::as_raw(self), hprinter.into(), cbdevmode, pdevmode, cboemdm, poemdm, pdpi).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintOemUIMXDC, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintOemUIMXDC {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintOemUIMXDC {
    type Vtable = IPrintOemUIMXDC_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintOemUIMXDC {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7349d725_e2c1_4dca_afb5_c13e91bc9306);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOemUIMXDC_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub AdjustImageableArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    AdjustImageableArea: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub AdjustImageCompression: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    AdjustImageCompression: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub AdjustDPI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    AdjustDPI: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintPipelineFilter(::windows::core::IUnknown);
impl IPrintPipelineFilter {
    pub unsafe fn InitializeFilter<P0, P1, P2>(&self, pinegotiation: P0, pipropertybag: P1, pipipelinecontrol: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IInterFilterCommunicator>>,
        P1: ::std::convert::Into<::windows::core::InParam<IPrintPipelinePropertyBag>>,
        P2: ::std::convert::Into<::windows::core::InParam<IPrintPipelineManagerControl>>,
    {
        (::windows::core::Vtable::vtable(self).InitializeFilter)(::windows::core::Vtable::as_raw(self), pinegotiation.into().abi(), pipropertybag.into().abi(), pipipelinecontrol.into().abi()).ok()
    }
    pub unsafe fn ShutdownOperation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ShutdownOperation)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn StartOperation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StartOperation)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintPipelineFilter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintPipelineFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintPipelineFilter {
    type Vtable = IPrintPipelineFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintPipelineFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdb62fc0_8bed_434e_86fb_a2cae55f19ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPipelineFilter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InitializeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinegotiation: *mut ::core::ffi::c_void, pipropertybag: *mut ::core::ffi::c_void, pipipelinecontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShutdownOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintPipelineManagerControl(::windows::core::IUnknown);
impl IPrintPipelineManagerControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RequestShutdown<P0>(&self, hrreason: ::windows::core::HRESULT, preason: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IImgErrorInfo>>,
    {
        (::windows::core::Vtable::vtable(self).RequestShutdown)(::windows::core::Vtable::as_raw(self), hrreason, preason.into().abi()).ok()
    }
    pub unsafe fn FilterFinished(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FilterFinished)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintPipelineManagerControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintPipelineManagerControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintPipelineManagerControl {
    type Vtable = IPrintPipelineManagerControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintPipelineManagerControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa3e4910_5889_4681_91ef_823ad4ed4e44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPipelineManagerControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RequestShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT, preason: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RequestShutdown: usize,
    pub FilterFinished: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintPipelineProgressReport(::windows::core::IUnknown);
impl IPrintPipelineProgressReport {
    pub unsafe fn ReportProgress(&self, update: EXpsJobConsumption) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReportProgress)(::windows::core::Vtable::as_raw(self), update).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintPipelineProgressReport, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintPipelineProgressReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintPipelineProgressReport {
    type Vtable = IPrintPipelineProgressReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintPipelineProgressReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedc12c7c_ed40_4ea5_96a6_5e4397497a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPipelineProgressReport_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReportProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, update: EXpsJobConsumption) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintPipelinePropertyBag(::windows::core::IUnknown);
impl IPrintPipelinePropertyBag {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddProperty<P0>(&self, pszname: P0, pvar: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).AddProperty)(::windows::core::Vtable::as_raw(self), pszname.into().abi(), pvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<P0>(&self, pszname: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperty)(::windows::core::Vtable::as_raw(self), pszname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteProperty<P0>(&self, pszname: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteProperty)(::windows::core::Vtable::as_raw(self), pszname.into().abi())
    }
}
::windows::core::interface_hierarchy!(IPrintPipelinePropertyBag, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintPipelinePropertyBag {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintPipelinePropertyBag {
    type Vtable = IPrintPipelinePropertyBag_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintPipelinePropertyBag {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b8c99dc_7892_4a95_8a04_57422e9fbb47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPipelinePropertyBag_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pvar: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteProperty: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintPreviewDxgiPackageTarget(::windows::core::IUnknown);
impl IPrintPreviewDxgiPackageTarget {
    pub unsafe fn SetJobPageCount(&self, counttype: PageCountType, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetJobPageCount)(::windows::core::Vtable::as_raw(self), counttype, count).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn DrawPage<P0>(&self, jobpagenumber: u32, pageimage: P0, dpix: f32, dpiy: f32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).DrawPage)(::windows::core::Vtable::as_raw(self), jobpagenumber, pageimage.into().abi(), dpix, dpiy).ok()
    }
    pub unsafe fn InvalidatePreview(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InvalidatePreview)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintPreviewDxgiPackageTarget, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintPreviewDxgiPackageTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintPreviewDxgiPackageTarget {
    type Vtable = IPrintPreviewDxgiPackageTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintPreviewDxgiPackageTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a6dd0ad_1e2a_4e99_a5ba_91f17818290e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPreviewDxgiPackageTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetJobPageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, counttype: PageCountType, count: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub DrawPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobpagenumber: u32, pageimage: *mut ::core::ffi::c_void, dpix: f32, dpiy: f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    DrawPage: usize,
    pub InvalidatePreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintReadStream(::windows::core::IUnknown);
impl IPrintReadStream {
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: u32, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Seek)(::windows::core::Vtable::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadBytes(&self, pvbuffer: *mut ::core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReadBytes)(::windows::core::Vtable::as_raw(self), pvbuffer, cbrequested, pcbread, pbendoffile).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintReadStream, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintReadStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintReadStream {
    type Vtable = IPrintReadStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintReadStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d47a67c_66cc_4430_850e_daf466fe5bc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintReadStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReadBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReadBytes: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintReadStreamFactory(::windows::core::IUnknown);
impl IPrintReadStreamFactory {
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<IPrintReadStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPrintReadStreamFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintReadStreamFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintReadStreamFactory {
    type Vtable = IPrintReadStreamFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintReadStreamFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacb971e3_df8d_4fc2_bee6_0609d15f3cf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintReadStreamFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaAsyncOperation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaAsyncOperation {
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaAsyncOperation, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaAsyncOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaAsyncOperation {
    type Vtable = IPrintSchemaAsyncOperation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaAsyncOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x143c8dcb_d37f_47f7_88e8_6b1d21f2c5f7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaAsyncOperation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaAsyncOperationEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaAsyncOperationEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Completed<P0>(&self, pticket: P0, hroperation: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintSchemaTicket>>,
    {
        (::windows::core::Vtable::vtable(self).Completed)(::windows::core::Vtable::as_raw(self), pticket.into().abi(), hroperation).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaAsyncOperationEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaAsyncOperationEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaAsyncOperationEvent {
    type Vtable = IPrintSchemaAsyncOperationEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaAsyncOperationEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23adbb16_0133_4906_b29a_1dce1d026379);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaAsyncOperationEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pticket: *mut ::core::ffi::c_void, hroperation: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Completed: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaCapabilities(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaCapabilities {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeatureByKeyName(&self, bstrkeyname: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaFeature> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFeatureByKeyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrkeyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeature(&self, bstrname: &::windows::core::BSTR, bstrnamespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaFeature> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFeature)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrnamespaceuri), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PageImageableSize(&self) -> ::windows::core::Result<IPrintSchemaPageImageableSize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PageImageableSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn JobCopiesAllDocumentsMinValue(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).JobCopiesAllDocumentsMinValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn JobCopiesAllDocumentsMaxValue(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).JobCopiesAllDocumentsMaxValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelectedOptionInPrintTicket<P0>(&self, pfeature: P0) -> ::windows::core::Result<IPrintSchemaOption>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintSchemaFeature>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSelectedOptionInPrintTicket)(::windows::core::Vtable::as_raw(self), pfeature.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetOptions<P0>(&self, pfeature: P0) -> ::windows::core::Result<IPrintSchemaOptionCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintSchemaFeature>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOptions)(::windows::core::Vtable::as_raw(self), pfeature.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaCapabilities, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaCapabilities {
    type Vtable = IPrintSchemaCapabilities_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a577640_501d_4927_bcd0_5ef57a7ed175);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaCapabilities_Vtbl {
    pub base__: IPrintSchemaElement_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFeatureByKeyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrkeyname: *mut ::core::ffi::c_void, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFeatureByKeyName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, bstrnamespaceuri: *mut ::core::ffi::c_void, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFeature: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PageImageableSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppageimageablesize: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PageImageableSize: usize,
    pub JobCopiesAllDocumentsMinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsminvalue: *mut u32) -> ::windows::core::HRESULT,
    pub JobCopiesAllDocumentsMaxValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsmaxvalue: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelectedOptionInPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfeature: *mut ::core::ffi::c_void, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelectedOptionInPrintTicket: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfeature: *mut ::core::ffi::c_void, ppoptioncollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetOptions: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaCapabilities2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaCapabilities2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetParameterDefinition(&self, bstrname: &::windows::core::BSTR, bstrnamespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaParameterDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetParameterDefinition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrnamespaceuri), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaCapabilities2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaCapabilities);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaCapabilities2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaCapabilities2 {
    type Vtable = IPrintSchemaCapabilities2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaCapabilities2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb58845f4_9970_4d87_a636_169fb82ed642);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaCapabilities2_Vtbl {
    pub base__: IPrintSchemaCapabilities_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParameterDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, bstrnamespaceuri: *mut ::core::ffi::c_void, ppparameterdefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParameterDefinition: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaDisplayableElement(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaDisplayableElement {
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaDisplayableElement, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaDisplayableElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaDisplayableElement {
    type Vtable = IPrintSchemaDisplayableElement_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaDisplayableElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf45af49_d6aa_407d_bf87_3912236e9d94);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaDisplayableElement_Vtbl {
    pub base__: IPrintSchemaElement_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaElement(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaElement {
    pub unsafe fn XmlNode(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).XmlNode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NamespaceUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaElement, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaElement {
    type Vtable = IPrintSchemaElement_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x724c1646_e64b_4bbf_8eb4_d45e4fd580da);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaElement_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnamespaceuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaFeature(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaFeature {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectedOption(&self) -> ::windows::core::Result<IPrintSchemaOption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SelectedOption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSelectedOption<P0>(&self, poption: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintSchemaOption>>,
    {
        (::windows::core::Vtable::vtable(self).SetSelectedOption)(::windows::core::Vtable::as_raw(self), poption.into().abi()).ok()
    }
    pub unsafe fn SelectionType(&self) -> ::windows::core::Result<PrintSchemaSelectionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SelectionType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetOption(&self, bstrname: &::windows::core::BSTR, bstrnamespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaOption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOption)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrnamespaceuri), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayUI(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DisplayUI)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaFeature, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaDisplayableElement);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaFeature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaFeature {
    type Vtable = IPrintSchemaFeature_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaFeature {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef189461_5d62_4626_8e57_ff83583c4826);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaFeature_Vtbl {
    pub base__: IPrintSchemaDisplayableElement_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectedOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectedOption: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSelectedOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poption: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSelectedOption: usize,
    pub SelectionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pselectiontype: *mut PrintSchemaSelectionType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, bstrnamespaceuri: *mut ::core::ffi::c_void, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetOption: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayUI: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaNUpOption(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaNUpOption {
    pub unsafe fn PagesPerSheet(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PagesPerSheet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaNUpOption, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaDisplayableElement, IPrintSchemaOption);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaNUpOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaNUpOption {
    type Vtable = IPrintSchemaNUpOption_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaNUpOption {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f6342f2_d848_42e3_8995_c10a9ef9a3ba);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaNUpOption_Vtbl {
    pub base__: IPrintSchemaOption_Vtbl,
    pub PagesPerSheet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulpagespersheet: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaOption(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaOption {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Selected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Selected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Constrained(&self) -> ::windows::core::Result<PrintSchemaConstrainedSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Constrained)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyValue(&self, bstrname: &::windows::core::BSTR, bstrnamespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPropertyValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrnamespaceuri), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaOption, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaDisplayableElement);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaOption {
    type Vtable = IPrintSchemaOption_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaOption {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66bb2f51_5844_4997_8d70_4b7cc221cf92);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaOption_Vtbl {
    pub base__: IPrintSchemaDisplayableElement_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Selected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Selected: usize,
    pub Constrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psetting: *mut PrintSchemaConstrainedSetting) -> ::windows::core::HRESULT,
    pub GetPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, bstrnamespaceuri: *mut ::core::ffi::c_void, ppxmlvaluenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaOptionCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaOptionCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAt(&self, ulindex: u32) -> ::windows::core::Result<IPrintSchemaOption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), ulindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaOptionCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaOptionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaOptionCollection {
    type Vtable = IPrintSchemaOptionCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaOptionCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbaecb0bd_a946_4771_bc30_e8b24f8d45c1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaOptionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAt: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaPageImageableSize(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaPageImageableSize {
    pub unsafe fn ImageableSizeWidthInMicrons(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ImageableSizeWidthInMicrons)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ImageableSizeHeightInMicrons(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ImageableSizeHeightInMicrons)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OriginWidthInMicrons(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OriginWidthInMicrons)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OriginHeightInMicrons(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OriginHeightInMicrons)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ExtentWidthInMicrons(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExtentWidthInMicrons)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ExtentHeightInMicrons(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExtentHeightInMicrons)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaPageImageableSize, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaPageImageableSize {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaPageImageableSize {
    type Vtable = IPrintSchemaPageImageableSize_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaPageImageableSize {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c85bf5e_dc7c_4f61_839b_4107e1c9b68e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaPageImageableSize_Vtbl {
    pub base__: IPrintSchemaElement_Vtbl,
    pub ImageableSizeWidthInMicrons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulimageablesizewidth: *mut u32) -> ::windows::core::HRESULT,
    pub ImageableSizeHeightInMicrons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulimageablesizeheight: *mut u32) -> ::windows::core::HRESULT,
    pub OriginWidthInMicrons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puloriginwidth: *mut u32) -> ::windows::core::HRESULT,
    pub OriginHeightInMicrons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puloriginheight: *mut u32) -> ::windows::core::HRESULT,
    pub ExtentWidthInMicrons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulextentwidth: *mut u32) -> ::windows::core::HRESULT,
    pub ExtentHeightInMicrons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulextentheight: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaPageMediaSizeOption(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaPageMediaSizeOption {
    pub unsafe fn WidthInMicrons(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WidthInMicrons)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HeightInMicrons(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HeightInMicrons)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaPageMediaSizeOption, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaDisplayableElement, IPrintSchemaOption);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaPageMediaSizeOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaPageMediaSizeOption {
    type Vtable = IPrintSchemaPageMediaSizeOption_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaPageMediaSizeOption {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68746729_f493_4830_a10f_69028774605d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaPageMediaSizeOption_Vtbl {
    pub base__: IPrintSchemaOption_Vtbl,
    pub WidthInMicrons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulwidth: *mut u32) -> ::windows::core::HRESULT,
    pub HeightInMicrons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulheight: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaParameterDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaParameterDefinition {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserInputRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserInputRequired)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnitType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UnitType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DataType(&self) -> ::windows::core::Result<PrintSchemaParameterDataType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DataType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeMin(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RangeMin)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeMax(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RangeMax)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaParameterDefinition, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaDisplayableElement);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaParameterDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaParameterDefinition {
    type Vtable = IPrintSchemaParameterDefinition_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaParameterDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ade81e_0e61_4fe1_81c6_c333e4ffe0f1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaParameterDefinition_Vtbl {
    pub base__: IPrintSchemaDisplayableElement_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub UserInputRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserInputRequired: usize,
    pub UnitType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrunittype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatatype: *mut PrintSchemaParameterDataType) -> ::windows::core::HRESULT,
    pub RangeMin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangemin: *mut i32) -> ::windows::core::HRESULT,
    pub RangeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangemax: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaParameterInitializer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaParameterInitializer {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, pvar: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), pvar).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaParameterInitializer, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaParameterInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaParameterInitializer {
    type Vtable = IPrintSchemaParameterInitializer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaParameterInitializer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52027082_0b74_4648_9564_828cc6cb656c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaParameterInitializer_Vtbl {
    pub base__: IPrintSchemaElement_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaTicket(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaTicket {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeatureByKeyName(&self, bstrkeyname: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaFeature> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFeatureByKeyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrkeyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFeature(&self, bstrname: &::windows::core::BSTR, bstrnamespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaFeature> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFeature)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrnamespaceuri), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValidateAsync(&self) -> ::windows::core::Result<IPrintSchemaAsyncOperation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ValidateAsync)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAsync<P0>(&self, pprintticketcommit: P0) -> ::windows::core::Result<IPrintSchemaAsyncOperation>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintSchemaTicket>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommitAsync)(::windows::core::Vtable::as_raw(self), pprintticketcommit.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NotifyXmlChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).NotifyXmlChanged)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<IPrintSchemaCapabilities> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn JobCopiesAllDocuments(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).JobCopiesAllDocuments)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetJobCopiesAllDocuments(&self, uljobcopiesalldocuments: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetJobCopiesAllDocuments)(::windows::core::Vtable::as_raw(self), uljobcopiesalldocuments).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaTicket, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaTicket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaTicket {
    type Vtable = IPrintSchemaTicket_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaTicket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe480b861_4708_4e6d_a5b4_a2b4eeb9baa4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaTicket_Vtbl {
    pub base__: IPrintSchemaElement_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFeatureByKeyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrkeyname: *mut ::core::ffi::c_void, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFeatureByKeyName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, bstrnamespaceuri: *mut ::core::ffi::c_void, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFeature: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ValidateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValidateAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprintticketcommit: *mut ::core::ffi::c_void, ppasyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAsync: usize,
    pub NotifyXmlChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCapabilities: usize,
    pub JobCopiesAllDocuments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puljobcopiesalldocuments: *mut u32) -> ::windows::core::HRESULT,
    pub SetJobCopiesAllDocuments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uljobcopiesalldocuments: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrintSchemaTicket2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrintSchemaTicket2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetParameterInitializer(&self, bstrname: &::windows::core::BSTR, bstrnamespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IPrintSchemaParameterInitializer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetParameterInitializer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrnamespaceuri), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrintSchemaTicket2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrintSchemaElement, IPrintSchemaTicket);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrintSchemaTicket2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrintSchemaTicket2 {
    type Vtable = IPrintSchemaTicket2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrintSchemaTicket2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec1f844_766a_47a1_91f4_2eeb6190f80c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchemaTicket2_Vtbl {
    pub base__: IPrintSchemaTicket_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParameterInitializer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, bstrnamespaceuri: *mut ::core::ffi::c_void, ppparameterinitializer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParameterInitializer: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintTicketProvider(::windows::core::IUnknown);
impl IPrintTicketProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSupportedVersions<P0>(&self, hprinter: P0, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).GetSupportedVersions)(::windows::core::Vtable::as_raw(self), hprinter.into(), ppversions, cversions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BindPrinter<P0>(&self, hprinter: P0, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).BindPrinter)(::windows::core::Vtable::as_raw(self), hprinter.into(), version, poptions, pdevmodeflags, cnamespaces, ppnamespaces).ok()
    }
    pub unsafe fn QueryDeviceNamespace(&self, pdefaultnamespace: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).QueryDeviceNamespace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdefaultnamespace)).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn ConvertPrintTicketToDevMode<P0>(&self, pprintticket: P0, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Data::Xml::MsXml::IXMLDOMDocument2>>,
    {
        (::windows::core::Vtable::vtable(self).ConvertPrintTicketToDevMode)(::windows::core::Vtable::as_raw(self), pprintticket.into().abi(), cbdevmodein, pdevmodein, pcbdevmodeout, ppdevmodeout).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn ConvertDevModeToPrintTicket<P0>(&self, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Data::Xml::MsXml::IXMLDOMDocument2>>,
    {
        (::windows::core::Vtable::vtable(self).ConvertDevModeToPrintTicket)(::windows::core::Vtable::as_raw(self), cbdevmode, pdevmode, pprintticket.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetPrintCapabilities<P0>(&self, pprintticket: P0) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Data::Xml::MsXml::IXMLDOMDocument2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrintCapabilities)(::windows::core::Vtable::as_raw(self), pprintticket.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn ValidatePrintTicket<P0>(&self, pbaseticket: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Data::Xml::MsXml::IXMLDOMDocument2>>,
    {
        (::windows::core::Vtable::vtable(self).ValidatePrintTicket)(::windows::core::Vtable::as_raw(self), pbaseticket.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintTicketProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintTicketProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintTicketProvider {
    type Vtable = IPrintTicketProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintTicketProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb5116db_0a23_4c3a_a6b6_89e5558dfb5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSupportedVersions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSupportedVersions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BindPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut ::windows::core::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BindPrinter: usize,
    pub QueryDeviceNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdefaultnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub ConvertPrintTicketToDevMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    ConvertPrintTicketToDevMode: usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub ConvertDevModeToPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    ConvertDevModeToPrintTicket: usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetPrintCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetPrintCapabilities: usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub ValidatePrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbaseticket: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    ValidatePrintTicket: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintTicketProvider2(::windows::core::IUnknown);
impl IPrintTicketProvider2 {
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetPrintDeviceCapabilities<P0>(&self, pprintticket: P0) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Data::Xml::MsXml::IXMLDOMDocument2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrintDeviceCapabilities)(::windows::core::Vtable::as_raw(self), pprintticket.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetPrintDeviceResources<P0, P1>(&self, pszlocalename: P0, pprintticket: P1) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Data::Xml::MsXml::IXMLDOMDocument2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrintDeviceResources)(::windows::core::Vtable::as_raw(self), pszlocalename.into().abi(), pprintticket.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPrintTicketProvider2, ::windows::core::IUnknown, IPrintTicketProvider);
impl ::core::clone::Clone for IPrintTicketProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintTicketProvider2 {
    type Vtable = IPrintTicketProvider2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintTicketProvider2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8a70ab2_3dfc_4fec_a074_511b13c651cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketProvider2_Vtbl {
    pub base__: IPrintTicketProvider_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetPrintDeviceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void, ppdevicecapabilities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetPrintDeviceCapabilities: usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetPrintDeviceResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlocalename: ::windows::core::PCWSTR, pprintticket: *mut ::core::ffi::c_void, ppdeviceresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetPrintDeviceResources: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintUnidiAsyncNotifyRegistration(::windows::core::IUnknown);
impl IPrintUnidiAsyncNotifyRegistration {
    pub unsafe fn AsyncGetNotification<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAsyncGetSendNotificationCookie>>,
    {
        (::windows::core::Vtable::vtable(self).AsyncGetNotification)(::windows::core::Vtable::as_raw(self), param0.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintUnidiAsyncNotifyRegistration, ::windows::core::IUnknown, IPrintAsyncNotifyRegistration);
impl ::core::clone::Clone for IPrintUnidiAsyncNotifyRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintUnidiAsyncNotifyRegistration {
    type Vtable = IPrintUnidiAsyncNotifyRegistration_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintUnidiAsyncNotifyRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintUnidiAsyncNotifyRegistration_Vtbl {
    pub base__: IPrintAsyncNotifyRegistration_Vtbl,
    pub AsyncGetNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWriteStream(::windows::core::IUnknown);
impl IPrintWriteStream {
    pub unsafe fn WriteBytes(&self, pvbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WriteBytes)(::windows::core::Vtable::as_raw(self), pvbuffer, cbbuffer, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Close(&self) {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IPrintWriteStream, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintWriteStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintWriteStream {
    type Vtable = IPrintWriteStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintWriteStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65bb7f1b_371e_4571_8ac7_912f510c1a38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWriteStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub WriteBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWriteStreamFlush(::windows::core::IUnknown);
impl IPrintWriteStreamFlush {
    pub unsafe fn FlushData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FlushData)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPrintWriteStreamFlush, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrintWriteStreamFlush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrintWriteStreamFlush {
    type Vtable = IPrintWriteStreamFlush_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintWriteStreamFlush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07d11ff8_1753_4873_b749_6cdaf068e4c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWriteStreamFlush_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FlushData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrinterBidiSetRequestCallback(::windows::core::IUnknown);
impl IPrinterBidiSetRequestCallback {
    pub unsafe fn Completed(&self, bstrresponse: &::windows::core::BSTR, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Completed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrresponse), hrstatus).ok()
    }
}
::windows::core::interface_hierarchy!(IPrinterBidiSetRequestCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrinterBidiSetRequestCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrinterBidiSetRequestCallback {
    type Vtable = IPrinterBidiSetRequestCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrinterBidiSetRequestCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc52d32dd_f2b4_4052_8502_ec4305ecb71f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterBidiSetRequestCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresponse: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrinterExtensionAsyncOperation(::windows::core::IUnknown);
impl IPrinterExtensionAsyncOperation {
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPrinterExtensionAsyncOperation, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrinterExtensionAsyncOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrinterExtensionAsyncOperation {
    type Vtable = IPrinterExtensionAsyncOperation_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrinterExtensionAsyncOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x108d6a23_6a4b_4552_9448_68b427186acd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionAsyncOperation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterExtensionContext(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionContext {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrinterQueue(&self) -> ::windows::core::Result<IPrinterQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrinterQueue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrintSchemaTicket(&self) -> ::windows::core::Result<IPrintSchemaTicket> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PrintSchemaTicket)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DriverProperties(&self) -> ::windows::core::Result<IPrinterPropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DriverProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UserProperties(&self) -> ::windows::core::Result<IPrinterPropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterExtensionContext, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterExtensionContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterExtensionContext {
    type Vtable = IPrinterExtensionContext_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterExtensionContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39843bf2_c4d2_41fd_b4b2_aedbee5e1900);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub PrinterQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrinterQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrintSchemaTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppticket: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrintSchemaTicket: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DriverProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DriverProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UserProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UserProperties: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterExtensionContextCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionContextCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAt(&self, ulindex: u32) -> ::windows::core::Result<IPrinterExtensionContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), ulindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterExtensionContextCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterExtensionContextCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterExtensionContextCollection {
    type Vtable = IPrinterExtensionContextCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterExtensionContextCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb476970_9bab_4861_811e_3e98b0c5addf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionContextCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAt: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterExtensionEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnDriverEvent<P0>(&self, peventargs: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrinterExtensionEventArgs>>,
    {
        (::windows::core::Vtable::vtable(self).OnDriverEvent)(::windows::core::Vtable::as_raw(self), peventargs.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnPrinterQueuesEnumerated<P0>(&self, pcontextcollection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrinterExtensionContextCollection>>,
    {
        (::windows::core::Vtable::vtable(self).OnPrinterQueuesEnumerated)(::windows::core::Vtable::as_raw(self), pcontextcollection.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterExtensionEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterExtensionEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterExtensionEvent {
    type Vtable = IPrinterExtensionEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterExtensionEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc093cb63_5ef5_4585_af8e_4d5637487b57);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDriverEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDriverEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnPrinterQueuesEnumerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontextcollection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnPrinterQueuesEnumerated: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterExtensionEventArgs(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionEventArgs {
    pub unsafe fn BidiNotification(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BidiNotification)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReasonId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReasonId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Request(&self) -> ::windows::core::Result<IPrinterExtensionRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Request)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SourceApplication(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SourceApplication)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DetailedReasonId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DetailedReasonId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WindowModal(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WindowModal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WindowParent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WindowParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterExtensionEventArgs, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrinterExtensionContext);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterExtensionEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterExtensionEventArgs {
    type Vtable = IPrinterExtensionEventArgs_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterExtensionEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39843bf4_c4d2_41fd_b4b2_aedbee5e1900);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionEventArgs_Vtbl {
    pub base__: IPrinterExtensionContext_Vtbl,
    pub BidiNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbidinotification: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReasonId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preasonid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprequest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Request: usize,
    pub SourceApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DetailedReasonId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdetailedreasonid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WindowModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmodal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WindowModal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WindowParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwndparent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WindowParent: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IPrinterExtensionManager(::windows::core::IUnknown);
impl IPrinterExtensionManager {
    pub unsafe fn EnableEvents(&self, printerdriverid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnableEvents)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(printerdriverid)).ok()
    }
    pub unsafe fn DisableEvents(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DisableEvents)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPrinterExtensionManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrinterExtensionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPrinterExtensionManager {
    type Vtable = IPrinterExtensionManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrinterExtensionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93c6eb8c_b001_4355_9629_8e8a1b3f8e77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnableEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printerdriverid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DisableEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterExtensionRequest(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterExtensionRequest {
    pub unsafe fn Cancel(&self, hrstatus: ::windows::core::HRESULT, bstrlogmessage: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self), hrstatus, ::core::mem::transmute_copy(bstrlogmessage)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Complete)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterExtensionRequest, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterExtensionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterExtensionRequest {
    type Vtable = IPrinterExtensionRequest_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterExtensionRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39843bf3_c4d2_41fd_b4b2_aedbee5e1900);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterExtensionRequest_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, bstrlogmessage: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterPropertyBag(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterPropertyBag {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBool(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBool)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBool<P0>(&self, bstrname: &::windows::core::BSTR, bvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetBool)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), bvalue.into()).ok()
    }
    pub unsafe fn GetInt32(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInt32)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInt32(&self, bstrname: &::windows::core::BSTR, nvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInt32)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), nvalue).ok()
    }
    pub unsafe fn GetString(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetString(&self, bstrname: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrvalue)).ok()
    }
    pub unsafe fn GetBytes(&self, bstrname: &::windows::core::BSTR, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetBytes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), pcbvalue, ppvalue).ok()
    }
    pub unsafe fn SetBytes(&self, bstrname: &::windows::core::BSTR, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBytes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), pvalue.len() as _, ::core::mem::transmute(pvalue.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetReadStream(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetReadStream)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWriteStream(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWriteStream)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterPropertyBag, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterPropertyBag {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterPropertyBag {
    type Vtable = IPrinterPropertyBag_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterPropertyBag {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfea77364_df95_4a23_a905_019b79a8e481);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterPropertyBag_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBool: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, bvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBool: usize,
    pub GetInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pnvalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, nvalue: i32) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pbstrvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, bstrvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, cbvalue: u32, pvalue: *const u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetReadStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetReadStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWriteStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWriteStream: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterQueue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueue {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Handle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SendBidiQuery(&self, bstrbidiquery: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SendBidiQuery)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrbidiquery)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<IPrinterPropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterQueue, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterQueue {
    type Vtable = IPrinterQueue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterQueue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3580a828_07fe_4b94_ac1a_757d9d2d3056);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phprinter: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Handle: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendBidiQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbidiquery: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProperties: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterQueue2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueue2 {
    pub unsafe fn SendBidiSetRequestAsync<P0>(&self, bstrbidirequest: &::windows::core::BSTR, pcallback: P0) -> ::windows::core::Result<IPrinterExtensionAsyncOperation>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrinterBidiSetRequestCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SendBidiSetRequestAsync)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrbidirequest), pcallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPrinterQueueView(&self, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::Result<IPrinterQueueView> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrinterQueueView)(::windows::core::Vtable::as_raw(self), ulviewoffset, ulviewsize, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterQueue2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrinterQueue);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterQueue2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterQueue2 {
    type Vtable = IPrinterQueue2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterQueue2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cd444e8_c9bb_49b3_8e38_e03209416131);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterQueue2_Vtbl {
    pub base__: IPrinterQueue_Vtbl,
    pub SendBidiSetRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbidirequest: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, ppasyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPrinterQueueView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ppjobview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPrinterQueueView: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterQueueEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueEvent {
    pub unsafe fn OnBidiResponseReceived(&self, bstrresponse: &::windows::core::BSTR, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnBidiResponseReceived)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrresponse), hrstatus).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterQueueEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterQueueEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterQueueEvent {
    type Vtable = IPrinterQueueEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterQueueEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x214685f6_7b78_4681_87e0_495f739273d1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterQueueEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub OnBidiResponseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresponse: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterQueueView(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueView {
    pub unsafe fn SetViewRange(&self, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetViewRange)(::windows::core::Vtable::as_raw(self), ulviewoffset, ulviewsize).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterQueueView, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterQueueView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterQueueView {
    type Vtable = IPrinterQueueView_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterQueueView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x476e2969_3b2b_4b3f_8277_cff6056042aa);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterQueueView_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetViewRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterQueueViewEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterQueueViewEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnChanged<P0>(&self, pcollection: P0, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPrintJobCollection>>,
    {
        (::windows::core::Vtable::vtable(self).OnChanged)(::windows::core::Vtable::as_raw(self), pcollection.into().abi(), ulviewoffset, ulviewsize, ulcountjobsinprintqueue).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterQueueViewEvent, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterQueueViewEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterQueueViewEvent {
    type Vtable = IPrinterQueueViewEvent_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterQueueViewEvent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5b6042b_fd21_404a_a0ef_e2fbb52b9080);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterQueueViewEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcollection: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnChanged: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterScriptContext(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptContext {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DriverProperties(&self) -> ::windows::core::Result<IPrinterScriptablePropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DriverProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueueProperties(&self) -> ::windows::core::Result<IPrinterScriptablePropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QueueProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UserProperties(&self) -> ::windows::core::Result<IPrinterScriptablePropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterScriptContext, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterScriptContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterScriptContext {
    type Vtable = IPrinterScriptContext_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterScriptContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x066acbca_8881_49c9_bb98_fae16b4889e1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterScriptContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DriverProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DriverProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UserProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UserProperties: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterScriptablePropertyBag(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptablePropertyBag {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBool(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBool)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBool<P0>(&self, bstrname: &::windows::core::BSTR, bvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetBool)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), bvalue.into()).ok()
    }
    pub unsafe fn GetInt32(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInt32)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInt32(&self, bstrname: &::windows::core::BSTR, nvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInt32)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), nvalue).ok()
    }
    pub unsafe fn GetString(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetString(&self, bstrname: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBytes(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBytes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBytes<P0>(&self, bstrname: &::windows::core::BSTR, parray: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).SetBytes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), parray.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetReadStream(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IPrinterScriptableStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetReadStream)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWriteStream(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IPrinterScriptableStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWriteStream)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterScriptablePropertyBag, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterScriptablePropertyBag {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterScriptablePropertyBag {
    type Vtable = IPrinterScriptablePropertyBag_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterScriptablePropertyBag {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91c7765f_ed57_49ad_8b01_dc24816a5294);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterScriptablePropertyBag_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBool: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, bvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBool: usize,
    pub GetInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pnvalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, nvalue: i32) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pbstrvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, bstrvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, pparray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBytes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, parray: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBytes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetReadStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetReadStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWriteStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWriteStream: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterScriptablePropertyBag2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptablePropertyBag2 {
    pub unsafe fn GetReadStreamAsXML(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetReadStreamAsXML)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterScriptablePropertyBag2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrinterScriptablePropertyBag);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterScriptablePropertyBag2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterScriptablePropertyBag2 {
    type Vtable = IPrinterScriptablePropertyBag2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterScriptablePropertyBag2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a1c53c4_8638_4b3e_b518_2773c94556a3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterScriptablePropertyBag2_Vtbl {
    pub base__: IPrinterScriptablePropertyBag_Vtbl,
    pub GetReadStreamAsXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterScriptableSequentialStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptableSequentialStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, cbread: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Read)(::windows::core::Vtable::as_raw(self), cbread, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write<P0>(&self, parray: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Write)(::windows::core::Vtable::as_raw(self), parray.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterScriptableSequentialStream, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterScriptableSequentialStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterScriptableSequentialStream {
    type Vtable = IPrinterScriptableSequentialStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterScriptableSequentialStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2072838a_316f_467a_a949_27f68c44a854);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterScriptableSequentialStream_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbread: i32, pparray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Read: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parray: *mut ::core::ffi::c_void, pcbwritten: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrinterScriptableStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrinterScriptableStream {
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Seek)(::windows::core::Vtable::as_raw(self), loffset, streamseek, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSize(&self, lsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSize)(::windows::core::Vtable::as_raw(self), lsize).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPrinterScriptableStream, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IPrinterScriptableSequentialStream);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrinterScriptableStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPrinterScriptableStream {
    type Vtable = IPrinterScriptableStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrinterScriptableStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7edf9a92_4750_41a5_a17f_879a6f4f7dcb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrinterScriptableStream_Vtbl {
    pub base__: IPrinterScriptableSequentialStream_Vtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK, plposition: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Seek: usize,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsize: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IXpsDocument(::windows::core::IUnknown);
impl IXpsDocument {
    pub unsafe fn GetThumbnail(&self) -> ::windows::core::Result<IPartThumbnail> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetThumbnail)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetThumbnail<P0>(&self, pthumbnail: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPartThumbnail>>,
    {
        (::windows::core::Vtable::vtable(self).SetThumbnail)(::windows::core::Vtable::as_raw(self), pthumbnail.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsDocument, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsDocument {
    type Vtable = IXpsDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8d907db_62a9_4a95_abe7_e01763dd30f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocument_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppthumbnail: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pthumbnail: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IXpsDocumentConsumer(::windows::core::IUnknown);
impl IXpsDocumentConsumer {
    pub unsafe fn SendXpsUnknown<P0>(&self, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SendXpsUnknown)(::windows::core::Vtable::as_raw(self), punknown.into().abi()).ok()
    }
    pub unsafe fn SendXpsDocument<P0>(&self, pixpsdocument: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsDocument>>,
    {
        (::windows::core::Vtable::vtable(self).SendXpsDocument)(::windows::core::Vtable::as_raw(self), pixpsdocument.into().abi()).ok()
    }
    pub unsafe fn SendFixedDocumentSequence<P0>(&self, pifixeddocumentsequence: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFixedDocumentSequence>>,
    {
        (::windows::core::Vtable::vtable(self).SendFixedDocumentSequence)(::windows::core::Vtable::as_raw(self), pifixeddocumentsequence.into().abi()).ok()
    }
    pub unsafe fn SendFixedDocument<P0>(&self, pifixeddocument: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFixedDocument>>,
    {
        (::windows::core::Vtable::vtable(self).SendFixedDocument)(::windows::core::Vtable::as_raw(self), pifixeddocument.into().abi()).ok()
    }
    pub unsafe fn SendFixedPage<P0>(&self, pifixedpage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFixedPage>>,
    {
        (::windows::core::Vtable::vtable(self).SendFixedPage)(::windows::core::Vtable::as_raw(self), pifixedpage.into().abi()).ok()
    }
    pub unsafe fn CloseSender(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CloseSender)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetNewEmptyPart<P0>(&self, uri: P0, riid: *const ::windows::core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void, ppwritestream: *mut ::core::option::Option<IPrintWriteStream>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetNewEmptyPart)(::windows::core::Vtable::as_raw(self), uri.into().abi(), riid, ppnewobject, ::core::mem::transmute(ppwritestream)).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsDocumentConsumer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsDocumentConsumer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsDocumentConsumer {
    type Vtable = IXpsDocumentConsumer_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsDocumentConsumer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4368d8a2_4181_4a9f_b295_3d9a38bb9ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentConsumer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SendXpsUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendXpsDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixpsdocument: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendFixedDocumentSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifixeddocumentsequence: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendFixedDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifixeddocument: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendFixedPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifixedpage: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNewEmptyPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void, ppwritestream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IXpsDocumentProvider(::windows::core::IUnknown);
impl IXpsDocumentProvider {
    pub unsafe fn GetXpsPart(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetXpsPart)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsDocumentProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsDocumentProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsDocumentProvider {
    type Vtable = IXpsDocumentProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsDocumentProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8cf8530_5562_47c4_ab67_b1f69ecf961e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetXpsPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixpspart: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IXpsPartIterator(::windows::core::IUnknown);
impl IXpsPartIterator {
    pub unsafe fn Reset(&self) {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Current(&self, puri: *mut ::windows::core::BSTR, ppxpspart: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Current)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(puri), ::core::mem::transmute(ppxpspart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDone(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsDone)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Next(&self) {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IXpsPartIterator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsPartIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsPartIterator {
    type Vtable = IXpsPartIterator_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsPartIterator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0021d3cd_af6f_42ab_9999_14bc82a62d2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsPartIterator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: *mut *mut ::core::ffi::c_void, ppxpspart: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDone: usize,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IXpsRasterizationFactory(::windows::core::IUnknown);
impl IXpsRasterizationFactory {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn CreateRasterizer<P0>(&self, xpspage: P0, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE) -> ::windows::core::Result<IXpsRasterizer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Storage::Xps::IXpsOMPage>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRasterizer)(::windows::core::Vtable::as_raw(self), xpspage.into().abi(), dpi, nontextrenderingmode, textrenderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsRasterizationFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsRasterizationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsRasterizationFactory {
    type Vtable = IXpsRasterizationFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsRasterizationFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe094808a_24c6_482b_a3a7_c21ac9b55f17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsRasterizationFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub CreateRasterizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpspage: *mut ::core::ffi::c_void, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, ppixpsrasterizer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    CreateRasterizer: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IXpsRasterizationFactory1(::windows::core::IUnknown);
impl IXpsRasterizationFactory1 {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn CreateRasterizer<P0>(&self, xpspage: P0, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT) -> ::windows::core::Result<IXpsRasterizer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Storage::Xps::IXpsOMPage>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRasterizer)(::windows::core::Vtable::as_raw(self), xpspage.into().abi(), dpi, nontextrenderingmode, textrenderingmode, pixelformat, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsRasterizationFactory1, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsRasterizationFactory1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsRasterizationFactory1 {
    type Vtable = IXpsRasterizationFactory1_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsRasterizationFactory1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d6e5f77_6414_4a1e_a8e0_d4194ce6a26f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsRasterizationFactory1_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub CreateRasterizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpspage: *mut ::core::ffi::c_void, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, ppixpsrasterizer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    CreateRasterizer: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IXpsRasterizationFactory2(::windows::core::IUnknown);
impl IXpsRasterizationFactory2 {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn CreateRasterizer<P0>(&self, xpspage: P0, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR) -> ::windows::core::Result<IXpsRasterizer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Storage::Xps::IXpsOMPage>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRasterizer)(::windows::core::Vtable::as_raw(self), xpspage.into().abi(), dpix, dpiy, nontextrenderingmode, textrenderingmode, pixelformat, backgroundcolor, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsRasterizationFactory2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsRasterizationFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsRasterizationFactory2 {
    type Vtable = IXpsRasterizationFactory2_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsRasterizationFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c16ce3e_10f5_41fd_9ddc_6826669c2ff6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsRasterizationFactory2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub CreateRasterizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpspage: *mut ::core::ffi::c_void, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR, ppixpsrasterizer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    CreateRasterizer: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IXpsRasterizer(::windows::core::IUnknown);
impl IXpsRasterizer {
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn RasterizeRect<P0>(&self, x: i32, y: i32, width: i32, height: i32, notificationcallback: P0) -> ::windows::core::Result<super::Imaging::IWICBitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsRasterizerNotificationCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RasterizeRect)(::windows::core::Vtable::as_raw(self), x, y, width, height, notificationcallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMinimalLineWidth(&self, width: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMinimalLineWidth)(::windows::core::Vtable::as_raw(self), width).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsRasterizer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsRasterizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsRasterizer {
    type Vtable = IXpsRasterizer_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsRasterizer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7567cfc8_c156_47a8_9dac_11a2ae5bdd6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsRasterizer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub RasterizeRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, height: i32, notificationcallback: *mut ::core::ffi::c_void, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    RasterizeRect: usize,
    pub SetMinimalLineWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
pub struct IXpsRasterizerNotificationCallback(::windows::core::IUnknown);
impl IXpsRasterizerNotificationCallback {
    pub unsafe fn Continue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Continue)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsRasterizerNotificationCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsRasterizerNotificationCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsRasterizerNotificationCallback {
    type Vtable = IXpsRasterizerNotificationCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsRasterizerNotificationCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ab8fd0d_cb94_49c2_9cb0_97ec1d5469d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsRasterizerNotificationCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Continue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APD_COPY_ALL_FILES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APD_COPY_FROM_DIRECTORY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APD_COPY_NEW_FILES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APD_STRICT_DOWNGRADE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APD_STRICT_UPGRADE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APPLYCPSUI_NO_NEWDEF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const APPLYCPSUI_OK_CANCEL_BUTTON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACCESS_ADMINISTRATOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACCESS_USER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACTION_ENUM_SCHEMA: ::windows::core::PCWSTR = ::windows::w!("EnumSchema");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACTION_GET: ::windows::core::PCWSTR = ::windows::w!("Get");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACTION_GET_ALL: ::windows::core::PCWSTR = ::windows::w!("GetAll");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACTION_GET_WITH_ARGUMENT: ::windows::core::PCWSTR = ::windows::w!("GetWithArgument");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ACTION_SET: ::windows::core::PCWSTR = ::windows::w!("Set");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BOOKLET_EDGE_LEFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BOOKLET_EDGE_RIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BOOKLET_PRINT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BORDER_PRINT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BidiRequest: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9162a23_45f9_47cc_80f5_fe0fe9b9e1a2);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BidiRequestContainer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc5b8a24_db05_4a01_8388_22edf6c2bbba);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BidiSpl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a614240_a4c5_4c33_bd87_1bc709331639);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_BIG5: i32 = -10i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_CP437: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_CP850: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_CP863: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_GB2312: i32 = -16i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_ISC: i32 = -11i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_JIS: i32 = -12i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_JIS_ANK: i32 = -13i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_NOPRECNV: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_NS86: i32 = -14i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_SJIS: i32 = -17i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_TCA: i32 = -15i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CC_WANSUNG: i32 = -18i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CDM_CONVERT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CDM_CONVERT351: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CDM_DRIVER_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_FALSE_PDATA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_FALSE_TRUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_NONE_PDATA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_NO_PDATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_NO_YES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_OFF_ON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHKBOXS_OFF_PDATA: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CLSID_OEMPTPROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91723892_45d2_48e2_9ec9_562379daf992);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CLSID_OEMRENDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d6abf26_9f38_11d1_882a_00c04fb961ec);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CLSID_OEMUI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabce80d7_9f46_11d1_882a_00c04fb961ec);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CLSID_OEMUIMXDC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e144300_5b43_4288_932a_5e4dd6d82bed);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CLSID_PTPROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46ac151b_8490_4531_96cc_55bf2bf19e11);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CLSID_XPSRASTERIZER_FACTORY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x503e79bf_1d09_4764_9d72_1eb0c65967c6);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COLOR_OPTIMIZATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_EVENT_ADD_PRINTER_CONNECTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_EVENT_DELETE_PRINTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_EVENT_DELETE_PRINTER_CONNECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_EVENT_FILES_CHANGED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_EVENT_SET_PRINTER_DATAEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_FLAG_CLIENT_SPOOLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const COPYFILE_FLAG_SERVER_SPOOLER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_HPROPSHEETPAGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PCOMPROPSHEETUI: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PCOMPROPSHEETUIA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PCOMPROPSHEETUIW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PFNPROPSHEETUI: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PFNPROPSHEETUIA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PFNPROPSHEETUIW: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PROPSHEETPAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PROPSHEETPAGEA: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_ADD_PROPSHEETPAGEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_DELETE_HCOMPROPSHEET: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_DO_APPLY_CPSUI: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_GET_HPSUIPAGES: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_GET_PAGECOUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_GET_PFNPROPSHEETUI_ICON: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_IGNORE_CPSUI_PSN_APPLY: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_INSERT_PSUIPAGE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_INSERT_PSUIPAGEA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_INSERT_PSUIPAGEW: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_LOAD_CPSUI_ICON: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_LOAD_CPSUI_STRING: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_LOAD_CPSUI_STRINGA: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_LOAD_CPSUI_STRINGW: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_QUERY_DATABLOCK: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_DATABLOCK: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_DMPUB_HIDEBITS: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_FUSION_CONTEXT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_HSTARTPAGE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_PSUIPAGE_ICON: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_PSUIPAGE_TITLE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_PSUIPAGE_TITLEA: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_PSUIPAGE_TITLEW: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSFUNC_SET_RESULT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_ACTION_ITEMS_APPLIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_ACTION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_ACTION_NO_APPLY_EXIT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_ACTION_OPTIF_CHANGED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_ACTION_REINIT_ITEMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_ABOUT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_APPLYNOW: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_DLGPROC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_ECB_CHANGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_EXTPUSH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_ITEMS_REVERTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_KILLACTIVE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_OPTITEM_SETFOCUS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_PUSHBUTTON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_SEL_CHANGED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_SETACTIVE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUICB_REASON_UNDO_CHANGES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUIF_ABOUT_CALLBACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUIF_ICONID_AS_HICON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUIF_UPDATE_PERMISSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUI_CANCEL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUI_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUI_REBOOTSYSTEM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CPSUI_RESTARTWINDOWS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_HEIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_HEIGHTOFFSET: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_MAX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_ORIENTATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_WIDTH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CUSTOMPARAM_WIDTHOFFSET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DEF_PRIORITY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_BKSP_OK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_NOITALIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_NOUNDER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_NO_BOLD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_NO_DOUBLE_UNDERLINE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_NO_STRIKETHRU: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_CAPSL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_HPINTELLIFONT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_OEM1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_OEM2: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_PST1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_TYPE_TRUETYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DF_XM_CR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT: u32 = 11800u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION: u32 = 12100u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION_COUNT: u32 = 12101u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXTCOLLECTION_GETAT: u32 = 12102u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_DRIVERPROPERTIES: u32 = 11803u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_PRINTERQUEUE: u32 = 11801u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_PRINTSCHEMATICKET: u32 = 11802u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_CONTEXT_USERPROPERTIES: u32 = 11804u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENT: u32 = 12200u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS: u32 = 12000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_BIDINOTIFICATION: u32 = 12001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_DETAILEDREASONID: u32 = 12005u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_REASONID: u32 = 12002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_REQUEST: u32 = 12003u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_SOURCEAPPLICATION: u32 = 12004u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_WINDOWMODAL: u32 = 12006u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENTARGS_WINDOWPARENT: u32 = 12007u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENT_ONDRIVEREVENT: u32 = 12201u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_EVENT_ONPRINTERQUEUESENUMERATED: u32 = 12202u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_REQUEST: u32 = 11900u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_REQUEST_CANCEL: u32 = 11901u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTEREXTENSION_REQUEST_COMPLETE: u32 = 11902u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG: u32 = 11400u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETBOOL: u32 = 11401u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETBYTES: u32 = 11407u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETINT32: u32 = 11403u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETREADSTREAM: u32 = 11409u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETSTRING: u32 = 11405u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_GETWRITESTREAM: u32 = 11410u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETBOOL: u32 = 11402u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETBYTES: u32 = 11408u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETINT32: u32 = 11404u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERPROPERTYBAG_SETSTRING: u32 = 11406u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE: u32 = 11600u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEEVENT: u32 = 11700u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEEVENT_ONBIDIRESPONSERECEIVED: u32 = 11701u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEVIEW: u32 = 12700u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEVIEW_EVENT: u32 = 12800u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEVIEW_EVENT_ONCHANGED: u32 = 12801u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUEVIEW_SETVIEWRANGE: u32 = 12701u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_GETPRINTERQUEUEVIEW: u32 = 11606u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_GETPROPERTIES: u32 = 11604u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_HANDLE: u32 = 11601u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_NAME: u32 = 11602u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_SENDBIDIQUERY: u32 = 11603u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERQUEUE_SENDBIDISETREQUESTASYNC: u32 = 11605u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG: u32 = 11500u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETBOOL: u32 = 11501u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETBYTES: u32 = 11507u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETINT32: u32 = 11503u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETREADSTREAM: u32 = 11509u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETSTREAMASXML: u32 = 11411u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETSTRING: u32 = 11505u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_GETWRITESTREAM: u32 = 11510u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETBOOL: u32 = 11502u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETBYTES: u32 = 11508u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETINT32: u32 = 11504u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLEPROPERTYBAG_SETSTRING: u32 = 11506u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM: u32 = 11200u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM_READ: u32 = 11201u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESEQUENTIALSTREAM_WRITE: u32 = 11202u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM: u32 = 11300u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM_COMMIT: u32 = 11301u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM_SEEK: u32 = 11302u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTABLESTREAM_SETSIZE: u32 = 11303u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT: u32 = 12300u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT_DRIVERPROPERTIES: u32 = 12301u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT_QUEUEPROPERTIES: u32 = 12302u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTERSCRIPTCONTEXT_USERPROPERTIES: u32 = 12303u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTJOBCOLLECTION: u32 = 12600u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTJOBCOLLECTION_COUNT: u32 = 12601u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTJOBCOLLECTION_GETAT: u32 = 12602u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION: u32 = 10900u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATIONEVENT: u32 = 11100u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATIONEVENT_COMPLETED: u32 = 11101u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION_CANCEL: u32 = 10902u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ASYNCOPERATION_START: u32 = 10901u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES: u32 = 10800u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETFEATURE: u32 = 10802u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETFEATURE_KEYNAME: u32 = 10801u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETOPTIONS: u32 = 10807u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETPARAMETERDEFINITION: u32 = 10808u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_GETSELECTEDOPTION: u32 = 10806u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_JOBCOPIESMAXVALUE: u32 = 10805u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_JOBCOPIESMINVALUE: u32 = 10804u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_CAPABILITIES_PAGEIMAGEABLESIZE: u32 = 10803u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_DISPLAYABLEELEMENT: u32 = 10100u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_DISPLAYABLEELEMENT_DISPLAYNAME: u32 = 10101u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT_NAME: u32 = 10002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT_NAMESPACEURI: u32 = 10003u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_ELEMENT_XMLNODE: u32 = 10001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_FEATURE: u32 = 10600u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_DISPLAYUI: u32 = 10604u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_GETOPTION: u32 = 10603u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_SELECTEDOPTION: u32 = 10601u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_FEATURE_SELECTIONTYPE: u32 = 10602u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_NUPOPTION: u32 = 10400u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_NUPOPTION_PAGESPERSHEET: u32 = 10401u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTION: u32 = 10200u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION: u32 = 10500u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION_COUNT: u32 = 10501u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTIONCOLLECTION_GETAT: u32 = 10502u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTION_CONSTRAINED: u32 = 10202u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTION_GETPROPERTYVALUE: u32 = 10203u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_OPTION_SELECTED: u32 = 10201u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE: u32 = 10700u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_EXTENT_HEIGHT: u32 = 10706u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_EXTENT_WIDTH: u32 = 10705u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_IMAGEABLE_HEIGHT: u32 = 10702u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_IMAGEABLE_WIDTH: u32 = 10701u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_ORIGIN_HEIGHT: u32 = 10704u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEIMAGEABLESIZE_ORIGIN_WIDTH: u32 = 10703u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION: u32 = 10300u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION_HEIGHT: u32 = 10302u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PAGEMEDIASIZEOPTION_WIDTH: u32 = 10301u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION: u32 = 12500u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_DATATYPE: u32 = 12503u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_RANGEMAX: u32 = 12505u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_RANGEMIN: u32 = 12504u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_UNITTYPE: u32 = 12502u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERDEFINITION_USERINPUTREQUIRED: u32 = 12501u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERINITIALIZER: u32 = 12400u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_PARAMETERINITIALIZER_VALUE: u32 = 12401u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET: u32 = 11000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_COMMITASYNC: u32 = 11004u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETCAPABILITIES: u32 = 11006u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETFEATURE: u32 = 11002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETFEATURE_KEYNAME: u32 = 11001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_GETPARAMETERINITIALIZER: u32 = 11008u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_JOBCOPIESALLDOCUMENTS: u32 = 11007u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_NOTIFYXMLCHANGED: u32 = 11005u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DISPID_PRINTSCHEMA_TICKET_VALIDATEASYNC: u32 = 11003u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DI_CHANNEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DI_MEMORYMAP_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DI_READ_SPOOL_JOB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_BOOKLET_EDGE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_COLOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_COPIES_COLLATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_DEFSOURCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_DITHERTYPE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_DUPLEX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_FIRST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_FORMNAME: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_ICMINTENT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_ICMMETHOD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_LAST: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_MANUAL_DUPLEX: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_MEDIATYPE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_NUP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_NUP_DIRECTION: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_OEM_GRAPHIC_ITEM: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_OEM_PAPER_ITEM: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_OEM_ROOT_ITEM: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_ORIENTATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_OUTPUTBIN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_PAGEORDER: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_PRINTQUALITY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_QUALITY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_SCALE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_STAPLE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_TTOPTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DMPUB_USER: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_ADVANCED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_INVALIDATE_DRIVER_CACHE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_NOPERMISSION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_PROMPT_NON_MODAL: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_RESERVED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DM_USER_DEFAULT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ABORTDOC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_CREATEDCPOST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_CREATEDCPRE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_DELETEDC: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ENDDOC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ENDDOCPOST: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ENDDOCPRE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ENDPAGE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_ESCAPE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_FAILURE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_FIRST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_LAST: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_QUERYFILTER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_RESETDCPOST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_RESETDCPRE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_SPOOLED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_STARTDOC: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_STARTDOCPOST: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_STARTDOCPRE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_STARTPAGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_UNSUPPORTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPOST: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPOST: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTPRINTTICKETPRE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPOST: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPOST: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDDOCUMENTSEQUENCEPRINTTICKETPRE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEEPRE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPOST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPOST: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_ADDFIXEDPAGEPRINTTICKETPRE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DOCUMENTEVENT_XPS_CANCELJOB: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPD_DELETE_ALL_FILES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPD_DELETE_SPECIFIC_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPD_DELETE_UNUSED_FILES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPF_ICONID_AS_HICON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPF_USE_HDLGTEMPLATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DPS_NOPERMISSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DP_STD_DOCPROPPAGE1: u32 = 65533u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DP_STD_DOCPROPPAGE2: u32 = 65534u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DP_STD_RESERVED_START: u32 = 65520u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DP_STD_TREEVIEWPAGE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DRIVER_EVENT_DELETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DRIVER_EVENT_INITIALIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DRIVER_KERNELMODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DRIVER_USERMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DSPRINT_PENDING: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DSPRINT_PUBLISH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DSPRINT_REPUBLISH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DSPRINT_UNPUBLISH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const DSPRINT_UPDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_CHECKNAME_AT_FRONT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_CHECKNAME_ONLY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_CHECKNAME_ONLY_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_ICONID_AS_HICON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_OVERLAY_ECBICON_IF_CHECKED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_OVERLAY_NO_ICON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_OVERLAY_STOP_ICON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ECBF_OVERLAY_WARNING_ICON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EMF_PP_COLOR_OPTIMIZATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_ICONID_AS_HICON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_INCL_SETUP_TITLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_NO_DOT_DOT_DOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_OVERLAY_NO_ICON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_OVERLAY_STOP_ICON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_OVERLAY_WARNING_ICON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_PUSH_TYPE_DLGPROC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const EPF_USE_HDLGTEMPLATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_DEVICE_CONFIG_UNCHANGED: u32 = 13014u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_DEVICE_OFFLINE: u32 = 13004u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_ERROR_BASE: u32 = 13000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_GET_ARGUMENT_NOT_SUPPORTED: u32 = 13012u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_GET_MISSING_ARGUMENT: u32 = 13013u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_GET_REQUIRES_ARGUMENT: u32 = 13011u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_NO_BIDI_SCHEMA_EXTENSIONS: u32 = 13016u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_NO_LOCALIZED_RESOURCES: u32 = 13015u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SCHEMA_NOT_SUPPORTED: u32 = 13005u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SCHEMA_READ_ONLY: u32 = 13002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SCHEMA_WRITE_ONLY: u32 = 13010u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SERVER_OFFLINE: u32 = 13003u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SET_DIFFERENT_TYPE: u32 = 13006u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SET_INVALID_SCHEMAPATH: u32 = 13008u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SET_MULTIPLE_SCHEMAPATH: u32 = 13007u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_SET_UNKNOWN_FAILURE: u32 = 13009u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_STATUS_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_STATUS_WARNING: u32 = 13001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_UNSUPPORTED_CLIENT_LANGUAGE: u32 = 13017u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERROR_BIDI_UNSUPPORTED_RESOURCE_FORMAT: u32 = 13018u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_ALLOCMEM_FAILED: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_CREATEPROPPAGE_FAILED: i32 = -10i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_CREATE_IMAGELIST_FAILED: i32 = -33i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_CREATE_TRACKBAR_FAILED: i32 = -31i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_CREATE_UDARROW_FAILED: i32 = -32i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_DMCOPIES_USE_EXTPUSH: i32 = -43i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_FUNCTION_NOT_IMPLEMENTED: i32 = -9999i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_GETLASTERROR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INTERNAL_ERROR: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_DLGPAGEIDX: i32 = -16i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_DLGPAGE_CBSIZE: i32 = -14i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_DMPUBID: i32 = -29i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_DMPUB_TVOT: i32 = -30i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_ECB_CBSIZE: i32 = -26i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_EDITBOX_BUF_SIZE: i32 = -25i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_EDITBOX_PSEL: i32 = -24i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_EXTPUSH_CBSIZE: i32 = -39i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_LBCB_TYPE: i32 = -35i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_LPARAM: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_OPTITEM_CBSIZE: i32 = -19i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_OPTPARAM_CBSIZE: i32 = -23i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_OPTTYPE_CBSIZE: i32 = -20i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_OPTTYPE_COUNT: i32 = -21i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_PDATA: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_PDLGPAGE: i32 = -13i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_PUSHBUTTON_TYPE: i32 = -38i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_INVALID_TVOT_TYPE: i32 = -34i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_MORE_THAN_ONE_STDPAGE: i32 = -12i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_MORE_THAN_ONE_TVPAGE: i32 = -11i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NO_EXTPUSH_DLGTEMPLATEID: i32 = -41i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NO_PROPSHEETPAGE: i32 = -8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_CALLERNAME: i32 = -6i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_ECB_PCHECKEDNAME: i32 = -28i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_ECB_PTITLE: i32 = -27i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_EXTPUSH_CALLBACK: i32 = -42i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_EXTPUSH_DLGPROC: i32 = -40i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_HINST: i32 = -5i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_OPTITEMNAME: i32 = -7i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_POPTITEM: i32 = -18i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_NULL_POPTPARAM: i32 = -22i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_SUBITEM_DIFF_DLGPAGEIDX: i32 = -17i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_SUBITEM_DIFF_OPTIF_HIDE: i32 = -36i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_TOO_MANY_DLGPAGES: i32 = -15i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_TOO_MANY_PROPSHEETPAGES: i32 = -9i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ERR_CPSUI_ZERO_OPTITEM: i32 = -44i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const E_VERSION_NOT_SUPPORTED: u32 = 2147745793u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FG_CANCHANGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FILL_WITH_DEFAULTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FMTID_PrinterPropertyBag: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75f9adca_097d_45c3_a6e4_bab29e276f3e);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_CURRENTFONTID: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTBOLD: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTHEIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTITALIC: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTMAXWIDTH: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTSTRIKETHRU: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTUNDERLINE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_FONTWIDTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_GRAYPERCENTAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_MAX: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_NEXTFONTID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_NEXTGLYPH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_PRINTDIRINCCDEGREES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_TEXTXRES: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FNT_INFO_TEXTYRES: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_DIR_SORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_DEVICEFONT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_GLYPHSET_GTT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_GLYPHSET_RLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_IFI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_PERMANENT_SF: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_RESERVED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_SOFTFONT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FONT_FL_UFM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FORM_BUILTIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FORM_PRINTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FORM_USER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const GPD_OEMCUSTOMDATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const GUID_DEVINTERFACE_IPPUSB_PRINT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2f40381_f46d_4e51_bce7_62de6cf2d098);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const GUID_DEVINTERFACE_USBPRINT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28d78fad_5a12_11d1_ae5b_0000f803a8c2);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ADVANCE: u32 = 64058u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_AUTOSEL: u32 = 64025u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_COLLATE: u32 = 64030u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_COLOR: u32 = 64040u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_COPY: u32 = 64046u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DEVICE: u32 = 64060u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DEVICE2: u32 = 64061u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DEVICE_FEATURE: u32 = 64080u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DITHER_COARSE: u32 = 64042u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DITHER_FINE: u32 = 64043u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DITHER_LINEART: u32 = 64044u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DITHER_NONE: u32 = 64041u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DOCUMENT: u32 = 64059u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_HORZ: u32 = 64032u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_HORZ_L: u32 = 64085u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_NONE: u32 = 64031u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_NONE_L: u32 = 64084u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_VERT: u32 = 64033u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_DUPLEX_VERT_L: u32 = 64086u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_EMPTY: u32 = 64000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ENVELOPE: u32 = 64010u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ENVELOPE_FEED: u32 = 64097u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ERROR: u32 = 64050u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FALSE: u32 = 64005u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FAX: u32 = 64095u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FONTCART: u32 = 64013u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FONTCARTHDR: u32 = 64012u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FONTCART_SLOT: u32 = 64098u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FONTSUB: u32 = 64081u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_FORMTRAYASSIGN: u32 = 64076u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_GENERIC_ITEM: u32 = 64073u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_GENERIC_OPTION: u32 = 64072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_GRAPHIC: u32 = 64057u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_HALFTONE_SETUP: u32 = 64048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_HTCLRADJ: u32 = 64047u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_HT_DEVICE: u32 = 64017u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_HT_HOST: u32 = 64016u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ICM_INTENT: u32 = 64053u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ICM_METHOD: u32 = 64052u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ICM_OPTION: u32 = 64051u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ICONID_FIRST: u32 = 64000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ICONID_LAST: u32 = 64111u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_INSTALLABLE_OPTION: u32 = 64078u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LANDSCAPE: u32 = 64023u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ARROWL: u32 = 64100u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ARROWLR: u32 = 64104u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ARROWS: u32 = 64101u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL: u32 = 64102u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL_NB: u32 = 64106u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP: u32 = 64103u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP_NB: u32 = 64107u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_PORTRAIT: u32 = 64099u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LAYOUT_BMP_ROT_PORT: u32 = 64105u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LF_PEN_PLOTTER: u32 = 64087u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_LF_RASTER_PLOTTER: u32 = 64089u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_MANUAL_FEED: u32 = 64094u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_MEM: u32 = 64011u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_MONO: u32 = 64039u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_NO: u32 = 64003u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_NOTINSTALLED: u32 = 64069u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_NUP_BORDER: u32 = 64111u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_OFF: u32 = 64007u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ON: u32 = 64008u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_OPTION: u32 = 64066u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_OPTION2: u32 = 64067u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_OUTBIN: u32 = 64055u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_OUTPUT: u32 = 64056u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PAGE_PROTECT: u32 = 64096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PAPER_OUTPUT: u32 = 64009u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PAPER_TRAY: u32 = 64026u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PAPER_TRAY2: u32 = 64027u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PAPER_TRAY3: u32 = 64028u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PEN_CARROUSEL: u32 = 64092u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PLOTTER_PEN: u32 = 64093u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PORTRAIT: u32 = 64022u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_POSTSCRIPT: u32 = 64082u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER: u32 = 64062u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER2: u32 = 64063u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER3: u32 = 64064u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER4: u32 = 64065u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER_FEATURE: u32 = 64079u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_PRINTER_FOLDER: u32 = 64077u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_QUESTION: u32 = 64075u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RES_DRAFT: u32 = 64034u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RES_HIGH: u32 = 64037u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RES_LOW: u32 = 64035u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RES_MEDIUM: u32 = 64036u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RES_PRESENTATION: u32 = 64038u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ROLL_PAPER: u32 = 64091u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ROT_LAND: u32 = 64024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_ROT_PORT: u32 = 64110u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_RUN_DIALOG: u32 = 64074u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_SCALING: u32 = 64045u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_SEL_NONE: u32 = 64001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_SF_PEN_PLOTTER: u32 = 64088u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_SF_RASTER_PLOTTER: u32 = 64090u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_STAPLER_OFF: u32 = 64015u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_STAPLER_ON: u32 = 64014u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_STD_FORM: u32 = 64054u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_STOP: u32 = 64068u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_STOP_WARNING_OVERLAY: u32 = 64071u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TELEPHONE: u32 = 64083u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TRANSPARENT: u32 = 64029u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TRUE: u32 = 64006u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TT_DOWNLOADSOFT: u32 = 64019u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TT_DOWNLOADVECT: u32 = 64020u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TT_PRINTASGRAPHIC: u32 = 64018u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_TT_SUBDEV: u32 = 64021u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_WARNING: u32 = 64002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_WARNING_OVERLAY: u32 = 64070u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_WATERMARK: u32 = 64049u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDI_CPSUI_YES: u32 = 64004u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ABOUT: u32 = 64848u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ADVANCED: u32 = 64722u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ADVANCEDOCUMENT: u32 = 64716u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ALL: u32 = 64841u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_AUTOSELECT: u32 = 64718u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BACKTOFRONT: u32 = 64857u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BOND: u32 = 64786u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BOOKLET: u32 = 64873u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BOOKLET_EDGE: u32 = 64888u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BOOKLET_EDGE_LEFT: u32 = 64889u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_BOOKLET_EDGE_RIGHT: u32 = 64890u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_CASSETTE_TRAY: u32 = 64810u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_CHANGE: u32 = 64702u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_CHANGED: u32 = 64846u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_CHANGES: u32 = 64845u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COARSE: u32 = 64787u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COLLATE: u32 = 64756u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COLLATED: u32 = 64757u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COLON_SEP: u32 = 64707u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COLOR: u32 = 64764u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COLOR_APPERANCE: u32 = 64744u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COPIES: u32 = 64831u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_COPY: u32 = 64830u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEFAULT: u32 = 64732u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEFAULTDOCUMENT: u32 = 64714u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEFAULT_TRAY: u32 = 64811u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEVICE: u32 = 64842u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEVICEOPTIONS: u32 = 64725u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DEVICE_SETTINGS: u32 = 64852u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DITHERING: u32 = 64752u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DOCUMENT: u32 = 64715u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DOWN_THEN_LEFT: u32 = 64882u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DOWN_THEN_RIGHT: u32 = 64880u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DRAFT: u32 = 64759u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_DUPLEX: u32 = 64745u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ENVELOPE_TRAY: u32 = 64804u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ENVMANUAL_TRAY: u32 = 64805u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ERRDIFFUSE: u32 = 64790u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ERROR: u32 = 64733u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_EXIST: u32 = 64736u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FALSE: u32 = 64726u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FAST: u32 = 64838u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FAX: u32 = 64835u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FINE: u32 = 64788u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FORMNAME: u32 = 64747u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FORMSOURCE: u32 = 64812u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FORMTRAYASSIGN: u32 = 64798u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_FRONTTOBACK: u32 = 64856u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_GLOSSY: u32 = 64783u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_GRAPHIC: u32 = 64720u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_GRAYSCALE: u32 = 64765u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_HALFTONE: u32 = 64791u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_HALFTONE_SETUP: u32 = 64817u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_HIGH: u32 = 64762u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_HORIZONTAL: u32 = 64768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_HTCLRADJ: u32 = 64792u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM: u32 = 64748u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICMINTENT: u32 = 64750u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICMMETHOD: u32 = 64749u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_BLACKWHITE: u32 = 64776u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_COLORMETRIC: u32 = 64781u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_CONTRAST: u32 = 64780u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_NO: u32 = 64777u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_SATURATION: u32 = 64779u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ICM_YES: u32 = 64778u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_INSTFONTCART: u32 = 64818u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LANDSCAPE: u32 = 64754u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LARGECAP_TRAY: u32 = 64809u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LARGEFMT_TRAY: u32 = 64808u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LBCB_NOSEL: u32 = 64712u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LEFT_ANGLE: u32 = 64708u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LEFT_SLOT: u32 = 64823u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LEFT_THEN_DOWN: u32 = 64881u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LINEART: u32 = 64789u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LONG_SIDE: u32 = 64770u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LOW: u32 = 64760u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_LOWER_TRAY: u32 = 64801u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MAILBOX: u32 = 64829u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MAKE: u32 = 64833u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MANUALFEED: u32 = 64813u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MANUAL_DUPLEX: u32 = 64883u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MANUAL_DUPLEX_OFF: u32 = 64885u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MANUAL_DUPLEX_ON: u32 = 64884u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MANUAL_TRAY: u32 = 64803u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MEDIA: u32 = 64751u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MEDIUM: u32 = 64761u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MIDDLE_TRAY: u32 = 64802u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MONOCHROME: u32 = 64766u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_MORE: u32 = 64701u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NO: u32 = 64728u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NONE: u32 = 64734u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NOT: u32 = 64735u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NOTINSTALLED: u32 = 64737u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NO_NAME: u32 = 64850u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUM_OF_COPIES: u32 = 64740u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP: u32 = 64864u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_BORDER: u32 = 64891u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_BORDERED: u32 = 64892u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_DIRECTION: u32 = 64878u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_FOURUP: u32 = 64867u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_NINEUP: u32 = 64869u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_NORMAL: u32 = 64865u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_SIXTEENUP: u32 = 64870u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_SIXUP: u32 = 64868u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_NUP_TWOUP: u32 = 64866u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OF: u32 = 64704u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OFF: u32 = 64730u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ON: u32 = 64731u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ONLYONE: u32 = 64800u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OPTION: u32 = 64703u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OPTIONS: u32 = 64721u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ORIENTATION: u32 = 64738u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OUTBINASSIGN: u32 = 64796u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_OUTPUTBIN: u32 = 64863u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PAGEORDER: u32 = 64855u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PAGEPROTECT: u32 = 64816u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PAPER_OUTPUT: u32 = 64719u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PERCENT: u32 = 64711u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PLOT: u32 = 64836u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PORTRAIT: u32 = 64753u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_POSTER: u32 = 64874u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_POSTER_2x2: u32 = 64875u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_POSTER_3x3: u32 = 64876u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_POSTER_4x4: u32 = 64877u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRESENTATION: u32 = 64763u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINT: u32 = 64834u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINTER: u32 = 64717u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINTERMEM_KB: u32 = 64814u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINTERMEM_MB: u32 = 64815u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINTFLDSETTING: u32 = 64758u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PRINTQUALITY: u32 = 64742u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_PROPERTIES: u32 = 64713u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_QUALITY_BEST: u32 = 64861u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_QUALITY_BETTER: u32 = 64860u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_QUALITY_CUSTOM: u32 = 64862u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_QUALITY_DRAFT: u32 = 64859u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_QUALITY_SETTINGS: u32 = 64858u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RANGE_FROM: u32 = 64705u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_REGULAR: u32 = 64785u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RESET: u32 = 64840u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RESOLUTION: u32 = 64743u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_REVERT: u32 = 64844u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RIGHT_ANGLE: u32 = 64709u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RIGHT_SLOT: u32 = 64824u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_RIGHT_THEN_DOWN: u32 = 64879u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ROTATED: u32 = 64839u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ROT_LAND: u32 = 64755u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_ROT_PORT: u32 = 64886u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SCALING: u32 = 64739u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SETTING: u32 = 64851u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SETTINGS: u32 = 64843u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SETUP: u32 = 64700u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SHORT_SIDE: u32 = 64771u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SIDE1: u32 = 64871u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SIDE2: u32 = 64872u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SIMPLEX: u32 = 64767u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLASH_SEP: u32 = 64710u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLOT1: u32 = 64819u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLOT2: u32 = 64820u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLOT3: u32 = 64821u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLOT4: u32 = 64822u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SLOW: u32 = 64837u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SMALLFMT_TRAY: u32 = 64807u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_SOURCE: u32 = 64741u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STACKER: u32 = 64828u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STANDARD: u32 = 64782u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STAPLE: u32 = 64887u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STAPLER: u32 = 64825u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STAPLER_OFF: u32 = 64827u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STAPLER_ON: u32 = 64826u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STDDOCPROPTAB: u32 = 64723u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STDDOCPROPTAB1: u32 = 64853u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STDDOCPROPTAB2: u32 = 64854u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STDDOCPROPTVTAB: u32 = 64724u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STRID_FIRST: u32 = 64700u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_STRID_LAST: u32 = 64892u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TO: u32 = 64706u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TOTAL: u32 = 64832u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TRACTOR_TRAY: u32 = 64806u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TRANSPARENCY: u32 = 64784u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TRUE: u32 = 64727u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TTOPTION: u32 = 64746u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TT_DOWNLOADSOFT: u32 = 64773u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TT_DOWNLOADVECT: u32 = 64774u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TT_PRINTASGRAPHIC: u32 = 64772u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_TT_SUBDEV: u32 = 64775u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_UPPER_TRAY: u32 = 64799u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_USE_DEVICE_HT: u32 = 64794u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_USE_HOST_HT: u32 = 64793u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_USE_PRINTER_HT: u32 = 64795u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_VERSION: u32 = 64849u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_VERTICAL: u32 = 64769u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_WARNING: u32 = 64847u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_WATERMARK: u32 = 64797u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IDS_CPSUI_YES: u32 = 64729u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INSPSUIPAGE_MODE_AFTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INSPSUIPAGE_MODE_BEFORE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INSPSUIPAGE_MODE_FIRST_CHILD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INSPSUIPAGE_MODE_INDEX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INSPSUIPAGE_MODE_LAST_CHILD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_ADD_CHILD_DEVICE: u32 = 2228316u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_ADD_MSIPP_COMPAT_ID: u32 = 2228308u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_CYCLE_PORT: u32 = 2228320u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_GET_1284_ID: u32 = 2228276u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_GET_INTERFACE_TYPE: u32 = 2228300u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_GET_LPT_STATUS: u32 = 2228272u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_GET_PROTOCOL: u32 = 2228292u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_SET_DEVICE_ID: u32 = 2228312u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_SET_PORT_NUMBER: u32 = 2228304u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_SET_PROTOCOL: u32 = 2228296u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_SOFT_RESET: u32 = 2228288u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_VENDOR_GET_COMMAND: u32 = 2228284u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IOCTL_USBPRINT_VENDOR_SET_COMMAND: u32 = 2228280u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IPDFP_COPY_ALL_FILES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_ACCESS_ADMINISTER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_ACCESS_READ: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_CANCEL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_DELETE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_LAST_PAGE_EJECTED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_PAUSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_RELEASE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_RESTART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_RESUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_RETAIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_CONTROL_SENT_TO_PRINTER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_BYTES_PRINTED: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_DATATYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_DEVMODE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_DOCUMENT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_DRIVER_NAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_MACHINE_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_NOTIFY_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PAGES_PRINTED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PARAMETERS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PORT_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_POSITION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PRINTER_NAME: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PRINT_PROCESSOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_PRIORITY: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_REMOTE_JOB_ID: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_SECURITY_DESCRIPTOR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_START_TIME: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_STATUS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_STATUS_STRING: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_SUBMITTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_TIME: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_TOTAL_BYTES: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_TOTAL_PAGES: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_UNTIL_TIME: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_FIELD_USER_NAME: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_NOTIFY_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_POSITION_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_BLOCKED_DEVQ: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_COMPLETE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_DELETED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_DELETING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_OFFLINE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_PAPEROUT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_PAUSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_PRINTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_PRINTING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_RENDERING_LOCALLY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_RESTART: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_RETAINED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_SPOOLING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const JOB_STATUS_USER_INTERVENTION: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const LPR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_ADDRESS_STR_LEN: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_CPSFUNC_INDEX: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_DEVICEDESCRIPTION_STR_LEN: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_DLGPAGE_COUNT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_FORM_KEYWORD_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_IPADDR_STR_LEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_NETWORKNAME2_LEN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_NETWORKNAME_LEN: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_PORTNAME_LEN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_PRIORITY: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_PROPSHEETUI_REASON_INDEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_PSUIPAGEINSERT_INDEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_QUEUENAME_LEN: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_RES_STR_CHARS: u32 = 160u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_SNMP_COMMUNITY_STR_LEN: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MIN_PRIORITY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MS_PRINT_JOB_OUTPUT_FILE: ::windows::core::PCWSTR = ::windows::w!("MsPrintJobOutputFile");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_ADD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_COMPOSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_DIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_DISABLE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_DOUBLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_DOUBLEBYTECHAR_MASK: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_FORMAT_MASK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_PAIRED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_PREDEFIN_MASK: u32 = 224u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_REPLACE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MTYPE_SINGLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_GRAPHICS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_PHYSICAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_RELATIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_SENDXMOVECMD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_SENDYMOVECMD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MV_UPDATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_GET_FILENAME: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_PRINTTICKET_FIXED_DOC: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_PRINTTICKET_FIXED_DOC_SEQ: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_PRINTTICKET_FIXED_PAGE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_SET_S0PAGE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_SET_S0PAGE_RESOURCE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDCOP_SET_XPSPASSTHRU_MODE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_ESCAPE: u32 = 4122u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NORMAL_PRINT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_RELEASE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba9a5027_a70e_4ae7_9b7d_eb3e06ad4157);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NO_BORDER_PRINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NO_COLOR_OPTIMIZATION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NO_PRIORITY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMCUIP_DOCPROP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMCUIP_PRNPROP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMDM_CONVERT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMDM_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMDM_MERGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMDM_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_FREEMEM: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_JOBTIMEOUT: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_MAX: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_MAXBITMAP: u32 = 32774u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_MINOUTLINE: u32 = 32773u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_MIN_DOCSTICKY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_MIN_PRINTERSTICKY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PRINTFLAGS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PROTOCOL: u32 = 32772u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_CUSTOMSIZE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_DIALECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_NUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_PSLEVEL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_PSDM_TTDLFMT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_UNIDM_FLAGS: u32 = 16385u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_UNIDM_GPDVER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGDS_WAITTIMEOUT: u32 = 32771u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGI_GETINTERFACEVERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGI_GETPUBLISHERINFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGI_GETREQUESTEDHELPERINTERFACES: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGI_GETSIGNATURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMGI_GETVERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMPUBLISH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMPUBLISH_IPRINTCOREHELPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMTTY_INFO_CODEPAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMTTY_INFO_MARGINS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMTTY_INFO_NUM_UFMS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEMTTY_INFO_UFM_IDS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OEM_MODE_PUBLISHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OIEXTF_ANSI_STRING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTCF_HIDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTCF_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_CALLBACK: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_CHANGED: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_CHANGEONCE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_COLLAPSE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_DISABLED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_ECB_CHECKED: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_EXT_DISABLED: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_EXT_HIDE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_EXT_IS_EXTPUSH: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_HAS_POIEXT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_HIDE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_INITIAL_TVITEM: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_MASK: i32 = 131071i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_NO_GROUPBOX_NAME: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_OVERLAY_NO_ICON: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_OVERLAY_STOP_ICON: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_OVERLAY_WARNING_ICON: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTIF_SEL_AS_HICON: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_HIDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_ICONID_AS_HICON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_MASK: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_OVERLAY_NO_ICON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_OVERLAY_STOP_ICON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_OVERLAY_WARNING_ICON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTPF_USE_HDLGTEMPLATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTTF_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTTF_NOSPACE_BEFORE_POSTFIX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OPTTF_TYPE_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_LBCB_INCL_ITEM_NONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_LBCB_NO_ICON16_IN_ITEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_LBCB_PROPPAGE_CBUSELB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_LBCB_PROPPAGE_LBUSECB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_LBCB_SORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_PUSH_ENABLE_ALWAYS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_PUSH_INCL_SETUP_TITLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const OTS_PUSH_NO_DOT_DOT_DOT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PDEV_ADJUST_PAPER_MARGIN_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PDEV_HOSTFONT_ENABLED_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PDEV_USE_TRUE_COLOR_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_DOOR_OPEN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_NO_TONER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_OUTPUT_BIN_FULL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_OUT_OF_MEMORY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_PAPER_JAM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_PAPER_OUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_PAPER_PROBLEM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_POWER_SAVE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_TONER_LOW: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_TYPE_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_TYPE_INFO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_TYPE_WARNING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_USER_INTERVENTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_STATUS_WARMING_UP: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_TYPE_NET_ATTACHED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_TYPE_READ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_TYPE_REDIRECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PORT_TYPE_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PPCAPS_BOOKLET_EDGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PPCAPS_BORDER_PRINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PPCAPS_REVERSE_PAGES_FOR_REVERSE_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PPCAPS_RIGHT_THEN_DOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PPCAPS_SQUARE_SCALING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_DIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_DO_COMPLETE_FIRST: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_ENABLE_BIDI: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_ENABLE_DEVQ: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_ENTERPRISE_CLOUD: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_FAX: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_FRIENDLY_NAME: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_HIDDEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_KEEPPRINTEDJOBS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_LOCAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_MACHINE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_NETWORK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_PER_USER: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_PUBLISHED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_PUSHED_MACHINE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_PUSHED_USER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_QUEUED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_RAW_ONLY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_SHARED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_TS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_TS_GENERIC_DRIVER: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ATTRIBUTE_WORK_OFFLINE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_FORM: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_JOB: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_PORT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_PRINTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_PRINTER_DRIVER: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ADD_PRINT_PROCESSOR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_ALL: u32 = 2138570751u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_CONFIGURE_PORT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_FORM: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_JOB: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_PORT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_PRINTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_PRINTER_DRIVER: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_DELETE_PRINT_PROCESSOR: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_FAILED_CONNECTION_PRINTER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_FORM: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_JOB: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_PORT: u32 = 7340032u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_PRINTER: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_PRINTER_DRIVER: u32 = 1879048192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_PRINT_PROCESSOR: u32 = 117440512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_SERVER: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_SET_FORM: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_SET_JOB: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_SET_PRINTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_SET_PRINTER_DRIVER: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_TIMEOUT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CHANGE_WRITE_JOB: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONNECTION_MISMATCH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONNECTION_NO_UI: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONTROL_PAUSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONTROL_PURGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONTROL_RESUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_CONTROL_SET_STATUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_3D: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_CLOUD: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_FAX: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_FILE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_SERVICE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CATEGORY_VIRTUAL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_CLASS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_DERIVED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_NOT_SHAREABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_PACKAGE_AWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_SANDBOX_DISABLED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_SANDBOX_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_SOFT_RESET_REQUIRED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DRIVER_XPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_CATEGORY_3D: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_CATEGORY_ALL: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_CONNECTIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_CONTAINER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_EXPAND: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_FAVORITE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_HIDE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON1: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON2: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON3: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON4: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON5: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON6: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON7: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICON8: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_ICONMASK: u32 = 16711680u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_LOCAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_NAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_NETWORK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_REMOTE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ENUM_SHARED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_INFORMATION: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_JAM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_OUTOFPAPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_OUTOFTONER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_SEVERE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ERROR_WARNING: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_ADD_CONNECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_ADD_CONNECTION_NO_UI: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_ATTRIBUTES_CHANGED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_CACHE_DELETE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_CACHE_REFRESH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_CONFIGURATION_CHANGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_CONFIGURATION_UPDATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_DELETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_DELETE_CONNECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_DELETE_CONNECTION_NO_UI: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_FLAG_NO_UI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EVENT_INITIALIZE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EXTENSION_DETAILEDREASON_PRINTER_STATUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d5a1704_dfd1_4181_8eee_815c86edad31);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EXTENSION_REASON_DRIVER_EVENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23bb1328_63de_4293_915b_a6a23d929acb);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EXTENSION_REASON_PRINT_PREFERENCES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec8f261f_267c_469f_b5d6_3933023c29cc);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_CATEGORY_3D: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_CATEGORY_ALL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_ATTRIBUTES: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_AVERAGE_PPM: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_BRANCH_OFFICE_PRINTING: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_BYTES_PRINTED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_CJOBS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_COMMENT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_DATATYPE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_DEFAULT_PRIORITY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_DEVMODE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_DRIVER_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_FRIENDLY_NAME: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_LOCATION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_OBJECT_GUID: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PAGES_PRINTED: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PARAMETERS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PORT_NAME: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PRINTER_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PRINT_PROCESSOR: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_PRIORITY: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_SECURITY_DESCRIPTOR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_SEPFILE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_SERVER_NAME: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_SHARE_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_START_TIME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_STATUS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_STATUS_STRING: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_TOTAL_BYTES: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_TOTAL_PAGES: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_FIELD_UNTIL_TIME: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_INFO_DATA_COMPACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_INFO_DISCARDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_OPTIONS_REFRESH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_STATUS_ENDPOINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_STATUS_INFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_STATUS_POLL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_NOTIFY_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_OEMINTF_VERSION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_BUSY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_DOOR_OPEN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_DRIVER_UPDATE_NEEDED: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_INITIALIZING: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_IO_ACTIVE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_MANUAL_FEED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_NOT_AVAILABLE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_NO_TONER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_OFFLINE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_OUTPUT_BIN_FULL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_OUT_OF_MEMORY: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PAGE_PUNT: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PAPER_JAM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PAPER_OUT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PAPER_PROBLEM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PAUSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PENDING_DELETION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_POWER_SAVE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PRINTING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_PROCESSING: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_SERVER_OFFLINE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_SERVER_UNKNOWN: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_TONER_LOW: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_USER_INTERVENTION: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_WAITING: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STATUS_WARMING_UP: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_APP_BIDI_NOTIFY_CHANNEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2abad223_b994_4aca_82fc_4571b1b585ac);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_PORT_MONITOR_NOTIFY_CHANNEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25df3b0e_74a9_47f5_80ce_79b4b1eb5c58);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_INFO_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_BEFORE_INIT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_DESTROY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_GET_ICON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_GET_INFO_HEADER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_INIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROPSHEETUI_REASON_SET_RESULT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROTOCOL_LPR_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROTOCOL_RAWTCP_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PROTOCOL_UNKNOWN_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_DEFTITLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_EXACT_PTITLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_NOAPPLYNOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_OBSOLETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_PROPTITLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIHDRF_USEHICON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIINFO_UNICODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_DLL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_GROUP_PARENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_HPROPSHEETPAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_PCOMPROPSHEETUI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_PFNPROPSHEETUI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PSUIPAGEINSERT_PROPSHEETPAGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PUSHBUTTON_TYPE_CALLBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PUSHBUTTON_TYPE_DLGPROC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PUSHBUTTON_TYPE_HTCLRADJ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PUSHBUTTON_TYPE_HTSETUP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaAsyncOperation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43b2f83d_10f2_48ab_831b_55fdbdbd34a4);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrinterExtensionManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x331b60da_9e90_4dd0_9c84_eac4e659b61f);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrinterQueue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb54c230_798c_4c9e_b461_29fad04039b1);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrinterQueueView: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb54c231_798c_4c9e_b461_29fad04039b1);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const QCP_DEVICEPROFILE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const QCP_PROFILEDISK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const QCP_PROFILEMEMORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const QCP_SOURCEPROFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const RAWTCP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const REVERSE_PAGES_FOR_REVERSE_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const REVERSE_PRINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const RIGHT_THEN_DOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ROUTER_STOP_ROUTING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ROUTER_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ROUTER_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_NOTIFY_FIELD_PRINT_DRIVER_ISOLATION_GROUP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_NOTIFY_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SETOPTIONS_FLAG_KEEP_CONFLICT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SETOPTIONS_FLAG_RESOLVE_CONFLICT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SETOPTIONS_RESULT_CONFLICT_REMAINED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SETOPTIONS_RESULT_CONFLICT_RESOLVED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SETOPTIONS_RESULT_NO_CONFLICT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_ASSET_NUMBER: ::windows::core::PCWSTR = ::windows::w!("assetNumber");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_BYTES_PER_MINUTE: ::windows::core::PCWSTR = ::windows::w!("bytesPerMinute");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_DESCRIPTION: ::windows::core::PCWSTR = ::windows::w!("description");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_DRIVER_KEY: ::windows::core::PCWSTR = ::windows::w!("DsDriver");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_DRIVER_NAME: ::windows::core::PCWSTR = ::windows::w!("driverName");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_DRIVER_VERSION: ::windows::core::PCWSTR = ::windows::w!("driverVersion");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_FLAGS: ::windows::core::PCWSTR = ::windows::w!("flags");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_LOCATION: ::windows::core::PCWSTR = ::windows::w!("location");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PORT_NAME: ::windows::core::PCWSTR = ::windows::w!("portName");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINTER_CLASS: ::windows::core::PCWSTR = ::windows::w!("printQueue");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINTER_LOCATIONS: ::windows::core::PCWSTR = ::windows::w!("printerLocations");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINTER_MODEL: ::windows::core::PCWSTR = ::windows::w!("printerModel");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINTER_NAME: ::windows::core::PCWSTR = ::windows::w!("printerName");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINTER_NAME_ALIASES: ::windows::core::PCWSTR = ::windows::w!("printerNameAliases");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_ATTRIBUTES: ::windows::core::PCWSTR = ::windows::w!("printAttributes");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_BIN_NAMES: ::windows::core::PCWSTR = ::windows::w!("printBinNames");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_COLLATE: ::windows::core::PCWSTR = ::windows::w!("printCollate");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_COLOR: ::windows::core::PCWSTR = ::windows::w!("printColor");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_DUPLEX_SUPPORTED: ::windows::core::PCWSTR = ::windows::w!("printDuplexSupported");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_END_TIME: ::windows::core::PCWSTR = ::windows::w!("printEndTime");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_KEEP_PRINTED_JOBS: ::windows::core::PCWSTR = ::windows::w!("printKeepPrintedJobs");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_LANGUAGE: ::windows::core::PCWSTR = ::windows::w!("printLanguage");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MAC_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("printMACAddress");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MAX_RESOLUTION_SUPPORTED: ::windows::core::PCWSTR = ::windows::w!("printMaxResolutionSupported");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MAX_X_EXTENT: ::windows::core::PCWSTR = ::windows::w!("printMaxXExtent");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MAX_Y_EXTENT: ::windows::core::PCWSTR = ::windows::w!("printMaxYExtent");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MEDIA_READY: ::windows::core::PCWSTR = ::windows::w!("printMediaReady");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MEDIA_SUPPORTED: ::windows::core::PCWSTR = ::windows::w!("printMediaSupported");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MEMORY: ::windows::core::PCWSTR = ::windows::w!("printMemory");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MIN_X_EXTENT: ::windows::core::PCWSTR = ::windows::w!("printMinXExtent");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_MIN_Y_EXTENT: ::windows::core::PCWSTR = ::windows::w!("printMinYExtent");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_NETWORK_ADDRESS: ::windows::core::PCWSTR = ::windows::w!("printNetworkAddress");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_NOTIFY: ::windows::core::PCWSTR = ::windows::w!("printNotify");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_NUMBER_UP: ::windows::core::PCWSTR = ::windows::w!("printNumberUp");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_ORIENTATIONS_SUPPORTED: ::windows::core::PCWSTR = ::windows::w!("printOrientationsSupported");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_OWNER: ::windows::core::PCWSTR = ::windows::w!("printOwner");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_PAGES_PER_MINUTE: ::windows::core::PCWSTR = ::windows::w!("printPagesPerMinute");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_RATE: ::windows::core::PCWSTR = ::windows::w!("printRate");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_RATE_UNIT: ::windows::core::PCWSTR = ::windows::w!("printRateUnit");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_SEPARATOR_FILE: ::windows::core::PCWSTR = ::windows::w!("printSeparatorFile");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_SHARE_NAME: ::windows::core::PCWSTR = ::windows::w!("printShareName");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_SPOOLING: ::windows::core::PCWSTR = ::windows::w!("printSpooling");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_STAPLING_SUPPORTED: ::windows::core::PCWSTR = ::windows::w!("printStaplingSupported");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_START_TIME: ::windows::core::PCWSTR = ::windows::w!("printStartTime");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRINT_STATUS: ::windows::core::PCWSTR = ::windows::w!("printStatus");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_PRIORITY: ::windows::core::PCWSTR = ::windows::w!("priority");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_SERVER_NAME: ::windows::core::PCWSTR = ::windows::w!("serverName");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_SHORT_SERVER_NAME: ::windows::core::PCWSTR = ::windows::w!("shortServerName");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_SPOOLER_KEY: ::windows::core::PCWSTR = ::windows::w!("DsSpooler");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_UNC_NAME: ::windows::core::PCWSTR = ::windows::w!("uNCName");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_URL: ::windows::core::PCWSTR = ::windows::w!("url");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_USER_KEY: ::windows::core::PCWSTR = ::windows::w!("DsUser");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLDS_VERSION_NUMBER: ::windows::core::PCWSTR = ::windows::w!("versionNumber");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLPRINTER_USER_MODE_PRINTER_DRIVER: ::windows::core::PCWSTR = ::windows::w!("SPLUserModePrinterDriver");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_ALLOW_USER_MANAGEFORMS: ::windows::core::PCWSTR = ::windows::w!("AllowUserManageForms");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_ARCHITECTURE: ::windows::core::PCWSTR = ::windows::w!("Architecture");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_BEEP_ENABLED: ::windows::core::PCWSTR = ::windows::w!("BeepEnabled");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_DEFAULT_SPOOL_DIRECTORY: ::windows::core::PCWSTR = ::windows::w!("DefaultSpoolDirectory");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_DNS_MACHINE_NAME: ::windows::core::PCWSTR = ::windows::w!("DNSMachineName");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_DS_PRESENT: ::windows::core::PCWSTR = ::windows::w!("DsPresent");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_DS_PRESENT_FOR_USER: ::windows::core::PCWSTR = ::windows::w!("DsPresentForUser");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_EVENT_LOG: ::windows::core::PCWSTR = ::windows::w!("EventLog");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_MAJOR_VERSION: ::windows::core::PCWSTR = ::windows::w!("MajorVersion");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_MINOR_VERSION: ::windows::core::PCWSTR = ::windows::w!("MinorVersion");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_NET_POPUP: ::windows::core::PCWSTR = ::windows::w!("NetPopup");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_NET_POPUP_TO_COMPUTER: ::windows::core::PCWSTR = ::windows::w!("NetPopupToComputer");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_OS_VERSION: ::windows::core::PCWSTR = ::windows::w!("OSVersion");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_OS_VERSIONEX: ::windows::core::PCWSTR = ::windows::w!("OSVersionEx");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PORT_THREAD_PRIORITY: ::windows::core::PCWSTR = ::windows::w!("PortThreadPriority");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PORT_THREAD_PRIORITY_DEFAULT: ::windows::core::PCWSTR = ::windows::w!("PortThreadPriorityDefault");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_EXECUTION_POLICY: ::windows::core::PCWSTR = ::windows::w!("PrintDriverIsolationExecutionPolicy");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_GROUPS: ::windows::core::PCWSTR = ::windows::w!("PrintDriverIsolationGroups");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_IDLE_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("PrintDriverIsolationIdleTimeout");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_MAX_OBJECTS_BEFORE_RECYCLE: ::windows::core::PCWSTR = ::windows::w!("PrintDriverIsolationMaxobjsBeforeRecycle");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_OVERRIDE_POLICY: ::windows::core::PCWSTR = ::windows::w!("PrintDriverIsolationOverrideCompat");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_DRIVER_ISOLATION_TIME_BEFORE_RECYCLE: ::windows::core::PCWSTR = ::windows::w!("PrintDriverIsolationTimeBeforeRecycle");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_PRINT_QUEUE_V4_DRIVER_DIRECTORY: ::windows::core::PCWSTR = ::windows::w!("PrintQueueV4DriverDirectory");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_REMOTE_FAX: ::windows::core::PCWSTR = ::windows::w!("RemoteFax");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_RESTART_JOB_ON_POOL_ENABLED: ::windows::core::PCWSTR = ::windows::w!("RestartJobOnPoolEnabled");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_RESTART_JOB_ON_POOL_ERROR: ::windows::core::PCWSTR = ::windows::w!("RestartJobOnPoolError");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_RETRY_POPUP: ::windows::core::PCWSTR = ::windows::w!("RetryPopup");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_SCHEDULER_THREAD_PRIORITY: ::windows::core::PCWSTR = ::windows::w!("SchedulerThreadPriority");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_SCHEDULER_THREAD_PRIORITY_DEFAULT: ::windows::core::PCWSTR = ::windows::w!("SchedulerThreadPriorityDefault");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPLREG_WEBSHAREMGMT: ::windows::core::PCWSTR = ::windows::w!("WebShareMgmt");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPOOL_FILE_PERSISTENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SPOOL_FILE_TEMPORARY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SR_OWNER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SR_OWNER_PARENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SSP_STDPAGE1: u32 = 10001u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SSP_STDPAGE2: u32 = 10002u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SSP_TVPAGE: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const STRING_LANGPAIR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const STRING_MUIDLL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const STRING_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const S_CONFLICT_RESOLVED: u32 = 262146u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const S_DEVCAP_OUTPUT_FULL_REPLACEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(318465i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const S_NO_CONFLICT: u32 = 262145u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TTDOWNLOAD_BITMAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TTDOWNLOAD_DONTCARE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TTDOWNLOAD_GRAPHICS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TTDOWNLOAD_TTOUTLINE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_2STATES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_3STATES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_CHKBOX: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_COMBOBOX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_EDITBOX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_LISTBOX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_NSTATES_EX: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_PUSHBUTTON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_SCROLLBAR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_TRACKBAR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TVOT_UDARROW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TYPE_GLYPHHANDLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TYPE_GLYPHID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TYPE_TRANSDATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const TYPE_UNICODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFF_VERSION_NUMBER: u32 = 65537u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFM_CART: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFM_SCALABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFM_SOFT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTDOWNLOAD_BITMAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTDOWNLOAD_TTOUTLINE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTFONT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTOUTLINE_BOLD_SIM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTOUTLINE_ITALIC_SIM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTOUTLINE_VERTICAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFOFLAG_TTSUBSTITUTED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_FONTOBJ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_GLYPHBITMAP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_GLYPHSTRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_GLYPHWIDTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_MEMORY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UFO_GETINFO_STDVARIABLE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UNIFM_VERSION_1_0: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UNI_GLYPHSETDATA_VERSION_1_0: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UNKNOWN_PROTOCOL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UPDP_CHECK_DRIVERSTORE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UPDP_SILENT_UPLOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UPDP_UPLOAD_ALWAYS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const USBPRINT_IOCTL_INDEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const USB_PRINTER_INTERFACE_CLASSIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const USB_PRINTER_INTERFACE_DUAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const USB_PRINTER_INTERFACE_IPP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const WM_FI_FILENAME: u32 = 900u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_DRIVER_PROPERTY_BAG: ::windows::core::PCWSTR = ::windows::w!("DriverPropertyBag");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_JOB_ID: ::windows::core::PCWSTR = ::windows::w!("PrintJobId");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_JOB_LEVEL_PRINTTICKET: ::windows::core::PCWSTR = ::windows::w!("JobPrintTicket");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_MERGED_DATAFILE_PATH: ::windows::core::PCWSTR = ::windows::w!("MergedDataFilePath");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_MS_CONTENT_TYPE: ::windows::core::PCWSTR = ::windows::w!("DriverMultiContentType");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_MS_CONTENT_TYPE_OPENXPS: ::windows::core::PCWSTR = ::windows::w!("OpenXPS");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_MS_CONTENT_TYPE_XPS: ::windows::core::PCWSTR = ::windows::w!("XPS");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_OUTPUT_FILE: ::windows::core::PCWSTR = ::windows::w!("PrintOutputFileName");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_PRINTDEVICECAPABILITIES: ::windows::core::PCWSTR = ::windows::w!("PrintDeviceCapabilities");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_PRINTER_HANDLE: ::windows::core::PCWSTR = ::windows::w!("PrinterHandle");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_PRINTER_NAME: ::windows::core::PCWSTR = ::windows::w!("PrinterName");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_PRINT_CLASS_FACTORY: ::windows::core::PCWSTR = ::windows::w!("PrintClassFactory");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_PROGRESS_REPORT: ::windows::core::PCWSTR = ::windows::w!("ProgressReport");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_QUEUE_PROPERTY_BAG: ::windows::core::PCWSTR = ::windows::w!("QueuePropertyBag");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_RESOURCE_DLL_PATHS: ::windows::core::PCWSTR = ::windows::w!("ResourceDLLPaths");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_USER_PRINT_TICKET: ::windows::core::PCWSTR = ::windows::w!("PerUserPrintTicket");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPS_FP_USER_TOKEN: ::windows::core::PCWSTR = ::windows::w!("UserSecurityToken");
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BIDI_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_NULL: BIDI_TYPE = BIDI_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_INT: BIDI_TYPE = BIDI_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_FLOAT: BIDI_TYPE = BIDI_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_BOOL: BIDI_TYPE = BIDI_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_STRING: BIDI_TYPE = BIDI_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_TEXT: BIDI_TYPE = BIDI_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_ENUM: BIDI_TYPE = BIDI_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const BIDI_BLOB: BIDI_TYPE = BIDI_TYPE(7i32);
impl ::core::marker::Copy for BIDI_TYPE {}
impl ::core::clone::Clone for BIDI_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BIDI_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EATTRIBUTE_DATATYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_UNKNOWN: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_BOOL: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_INT: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_LONG: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_DWORD: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_ASCII: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_UNICODE: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_BINARY: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_SIZE: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_RECT: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kADT_CUSTOMSIZEPARAMS: EATTRIBUTE_DATATYPE = EATTRIBUTE_DATATYPE(10i32);
impl ::core::marker::Copy for EATTRIBUTE_DATATYPE {}
impl ::core::clone::Clone for EATTRIBUTE_DATATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EATTRIBUTE_DATATYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EBranchOfficeJobEventType(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kInvalidJobState: EBranchOfficeJobEventType = EBranchOfficeJobEventType(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kLogJobPrinted: EBranchOfficeJobEventType = EBranchOfficeJobEventType(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kLogJobRendered: EBranchOfficeJobEventType = EBranchOfficeJobEventType(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kLogJobError: EBranchOfficeJobEventType = EBranchOfficeJobEventType(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kLogJobPipelineError: EBranchOfficeJobEventType = EBranchOfficeJobEventType(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kLogOfflineFileFull: EBranchOfficeJobEventType = EBranchOfficeJobEventType(5i32);
impl ::core::marker::Copy for EBranchOfficeJobEventType {}
impl ::core::clone::Clone for EBranchOfficeJobEventType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EBranchOfficeJobEventType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EPrintPropertyType(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeString: EPrintPropertyType = EPrintPropertyType(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeInt32: EPrintPropertyType = EPrintPropertyType(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeInt64: EPrintPropertyType = EPrintPropertyType(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeByte: EPrintPropertyType = EPrintPropertyType(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeTime: EPrintPropertyType = EPrintPropertyType(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeDevMode: EPrintPropertyType = EPrintPropertyType(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeSD: EPrintPropertyType = EPrintPropertyType(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeNotificationReply: EPrintPropertyType = EPrintPropertyType(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeNotificationOptions: EPrintPropertyType = EPrintPropertyType(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPropertyTypeBuffer: EPrintPropertyType = EPrintPropertyType(10i32);
impl ::core::marker::Copy for EPrintPropertyType {}
impl ::core::clone::Clone for EPrintPropertyType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EPrintPropertyType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EPrintXPSJobOperation(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kJobProduction: EPrintXPSJobOperation = EPrintXPSJobOperation(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kJobConsumption: EPrintXPSJobOperation = EPrintXPSJobOperation(2i32);
impl ::core::marker::Copy for EPrintXPSJobOperation {}
impl ::core::clone::Clone for EPrintXPSJobOperation {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EPrintXPSJobOperation {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EPrintXPSJobProgress(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kAddingDocumentSequence: EPrintXPSJobProgress = EPrintXPSJobProgress(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kDocumentSequenceAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kAddingFixedDocument: EPrintXPSJobProgress = EPrintXPSJobProgress(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kFixedDocumentAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kAddingFixedPage: EPrintXPSJobProgress = EPrintXPSJobProgress(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kFixedPageAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kResourceAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kFontAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kImageAdded: EPrintXPSJobProgress = EPrintXPSJobProgress(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kXpsDocumentCommitted: EPrintXPSJobProgress = EPrintXPSJobProgress(9i32);
impl ::core::marker::Copy for EPrintXPSJobProgress {}
impl ::core::clone::Clone for EPrintXPSJobProgress {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EPrintXPSJobProgress {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXpsCompressionOptions(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Compression_NotCompressed: EXpsCompressionOptions = EXpsCompressionOptions(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Compression_Normal: EXpsCompressionOptions = EXpsCompressionOptions(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Compression_Small: EXpsCompressionOptions = EXpsCompressionOptions(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Compression_Fast: EXpsCompressionOptions = EXpsCompressionOptions(3i32);
impl ::core::marker::Copy for EXpsCompressionOptions {}
impl ::core::clone::Clone for EXpsCompressionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXpsCompressionOptions {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXpsFontOptions(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Font_Normal: EXpsFontOptions = EXpsFontOptions(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Font_Obfusticate: EXpsFontOptions = EXpsFontOptions(1i32);
impl ::core::marker::Copy for EXpsFontOptions {}
impl ::core::clone::Clone for EXpsFontOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXpsFontOptions {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXpsFontRestriction(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Xps_Restricted_Font_Installable: EXpsFontRestriction = EXpsFontRestriction(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Xps_Restricted_Font_NoEmbedding: EXpsFontRestriction = EXpsFontRestriction(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Xps_Restricted_Font_PreviewPrint: EXpsFontRestriction = EXpsFontRestriction(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const Xps_Restricted_Font_Editable: EXpsFontRestriction = EXpsFontRestriction(8i32);
impl ::core::marker::Copy for EXpsFontRestriction {}
impl ::core::clone::Clone for EXpsFontRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXpsFontRestriction {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXpsJobConsumption(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XpsJob_DocumentSequenceAdded: EXpsJobConsumption = EXpsJobConsumption(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XpsJob_FixedDocumentAdded: EXpsJobConsumption = EXpsJobConsumption(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XpsJob_FixedPageAdded: EXpsJobConsumption = EXpsJobConsumption(2i32);
impl ::core::marker::Copy for EXpsJobConsumption {}
impl ::core::clone::Clone for EXpsJobConsumption {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXpsJobConsumption {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MXDC_IMAGE_TYPE_ENUMS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_IMAGETYPE_JPEGHIGH_COMPRESSION: MXDC_IMAGE_TYPE_ENUMS = MXDC_IMAGE_TYPE_ENUMS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_IMAGETYPE_JPEGMEDIUM_COMPRESSION: MXDC_IMAGE_TYPE_ENUMS = MXDC_IMAGE_TYPE_ENUMS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_IMAGETYPE_JPEGLOW_COMPRESSION: MXDC_IMAGE_TYPE_ENUMS = MXDC_IMAGE_TYPE_ENUMS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_IMAGETYPE_PNG: MXDC_IMAGE_TYPE_ENUMS = MXDC_IMAGE_TYPE_ENUMS(4i32);
impl ::core::marker::Copy for MXDC_IMAGE_TYPE_ENUMS {}
impl ::core::clone::Clone for MXDC_IMAGE_TYPE_ENUMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_IMAGE_TYPE_ENUMS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MXDC_LANDSCAPE_ROTATION_ENUMS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_LANDSCAPE_ROTATE_COUNTERCLOCKWISE_90_DEGREES: MXDC_LANDSCAPE_ROTATION_ENUMS = MXDC_LANDSCAPE_ROTATION_ENUMS(90i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_LANDSCAPE_ROTATE_NONE: MXDC_LANDSCAPE_ROTATION_ENUMS = MXDC_LANDSCAPE_ROTATION_ENUMS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_LANDSCAPE_ROTATE_COUNTERCLOCKWISE_270_DEGREES: MXDC_LANDSCAPE_ROTATION_ENUMS = MXDC_LANDSCAPE_ROTATION_ENUMS(-90i32);
impl ::core::marker::Copy for MXDC_LANDSCAPE_ROTATION_ENUMS {}
impl ::core::clone::Clone for MXDC_LANDSCAPE_ROTATION_ENUMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_LANDSCAPE_ROTATION_ENUMS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MXDC_S0_PAGE_ENUMS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_TTF: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_JPEG: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_PNG: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_TIFF: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_WDP: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_DICTIONARY: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_ICC_PROFILE: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_JPEG_THUMBNAIL: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_PNG_THUMBNAIL: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MXDC_RESOURCE_MAX: MXDC_S0_PAGE_ENUMS = MXDC_S0_PAGE_ENUMS(9i32);
impl ::core::marker::Copy for MXDC_S0_PAGE_ENUMS {}
impl ::core::clone::Clone for MXDC_S0_PAGE_ENUMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_S0_PAGE_ENUMS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NOTIFICATION_CALLBACK_COMMANDS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_COMMAND_NOTIFY: NOTIFICATION_CALLBACK_COMMANDS = NOTIFICATION_CALLBACK_COMMANDS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_COMMAND_CONTEXT_ACQUIRE: NOTIFICATION_CALLBACK_COMMANDS = NOTIFICATION_CALLBACK_COMMANDS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_COMMAND_CONTEXT_RELEASE: NOTIFICATION_CALLBACK_COMMANDS = NOTIFICATION_CALLBACK_COMMANDS(2i32);
impl ::core::marker::Copy for NOTIFICATION_CALLBACK_COMMANDS {}
impl ::core::clone::Clone for NOTIFICATION_CALLBACK_COMMANDS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NOTIFICATION_CALLBACK_COMMANDS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NOTIFICATION_CONFIG_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_CONFIG_CREATE_EVENT: NOTIFICATION_CONFIG_FLAGS = NOTIFICATION_CONFIG_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_CONFIG_REGISTER_CALLBACK: NOTIFICATION_CONFIG_FLAGS = NOTIFICATION_CONFIG_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_CONFIG_EVENT_TRIGGER: NOTIFICATION_CONFIG_FLAGS = NOTIFICATION_CONFIG_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOTIFICATION_CONFIG_ASYNC_CHANNEL: NOTIFICATION_CONFIG_FLAGS = NOTIFICATION_CONFIG_FLAGS(8i32);
impl ::core::marker::Copy for NOTIFICATION_CONFIG_FLAGS {}
impl ::core::clone::Clone for NOTIFICATION_CONFIG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NOTIFICATION_CONFIG_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRINTER_ACCESS_RIGHTS(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ALL_ACCESS: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(983052u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_READ: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131080u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_WRITE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131080u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_EXECUTE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131080u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_ALL_ACCESS: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(983043u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_READ: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131074u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_WRITE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131075u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_EXECUTE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131074u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_DELETE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(65536u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_READ_CONTROL: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_WRITE_DAC: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(262144u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_WRITE_OWNER: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(524288u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_SYNCHRONIZE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(1048576u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STANDARD_RIGHTS_REQUIRED: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(983040u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STANDARD_RIGHTS_READ: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STANDARD_RIGHTS_WRITE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_STANDARD_RIGHTS_EXECUTE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_ACCESS_ADMINISTER: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const SERVER_ACCESS_ENUMERATE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ACCESS_ADMINISTER: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ACCESS_USE: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_ACCESS_MANAGE_LIMITED: PRINTER_ACCESS_RIGHTS = PRINTER_ACCESS_RIGHTS(64u32);
impl ::core::marker::Copy for PRINTER_ACCESS_RIGHTS {}
impl ::core::clone::Clone for PRINTER_ACCESS_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_ACCESS_RIGHTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRINTER_OPTION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_OPTION_NO_CACHE: PRINTER_OPTION_FLAGS = PRINTER_OPTION_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_OPTION_CACHE: PRINTER_OPTION_FLAGS = PRINTER_OPTION_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_OPTION_CLIENT_CHANGE: PRINTER_OPTION_FLAGS = PRINTER_OPTION_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINTER_OPTION_NO_CLIENT_DATA: PRINTER_OPTION_FLAGS = PRINTER_OPTION_FLAGS(8i32);
impl ::core::marker::Copy for PRINTER_OPTION_FLAGS {}
impl ::core::clone::Clone for PRINTER_OPTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_OPTION_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRINT_EXECUTION_CONTEXT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_EXECUTION_CONTEXT_APPLICATION: PRINT_EXECUTION_CONTEXT = PRINT_EXECUTION_CONTEXT(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_SERVICE: PRINT_EXECUTION_CONTEXT = PRINT_EXECUTION_CONTEXT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_EXECUTION_CONTEXT_SPOOLER_ISOLATION_HOST: PRINT_EXECUTION_CONTEXT = PRINT_EXECUTION_CONTEXT(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_EXECUTION_CONTEXT_FILTER_PIPELINE: PRINT_EXECUTION_CONTEXT = PRINT_EXECUTION_CONTEXT(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PRINT_EXECUTION_CONTEXT_WOW64: PRINT_EXECUTION_CONTEXT = PRINT_EXECUTION_CONTEXT(4i32);
impl ::core::marker::Copy for PRINT_EXECUTION_CONTEXT {}
impl ::core::clone::Clone for PRINT_EXECUTION_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINT_EXECUTION_CONTEXT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PageCountType(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const FinalPageCount: PageCountType = PageCountType(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const IntermediatePageCount: PageCountType = PageCountType(1i32);
impl ::core::marker::Copy for PageCountType {}
impl ::core::clone::Clone for PageCountType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PageCountType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintAsyncNotifyConversationStyle(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kBiDirectional: PrintAsyncNotifyConversationStyle = PrintAsyncNotifyConversationStyle(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kUniDirectional: PrintAsyncNotifyConversationStyle = PrintAsyncNotifyConversationStyle(1i32);
impl ::core::marker::Copy for PrintAsyncNotifyConversationStyle {}
impl ::core::clone::Clone for PrintAsyncNotifyConversationStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintAsyncNotifyConversationStyle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintAsyncNotifyError(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_CLOSED_BY_SERVER: PrintAsyncNotifyError = PrintAsyncNotifyError(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_CLOSED_BY_ANOTHER_LISTENER: PrintAsyncNotifyError = PrintAsyncNotifyError(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_CLOSED_BY_SAME_LISTENER: PrintAsyncNotifyError = PrintAsyncNotifyError(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_RELEASED_BY_LISTENER: PrintAsyncNotifyError = PrintAsyncNotifyError(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const UNIRECTIONAL_NOTIFICATION_LOST: PrintAsyncNotifyError = PrintAsyncNotifyError(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ASYNC_NOTIFICATION_FAILURE: PrintAsyncNotifyError = PrintAsyncNotifyError(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NO_LISTENERS: PrintAsyncNotifyError = PrintAsyncNotifyError(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_ALREADY_CLOSED: PrintAsyncNotifyError = PrintAsyncNotifyError(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_ALREADY_OPENED: PrintAsyncNotifyError = PrintAsyncNotifyError(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_WAITING_FOR_CLIENT_NOTIFICATION: PrintAsyncNotifyError = PrintAsyncNotifyError(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_NOT_OPENED: PrintAsyncNotifyError = PrintAsyncNotifyError(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ASYNC_CALL_ALREADY_PARKED: PrintAsyncNotifyError = PrintAsyncNotifyError(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const NOT_REGISTERED: PrintAsyncNotifyError = PrintAsyncNotifyError(13i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ALREADY_UNREGISTERED: PrintAsyncNotifyError = PrintAsyncNotifyError(14i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ALREADY_REGISTERED: PrintAsyncNotifyError = PrintAsyncNotifyError(15i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const CHANNEL_ACQUIRED: PrintAsyncNotifyError = PrintAsyncNotifyError(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const ASYNC_CALL_IN_PROGRESS: PrintAsyncNotifyError = PrintAsyncNotifyError(17i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_NOTIFICATION_SIZE_EXCEEDED: PrintAsyncNotifyError = PrintAsyncNotifyError(18i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INTERNAL_NOTIFICATION_QUEUE_IS_FULL: PrintAsyncNotifyError = PrintAsyncNotifyError(19i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const INVALID_NOTIFICATION_TYPE: PrintAsyncNotifyError = PrintAsyncNotifyError(20i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_REGISTRATION_COUNT_EXCEEDED: PrintAsyncNotifyError = PrintAsyncNotifyError(21i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const MAX_CHANNEL_COUNT_EXCEEDED: PrintAsyncNotifyError = PrintAsyncNotifyError(22i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const LOCAL_ONLY_REGISTRATION: PrintAsyncNotifyError = PrintAsyncNotifyError(23i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const REMOTE_ONLY_REGISTRATION: PrintAsyncNotifyError = PrintAsyncNotifyError(24i32);
impl ::core::marker::Copy for PrintAsyncNotifyError {}
impl ::core::clone::Clone for PrintAsyncNotifyError {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintAsyncNotifyError {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintAsyncNotifyUserFilter(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kPerUser: PrintAsyncNotifyUserFilter = PrintAsyncNotifyUserFilter(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kAllUsers: PrintAsyncNotifyUserFilter = PrintAsyncNotifyUserFilter(1i32);
impl ::core::marker::Copy for PrintAsyncNotifyUserFilter {}
impl ::core::clone::Clone for PrintAsyncNotifyUserFilter {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintAsyncNotifyUserFilter {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintJobStatus(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Paused: PrintJobStatus = PrintJobStatus(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Error: PrintJobStatus = PrintJobStatus(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Deleting: PrintJobStatus = PrintJobStatus(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Spooling: PrintJobStatus = PrintJobStatus(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Printing: PrintJobStatus = PrintJobStatus(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Offline: PrintJobStatus = PrintJobStatus(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_PaperOut: PrintJobStatus = PrintJobStatus(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Printed: PrintJobStatus = PrintJobStatus(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Deleted: PrintJobStatus = PrintJobStatus(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_BlockedDeviceQueue: PrintJobStatus = PrintJobStatus(512i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_UserIntervention: PrintJobStatus = PrintJobStatus(1024i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Restarted: PrintJobStatus = PrintJobStatus(2048i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Complete: PrintJobStatus = PrintJobStatus(4096i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintJobStatus_Retained: PrintJobStatus = PrintJobStatus(8192i32);
impl ::core::marker::Copy for PrintJobStatus {}
impl ::core::clone::Clone for PrintJobStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintJobStatus {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintSchemaConstrainedSetting(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaConstrainedSetting_None: PrintSchemaConstrainedSetting = PrintSchemaConstrainedSetting(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaConstrainedSetting_PrintTicket: PrintSchemaConstrainedSetting = PrintSchemaConstrainedSetting(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaConstrainedSetting_Admin: PrintSchemaConstrainedSetting = PrintSchemaConstrainedSetting(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaConstrainedSetting_Device: PrintSchemaConstrainedSetting = PrintSchemaConstrainedSetting(3i32);
impl ::core::marker::Copy for PrintSchemaConstrainedSetting {}
impl ::core::clone::Clone for PrintSchemaConstrainedSetting {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintSchemaConstrainedSetting {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintSchemaParameterDataType(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaParameterDataType_Integer: PrintSchemaParameterDataType = PrintSchemaParameterDataType(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaParameterDataType_NumericString: PrintSchemaParameterDataType = PrintSchemaParameterDataType(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaParameterDataType_String: PrintSchemaParameterDataType = PrintSchemaParameterDataType(2i32);
impl ::core::marker::Copy for PrintSchemaParameterDataType {}
impl ::core::clone::Clone for PrintSchemaParameterDataType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintSchemaParameterDataType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintSchemaSelectionType(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaSelectionType_PickOne: PrintSchemaSelectionType = PrintSchemaSelectionType(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PrintSchemaSelectionType_PickMany: PrintSchemaSelectionType = PrintSchemaSelectionType(1i32);
impl ::core::marker::Copy for PrintSchemaSelectionType {}
impl ::core::clone::Clone for PrintSchemaSelectionType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintSchemaSelectionType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SHIMOPTS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PTSHIM_DEFAULT: SHIMOPTS = SHIMOPTS(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const PTSHIM_NOSNAPSHOT: SHIMOPTS = SHIMOPTS(1i32);
impl ::core::marker::Copy for SHIMOPTS {}
impl ::core::clone::Clone for SHIMOPTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SHIMOPTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const kMessageBox: UI_TYPE = UI_TYPE(0i32);
impl ::core::marker::Copy for UI_TYPE {}
impl ::core::clone::Clone for UI_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UI_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPSRAS_BACKGROUND_COLOR(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_BACKGROUND_COLOR_TRANSPARENT: XPSRAS_BACKGROUND_COLOR = XPSRAS_BACKGROUND_COLOR(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_BACKGROUND_COLOR_OPAQUE: XPSRAS_BACKGROUND_COLOR = XPSRAS_BACKGROUND_COLOR(1i32);
impl ::core::marker::Copy for XPSRAS_BACKGROUND_COLOR {}
impl ::core::clone::Clone for XPSRAS_BACKGROUND_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPSRAS_BACKGROUND_COLOR {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPSRAS_PIXEL_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_PIXEL_FORMAT_32BPP_PBGRA_UINT_SRGB: XPSRAS_PIXEL_FORMAT = XPSRAS_PIXEL_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_PIXEL_FORMAT_64BPP_PRGBA_HALF_SCRGB: XPSRAS_PIXEL_FORMAT = XPSRAS_PIXEL_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_PIXEL_FORMAT_128BPP_PRGBA_FLOAT_SCRGB: XPSRAS_PIXEL_FORMAT = XPSRAS_PIXEL_FORMAT(3i32);
impl ::core::marker::Copy for XPSRAS_PIXEL_FORMAT {}
impl ::core::clone::Clone for XPSRAS_PIXEL_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPSRAS_PIXEL_FORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPSRAS_RENDERING_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_RENDERING_MODE_ANTIALIASED: XPSRAS_RENDERING_MODE = XPSRAS_RENDERING_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub const XPSRAS_RENDERING_MODE_ALIASED: XPSRAS_RENDERING_MODE = XPSRAS_RENDERING_MODE(1i32);
impl ::core::marker::Copy for XPSRAS_RENDERING_MODE {}
impl ::core::clone::Clone for XPSRAS_RENDERING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPSRAS_RENDERING_MODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct ADDJOB_INFO_1A {
    pub Path: ::windows::core::PSTR,
    pub JobId: u32,
}
impl ::core::marker::Copy for ADDJOB_INFO_1A {}
impl ::core::clone::Clone for ADDJOB_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADDJOB_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct ADDJOB_INFO_1W {
    pub Path: ::windows::core::PWSTR,
    pub JobId: u32,
}
impl ::core::marker::Copy for ADDJOB_INFO_1W {}
impl ::core::clone::Clone for ADDJOB_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADDJOB_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct ATTRIBUTE_INFO_1 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
}
impl ::core::marker::Copy for ATTRIBUTE_INFO_1 {}
impl ::core::clone::Clone for ATTRIBUTE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTE_INFO_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct ATTRIBUTE_INFO_2 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
    pub dwColorOptimization: u32,
}
impl ::core::marker::Copy for ATTRIBUTE_INFO_2 {}
impl ::core::clone::Clone for ATTRIBUTE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTE_INFO_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct ATTRIBUTE_INFO_3 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
    pub dwColorOptimization: u32,
    pub dmPrintQuality: i16,
    pub dmYResolution: i16,
}
impl ::core::marker::Copy for ATTRIBUTE_INFO_3 {}
impl ::core::clone::Clone for ATTRIBUTE_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTE_INFO_3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct ATTRIBUTE_INFO_4 {
    pub dwJobNumberOfPagesPerSide: u32,
    pub dwDrvNumberOfPagesPerSide: u32,
    pub dwNupBorderFlags: u32,
    pub dwJobPageOrderFlags: u32,
    pub dwDrvPageOrderFlags: u32,
    pub dwJobNumberOfCopies: u32,
    pub dwDrvNumberOfCopies: u32,
    pub dwColorOptimization: u32,
    pub dmPrintQuality: i16,
    pub dmYResolution: i16,
    pub dwDuplexFlags: u32,
    pub dwNupDirection: u32,
    pub dwBookletFlags: u32,
    pub dwScalingPercentX: u32,
    pub dwScalingPercentY: u32,
}
impl ::core::marker::Copy for ATTRIBUTE_INFO_4 {}
impl ::core::clone::Clone for ATTRIBUTE_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTE_INFO_4 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIDI_DATA {
    pub dwBidiType: u32,
    pub u: BIDI_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BIDI_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union BIDI_DATA_0 {
    pub bData: super::super::Foundation::BOOL,
    pub iData: i32,
    pub sData: ::windows::core::PWSTR,
    pub fData: f32,
    pub biData: BINARY_CONTAINER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BIDI_DATA_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIDI_REQUEST_CONTAINER {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [BIDI_REQUEST_DATA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_REQUEST_CONTAINER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_REQUEST_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BIDI_REQUEST_CONTAINER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIDI_REQUEST_DATA {
    pub dwReqNumber: u32,
    pub pSchema: ::windows::core::PWSTR,
    pub data: BIDI_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_REQUEST_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_REQUEST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BIDI_REQUEST_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIDI_RESPONSE_CONTAINER {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [BIDI_RESPONSE_DATA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_RESPONSE_CONTAINER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_RESPONSE_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BIDI_RESPONSE_CONTAINER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIDI_RESPONSE_DATA {
    pub dwResult: u32,
    pub dwReqNumber: u32,
    pub pSchema: ::windows::core::PWSTR,
    pub data: BIDI_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIDI_RESPONSE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIDI_RESPONSE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BIDI_RESPONSE_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BINARY_CONTAINER {
    pub cbBuf: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BINARY_CONTAINER {}
impl ::core::clone::Clone for BINARY_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BINARY_CONTAINER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobData {
    pub eEventType: EBranchOfficeJobEventType,
    pub JobId: u32,
    pub JobInfo: BranchOfficeJobData_0,
}
impl ::core::marker::Copy for BranchOfficeJobData {}
impl ::core::clone::Clone for BranchOfficeJobData {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BranchOfficeJobData {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub union BranchOfficeJobData_0 {
    pub LogJobPrinted: BranchOfficeJobDataPrinted,
    pub LogJobRendered: BranchOfficeJobDataRendered,
    pub LogJobError: BranchOfficeJobDataError,
    pub LogPipelineFailed: BranchOfficeJobDataPipelineFailed,
    pub LogOfflineFileFull: BranchOfficeLogOfflineFileFull,
}
impl ::core::marker::Copy for BranchOfficeJobData_0 {}
impl ::core::clone::Clone for BranchOfficeJobData_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BranchOfficeJobData_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobDataContainer {
    pub cJobDataEntries: u32,
    pub JobData: [BranchOfficeJobData; 1],
}
impl ::core::marker::Copy for BranchOfficeJobDataContainer {}
impl ::core::clone::Clone for BranchOfficeJobDataContainer {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BranchOfficeJobDataContainer {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobDataError {
    pub LastError: u32,
    pub pDocumentName: ::windows::core::PWSTR,
    pub pUserName: ::windows::core::PWSTR,
    pub pPrinterName: ::windows::core::PWSTR,
    pub pDataType: ::windows::core::PWSTR,
    pub TotalSize: i64,
    pub PrintedSize: i64,
    pub TotalPages: u32,
    pub PrintedPages: u32,
    pub pMachineName: ::windows::core::PWSTR,
    pub pJobError: ::windows::core::PWSTR,
    pub pErrorDescription: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for BranchOfficeJobDataError {}
impl ::core::clone::Clone for BranchOfficeJobDataError {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BranchOfficeJobDataError {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobDataPipelineFailed {
    pub pDocumentName: ::windows::core::PWSTR,
    pub pPrinterName: ::windows::core::PWSTR,
    pub pExtraErrorInfo: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for BranchOfficeJobDataPipelineFailed {}
impl ::core::clone::Clone for BranchOfficeJobDataPipelineFailed {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BranchOfficeJobDataPipelineFailed {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobDataPrinted {
    pub Status: u32,
    pub pDocumentName: ::windows::core::PWSTR,
    pub pUserName: ::windows::core::PWSTR,
    pub pMachineName: ::windows::core::PWSTR,
    pub pPrinterName: ::windows::core::PWSTR,
    pub pPortName: ::windows::core::PWSTR,
    pub Size: i64,
    pub TotalPages: u32,
}
impl ::core::marker::Copy for BranchOfficeJobDataPrinted {}
impl ::core::clone::Clone for BranchOfficeJobDataPrinted {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BranchOfficeJobDataPrinted {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeJobDataRendered {
    pub Size: i64,
    pub ICMMethod: u32,
    pub Color: i16,
    pub PrintQuality: i16,
    pub YResolution: i16,
    pub Copies: i16,
    pub TTOption: i16,
}
impl ::core::marker::Copy for BranchOfficeJobDataRendered {}
impl ::core::clone::Clone for BranchOfficeJobDataRendered {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BranchOfficeJobDataRendered {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct BranchOfficeLogOfflineFileFull {
    pub pMachineName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for BranchOfficeLogOfflineFileFull {}
impl ::core::clone::Clone for BranchOfficeLogOfflineFileFull {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BranchOfficeLogOfflineFileFull {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct COMPROPSHEETUI {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::super::Foundation::HINSTANCE,
    pub pCallerName: *mut i8,
    pub UserData: usize,
    pub pHelpFile: *mut i8,
    pub pfnCallBack: _CPSUICALLBACK,
    pub pOptItem: *mut OPTITEM,
    pub pDlgPage: *mut DLGPAGE,
    pub cOptItem: u16,
    pub cDlgPage: u16,
    pub IconID: usize,
    pub pOptItemName: *mut i8,
    pub CallerVersion: u16,
    pub OptItemVersion: u16,
    pub dwReserved: [usize; 4],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for COMPROPSHEETUI {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for COMPROPSHEETUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for COMPROPSHEETUI {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct CONFIG_INFO_DATA_1 {
    pub Reserved: [u8; 128],
    pub dwVersion: u32,
}
impl ::core::marker::Copy for CONFIG_INFO_DATA_1 {}
impl ::core::clone::Clone for CONFIG_INFO_DATA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONFIG_INFO_DATA_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CORE_PRINTER_DRIVERA {
    pub CoreDriverGUID: ::windows::core::GUID,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub szPackageID: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CORE_PRINTER_DRIVERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CORE_PRINTER_DRIVERA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CORE_PRINTER_DRIVERA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CORE_PRINTER_DRIVERW {
    pub CoreDriverGUID: ::windows::core::GUID,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub szPackageID: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CORE_PRINTER_DRIVERW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CORE_PRINTER_DRIVERW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CORE_PRINTER_DRIVERW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CPSUICBPARAM {
    pub cbSize: u16,
    pub Reason: u16,
    pub hDlg: super::super::Foundation::HWND,
    pub pOptItem: *mut OPTITEM,
    pub cOptItem: u16,
    pub Flags: u16,
    pub pCurItem: *mut OPTITEM,
    pub Anonymous: CPSUICBPARAM_0,
    pub UserData: usize,
    pub Result: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CPSUICBPARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CPSUICBPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for CPSUICBPARAM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union CPSUICBPARAM_0 {
    pub OldSel: i32,
    pub pOldSel: *mut i8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CPSUICBPARAM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CPSUICBPARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for CPSUICBPARAM_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct CPSUIDATABLOCK {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for CPSUIDATABLOCK {}
impl ::core::clone::Clone for CPSUIDATABLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CPSUIDATABLOCK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct CUSTOMSIZEPARAM {
    pub dwOrder: i32,
    pub lMinVal: i32,
    pub lMaxVal: i32,
}
impl ::core::marker::Copy for CUSTOMSIZEPARAM {}
impl ::core::clone::Clone for CUSTOMSIZEPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CUSTOMSIZEPARAM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DATATYPES_INFO_1A {
    pub pName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for DATATYPES_INFO_1A {}
impl ::core::clone::Clone for DATATYPES_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DATATYPES_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DATATYPES_INFO_1W {
    pub pName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DATATYPES_INFO_1W {}
impl ::core::clone::Clone for DATATYPES_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DATATYPES_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DATA_HEADER {
    pub dwSignature: u32,
    pub wSize: u16,
    pub wDataID: u16,
    pub dwDataSize: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DATA_HEADER {}
impl ::core::clone::Clone for DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DATA_HEADER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DELETE_PORT_DATA_1 {
    pub psztPortName: [u16; 64],
    pub Reserved: [u8; 98],
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DELETE_PORT_DATA_1 {}
impl ::core::clone::Clone for DELETE_PORT_DATA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DELETE_PORT_DATA_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICEPROPERTYHEADER {
    pub cbSize: u16,
    pub Flags: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pszPrinterName: *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICEPROPERTYHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICEPROPERTYHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVICEPROPERTYHEADER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DEVQUERYPRINT_INFO {
    pub cbSize: u16,
    pub Level: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pszErrorStr: ::windows::core::PWSTR,
    pub cchErrorStr: u32,
    pub cchNeeded: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DEVQUERYPRINT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DEVQUERYPRINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DEVQUERYPRINT_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct DLGPAGE {
    pub cbSize: u16,
    pub Flags: u16,
    pub DlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub pTabName: *mut i8,
    pub IconID: usize,
    pub Anonymous: DLGPAGE_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for DLGPAGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for DLGPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for DLGPAGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union DLGPAGE_0 {
    pub DlgTemplateID: u16,
    pub hDlgTemplate: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for DLGPAGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for DLGPAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for DLGPAGE_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DOCEVENT_CREATEDCPRE {
    pub pszDriver: ::windows::core::PWSTR,
    pub pszDevice: ::windows::core::PWSTR,
    pub pdm: *mut super::Gdi::DEVMODEW,
    pub bIC: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DOCEVENT_CREATEDCPRE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DOCEVENT_CREATEDCPRE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DOCEVENT_CREATEDCPRE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOCEVENT_ESCAPE {
    pub iEscape: i32,
    pub cjInput: i32,
    pub pvInData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DOCEVENT_ESCAPE {}
impl ::core::clone::Clone for DOCEVENT_ESCAPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOCEVENT_ESCAPE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOCEVENT_FILTER {
    pub cbSize: u32,
    pub cElementsAllocated: u32,
    pub cElementsNeeded: u32,
    pub cElementsReturned: u32,
    pub aDocEventCall: [u32; 1],
}
impl ::core::marker::Copy for DOCEVENT_FILTER {}
impl ::core::clone::Clone for DOCEVENT_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOCEVENT_FILTER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DOCUMENTPROPERTYHEADER {
    pub cbSize: u16,
    pub Reserved: u16,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pszPrinterName: *mut i8,
    pub pdmIn: *mut super::Gdi::DEVMODEA,
    pub pdmOut: *mut super::Gdi::DEVMODEA,
    pub cbOut: u32,
    pub fMode: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DOCUMENTPROPERTYHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DOCUMENTPROPERTYHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DOCUMENTPROPERTYHEADER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_1A {
    pub pDocName: ::windows::core::PSTR,
    pub pOutputFile: ::windows::core::PSTR,
    pub pDatatype: ::windows::core::PSTR,
}
impl ::core::marker::Copy for DOC_INFO_1A {}
impl ::core::clone::Clone for DOC_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOC_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_1W {
    pub pDocName: ::windows::core::PWSTR,
    pub pOutputFile: ::windows::core::PWSTR,
    pub pDatatype: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DOC_INFO_1W {}
impl ::core::clone::Clone for DOC_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOC_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_2A {
    pub pDocName: ::windows::core::PSTR,
    pub pOutputFile: ::windows::core::PSTR,
    pub pDatatype: ::windows::core::PSTR,
    pub dwMode: u32,
    pub JobId: u32,
}
impl ::core::marker::Copy for DOC_INFO_2A {}
impl ::core::clone::Clone for DOC_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOC_INFO_2A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_2W {
    pub pDocName: ::windows::core::PWSTR,
    pub pOutputFile: ::windows::core::PWSTR,
    pub pDatatype: ::windows::core::PWSTR,
    pub dwMode: u32,
    pub JobId: u32,
}
impl ::core::marker::Copy for DOC_INFO_2W {}
impl ::core::clone::Clone for DOC_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOC_INFO_2W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_3A {
    pub pDocName: ::windows::core::PSTR,
    pub pOutputFile: ::windows::core::PSTR,
    pub pDatatype: ::windows::core::PSTR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DOC_INFO_3A {}
impl ::core::clone::Clone for DOC_INFO_3A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOC_INFO_3A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DOC_INFO_3W {
    pub pDocName: ::windows::core::PWSTR,
    pub pOutputFile: ::windows::core::PWSTR,
    pub pDatatype: ::windows::core::PWSTR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DOC_INFO_3W {}
impl ::core::clone::Clone for DOC_INFO_3W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOC_INFO_3W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_1A {
    pub pName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_1A {}
impl ::core::clone::Clone for DRIVER_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_1W {
    pub pName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_1W {}
impl ::core::clone::Clone for DRIVER_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_2A {
    pub cVersion: u32,
    pub pName: ::windows::core::PSTR,
    pub pEnvironment: ::windows::core::PSTR,
    pub pDriverPath: ::windows::core::PSTR,
    pub pDataFile: ::windows::core::PSTR,
    pub pConfigFile: ::windows::core::PSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_2A {}
impl ::core::clone::Clone for DRIVER_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_INFO_2A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_2W {
    pub cVersion: u32,
    pub pName: ::windows::core::PWSTR,
    pub pEnvironment: ::windows::core::PWSTR,
    pub pDriverPath: ::windows::core::PWSTR,
    pub pDataFile: ::windows::core::PWSTR,
    pub pConfigFile: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_2W {}
impl ::core::clone::Clone for DRIVER_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_INFO_2W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_3A {
    pub cVersion: u32,
    pub pName: ::windows::core::PSTR,
    pub pEnvironment: ::windows::core::PSTR,
    pub pDriverPath: ::windows::core::PSTR,
    pub pDataFile: ::windows::core::PSTR,
    pub pConfigFile: ::windows::core::PSTR,
    pub pHelpFile: ::windows::core::PSTR,
    pub pDependentFiles: ::windows::core::PSTR,
    pub pMonitorName: ::windows::core::PSTR,
    pub pDefaultDataType: ::windows::core::PSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_3A {}
impl ::core::clone::Clone for DRIVER_INFO_3A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_INFO_3A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_3W {
    pub cVersion: u32,
    pub pName: ::windows::core::PWSTR,
    pub pEnvironment: ::windows::core::PWSTR,
    pub pDriverPath: ::windows::core::PWSTR,
    pub pDataFile: ::windows::core::PWSTR,
    pub pConfigFile: ::windows::core::PWSTR,
    pub pHelpFile: ::windows::core::PWSTR,
    pub pDependentFiles: ::windows::core::PWSTR,
    pub pMonitorName: ::windows::core::PWSTR,
    pub pDefaultDataType: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_3W {}
impl ::core::clone::Clone for DRIVER_INFO_3W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_INFO_3W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_4A {
    pub cVersion: u32,
    pub pName: ::windows::core::PSTR,
    pub pEnvironment: ::windows::core::PSTR,
    pub pDriverPath: ::windows::core::PSTR,
    pub pDataFile: ::windows::core::PSTR,
    pub pConfigFile: ::windows::core::PSTR,
    pub pHelpFile: ::windows::core::PSTR,
    pub pDependentFiles: ::windows::core::PSTR,
    pub pMonitorName: ::windows::core::PSTR,
    pub pDefaultDataType: ::windows::core::PSTR,
    pub pszzPreviousNames: ::windows::core::PSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_4A {}
impl ::core::clone::Clone for DRIVER_INFO_4A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_INFO_4A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_4W {
    pub cVersion: u32,
    pub pName: ::windows::core::PWSTR,
    pub pEnvironment: ::windows::core::PWSTR,
    pub pDriverPath: ::windows::core::PWSTR,
    pub pDataFile: ::windows::core::PWSTR,
    pub pConfigFile: ::windows::core::PWSTR,
    pub pHelpFile: ::windows::core::PWSTR,
    pub pDependentFiles: ::windows::core::PWSTR,
    pub pMonitorName: ::windows::core::PWSTR,
    pub pDefaultDataType: ::windows::core::PWSTR,
    pub pszzPreviousNames: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DRIVER_INFO_4W {}
impl ::core::clone::Clone for DRIVER_INFO_4W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_INFO_4W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_5A {
    pub cVersion: u32,
    pub pName: ::windows::core::PSTR,
    pub pEnvironment: ::windows::core::PSTR,
    pub pDriverPath: ::windows::core::PSTR,
    pub pDataFile: ::windows::core::PSTR,
    pub pConfigFile: ::windows::core::PSTR,
    pub dwDriverAttributes: u32,
    pub dwConfigVersion: u32,
    pub dwDriverVersion: u32,
}
impl ::core::marker::Copy for DRIVER_INFO_5A {}
impl ::core::clone::Clone for DRIVER_INFO_5A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_INFO_5A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_INFO_5W {
    pub cVersion: u32,
    pub pName: ::windows::core::PWSTR,
    pub pEnvironment: ::windows::core::PWSTR,
    pub pDriverPath: ::windows::core::PWSTR,
    pub pDataFile: ::windows::core::PWSTR,
    pub pConfigFile: ::windows::core::PWSTR,
    pub dwDriverAttributes: u32,
    pub dwConfigVersion: u32,
    pub dwDriverVersion: u32,
}
impl ::core::marker::Copy for DRIVER_INFO_5W {}
impl ::core::clone::Clone for DRIVER_INFO_5W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_INFO_5W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVER_INFO_6A {
    pub cVersion: u32,
    pub pName: ::windows::core::PSTR,
    pub pEnvironment: ::windows::core::PSTR,
    pub pDriverPath: ::windows::core::PSTR,
    pub pDataFile: ::windows::core::PSTR,
    pub pConfigFile: ::windows::core::PSTR,
    pub pHelpFile: ::windows::core::PSTR,
    pub pDependentFiles: ::windows::core::PSTR,
    pub pMonitorName: ::windows::core::PSTR,
    pub pDefaultDataType: ::windows::core::PSTR,
    pub pszzPreviousNames: ::windows::core::PSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: ::windows::core::PSTR,
    pub pszOEMUrl: ::windows::core::PSTR,
    pub pszHardwareID: ::windows::core::PSTR,
    pub pszProvider: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVER_INFO_6A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVER_INFO_6A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRIVER_INFO_6A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVER_INFO_6W {
    pub cVersion: u32,
    pub pName: ::windows::core::PWSTR,
    pub pEnvironment: ::windows::core::PWSTR,
    pub pDriverPath: ::windows::core::PWSTR,
    pub pDataFile: ::windows::core::PWSTR,
    pub pConfigFile: ::windows::core::PWSTR,
    pub pHelpFile: ::windows::core::PWSTR,
    pub pDependentFiles: ::windows::core::PWSTR,
    pub pMonitorName: ::windows::core::PWSTR,
    pub pDefaultDataType: ::windows::core::PWSTR,
    pub pszzPreviousNames: ::windows::core::PWSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: ::windows::core::PWSTR,
    pub pszOEMUrl: ::windows::core::PWSTR,
    pub pszHardwareID: ::windows::core::PWSTR,
    pub pszProvider: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVER_INFO_6W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVER_INFO_6W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRIVER_INFO_6W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVER_INFO_8A {
    pub cVersion: u32,
    pub pName: ::windows::core::PSTR,
    pub pEnvironment: ::windows::core::PSTR,
    pub pDriverPath: ::windows::core::PSTR,
    pub pDataFile: ::windows::core::PSTR,
    pub pConfigFile: ::windows::core::PSTR,
    pub pHelpFile: ::windows::core::PSTR,
    pub pDependentFiles: ::windows::core::PSTR,
    pub pMonitorName: ::windows::core::PSTR,
    pub pDefaultDataType: ::windows::core::PSTR,
    pub pszzPreviousNames: ::windows::core::PSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: ::windows::core::PSTR,
    pub pszOEMUrl: ::windows::core::PSTR,
    pub pszHardwareID: ::windows::core::PSTR,
    pub pszProvider: ::windows::core::PSTR,
    pub pszPrintProcessor: ::windows::core::PSTR,
    pub pszVendorSetup: ::windows::core::PSTR,
    pub pszzColorProfiles: ::windows::core::PSTR,
    pub pszInfPath: ::windows::core::PSTR,
    pub dwPrinterDriverAttributes: u32,
    pub pszzCoreDriverDependencies: ::windows::core::PSTR,
    pub ftMinInboxDriverVerDate: super::super::Foundation::FILETIME,
    pub dwlMinInboxDriverVerVersion: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVER_INFO_8A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVER_INFO_8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRIVER_INFO_8A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRIVER_INFO_8W {
    pub cVersion: u32,
    pub pName: ::windows::core::PWSTR,
    pub pEnvironment: ::windows::core::PWSTR,
    pub pDriverPath: ::windows::core::PWSTR,
    pub pDataFile: ::windows::core::PWSTR,
    pub pConfigFile: ::windows::core::PWSTR,
    pub pHelpFile: ::windows::core::PWSTR,
    pub pDependentFiles: ::windows::core::PWSTR,
    pub pMonitorName: ::windows::core::PWSTR,
    pub pDefaultDataType: ::windows::core::PWSTR,
    pub pszzPreviousNames: ::windows::core::PWSTR,
    pub ftDriverDate: super::super::Foundation::FILETIME,
    pub dwlDriverVersion: u64,
    pub pszMfgName: ::windows::core::PWSTR,
    pub pszOEMUrl: ::windows::core::PWSTR,
    pub pszHardwareID: ::windows::core::PWSTR,
    pub pszProvider: ::windows::core::PWSTR,
    pub pszPrintProcessor: ::windows::core::PWSTR,
    pub pszVendorSetup: ::windows::core::PWSTR,
    pub pszzColorProfiles: ::windows::core::PWSTR,
    pub pszInfPath: ::windows::core::PWSTR,
    pub dwPrinterDriverAttributes: u32,
    pub pszzCoreDriverDependencies: ::windows::core::PWSTR,
    pub ftMinInboxDriverVerDate: super::super::Foundation::FILETIME,
    pub dwlMinInboxDriverVerVersion: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRIVER_INFO_8W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRIVER_INFO_8W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRIVER_INFO_8W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_UPGRADE_INFO_1 {
    pub pPrinterName: *mut i8,
    pub pOldDriverDirectory: *mut i8,
}
impl ::core::marker::Copy for DRIVER_UPGRADE_INFO_1 {}
impl ::core::clone::Clone for DRIVER_UPGRADE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_UPGRADE_INFO_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct DRIVER_UPGRADE_INFO_2 {
    pub pPrinterName: *mut i8,
    pub pOldDriverDirectory: *mut i8,
    pub cVersion: u32,
    pub pName: *mut i8,
    pub pEnvironment: *mut i8,
    pub pDriverPath: *mut i8,
    pub pDataFile: *mut i8,
    pub pConfigFile: *mut i8,
    pub pHelpFile: *mut i8,
    pub pDependentFiles: *mut i8,
    pub pMonitorName: *mut i8,
    pub pDefaultDataType: *mut i8,
    pub pszzPreviousNames: *mut i8,
}
impl ::core::marker::Copy for DRIVER_UPGRADE_INFO_2 {}
impl ::core::clone::Clone for DRIVER_UPGRADE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRIVER_UPGRADE_INFO_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct EXTCHKBOX {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub pSeparator: *mut i8,
    pub pCheckedName: *mut i8,
    pub IconID: usize,
    pub wReserved: [u16; 4],
    pub dwReserved: [usize; 2],
}
impl ::core::marker::Copy for EXTCHKBOX {}
impl ::core::clone::Clone for EXTCHKBOX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXTCHKBOX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct EXTPUSH {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub Anonymous1: EXTPUSH_0,
    pub IconID: usize,
    pub Anonymous2: EXTPUSH_1,
    pub dwReserved: [usize; 3],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for EXTPUSH {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for EXTPUSH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for EXTPUSH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union EXTPUSH_0 {
    pub DlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub pfnCallBack: super::super::Foundation::FARPROC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for EXTPUSH_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for EXTPUSH_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for EXTPUSH_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union EXTPUSH_1 {
    pub DlgTemplateID: u16,
    pub hDlgTemplate: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for EXTPUSH_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for EXTPUSH_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for EXTPUSH_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct EXTTEXTMETRIC {
    pub emSize: i16,
    pub emPointSize: i16,
    pub emOrientation: i16,
    pub emMasterHeight: i16,
    pub emMinScale: i16,
    pub emMaxScale: i16,
    pub emMasterUnits: i16,
    pub emCapHeight: i16,
    pub emXHeight: i16,
    pub emLowerCaseAscent: i16,
    pub emLowerCaseDescent: i16,
    pub emSlant: i16,
    pub emSuperScript: i16,
    pub emSubScript: i16,
    pub emSuperScriptSize: i16,
    pub emSubScriptSize: i16,
    pub emUnderlineOffset: i16,
    pub emUnderlineWidth: i16,
    pub emDoubleUpperUnderlineOffset: i16,
    pub emDoubleLowerUnderlineOffset: i16,
    pub emDoubleUpperUnderlineWidth: i16,
    pub emDoubleLowerUnderlineWidth: i16,
    pub emStrikeOutOffset: i16,
    pub emStrikeOutWidth: i16,
    pub emKernPairs: u16,
    pub emKernTracks: u16,
}
impl ::core::marker::Copy for EXTTEXTMETRIC {}
impl ::core::clone::Clone for EXTTEXTMETRIC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXTTEXTMETRIC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FORM_INFO_1A {
    pub Flags: u32,
    pub pName: ::windows::core::PSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FORM_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FORM_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FORM_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FORM_INFO_1W {
    pub Flags: u32,
    pub pName: ::windows::core::PWSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FORM_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FORM_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FORM_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FORM_INFO_2A {
    pub Flags: u32,
    pub pName: ::windows::core::PCSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
    pub pKeyword: ::windows::core::PCSTR,
    pub StringType: u32,
    pub pMuiDll: ::windows::core::PCSTR,
    pub dwResourceId: u32,
    pub pDisplayName: ::windows::core::PCSTR,
    pub wLangId: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FORM_INFO_2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FORM_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FORM_INFO_2A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FORM_INFO_2W {
    pub Flags: u32,
    pub pName: ::windows::core::PCWSTR,
    pub Size: super::super::Foundation::SIZE,
    pub ImageableArea: super::super::Foundation::RECTL,
    pub pKeyword: ::windows::core::PCSTR,
    pub StringType: u32,
    pub pMuiDll: ::windows::core::PCWSTR,
    pub dwResourceId: u32,
    pub pDisplayName: ::windows::core::PCWSTR,
    pub wLangId: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FORM_INFO_2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FORM_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FORM_INFO_2W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct GLYPHRUN {
    pub wcLow: u16,
    pub wGlyphCount: u16,
}
impl ::core::marker::Copy for GLYPHRUN {}
impl ::core::clone::Clone for GLYPHRUN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GLYPHRUN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct INSERTPSUIPAGE_INFO {
    pub cbSize: u16,
    pub Type: u8,
    pub Mode: u8,
    pub dwData1: usize,
    pub dwData2: usize,
    pub dwData3: usize,
}
impl ::core::marker::Copy for INSERTPSUIPAGE_INFO {}
impl ::core::clone::Clone for INSERTPSUIPAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INSERTPSUIPAGE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct INVOC {
    pub dwCount: u32,
    pub loOffset: u32,
}
impl ::core::marker::Copy for INVOC {}
impl ::core::clone::Clone for INVOC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INVOC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct ImgErrorInfo {
    pub description: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub guid: ::windows::core::GUID,
    pub helpContext: u32,
    pub helpFile: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub source: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub devDescription: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub errorID: ::windows::core::GUID,
    pub cUserParameters: u32,
    pub aUserParameters: *mut ::windows::core::BSTR,
    pub userFallback: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub exceptionID: u32,
}
impl ::core::clone::Clone for ImgErrorInfo {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for ImgErrorInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JOB_INFO_1A {
    pub JobId: u32,
    pub pPrinterName: ::windows::core::PSTR,
    pub pMachineName: ::windows::core::PSTR,
    pub pUserName: ::windows::core::PSTR,
    pub pDocument: ::windows::core::PSTR,
    pub pDatatype: ::windows::core::PSTR,
    pub pStatus: ::windows::core::PSTR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub TotalPages: u32,
    pub PagesPrinted: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JOB_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JOB_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOB_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JOB_INFO_1W {
    pub JobId: u32,
    pub pPrinterName: ::windows::core::PWSTR,
    pub pMachineName: ::windows::core::PWSTR,
    pub pUserName: ::windows::core::PWSTR,
    pub pDocument: ::windows::core::PWSTR,
    pub pDatatype: ::windows::core::PWSTR,
    pub pStatus: ::windows::core::PWSTR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub TotalPages: u32,
    pub PagesPrinted: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JOB_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JOB_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOB_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct JOB_INFO_2A {
    pub JobId: u32,
    pub pPrinterName: ::windows::core::PSTR,
    pub pMachineName: ::windows::core::PSTR,
    pub pUserName: ::windows::core::PSTR,
    pub pDocument: ::windows::core::PSTR,
    pub pNotifyName: ::windows::core::PSTR,
    pub pDatatype: ::windows::core::PSTR,
    pub pPrintProcessor: ::windows::core::PSTR,
    pub pParameters: ::windows::core::PSTR,
    pub pDriverName: ::windows::core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pStatus: ::windows::core::PSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for JOB_INFO_2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for JOB_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for JOB_INFO_2A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct JOB_INFO_2W {
    pub JobId: u32,
    pub pPrinterName: ::windows::core::PWSTR,
    pub pMachineName: ::windows::core::PWSTR,
    pub pUserName: ::windows::core::PWSTR,
    pub pDocument: ::windows::core::PWSTR,
    pub pNotifyName: ::windows::core::PWSTR,
    pub pDatatype: ::windows::core::PWSTR,
    pub pPrintProcessor: ::windows::core::PWSTR,
    pub pParameters: ::windows::core::PWSTR,
    pub pDriverName: ::windows::core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pStatus: ::windows::core::PWSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for JOB_INFO_2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for JOB_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for JOB_INFO_2W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct JOB_INFO_3 {
    pub JobId: u32,
    pub NextJobId: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for JOB_INFO_3 {}
impl ::core::clone::Clone for JOB_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOB_INFO_3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct JOB_INFO_4A {
    pub JobId: u32,
    pub pPrinterName: ::windows::core::PSTR,
    pub pMachineName: ::windows::core::PSTR,
    pub pUserName: ::windows::core::PSTR,
    pub pDocument: ::windows::core::PSTR,
    pub pNotifyName: ::windows::core::PSTR,
    pub pDatatype: ::windows::core::PSTR,
    pub pPrintProcessor: ::windows::core::PSTR,
    pub pParameters: ::windows::core::PSTR,
    pub pDriverName: ::windows::core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pStatus: ::windows::core::PSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
    pub SizeHigh: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for JOB_INFO_4A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for JOB_INFO_4A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for JOB_INFO_4A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct JOB_INFO_4W {
    pub JobId: u32,
    pub pPrinterName: ::windows::core::PWSTR,
    pub pMachineName: ::windows::core::PWSTR,
    pub pUserName: ::windows::core::PWSTR,
    pub pDocument: ::windows::core::PWSTR,
    pub pNotifyName: ::windows::core::PWSTR,
    pub pDatatype: ::windows::core::PWSTR,
    pub pPrintProcessor: ::windows::core::PWSTR,
    pub pParameters: ::windows::core::PWSTR,
    pub pDriverName: ::windows::core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pStatus: ::windows::core::PWSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Status: u32,
    pub Priority: u32,
    pub Position: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub TotalPages: u32,
    pub Size: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub Time: u32,
    pub PagesPrinted: u32,
    pub SizeHigh: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for JOB_INFO_4W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for JOB_INFO_4W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for JOB_INFO_4W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Devices_Display\"`*"]
#[cfg(feature = "Win32_Devices_Display")]
pub struct KERNDATA {
    pub dwSize: u32,
    pub dwKernPairNum: u32,
    pub KernPair: [super::super::Devices::Display::FD_KERNINGPAIR; 1],
}
#[cfg(feature = "Win32_Devices_Display")]
impl ::core::marker::Copy for KERNDATA {}
#[cfg(feature = "Win32_Devices_Display")]
impl ::core::clone::Clone for KERNDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Devices_Display")]
unsafe impl ::windows::core::Abi for KERNDATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MAPTABLE {
    pub dwSize: u32,
    pub dwGlyphNum: u32,
    pub Trans: [TRANSDATA; 1],
}
impl ::core::marker::Copy for MAPTABLE {}
impl ::core::clone::Clone for MAPTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MAPTABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MESSAGEBOX_PARAMS {
    pub cbSize: u32,
    pub pTitle: ::windows::core::PWSTR,
    pub pMessage: ::windows::core::PWSTR,
    pub Style: u32,
    pub dwTimeout: u32,
    pub bWait: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MESSAGEBOX_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MESSAGEBOX_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MESSAGEBOX_PARAMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR {
    pub pfnEnumPorts: isize,
    pub pfnOpenPort: isize,
    pub pfnOpenPortEx: isize,
    pub pfnStartDocPort: isize,
    pub pfnWritePort: isize,
    pub pfnReadPort: isize,
    pub pfnEndDocPort: isize,
    pub pfnClosePort: isize,
    pub pfnAddPort: isize,
    pub pfnAddPortEx: isize,
    pub pfnConfigurePort: isize,
    pub pfnDeletePort: isize,
    pub pfnGetPrinterDataFromPort: isize,
    pub pfnSetPortTimeOuts: isize,
    pub pfnXcvOpenPort: isize,
    pub pfnXcvDataPort: isize,
    pub pfnXcvClosePort: isize,
}
impl ::core::marker::Copy for MONITOR {}
impl ::core::clone::Clone for MONITOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONITOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR2 {
    pub cbSize: u32,
    pub pfnEnumPorts: isize,
    pub pfnOpenPort: isize,
    pub pfnOpenPortEx: isize,
    pub pfnStartDocPort: isize,
    pub pfnWritePort: isize,
    pub pfnReadPort: isize,
    pub pfnEndDocPort: isize,
    pub pfnClosePort: isize,
    pub pfnAddPort: isize,
    pub pfnAddPortEx: isize,
    pub pfnConfigurePort: isize,
    pub pfnDeletePort: isize,
    pub pfnGetPrinterDataFromPort: isize,
    pub pfnSetPortTimeOuts: isize,
    pub pfnXcvOpenPort: isize,
    pub pfnXcvDataPort: isize,
    pub pfnXcvClosePort: isize,
    pub pfnShutdown: isize,
    pub pfnSendRecvBidiDataFromPort: isize,
    pub pfnNotifyUsedPorts: isize,
    pub pfnNotifyUnusedPorts: isize,
    pub pfnPowerEvent: isize,
}
impl ::core::marker::Copy for MONITOR2 {}
impl ::core::clone::Clone for MONITOR2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONITOR2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOREX {
    pub dwMonitorSize: u32,
    pub Monitor: MONITOR,
}
impl ::core::marker::Copy for MONITOREX {}
impl ::core::clone::Clone for MONITOREX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONITOREX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct MONITORINIT {
    pub cbSize: u32,
    pub hSpooler: super::super::Foundation::HANDLE,
    pub hckRegistryRoot: super::super::System::Registry::HKEY,
    pub pMonitorReg: *mut MONITORREG,
    pub bLocal: super::super::Foundation::BOOL,
    pub pszServerName: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for MONITORINIT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for MONITORINIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
unsafe impl ::windows::core::Abi for MONITORINIT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITORREG {
    pub cbSize: u32,
    pub fpCreateKey: isize,
    pub fpOpenKey: isize,
    pub fpCloseKey: isize,
    pub fpDeleteKey: isize,
    pub fpEnumKey: isize,
    pub fpQueryInfoKey: isize,
    pub fpSetValue: isize,
    pub fpDeleteValue: isize,
    pub fpEnumValue: isize,
    pub fpQueryValue: isize,
}
impl ::core::marker::Copy for MONITORREG {}
impl ::core::clone::Clone for MONITORREG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONITORREG {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITORUI {
    pub dwMonitorUISize: u32,
    pub pfnAddPortUI: isize,
    pub pfnConfigurePortUI: isize,
    pub pfnDeletePortUI: isize,
}
impl ::core::marker::Copy for MONITORUI {}
impl ::core::clone::Clone for MONITORUI {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONITORUI {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR_INFO_1A {
    pub pName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for MONITOR_INFO_1A {}
impl ::core::clone::Clone for MONITOR_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONITOR_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR_INFO_1W {
    pub pName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MONITOR_INFO_1W {}
impl ::core::clone::Clone for MONITOR_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONITOR_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR_INFO_2A {
    pub pName: ::windows::core::PSTR,
    pub pEnvironment: ::windows::core::PSTR,
    pub pDLLName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for MONITOR_INFO_2A {}
impl ::core::clone::Clone for MONITOR_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONITOR_INFO_2A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MONITOR_INFO_2W {
    pub pName: ::windows::core::PWSTR,
    pub pEnvironment: ::windows::core::PWSTR,
    pub pDLLName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MONITOR_INFO_2W {}
impl ::core::clone::Clone for MONITOR_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONITOR_INFO_2W {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MXDC_ESCAPE_HEADER_T {
    pub cbInput: u32,
    pub cbOutput: u32,
    pub opCode: u32,
}
impl ::core::marker::Copy for MXDC_ESCAPE_HEADER_T {}
impl ::core::clone::Clone for MXDC_ESCAPE_HEADER_T {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_ESCAPE_HEADER_T {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MXDC_GET_FILENAME_DATA_T {
    pub cbOutput: u32,
    pub wszData: [u16; 1],
}
impl ::core::marker::Copy for MXDC_GET_FILENAME_DATA_T {}
impl ::core::clone::Clone for MXDC_GET_FILENAME_DATA_T {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_GET_FILENAME_DATA_T {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MXDC_PRINTTICKET_DATA_T {
    pub dwDataSize: u32,
    pub bData: [u8; 1],
}
impl ::core::marker::Copy for MXDC_PRINTTICKET_DATA_T {}
impl ::core::clone::Clone for MXDC_PRINTTICKET_DATA_T {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_PRINTTICKET_DATA_T {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MXDC_PRINTTICKET_ESCAPE_T {
    pub mxdcEscape: MXDC_ESCAPE_HEADER_T,
    pub printTicketData: MXDC_PRINTTICKET_DATA_T,
}
impl ::core::marker::Copy for MXDC_PRINTTICKET_ESCAPE_T {}
impl ::core::clone::Clone for MXDC_PRINTTICKET_ESCAPE_T {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_PRINTTICKET_ESCAPE_T {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MXDC_S0PAGE_DATA_T {
    pub dwSize: u32,
    pub bData: [u8; 1],
}
impl ::core::marker::Copy for MXDC_S0PAGE_DATA_T {}
impl ::core::clone::Clone for MXDC_S0PAGE_DATA_T {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_S0PAGE_DATA_T {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MXDC_S0PAGE_PASSTHROUGH_ESCAPE_T {
    pub mxdcEscape: MXDC_ESCAPE_HEADER_T,
    pub xpsS0PageData: MXDC_S0PAGE_DATA_T,
}
impl ::core::marker::Copy for MXDC_S0PAGE_PASSTHROUGH_ESCAPE_T {}
impl ::core::clone::Clone for MXDC_S0PAGE_PASSTHROUGH_ESCAPE_T {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_S0PAGE_PASSTHROUGH_ESCAPE_T {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MXDC_S0PAGE_RESOURCE_ESCAPE_T {
    pub mxdcEscape: MXDC_ESCAPE_HEADER_T,
    pub xpsS0PageResourcePassthrough: MXDC_XPS_S0PAGE_RESOURCE_T,
}
impl ::core::marker::Copy for MXDC_S0PAGE_RESOURCE_ESCAPE_T {}
impl ::core::clone::Clone for MXDC_S0PAGE_RESOURCE_ESCAPE_T {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_S0PAGE_RESOURCE_ESCAPE_T {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct MXDC_XPS_S0PAGE_RESOURCE_T {
    pub dwSize: u32,
    pub dwResourceType: u32,
    pub szUri: [u8; 260],
    pub dwDataSize: u32,
    pub bData: [u8; 1],
}
impl ::core::marker::Copy for MXDC_XPS_S0PAGE_RESOURCE_T {}
impl ::core::clone::Clone for MXDC_XPS_S0PAGE_RESOURCE_T {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MXDC_XPS_S0PAGE_RESOURCE_T {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFICATION_CONFIG_1 {
    pub cbSize: u32,
    pub fdwFlags: u32,
    pub pfnNotifyCallback: ROUTER_NOTIFY_CALLBACK,
    pub pContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NOTIFICATION_CONFIG_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NOTIFICATION_CONFIG_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NOTIFICATION_CONFIG_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OEMCUIPPARAM {
    pub cbSize: u32,
    pub poemuiobj: *mut OEMUIOBJ,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pPrinterName: ::windows::core::PWSTR,
    pub hModule: super::super::Foundation::HANDLE,
    pub hOEMHeap: super::super::Foundation::HANDLE,
    pub pPublicDM: *mut super::Gdi::DEVMODEA,
    pub pOEMDM: *mut ::core::ffi::c_void,
    pub dwFlags: u32,
    pub pDrvOptItems: *mut OPTITEM,
    pub cDrvOptItems: u32,
    pub pOEMOptItems: *mut OPTITEM,
    pub cOEMOptItems: u32,
    pub pOEMUserData: *mut ::core::ffi::c_void,
    pub OEMCUIPCallback: OEMCUIPCALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OEMCUIPPARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OEMCUIPPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OEMCUIPPARAM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct OEMDMPARAM {
    pub cbSize: u32,
    pub pdriverobj: *mut ::core::ffi::c_void,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub hModule: super::super::Foundation::HANDLE,
    pub pPublicDMIn: *mut super::Gdi::DEVMODEA,
    pub pPublicDMOut: *mut super::Gdi::DEVMODEA,
    pub pOEMDMIn: *mut ::core::ffi::c_void,
    pub pOEMDMOut: *mut ::core::ffi::c_void,
    pub cbBufSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for OEMDMPARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for OEMDMPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for OEMDMPARAM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OEMFONTINSTPARAM {
    pub cbSize: u32,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub hModule: super::super::Foundation::HANDLE,
    pub hHeap: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
    pub pFontInstallerName: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OEMFONTINSTPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OEMFONTINSTPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OEMFONTINSTPARAM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OEMUIOBJ {
    pub cbSize: u32,
    pub pOemUIProcs: *mut OEMUIPROCS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OEMUIOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OEMUIOBJ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OEMUIOBJ {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OEMUIPROCS {
    pub DrvGetDriverSetting: PFN_DrvGetDriverSetting,
    pub DrvUpdateUISetting: PFN_DrvUpdateUISetting,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OEMUIPROCS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OEMUIPROCS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OEMUIPROCS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct OEMUIPSPARAM {
    pub cbSize: u32,
    pub poemuiobj: *mut OEMUIOBJ,
    pub hPrinter: super::super::Foundation::HANDLE,
    pub pPrinterName: ::windows::core::PWSTR,
    pub hModule: super::super::Foundation::HANDLE,
    pub hOEMHeap: super::super::Foundation::HANDLE,
    pub pPublicDM: *mut super::Gdi::DEVMODEA,
    pub pOEMDM: *mut ::core::ffi::c_void,
    pub pOEMUserData: *mut ::core::ffi::c_void,
    pub dwFlags: u32,
    pub pOemEntry: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for OEMUIPSPARAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for OEMUIPSPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for OEMUIPSPARAM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct OEM_DMEXTRAHEADER {
    pub dwSize: u32,
    pub dwSignature: u32,
    pub dwVersion: u32,
}
impl ::core::marker::Copy for OEM_DMEXTRAHEADER {}
impl ::core::clone::Clone for OEM_DMEXTRAHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OEM_DMEXTRAHEADER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OIEXT {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::super::Foundation::HINSTANCE,
    pub pHelpFile: *mut i8,
    pub dwReserved: [usize; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OIEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OIEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OIEXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPTCOMBO {
    pub cbSize: u16,
    pub Flags: u8,
    pub cListItem: u16,
    pub pListItem: *mut OPTPARAM,
    pub Sel: i32,
    pub dwReserved: [u32; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPTCOMBO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPTCOMBO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPTCOMBO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OPTITEM {
    pub cbSize: u16,
    pub Level: u8,
    pub DlgPageIdx: u8,
    pub Flags: u32,
    pub UserData: usize,
    pub pName: *mut i8,
    pub Anonymous1: OPTITEM_0,
    pub Anonymous2: OPTITEM_1,
    pub pOptType: *mut OPTTYPE,
    pub HelpIndex: u32,
    pub DMPubID: u8,
    pub UserItemID: u8,
    pub wReserved: u16,
    pub pOIExt: *mut OIEXT,
    pub dwReserved: [usize; 3],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPTITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPTITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OPTITEM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union OPTITEM_0 {
    pub Sel: i32,
    pub pSel: *mut i8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPTITEM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPTITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OPTITEM_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union OPTITEM_1 {
    pub pExtChkBox: *mut EXTCHKBOX,
    pub pExtPush: *mut EXTPUSH,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPTITEM_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPTITEM_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OPTITEM_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPTPARAM {
    pub cbSize: u16,
    pub Flags: u8,
    pub Style: u8,
    pub pData: *mut i8,
    pub IconID: usize,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwReserved: [usize; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPTPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPTPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPTPARAM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OPTTYPE {
    pub cbSize: u16,
    pub Type: u8,
    pub Flags: u8,
    pub Count: u16,
    pub BegCtrlID: u16,
    pub pOptParam: *mut OPTPARAM,
    pub Style: u16,
    pub wReserved: [u16; 3],
    pub dwReserved: [usize; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPTTYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPTTYPE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_DATA_1 {
    pub sztPortName: [u16; 64],
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub cbSize: u32,
    pub dwReserved: u32,
    pub sztHostAddress: [u16; 49],
    pub sztSNMPCommunity: [u16; 33],
    pub dwDoubleSpool: u32,
    pub sztQueue: [u16; 33],
    pub sztIPAddress: [u16; 16],
    pub Reserved: [u8; 540],
    pub dwPortNumber: u32,
    pub dwSNMPEnabled: u32,
    pub dwSNMPDevIndex: u32,
}
impl ::core::marker::Copy for PORT_DATA_1 {}
impl ::core::clone::Clone for PORT_DATA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PORT_DATA_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_DATA_2 {
    pub sztPortName: [u16; 64],
    pub dwVersion: u32,
    pub dwProtocol: u32,
    pub cbSize: u32,
    pub dwReserved: u32,
    pub sztHostAddress: [u16; 128],
    pub sztSNMPCommunity: [u16; 33],
    pub dwDoubleSpool: u32,
    pub sztQueue: [u16; 33],
    pub Reserved: [u8; 514],
    pub dwPortNumber: u32,
    pub dwSNMPEnabled: u32,
    pub dwSNMPDevIndex: u32,
    pub dwPortMonitorMibIndex: u32,
}
impl ::core::marker::Copy for PORT_DATA_2 {}
impl ::core::clone::Clone for PORT_DATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PORT_DATA_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_DATA_LIST_1 {
    pub dwVersion: u32,
    pub cPortData: u32,
    pub pPortData: [PORT_DATA_2; 1],
}
impl ::core::marker::Copy for PORT_DATA_LIST_1 {}
impl ::core::clone::Clone for PORT_DATA_LIST_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PORT_DATA_LIST_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_1A {
    pub pName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for PORT_INFO_1A {}
impl ::core::clone::Clone for PORT_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PORT_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_1W {
    pub pName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PORT_INFO_1W {}
impl ::core::clone::Clone for PORT_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PORT_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_2A {
    pub pPortName: ::windows::core::PSTR,
    pub pMonitorName: ::windows::core::PSTR,
    pub pDescription: ::windows::core::PSTR,
    pub fPortType: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PORT_INFO_2A {}
impl ::core::clone::Clone for PORT_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PORT_INFO_2A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_2W {
    pub pPortName: ::windows::core::PWSTR,
    pub pMonitorName: ::windows::core::PWSTR,
    pub pDescription: ::windows::core::PWSTR,
    pub fPortType: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PORT_INFO_2W {}
impl ::core::clone::Clone for PORT_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PORT_INFO_2W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_3A {
    pub dwStatus: u32,
    pub pszStatus: ::windows::core::PSTR,
    pub dwSeverity: u32,
}
impl ::core::marker::Copy for PORT_INFO_3A {}
impl ::core::clone::Clone for PORT_INFO_3A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PORT_INFO_3A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PORT_INFO_3W {
    pub dwStatus: u32,
    pub pszStatus: ::windows::core::PWSTR,
    pub dwSeverity: u32,
}
impl ::core::marker::Copy for PORT_INFO_3W {}
impl ::core::clone::Clone for PORT_INFO_3W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PORT_INFO_3W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_CONNECTION_INFO_1A {
    pub dwFlags: u32,
    pub pszDriverName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for PRINTER_CONNECTION_INFO_1A {}
impl ::core::clone::Clone for PRINTER_CONNECTION_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_CONNECTION_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_CONNECTION_INFO_1W {
    pub dwFlags: u32,
    pub pszDriverName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PRINTER_CONNECTION_INFO_1W {}
impl ::core::clone::Clone for PRINTER_CONNECTION_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_CONNECTION_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_DEFAULTSA {
    pub pDatatype: ::windows::core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub DesiredAccess: PRINTER_ACCESS_RIGHTS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_DEFAULTSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_DEFAULTSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTER_DEFAULTSA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_DEFAULTSW {
    pub pDatatype: ::windows::core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub DesiredAccess: PRINTER_ACCESS_RIGHTS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_DEFAULTSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_DEFAULTSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTER_DEFAULTSW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_ENUM_VALUESA {
    pub pValueName: ::windows::core::PSTR,
    pub cbValueName: u32,
    pub dwType: u32,
    pub pData: *mut u8,
    pub cbData: u32,
}
impl ::core::marker::Copy for PRINTER_ENUM_VALUESA {}
impl ::core::clone::Clone for PRINTER_ENUM_VALUESA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_ENUM_VALUESA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_ENUM_VALUESW {
    pub pValueName: ::windows::core::PWSTR,
    pub cbValueName: u32,
    pub dwType: u32,
    pub pData: *mut u8,
    pub cbData: u32,
}
impl ::core::marker::Copy for PRINTER_ENUM_VALUESW {}
impl ::core::clone::Clone for PRINTER_ENUM_VALUESW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_ENUM_VALUESW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_EVENT_ATTRIBUTES_INFO {
    pub cbSize: u32,
    pub dwOldAttributes: u32,
    pub dwNewAttributes: u32,
}
impl ::core::marker::Copy for PRINTER_EVENT_ATTRIBUTES_INFO {}
impl ::core::clone::Clone for PRINTER_EVENT_ATTRIBUTES_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_EVENT_ATTRIBUTES_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_1A {
    pub Flags: u32,
    pub pDescription: ::windows::core::PSTR,
    pub pName: ::windows::core::PSTR,
    pub pComment: ::windows::core::PSTR,
}
impl ::core::marker::Copy for PRINTER_INFO_1A {}
impl ::core::clone::Clone for PRINTER_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_1W {
    pub Flags: u32,
    pub pDescription: ::windows::core::PWSTR,
    pub pName: ::windows::core::PWSTR,
    pub pComment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PRINTER_INFO_1W {}
impl ::core::clone::Clone for PRINTER_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct PRINTER_INFO_2A {
    pub pServerName: ::windows::core::PSTR,
    pub pPrinterName: ::windows::core::PSTR,
    pub pShareName: ::windows::core::PSTR,
    pub pPortName: ::windows::core::PSTR,
    pub pDriverName: ::windows::core::PSTR,
    pub pComment: ::windows::core::PSTR,
    pub pLocation: ::windows::core::PSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pSepFile: ::windows::core::PSTR,
    pub pPrintProcessor: ::windows::core::PSTR,
    pub pDatatype: ::windows::core::PSTR,
    pub pParameters: ::windows::core::PSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Attributes: u32,
    pub Priority: u32,
    pub DefaultPriority: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub Status: u32,
    pub cJobs: u32,
    pub AveragePPM: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for PRINTER_INFO_2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for PRINTER_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for PRINTER_INFO_2A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
pub struct PRINTER_INFO_2W {
    pub pServerName: ::windows::core::PWSTR,
    pub pPrinterName: ::windows::core::PWSTR,
    pub pShareName: ::windows::core::PWSTR,
    pub pPortName: ::windows::core::PWSTR,
    pub pDriverName: ::windows::core::PWSTR,
    pub pComment: ::windows::core::PWSTR,
    pub pLocation: ::windows::core::PWSTR,
    pub pDevMode: *mut super::Gdi::DEVMODEW,
    pub pSepFile: ::windows::core::PWSTR,
    pub pPrintProcessor: ::windows::core::PWSTR,
    pub pDatatype: ::windows::core::PWSTR,
    pub pParameters: ::windows::core::PWSTR,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub Attributes: u32,
    pub Priority: u32,
    pub DefaultPriority: u32,
    pub StartTime: u32,
    pub UntilTime: u32,
    pub Status: u32,
    pub cJobs: u32,
    pub AveragePPM: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::marker::Copy for PRINTER_INFO_2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
impl ::core::clone::Clone for PRINTER_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for PRINTER_INFO_2W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct PRINTER_INFO_3 {
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for PRINTER_INFO_3 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for PRINTER_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for PRINTER_INFO_3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_4A {
    pub pPrinterName: ::windows::core::PSTR,
    pub pServerName: ::windows::core::PSTR,
    pub Attributes: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_4A {}
impl ::core::clone::Clone for PRINTER_INFO_4A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_INFO_4A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_4W {
    pub pPrinterName: ::windows::core::PWSTR,
    pub pServerName: ::windows::core::PWSTR,
    pub Attributes: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_4W {}
impl ::core::clone::Clone for PRINTER_INFO_4W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_INFO_4W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_5A {
    pub pPrinterName: ::windows::core::PSTR,
    pub pPortName: ::windows::core::PSTR,
    pub Attributes: u32,
    pub DeviceNotSelectedTimeout: u32,
    pub TransmissionRetryTimeout: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_5A {}
impl ::core::clone::Clone for PRINTER_INFO_5A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_INFO_5A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_5W {
    pub pPrinterName: ::windows::core::PWSTR,
    pub pPortName: ::windows::core::PWSTR,
    pub Attributes: u32,
    pub DeviceNotSelectedTimeout: u32,
    pub TransmissionRetryTimeout: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_5W {}
impl ::core::clone::Clone for PRINTER_INFO_5W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_INFO_5W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_6 {
    pub dwStatus: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_6 {}
impl ::core::clone::Clone for PRINTER_INFO_6 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_INFO_6 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_7A {
    pub pszObjectGUID: ::windows::core::PSTR,
    pub dwAction: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_7A {}
impl ::core::clone::Clone for PRINTER_INFO_7A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_INFO_7A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_INFO_7W {
    pub pszObjectGUID: ::windows::core::PWSTR,
    pub dwAction: u32,
}
impl ::core::marker::Copy for PRINTER_INFO_7W {}
impl ::core::clone::Clone for PRINTER_INFO_7W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_INFO_7W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_INFO_8A {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_INFO_8A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_INFO_8A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTER_INFO_8A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_INFO_8W {
    pub pDevMode: *mut super::Gdi::DEVMODEW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_INFO_8W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_INFO_8W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTER_INFO_8W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_INFO_9A {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_INFO_9A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_INFO_9A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTER_INFO_9A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTER_INFO_9W {
    pub pDevMode: *mut super::Gdi::DEVMODEW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTER_INFO_9W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTER_INFO_9W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTER_INFO_9W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub aData: [PRINTER_NOTIFY_INFO_DATA; 1],
}
impl ::core::marker::Copy for PRINTER_NOTIFY_INFO {}
impl ::core::clone::Clone for PRINTER_NOTIFY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_NOTIFY_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_INFO_DATA {
    pub Type: u16,
    pub Field: u16,
    pub Reserved: u32,
    pub Id: u32,
    pub NotifyData: PRINTER_NOTIFY_INFO_DATA_0,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_INFO_DATA {}
impl ::core::clone::Clone for PRINTER_NOTIFY_INFO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_NOTIFY_INFO_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub union PRINTER_NOTIFY_INFO_DATA_0 {
    pub adwData: [u32; 2],
    pub Data: PRINTER_NOTIFY_INFO_DATA_0_0,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_INFO_DATA_0 {}
impl ::core::clone::Clone for PRINTER_NOTIFY_INFO_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_NOTIFY_INFO_DATA_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_INFO_DATA_0_0 {
    pub cbBuf: u32,
    pub pBuf: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_INFO_DATA_0_0 {}
impl ::core::clone::Clone for PRINTER_NOTIFY_INFO_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_NOTIFY_INFO_DATA_0_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_INIT {
    pub Size: u32,
    pub Reserved: u32,
    pub PollTime: u32,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_INIT {}
impl ::core::clone::Clone for PRINTER_NOTIFY_INIT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_NOTIFY_INIT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_OPTIONS {
    pub Version: u32,
    pub Flags: u32,
    pub Count: u32,
    pub pTypes: *mut PRINTER_NOTIFY_OPTIONS_TYPE,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_OPTIONS {}
impl ::core::clone::Clone for PRINTER_NOTIFY_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_NOTIFY_OPTIONS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_NOTIFY_OPTIONS_TYPE {
    pub Type: u16,
    pub Reserved0: u16,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Count: u32,
    pub pFields: *mut u16,
}
impl ::core::marker::Copy for PRINTER_NOTIFY_OPTIONS_TYPE {}
impl ::core::clone::Clone for PRINTER_NOTIFY_OPTIONS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_NOTIFY_OPTIONS_TYPE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_OPTIONSA {
    pub cbSize: u32,
    pub dwFlags: PRINTER_OPTION_FLAGS,
}
impl ::core::marker::Copy for PRINTER_OPTIONSA {}
impl ::core::clone::Clone for PRINTER_OPTIONSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_OPTIONSA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTER_OPTIONSW {
    pub cbSize: u32,
    pub dwFlags: PRINTER_OPTION_FLAGS,
}
impl ::core::marker::Copy for PRINTER_OPTIONSW {}
impl ::core::clone::Clone for PRINTER_OPTIONSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTER_OPTIONSW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTIFI32 {
    pub cjThis: u32,
    pub cjIfiExtra: u32,
    pub dpwszFamilyName: i32,
    pub dpwszStyleName: i32,
    pub dpwszFaceName: i32,
    pub dpwszUniqueName: i32,
    pub dpFontSim: i32,
    pub lEmbedId: i32,
    pub lItalicAngle: i32,
    pub lCharBias: i32,
    pub dpCharSets: i32,
    pub jWinCharSet: u8,
    pub jWinPitchAndFamily: u8,
    pub usWinWeight: u16,
    pub flInfo: u32,
    pub fsSelection: u16,
    pub fsType: u16,
    pub fwdUnitsPerEm: i16,
    pub fwdLowestPPEm: i16,
    pub fwdWinAscender: i16,
    pub fwdWinDescender: i16,
    pub fwdMacAscender: i16,
    pub fwdMacDescender: i16,
    pub fwdMacLineGap: i16,
    pub fwdTypoAscender: i16,
    pub fwdTypoDescender: i16,
    pub fwdTypoLineGap: i16,
    pub fwdAveCharWidth: i16,
    pub fwdMaxCharInc: i16,
    pub fwdCapHeight: i16,
    pub fwdXHeight: i16,
    pub fwdSubscriptXSize: i16,
    pub fwdSubscriptYSize: i16,
    pub fwdSubscriptXOffset: i16,
    pub fwdSubscriptYOffset: i16,
    pub fwdSuperscriptXSize: i16,
    pub fwdSuperscriptYSize: i16,
    pub fwdSuperscriptXOffset: i16,
    pub fwdSuperscriptYOffset: i16,
    pub fwdUnderscoreSize: i16,
    pub fwdUnderscorePosition: i16,
    pub fwdStrikeoutSize: i16,
    pub fwdStrikeoutPosition: i16,
    pub chFirstChar: u8,
    pub chLastChar: u8,
    pub chDefaultChar: u8,
    pub chBreakChar: u8,
    pub wcFirstChar: u16,
    pub wcLastChar: u16,
    pub wcDefaultChar: u16,
    pub wcBreakChar: u16,
    pub ptlBaseline: super::super::Foundation::POINTL,
    pub ptlAspect: super::super::Foundation::POINTL,
    pub ptlCaret: super::super::Foundation::POINTL,
    pub rclFontBox: super::super::Foundation::RECTL,
    pub achVendId: [u8; 4],
    pub cKerningPairs: u32,
    pub ulPanoseCulture: u32,
    pub panose: super::Gdi::PANOSE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTIFI32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTIFI32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTIFI32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTPROCESSOROPENDATA {
    pub pDevMode: *mut super::Gdi::DEVMODEA,
    pub pDatatype: ::windows::core::PWSTR,
    pub pParameters: ::windows::core::PWSTR,
    pub pDocumentName: ::windows::core::PWSTR,
    pub JobId: u32,
    pub pOutputFile: ::windows::core::PWSTR,
    pub pPrinterName: ::windows::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTPROCESSOROPENDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTPROCESSOROPENDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTPROCESSOROPENDATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTPROCESSOR_CAPS_1 {
    pub dwLevel: u32,
    pub dwNupOptions: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
}
impl ::core::marker::Copy for PRINTPROCESSOR_CAPS_1 {}
impl ::core::clone::Clone for PRINTPROCESSOR_CAPS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTPROCESSOR_CAPS_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTPROCESSOR_CAPS_2 {
    pub dwLevel: u32,
    pub dwNupOptions: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
    pub dwDuplexHandlingCaps: u32,
    pub dwNupDirectionCaps: u32,
    pub dwNupBorderCaps: u32,
    pub dwBookletHandlingCaps: u32,
    pub dwScalingCaps: u32,
}
impl ::core::marker::Copy for PRINTPROCESSOR_CAPS_2 {}
impl ::core::clone::Clone for PRINTPROCESSOR_CAPS_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTPROCESSOR_CAPS_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTPROCESSOR_INFO_1A {
    pub pName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for PRINTPROCESSOR_INFO_1A {}
impl ::core::clone::Clone for PRINTPROCESSOR_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTPROCESSOR_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTPROCESSOR_INFO_1W {
    pub pName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PRINTPROCESSOR_INFO_1W {}
impl ::core::clone::Clone for PRINTPROCESSOR_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTPROCESSOR_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINTPROVIDOR {
    pub fpOpenPrinter: isize,
    pub fpSetJob: isize,
    pub fpGetJob: isize,
    pub fpEnumJobs: isize,
    pub fpAddPrinter: isize,
    pub fpDeletePrinter: isize,
    pub fpSetPrinter: isize,
    pub fpGetPrinter: isize,
    pub fpEnumPrinters: isize,
    pub fpAddPrinterDriver: isize,
    pub fpEnumPrinterDrivers: isize,
    pub fpGetPrinterDriver: isize,
    pub fpGetPrinterDriverDirectory: isize,
    pub fpDeletePrinterDriver: isize,
    pub fpAddPrintProcessor: isize,
    pub fpEnumPrintProcessors: isize,
    pub fpGetPrintProcessorDirectory: isize,
    pub fpDeletePrintProcessor: isize,
    pub fpEnumPrintProcessorDatatypes: isize,
    pub fpStartDocPrinter: isize,
    pub fpStartPagePrinter: isize,
    pub fpWritePrinter: isize,
    pub fpEndPagePrinter: isize,
    pub fpAbortPrinter: isize,
    pub fpReadPrinter: isize,
    pub fpEndDocPrinter: isize,
    pub fpAddJob: isize,
    pub fpScheduleJob: isize,
    pub fpGetPrinterData: isize,
    pub fpSetPrinterData: isize,
    pub fpWaitForPrinterChange: isize,
    pub fpClosePrinter: isize,
    pub fpAddForm: isize,
    pub fpDeleteForm: isize,
    pub fpGetForm: isize,
    pub fpSetForm: isize,
    pub fpEnumForms: isize,
    pub fpEnumMonitors: isize,
    pub fpEnumPorts: isize,
    pub fpAddPort: isize,
    pub fpConfigurePort: isize,
    pub fpDeletePort: isize,
    pub fpCreatePrinterIC: isize,
    pub fpPlayGdiScriptOnPrinterIC: isize,
    pub fpDeletePrinterIC: isize,
    pub fpAddPrinterConnection: isize,
    pub fpDeletePrinterConnection: isize,
    pub fpPrinterMessageBox: isize,
    pub fpAddMonitor: isize,
    pub fpDeleteMonitor: isize,
    pub fpResetPrinter: isize,
    pub fpGetPrinterDriverEx: isize,
    pub fpFindFirstPrinterChangeNotification: isize,
    pub fpFindClosePrinterChangeNotification: isize,
    pub fpAddPortEx: isize,
    pub fpShutDown: isize,
    pub fpRefreshPrinterChangeNotification: isize,
    pub fpOpenPrinterEx: isize,
    pub fpAddPrinterEx: isize,
    pub fpSetPort: isize,
    pub fpEnumPrinterData: isize,
    pub fpDeletePrinterData: isize,
    pub fpClusterSplOpen: isize,
    pub fpClusterSplClose: isize,
    pub fpClusterSplIsAlive: isize,
    pub fpSetPrinterDataEx: isize,
    pub fpGetPrinterDataEx: isize,
    pub fpEnumPrinterDataEx: isize,
    pub fpEnumPrinterKey: isize,
    pub fpDeletePrinterDataEx: isize,
    pub fpDeletePrinterKey: isize,
    pub fpSeekPrinter: isize,
    pub fpDeletePrinterDriverEx: isize,
    pub fpAddPerMachineConnection: isize,
    pub fpDeletePerMachineConnection: isize,
    pub fpEnumPerMachineConnections: isize,
    pub fpXcvData: isize,
    pub fpAddPrinterDriverEx: isize,
    pub fpSplReadPrinter: isize,
    pub fpDriverUnloadComplete: isize,
    pub fpGetSpoolFileInfo: isize,
    pub fpCommitSpoolData: isize,
    pub fpCloseSpoolFileHandle: isize,
    pub fpFlushPrinter: isize,
    pub fpSendRecvBidiData: isize,
    pub fpAddPrinterConnection2: isize,
    pub fpGetPrintClassObject: isize,
    pub fpReportJobProcessingProgress: isize,
    pub fpEnumAndLogProvidorObjects: isize,
    pub fpInternalGetPrinterDriver: isize,
    pub fpFindCompatibleDriver: isize,
    pub fpGetJobNamedPropertyValue: isize,
    pub fpSetJobNamedProperty: isize,
    pub fpDeleteJobNamedProperty: isize,
    pub fpEnumJobNamedProperties: isize,
    pub fpPowerEvent: isize,
    pub fpGetUserPropertyBag: isize,
    pub fpCanShutdown: isize,
    pub fpLogJobInfoForBranchOffice: isize,
    pub fpRegeneratePrintDeviceCapabilities: isize,
    pub fpPrintSupportOperation: isize,
    pub fpIppCreateJobOnPrinter: isize,
    pub fpIppGetJobAttributes: isize,
    pub fpIppSetJobAttributes: isize,
    pub fpIppGetPrinterAttributes: isize,
    pub fpIppSetPrinterAttributes: isize,
}
impl ::core::marker::Copy for PRINTPROVIDOR {}
impl ::core::clone::Clone for PRINTPROVIDOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINTPROVIDOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINT_EXECUTION_DATA {
    pub context: PRINT_EXECUTION_CONTEXT,
    pub clientAppPID: u32,
}
impl ::core::marker::Copy for PRINT_EXECUTION_DATA {}
impl ::core::clone::Clone for PRINT_EXECUTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINT_EXECUTION_DATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PRINT_FEATURE_OPTION {
    pub pszFeature: ::windows::core::PCSTR,
    pub pszOption: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for PRINT_FEATURE_OPTION {}
impl ::core::clone::Clone for PRINT_FEATURE_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINT_FEATURE_OPTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct PROPSHEETUI_GETICON_INFO {
    pub cbSize: u16,
    pub Flags: u16,
    pub cxIcon: u16,
    pub cyIcon: u16,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for PROPSHEETUI_GETICON_INFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for PROPSHEETUI_GETICON_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for PROPSHEETUI_GETICON_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPSHEETUI_INFO {
    pub cbSize: u16,
    pub Version: u16,
    pub Flags: u16,
    pub Reason: u16,
    pub hComPropSheet: super::super::Foundation::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
    pub lParamInit: super::super::Foundation::LPARAM,
    pub UserData: usize,
    pub Result: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPSHEETUI_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPSHEETUI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROPSHEETUI_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETUI_INFO_HEADER {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: *mut i8,
    pub hWndParent: super::super::Foundation::HWND,
    pub hInst: super::super::Foundation::HINSTANCE,
    pub Anonymous: PROPSHEETUI_INFO_HEADER_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETUI_INFO_HEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETUI_INFO_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETUI_INFO_HEADER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETUI_INFO_HEADER_0 {
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub IconID: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETUI_INFO_HEADER_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETUI_INFO_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETUI_INFO_HEADER_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PROVIDOR_INFO_1A {
    pub pName: ::windows::core::PSTR,
    pub pEnvironment: ::windows::core::PSTR,
    pub pDLLName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for PROVIDOR_INFO_1A {}
impl ::core::clone::Clone for PROVIDOR_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROVIDOR_INFO_1A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PROVIDOR_INFO_1W {
    pub pName: ::windows::core::PWSTR,
    pub pEnvironment: ::windows::core::PWSTR,
    pub pDLLName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PROVIDOR_INFO_1W {}
impl ::core::clone::Clone for PROVIDOR_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROVIDOR_INFO_1W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PROVIDOR_INFO_2A {
    pub pOrder: ::windows::core::PSTR,
}
impl ::core::marker::Copy for PROVIDOR_INFO_2A {}
impl ::core::clone::Clone for PROVIDOR_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROVIDOR_INFO_2A {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PROVIDOR_INFO_2W {
    pub pOrder: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PROVIDOR_INFO_2W {}
impl ::core::clone::Clone for PROVIDOR_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROVIDOR_INFO_2W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PSCRIPT5_PRIVATE_DEVMODE {
    pub wReserved: [u16; 57],
    pub wSize: u16,
}
impl ::core::marker::Copy for PSCRIPT5_PRIVATE_DEVMODE {}
impl ::core::clone::Clone for PSCRIPT5_PRIVATE_DEVMODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PSCRIPT5_PRIVATE_DEVMODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PSPINFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hComPropSheet: super::super::Foundation::HANDLE,
    pub hCPSUIPage: super::super::Foundation::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSPINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PSPINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PUBLISHERINFO {
    pub dwMode: u32,
    pub wMinoutlinePPEM: u16,
    pub wMaxbitmapPPEM: u16,
}
impl ::core::marker::Copy for PUBLISHERINFO {}
impl ::core::clone::Clone for PUBLISHERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PUBLISHERINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PrintNamedProperty {
    pub propertyName: ::windows::core::PWSTR,
    pub propertyValue: PrintPropertyValue,
}
impl ::core::marker::Copy for PrintNamedProperty {}
impl ::core::clone::Clone for PrintNamedProperty {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintNamedProperty {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PrintPropertiesCollection {
    pub numberOfProperties: u32,
    pub propertiesCollection: *mut PrintNamedProperty,
}
impl ::core::marker::Copy for PrintPropertiesCollection {}
impl ::core::clone::Clone for PrintPropertiesCollection {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintPropertiesCollection {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PrintPropertyValue {
    pub ePropertyType: EPrintPropertyType,
    pub value: PrintPropertyValue_0,
}
impl ::core::marker::Copy for PrintPropertyValue {}
impl ::core::clone::Clone for PrintPropertyValue {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintPropertyValue {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub union PrintPropertyValue_0 {
    pub propertyByte: u8,
    pub propertyString: ::windows::core::PWSTR,
    pub propertyInt32: i32,
    pub propertyInt64: i64,
    pub propertyBlob: PrintPropertyValue_0_0,
}
impl ::core::marker::Copy for PrintPropertyValue_0 {}
impl ::core::clone::Clone for PrintPropertyValue_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintPropertyValue_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct PrintPropertyValue_0_0 {
    pub cbBuf: u32,
    pub pBuf: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PrintPropertyValue_0_0 {}
impl ::core::clone::Clone for PrintPropertyValue_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PrintPropertyValue_0_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SETRESULT_INFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hSetResult: super::super::Foundation::HANDLE,
    pub Result: super::super::Foundation::LRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SETRESULT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SETRESULT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SETRESULT_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SHOWUIPARAMS {
    pub UIType: UI_TYPE,
    pub MessageBoxParams: MESSAGEBOX_PARAMS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SHOWUIPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SHOWUIPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SHOWUIPARAMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct SIMULATE_CAPS_1 {
    pub dwLevel: u32,
    pub dwPageOrderFlags: u32,
    pub dwNumberOfCopies: u32,
    pub dwCollate: u32,
    pub dwNupOptions: u32,
}
impl ::core::marker::Copy for SIMULATE_CAPS_1 {}
impl ::core::clone::Clone for SIMULATE_CAPS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SIMULATE_CAPS_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct SPLCLIENT_INFO_1 {
    pub dwSize: u32,
    pub pMachineName: ::windows::core::PWSTR,
    pub pUserName: ::windows::core::PWSTR,
    pub dwBuildNum: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub wProcessorArchitecture: u16,
}
impl ::core::marker::Copy for SPLCLIENT_INFO_1 {}
impl ::core::clone::Clone for SPLCLIENT_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SPLCLIENT_INFO_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct SPLCLIENT_INFO_2_W2K {
    pub hSplPrinter: usize,
}
impl ::core::marker::Copy for SPLCLIENT_INFO_2_W2K {}
impl ::core::clone::Clone for SPLCLIENT_INFO_2_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SPLCLIENT_INFO_2_W2K {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SPLCLIENT_INFO_2_WINXP {
    pub hSplPrinter: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SPLCLIENT_INFO_2_WINXP {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SPLCLIENT_INFO_2_WINXP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SPLCLIENT_INFO_2_WINXP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
#[cfg(target_arch = "x86")]
pub struct SPLCLIENT_INFO_2_WINXP {
    pub hSplPrinter: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SPLCLIENT_INFO_2_WINXP {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SPLCLIENT_INFO_2_WINXP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SPLCLIENT_INFO_2_WINXP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct SPLCLIENT_INFO_3_VISTA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwSize: u32,
    pub pMachineName: ::windows::core::PWSTR,
    pub pUserName: ::windows::core::PWSTR,
    pub dwBuildNum: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub wProcessorArchitecture: u16,
    pub hSplPrinter: u64,
}
impl ::core::marker::Copy for SPLCLIENT_INFO_3_VISTA {}
impl ::core::clone::Clone for SPLCLIENT_INFO_3_VISTA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SPLCLIENT_INFO_3_VISTA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct TRANSDATA {
    pub ubCodePageID: u8,
    pub ubType: u8,
    pub uCode: TRANSDATA_0,
}
impl ::core::marker::Copy for TRANSDATA {}
impl ::core::clone::Clone for TRANSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRANSDATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub union TRANSDATA_0 {
    pub sCode: i16,
    pub ubCode: u8,
    pub ubPairs: [u8; 2],
}
impl ::core::marker::Copy for TRANSDATA_0 {}
impl ::core::clone::Clone for TRANSDATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRANSDATA_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct UFF_FILEHEADER {
    pub dwSignature: u32,
    pub dwVersion: u32,
    pub dwSize: u32,
    pub nFonts: u32,
    pub nGlyphSets: u32,
    pub nVarData: u32,
    pub offFontDir: u32,
    pub dwFlags: u32,
    pub dwReserved: [u32; 4],
}
impl ::core::marker::Copy for UFF_FILEHEADER {}
impl ::core::clone::Clone for UFF_FILEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UFF_FILEHEADER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct UFF_FONTDIRECTORY {
    pub dwSignature: u32,
    pub wSize: u16,
    pub wFontID: u16,
    pub sGlyphID: i16,
    pub wFlags: u16,
    pub dwInstallerSig: u32,
    pub offFontName: u32,
    pub offCartridgeName: u32,
    pub offFontData: u32,
    pub offGlyphData: u32,
    pub offVarData: u32,
}
impl ::core::marker::Copy for UFF_FONTDIRECTORY {}
impl ::core::clone::Clone for UFF_FONTDIRECTORY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UFF_FONTDIRECTORY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct UNIDRVINFO {
    pub dwSize: u32,
    pub flGenFlags: u32,
    pub wType: u16,
    pub fCaps: u16,
    pub wXRes: u16,
    pub wYRes: u16,
    pub sYAdjust: i16,
    pub sYMoved: i16,
    pub wPrivateData: u16,
    pub sShift: i16,
    pub SelectFont: INVOC,
    pub UnSelectFont: INVOC,
    pub wReserved: [u16; 4],
}
impl ::core::marker::Copy for UNIDRVINFO {}
impl ::core::clone::Clone for UNIDRVINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UNIDRVINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct UNIDRV_PRIVATE_DEVMODE {
    pub wReserved: [u16; 4],
    pub wSize: u16,
}
impl ::core::marker::Copy for UNIDRV_PRIVATE_DEVMODE {}
impl ::core::clone::Clone for UNIDRV_PRIVATE_DEVMODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UNIDRV_PRIVATE_DEVMODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct UNIFM_HDR {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub ulDefaultCodepage: u32,
    pub lGlyphSetDataRCID: i32,
    pub loUnidrvInfo: u32,
    pub loIFIMetrics: u32,
    pub loExtTextMetric: u32,
    pub loWidthTable: u32,
    pub loKernPair: u32,
    pub dwReserved: [u32; 2],
}
impl ::core::marker::Copy for UNIFM_HDR {}
impl ::core::clone::Clone for UNIFM_HDR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UNIFM_HDR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct UNI_CODEPAGEINFO {
    pub dwCodePage: u32,
    pub SelectSymbolSet: INVOC,
    pub UnSelectSymbolSet: INVOC,
}
impl ::core::marker::Copy for UNI_CODEPAGEINFO {}
impl ::core::clone::Clone for UNI_CODEPAGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UNI_CODEPAGEINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct UNI_GLYPHSETDATA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub lPredefinedID: i32,
    pub dwGlyphCount: u32,
    pub dwRunCount: u32,
    pub loRunOffset: u32,
    pub dwCodePageCount: u32,
    pub loCodePageOffset: u32,
    pub loMapTableOffset: u32,
    pub dwReserved: [u32; 2],
}
impl ::core::marker::Copy for UNI_GLYPHSETDATA {}
impl ::core::clone::Clone for UNI_GLYPHSETDATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UNI_GLYPHSETDATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct USERDATA {
    pub dwSize: u32,
    pub dwItemID: usize,
    pub pKeyWordName: ::windows::core::PSTR,
    pub dwReserved: [u32; 8],
}
impl ::core::marker::Copy for USERDATA {}
impl ::core::clone::Clone for USERDATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USERDATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct WIDTHRUN {
    pub wStartGlyph: u16,
    pub wGlyphCount: u16,
    pub loCharWidthOffset: u32,
}
impl ::core::marker::Copy for WIDTHRUN {}
impl ::core::clone::Clone for WIDTHRUN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WIDTHRUN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct WIDTHTABLE {
    pub dwSize: u32,
    pub dwRunNum: u32,
    pub WidthRun: [WIDTHRUN; 1],
}
impl ::core::marker::Copy for WIDTHTABLE {}
impl ::core::clone::Clone for WIDTHTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WIDTHTABLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`*"]
pub struct _SPLCLIENT_INFO_2_V3 {
    pub hSplPrinter: u64,
}
impl ::core::marker::Copy for _SPLCLIENT_INFO_2_V3 {}
impl ::core::clone::Clone for _SPLCLIENT_INFO_2_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for _SPLCLIENT_INFO_2_V3 {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type EMFPLAYPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::Gdi::HDC, param1: i32, param2: super::super::Foundation::HANDLE) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type OEMCUIPCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut CPSUICBPARAM, param1: *mut OEMCUIPPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNCOMPROPSHEET = ::core::option::Option<unsafe extern "system" fn(hcompropsheet: super::super::Foundation::HANDLE, function: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> isize>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNPROPSHEETUI = ::core::option::Option<unsafe extern "system" fn(ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvGetDriverSetting = ::core::option::Option<unsafe extern "system" fn(pdriverobj: *mut ::core::ffi::c_void, feature: ::windows::core::PCSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvUpdateUISetting = ::core::option::Option<unsafe extern "system" fn(pdriverobj: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvUpgradeRegistrySetting = ::core::option::Option<unsafe extern "system" fn(hprinter: super::super::Foundation::HANDLE, pfeature: ::windows::core::PCSTR, poption: ::windows::core::PCSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ROUTER_NOTIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwcommand: u32, pcontext: *const ::core::ffi::c_void, dwcolor: u32, pnofityinfo: *const PRINTER_NOTIFY_INFO, fdwflags: u32, pdwresult: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type _CPSUICALLBACK = ::core::option::Option<unsafe extern "system" fn(pcpsuicbparam: *mut CPSUICBPARAM) -> i32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
