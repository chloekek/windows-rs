#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccNotifyTouchInteraction<P0, P1>(hwndapp: P0, hwndtarget: P1, pttarget: super::super::Foundation::POINT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn AccNotifyTouchInteraction ( hwndapp : super::super::Foundation:: HWND , hwndtarget : super::super::Foundation:: HWND , pttarget : super::super::Foundation:: POINT ) -> :: windows::core::HRESULT );
    AccNotifyTouchInteraction(hwndapp.into(), hwndtarget.into(), ::core::mem::transmute(pttarget)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccSetRunningUtilityState<P0>(hwndapp: P0, dwutilitystatemask: u32, dwutilitystate: ACC_UTILITY_STATE_FLAGS) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn AccSetRunningUtilityState ( hwndapp : super::super::Foundation:: HWND , dwutilitystatemask : u32 , dwutilitystate : ACC_UTILITY_STATE_FLAGS ) -> :: windows::core::HRESULT );
    AccSetRunningUtilityState(hwndapp.into(), dwutilitystatemask, dwutilitystate).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn AccessibleChildren<P0>(pacccontainer: P0, ichildstart: i32, rgvarchildren: &mut [super::super::System::Com::VARIANT], pcobtained: *mut i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn AccessibleChildren ( pacccontainer : * mut::core::ffi::c_void , ichildstart : i32 , cchildren : i32 , rgvarchildren : *mut super::super::System::Com:: VARIANT , pcobtained : *mut i32 ) -> :: windows::core::HRESULT );
    AccessibleChildren(pacccontainer.into().abi(), ichildstart, rgvarchildren.len() as _, ::core::mem::transmute(rgvarchildren.as_ptr()), pcobtained).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn AccessibleObjectFromEvent<P0>(hwnd: P0, dwid: u32, dwchildid: u32, ppacc: *mut ::core::option::Option<IAccessible>, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn AccessibleObjectFromEvent ( hwnd : super::super::Foundation:: HWND , dwid : u32 , dwchildid : u32 , ppacc : *mut * mut::core::ffi::c_void , pvarchild : *mut super::super::System::Com:: VARIANT ) -> :: windows::core::HRESULT );
    AccessibleObjectFromEvent(hwnd.into(), dwid, dwchildid, ::core::mem::transmute(ppacc), pvarchild).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn AccessibleObjectFromPoint(ptscreen: super::super::Foundation::POINT, ppacc: *mut ::core::option::Option<IAccessible>, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleacc.dll""system" fn AccessibleObjectFromPoint ( ptscreen : super::super::Foundation:: POINT , ppacc : *mut * mut::core::ffi::c_void , pvarchild : *mut super::super::System::Com:: VARIANT ) -> :: windows::core::HRESULT );
    AccessibleObjectFromPoint(::core::mem::transmute(ptscreen), ::core::mem::transmute(ppacc), pvarchild).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessibleObjectFromWindow<P0>(hwnd: P0, dwid: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn AccessibleObjectFromWindow ( hwnd : super::super::Foundation:: HWND , dwid : u32 , riid : *const :: windows::core::GUID , ppvobject : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    AccessibleObjectFromWindow(hwnd.into(), dwid, riid, ppvobject).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStdAccessibleObject<P0>(hwnd: P0, idobject: i32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn CreateStdAccessibleObject ( hwnd : super::super::Foundation:: HWND , idobject : i32 , riid : *const :: windows::core::GUID , ppvobject : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CreateStdAccessibleObject(hwnd.into(), idobject, riid, ppvobject).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStdAccessibleProxyA<P0, P1>(hwnd: P0, pclassname: P1, idobject: i32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn CreateStdAccessibleProxyA ( hwnd : super::super::Foundation:: HWND , pclassname : :: windows::core::PCSTR , idobject : i32 , riid : *const :: windows::core::GUID , ppvobject : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CreateStdAccessibleProxyA(hwnd.into(), pclassname.into().abi(), idobject, riid, ppvobject).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStdAccessibleProxyW<P0, P1>(hwnd: P0, pclassname: P1, idobject: i32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn CreateStdAccessibleProxyW ( hwnd : super::super::Foundation:: HWND , pclassname : :: windows::core::PCWSTR , idobject : i32 , riid : *const :: windows::core::GUID , ppvobject : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CreateStdAccessibleProxyW(hwnd.into(), pclassname.into().abi(), idobject, riid, ppvobject).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn DockPattern_SetDockPosition<P0>(hobj: P0, dockposition: DockPosition) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn DockPattern_SetDockPosition ( hobj : HUIAPATTERNOBJECT , dockposition : DockPosition ) -> :: windows::core::HRESULT );
    DockPattern_SetDockPosition(hobj.into(), dockposition).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn ExpandCollapsePattern_Collapse<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn ExpandCollapsePattern_Collapse ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    ExpandCollapsePattern_Collapse(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn ExpandCollapsePattern_Expand<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn ExpandCollapsePattern_Expand ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    ExpandCollapsePattern_Expand(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn GetOleaccVersionInfo(pver: *mut u32, pbuild: *mut u32) {
    ::windows::core::link ! ( "oleacc.dll""system" fn GetOleaccVersionInfo ( pver : *mut u32 , pbuild : *mut u32 ) -> ( ) );
    GetOleaccVersionInfo(pver, pbuild)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn GetRoleTextA(lrole: u32, lpszrole: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows::core::link ! ( "oleacc.dll""system" fn GetRoleTextA ( lrole : u32 , lpszrole : :: windows::core::PSTR , cchrolemax : u32 ) -> u32 );
    GetRoleTextA(lrole, ::core::mem::transmute(lpszrole.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszrole.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn GetRoleTextW(lrole: u32, lpszrole: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows::core::link ! ( "oleacc.dll""system" fn GetRoleTextW ( lrole : u32 , lpszrole : :: windows::core::PWSTR , cchrolemax : u32 ) -> u32 );
    GetRoleTextW(lrole, ::core::mem::transmute(lpszrole.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszrole.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn GetStateTextA(lstatebit: u32, lpszstate: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows::core::link ! ( "oleacc.dll""system" fn GetStateTextA ( lstatebit : u32 , lpszstate : :: windows::core::PSTR , cchstate : u32 ) -> u32 );
    GetStateTextA(lstatebit, ::core::mem::transmute(lpszstate.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszstate.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn GetStateTextW(lstatebit: u32, lpszstate: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows::core::link ! ( "oleacc.dll""system" fn GetStateTextW ( lstatebit : u32 , lpszstate : :: windows::core::PWSTR , cchstate : u32 ) -> u32 );
    GetStateTextW(lstatebit, ::core::mem::transmute(lpszstate.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszstate.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn GridPattern_GetItem<P0>(hobj: P0, row: i32, column: i32, presult: *mut HUIANODE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn GridPattern_GetItem ( hobj : HUIAPATTERNOBJECT , row : i32 , column : i32 , presult : *mut HUIANODE ) -> :: windows::core::HRESULT );
    GridPattern_GetItem(hobj.into(), row, column, presult).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn InvokePattern_Invoke<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn InvokePattern_Invoke ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    InvokePattern_Invoke(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWinEventHookInstalled(event: u32) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "user32.dll""system" fn IsWinEventHookInstalled ( event : u32 ) -> super::super::Foundation:: BOOL );
    IsWinEventHookInstalled(event)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ItemContainerPattern_FindItemByProperty<P0, P1>(hobj: P0, hnodestartafter: P1, propertyid: i32, value: super::super::System::Com::VARIANT, pfound: *mut HUIANODE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
    P1: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn ItemContainerPattern_FindItemByProperty ( hobj : HUIAPATTERNOBJECT , hnodestartafter : HUIANODE , propertyid : i32 , value : super::super::System::Com:: VARIANT , pfound : *mut HUIANODE ) -> :: windows::core::HRESULT );
    ItemContainerPattern_FindItemByProperty(hobj.into(), hnodestartafter.into(), propertyid, ::core::mem::transmute(value), pfound).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn LegacyIAccessiblePattern_DoDefaultAction<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn LegacyIAccessiblePattern_DoDefaultAction ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    LegacyIAccessiblePattern_DoDefaultAction(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LegacyIAccessiblePattern_GetIAccessible<P0>(hobj: P0) -> ::windows::core::Result<IAccessible>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn LegacyIAccessiblePattern_GetIAccessible ( hobj : HUIAPATTERNOBJECT , paccessible : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    LegacyIAccessiblePattern_GetIAccessible(hobj.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn LegacyIAccessiblePattern_Select<P0>(hobj: P0, flagsselect: i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn LegacyIAccessiblePattern_Select ( hobj : HUIAPATTERNOBJECT , flagsselect : i32 ) -> :: windows::core::HRESULT );
    LegacyIAccessiblePattern_Select(hobj.into(), flagsselect).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn LegacyIAccessiblePattern_SetValue<P0, P1>(hobj: P0, szvalue: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn LegacyIAccessiblePattern_SetValue ( hobj : HUIAPATTERNOBJECT , szvalue : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    LegacyIAccessiblePattern_SetValue(hobj.into(), szvalue.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LresultFromObject<P0, P1>(riid: *const ::windows::core::GUID, wparam: P0, punk: P1) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn LresultFromObject ( riid : *const :: windows::core::GUID , wparam : super::super::Foundation:: WPARAM , punk : * mut::core::ffi::c_void ) -> super::super::Foundation:: LRESULT );
    LresultFromObject(riid, wparam.into(), punk.into().abi())
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn MultipleViewPattern_GetViewName<P0>(hobj: P0, viewid: i32, ppstr: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn MultipleViewPattern_GetViewName ( hobj : HUIAPATTERNOBJECT , viewid : i32 , ppstr : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    MultipleViewPattern_GetViewName(hobj.into(), viewid, ::core::mem::transmute(ppstr)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn MultipleViewPattern_SetCurrentView<P0>(hobj: P0, viewid: i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn MultipleViewPattern_SetCurrentView ( hobj : HUIAPATTERNOBJECT , viewid : i32 ) -> :: windows::core::HRESULT );
    MultipleViewPattern_SetCurrentView(hobj.into(), viewid).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NotifyWinEvent<P0>(event: u32, hwnd: P0, idobject: i32, idchild: i32)
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn NotifyWinEvent ( event : u32 , hwnd : super::super::Foundation:: HWND , idobject : i32 , idchild : i32 ) -> ( ) );
    NotifyWinEvent(event, hwnd.into(), idobject, idchild)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectFromLresult<P0, P1>(lresult: P0, riid: *const ::windows::core::GUID, wparam: P1, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::LRESULT>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn ObjectFromLresult ( lresult : super::super::Foundation:: LRESULT , riid : *const :: windows::core::GUID , wparam : super::super::Foundation:: WPARAM , ppvobject : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    ObjectFromLresult(lresult.into(), riid, wparam.into(), ppvobject).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn RangeValuePattern_SetValue<P0>(hobj: P0, val: f64) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn RangeValuePattern_SetValue ( hobj : HUIAPATTERNOBJECT , val : f64 ) -> :: windows::core::HRESULT );
    RangeValuePattern_SetValue(hobj.into(), val).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn RegisterPointerInputTarget<P0>(hwnd: P0, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn RegisterPointerInputTarget ( hwnd : super::super::Foundation:: HWND , pointertype : super::WindowsAndMessaging:: POINTER_INPUT_TYPE ) -> super::super::Foundation:: BOOL );
    RegisterPointerInputTarget(hwnd.into(), pointertype)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn RegisterPointerInputTargetEx<P0, P1>(hwnd: P0, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, fobserve: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "user32.dll""system" fn RegisterPointerInputTargetEx ( hwnd : super::super::Foundation:: HWND , pointertype : super::WindowsAndMessaging:: POINTER_INPUT_TYPE , fobserve : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    RegisterPointerInputTargetEx(hwnd.into(), pointertype, fobserve.into())
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn ScrollItemPattern_ScrollIntoView<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn ScrollItemPattern_ScrollIntoView ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    ScrollItemPattern_ScrollIntoView(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn ScrollPattern_Scroll<P0>(hobj: P0, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn ScrollPattern_Scroll ( hobj : HUIAPATTERNOBJECT , horizontalamount : ScrollAmount , verticalamount : ScrollAmount ) -> :: windows::core::HRESULT );
    ScrollPattern_Scroll(hobj.into(), horizontalamount, verticalamount).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn ScrollPattern_SetScrollPercent<P0>(hobj: P0, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn ScrollPattern_SetScrollPercent ( hobj : HUIAPATTERNOBJECT , horizontalpercent : f64 , verticalpercent : f64 ) -> :: windows::core::HRESULT );
    ScrollPattern_SetScrollPercent(hobj.into(), horizontalpercent, verticalpercent).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn SelectionItemPattern_AddToSelection<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn SelectionItemPattern_AddToSelection ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    SelectionItemPattern_AddToSelection(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn SelectionItemPattern_RemoveFromSelection<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn SelectionItemPattern_RemoveFromSelection ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    SelectionItemPattern_RemoveFromSelection(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn SelectionItemPattern_Select<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn SelectionItemPattern_Select ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    SelectionItemPattern_Select(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWinEventHook<P0>(eventmin: u32, eventmax: u32, hmodwineventproc: P0, pfnwineventproc: WINEVENTPROC, idprocess: u32, idthread: u32, dwflags: u32) -> HWINEVENTHOOK
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    ::windows::core::link ! ( "user32.dll""system" fn SetWinEventHook ( eventmin : u32 , eventmax : u32 , hmodwineventproc : super::super::Foundation:: HINSTANCE , pfnwineventproc : WINEVENTPROC , idprocess : u32 , idthread : u32 , dwflags : u32 ) -> HWINEVENTHOOK );
    SetWinEventHook(eventmin, eventmax, hmodwineventproc.into(), pfnwineventproc, idprocess, idthread, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn SynchronizedInputPattern_Cancel<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn SynchronizedInputPattern_Cancel ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    SynchronizedInputPattern_Cancel(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn SynchronizedInputPattern_StartListening<P0>(hobj: P0, inputtype: SynchronizedInputType) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn SynchronizedInputPattern_StartListening ( hobj : HUIAPATTERNOBJECT , inputtype : SynchronizedInputType ) -> :: windows::core::HRESULT );
    SynchronizedInputPattern_StartListening(hobj.into(), inputtype).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn TextPattern_GetSelection<P0>(hobj: P0, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextPattern_GetSelection ( hobj : HUIAPATTERNOBJECT , pretval : *mut *mut super::super::System::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    TextPattern_GetSelection(hobj.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn TextPattern_GetVisibleRanges<P0>(hobj: P0, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextPattern_GetVisibleRanges ( hobj : HUIAPATTERNOBJECT , pretval : *mut *mut super::super::System::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    TextPattern_GetVisibleRanges(hobj.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextPattern_RangeFromChild<P0, P1>(hobj: P0, hnodechild: P1, pretval: *mut HUIATEXTRANGE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
    P1: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextPattern_RangeFromChild ( hobj : HUIAPATTERNOBJECT , hnodechild : HUIANODE , pretval : *mut HUIATEXTRANGE ) -> :: windows::core::HRESULT );
    TextPattern_RangeFromChild(hobj.into(), hnodechild.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextPattern_RangeFromPoint<P0>(hobj: P0, point: UiaPoint, pretval: *mut HUIATEXTRANGE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextPattern_RangeFromPoint ( hobj : HUIAPATTERNOBJECT , point : UiaPoint , pretval : *mut HUIATEXTRANGE ) -> :: windows::core::HRESULT );
    TextPattern_RangeFromPoint(hobj.into(), ::core::mem::transmute(point), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextPattern_get_DocumentRange<P0>(hobj: P0, pretval: *mut HUIATEXTRANGE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextPattern_get_DocumentRange ( hobj : HUIAPATTERNOBJECT , pretval : *mut HUIATEXTRANGE ) -> :: windows::core::HRESULT );
    TextPattern_get_DocumentRange(hobj.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextPattern_get_SupportedTextSelection<P0>(hobj: P0, pretval: *mut SupportedTextSelection) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextPattern_get_SupportedTextSelection ( hobj : HUIAPATTERNOBJECT , pretval : *mut SupportedTextSelection ) -> :: windows::core::HRESULT );
    TextPattern_get_SupportedTextSelection(hobj.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_AddToSelection<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_AddToSelection ( hobj : HUIATEXTRANGE ) -> :: windows::core::HRESULT );
    TextRange_AddToSelection(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_Clone<P0>(hobj: P0, pretval: *mut HUIATEXTRANGE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_Clone ( hobj : HUIATEXTRANGE , pretval : *mut HUIATEXTRANGE ) -> :: windows::core::HRESULT );
    TextRange_Clone(hobj.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TextRange_Compare<P0, P1>(hobj: P0, range: P1, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
    P1: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_Compare ( hobj : HUIATEXTRANGE , range : HUIATEXTRANGE , pretval : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    TextRange_Compare(hobj.into(), range.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_CompareEndpoints<P0, P1>(hobj: P0, endpoint: TextPatternRangeEndpoint, targetrange: P1, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
    P1: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_CompareEndpoints ( hobj : HUIATEXTRANGE , endpoint : TextPatternRangeEndpoint , targetrange : HUIATEXTRANGE , targetendpoint : TextPatternRangeEndpoint , pretval : *mut i32 ) -> :: windows::core::HRESULT );
    TextRange_CompareEndpoints(hobj.into(), endpoint, targetrange.into(), targetendpoint, pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_ExpandToEnclosingUnit<P0>(hobj: P0, unit: TextUnit) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_ExpandToEnclosingUnit ( hobj : HUIATEXTRANGE , unit : TextUnit ) -> :: windows::core::HRESULT );
    TextRange_ExpandToEnclosingUnit(hobj.into(), unit).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn TextRange_FindAttribute<P0, P1>(hobj: P0, attributeid: i32, val: super::super::System::Com::VARIANT, backward: P1, pretval: *mut HUIATEXTRANGE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_FindAttribute ( hobj : HUIATEXTRANGE , attributeid : i32 , val : super::super::System::Com:: VARIANT , backward : super::super::Foundation:: BOOL , pretval : *mut HUIATEXTRANGE ) -> :: windows::core::HRESULT );
    TextRange_FindAttribute(hobj.into(), attributeid, ::core::mem::transmute(val), backward.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TextRange_FindText<P0, P1, P2>(hobj: P0, text: &::windows::core::BSTR, backward: P1, ignorecase: P2, pretval: *mut HUIATEXTRANGE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_FindText ( hobj : HUIATEXTRANGE , text : * mut::core::ffi::c_void , backward : super::super::Foundation:: BOOL , ignorecase : super::super::Foundation:: BOOL , pretval : *mut HUIATEXTRANGE ) -> :: windows::core::HRESULT );
    TextRange_FindText(hobj.into(), ::core::mem::transmute_copy(text), backward.into(), ignorecase.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn TextRange_GetAttributeValue<P0>(hobj: P0, attributeid: i32, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_GetAttributeValue ( hobj : HUIATEXTRANGE , attributeid : i32 , pretval : *mut super::super::System::Com:: VARIANT ) -> :: windows::core::HRESULT );
    TextRange_GetAttributeValue(hobj.into(), attributeid, pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn TextRange_GetBoundingRectangles<P0>(hobj: P0, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_GetBoundingRectangles ( hobj : HUIATEXTRANGE , pretval : *mut *mut super::super::System::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    TextRange_GetBoundingRectangles(hobj.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn TextRange_GetChildren<P0>(hobj: P0, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_GetChildren ( hobj : HUIATEXTRANGE , pretval : *mut *mut super::super::System::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    TextRange_GetChildren(hobj.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_GetEnclosingElement<P0>(hobj: P0, pretval: *mut HUIANODE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_GetEnclosingElement ( hobj : HUIATEXTRANGE , pretval : *mut HUIANODE ) -> :: windows::core::HRESULT );
    TextRange_GetEnclosingElement(hobj.into(), pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_GetText<P0>(hobj: P0, maxlength: i32, pretval: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_GetText ( hobj : HUIATEXTRANGE , maxlength : i32 , pretval : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    TextRange_GetText(hobj.into(), maxlength, ::core::mem::transmute(pretval)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_Move<P0>(hobj: P0, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_Move ( hobj : HUIATEXTRANGE , unit : TextUnit , count : i32 , pretval : *mut i32 ) -> :: windows::core::HRESULT );
    TextRange_Move(hobj.into(), unit, count, pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_MoveEndpointByRange<P0, P1>(hobj: P0, endpoint: TextPatternRangeEndpoint, targetrange: P1, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
    P1: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_MoveEndpointByRange ( hobj : HUIATEXTRANGE , endpoint : TextPatternRangeEndpoint , targetrange : HUIATEXTRANGE , targetendpoint : TextPatternRangeEndpoint ) -> :: windows::core::HRESULT );
    TextRange_MoveEndpointByRange(hobj.into(), endpoint, targetrange.into(), targetendpoint).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_MoveEndpointByUnit<P0>(hobj: P0, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_MoveEndpointByUnit ( hobj : HUIATEXTRANGE , endpoint : TextPatternRangeEndpoint , unit : TextUnit , count : i32 , pretval : *mut i32 ) -> :: windows::core::HRESULT );
    TextRange_MoveEndpointByUnit(hobj.into(), endpoint, unit, count, pretval).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_RemoveFromSelection<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_RemoveFromSelection ( hobj : HUIATEXTRANGE ) -> :: windows::core::HRESULT );
    TextRange_RemoveFromSelection(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TextRange_ScrollIntoView<P0, P1>(hobj: P0, aligntotop: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_ScrollIntoView ( hobj : HUIATEXTRANGE , aligntotop : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    TextRange_ScrollIntoView(hobj.into(), aligntotop.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TextRange_Select<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TextRange_Select ( hobj : HUIATEXTRANGE ) -> :: windows::core::HRESULT );
    TextRange_Select(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TogglePattern_Toggle<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TogglePattern_Toggle ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    TogglePattern_Toggle(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TransformPattern_Move<P0>(hobj: P0, x: f64, y: f64) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TransformPattern_Move ( hobj : HUIAPATTERNOBJECT , x : f64 , y : f64 ) -> :: windows::core::HRESULT );
    TransformPattern_Move(hobj.into(), x, y).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TransformPattern_Resize<P0>(hobj: P0, width: f64, height: f64) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TransformPattern_Resize ( hobj : HUIAPATTERNOBJECT , width : f64 , height : f64 ) -> :: windows::core::HRESULT );
    TransformPattern_Resize(hobj.into(), width, height).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn TransformPattern_Rotate<P0>(hobj: P0, degrees: f64) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn TransformPattern_Rotate ( hobj : HUIAPATTERNOBJECT , degrees : f64 ) -> :: windows::core::HRESULT );
    TransformPattern_Rotate(hobj.into(), degrees).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaAddEvent<P0>(hnode: P0, eventid: i32, pcallback: *mut UiaEventCallback, scope: TreeScope, pproperties: *mut i32, cproperties: i32, prequest: *mut UiaCacheRequest, phevent: *mut HUIAEVENT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaAddEvent ( hnode : HUIANODE , eventid : i32 , pcallback : *mut UiaEventCallback , scope : TreeScope , pproperties : *mut i32 , cproperties : i32 , prequest : *mut UiaCacheRequest , phevent : *mut HUIAEVENT ) -> :: windows::core::HRESULT );
    UiaAddEvent(hnode.into(), eventid, pcallback, scope, pproperties, cproperties, prequest, phevent).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaClientsAreListening() -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaClientsAreListening ( ) -> super::super::Foundation:: BOOL );
    UiaClientsAreListening()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaDisconnectAllProviders() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaDisconnectAllProviders ( ) -> :: windows::core::HRESULT );
    UiaDisconnectAllProviders().ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaDisconnectProvider<P0>(pprovider: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaDisconnectProvider ( pprovider : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    UiaDisconnectProvider(pprovider.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaEventAddWindow<P0, P1>(hevent: P0, hwnd: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAEVENT>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaEventAddWindow ( hevent : HUIAEVENT , hwnd : super::super::Foundation:: HWND ) -> :: windows::core::HRESULT );
    UiaEventAddWindow(hevent.into(), hwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaEventRemoveWindow<P0, P1>(hevent: P0, hwnd: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAEVENT>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaEventRemoveWindow ( hevent : HUIAEVENT , hwnd : super::super::Foundation:: HWND ) -> :: windows::core::HRESULT );
    UiaEventRemoveWindow(hevent.into(), hwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn UiaFind<P0>(hnode: P0, pparams: *mut UiaFindParams, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, ppoffsets: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructures: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaFind ( hnode : HUIANODE , pparams : *mut UiaFindParams , prequest : *mut UiaCacheRequest , pprequesteddata : *mut *mut super::super::System::Com:: SAFEARRAY , ppoffsets : *mut *mut super::super::System::Com:: SAFEARRAY , pptreestructures : *mut *mut super::super::System::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    UiaFind(hnode.into(), pparams, prequest, pprequesteddata, ppoffsets, pptreestructures).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaGetErrorDescription(pdescription: *mut ::windows::core::BSTR) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaGetErrorDescription ( pdescription : *mut * mut::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    UiaGetErrorDescription(::core::mem::transmute(pdescription))
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaGetPatternProvider<P0>(hnode: P0, patternid: i32, phobj: *mut HUIAPATTERNOBJECT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaGetPatternProvider ( hnode : HUIANODE , patternid : i32 , phobj : *mut HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    UiaGetPatternProvider(hnode.into(), patternid, phobj).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn UiaGetPropertyValue<P0>(hnode: P0, propertyid: i32, pvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaGetPropertyValue ( hnode : HUIANODE , propertyid : i32 , pvalue : *mut super::super::System::Com:: VARIANT ) -> :: windows::core::HRESULT );
    UiaGetPropertyValue(hnode.into(), propertyid, pvalue).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaGetReservedMixedAttributeValue() -> ::windows::core::Result<::windows::core::IUnknown> {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaGetReservedMixedAttributeValue ( punkmixedattributevalue : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    UiaGetReservedMixedAttributeValue(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaGetReservedNotSupportedValue() -> ::windows::core::Result<::windows::core::IUnknown> {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaGetReservedNotSupportedValue ( punknotsupportedvalue : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    UiaGetReservedNotSupportedValue(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaGetRootNode(phnode: *mut HUIANODE) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaGetRootNode ( phnode : *mut HUIANODE ) -> :: windows::core::HRESULT );
    UiaGetRootNode(phnode).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaGetRuntimeId<P0>(hnode: P0, pruntimeid: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaGetRuntimeId ( hnode : HUIANODE , pruntimeid : *mut *mut super::super::System::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    UiaGetRuntimeId(hnode.into(), pruntimeid).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaGetUpdatedCache<P0>(hnode: P0, prequest: *mut UiaCacheRequest, normalizestate: NormalizeState, pnormalizecondition: *mut UiaCondition, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaGetUpdatedCache ( hnode : HUIANODE , prequest : *mut UiaCacheRequest , normalizestate : NormalizeState , pnormalizecondition : *mut UiaCondition , pprequesteddata : *mut *mut super::super::System::Com:: SAFEARRAY , pptreestructure : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    UiaGetUpdatedCache(hnode.into(), prequest, normalizestate, pnormalizecondition, pprequesteddata, ::core::mem::transmute(pptreestructure)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn UiaHPatternObjectFromVariant(pvar: *mut super::super::System::Com::VARIANT, phobj: *mut HUIAPATTERNOBJECT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaHPatternObjectFromVariant ( pvar : *mut super::super::System::Com:: VARIANT , phobj : *mut HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    UiaHPatternObjectFromVariant(pvar, phobj).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn UiaHTextRangeFromVariant(pvar: *mut super::super::System::Com::VARIANT, phtextrange: *mut HUIATEXTRANGE) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaHTextRangeFromVariant ( pvar : *mut super::super::System::Com:: VARIANT , phtextrange : *mut HUIATEXTRANGE ) -> :: windows::core::HRESULT );
    UiaHTextRangeFromVariant(pvar, phtextrange).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn UiaHUiaNodeFromVariant(pvar: *mut super::super::System::Com::VARIANT, phnode: *mut HUIANODE) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaHUiaNodeFromVariant ( pvar : *mut super::super::System::Com:: VARIANT , phnode : *mut HUIANODE ) -> :: windows::core::HRESULT );
    UiaHUiaNodeFromVariant(pvar, phnode).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaHasServerSideProvider<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaHasServerSideProvider ( hwnd : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    UiaHasServerSideProvider(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaHostProviderFromHwnd<P0>(hwnd: P0) -> ::windows::core::Result<IRawElementProviderSimple>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaHostProviderFromHwnd ( hwnd : super::super::Foundation:: HWND , ppprovider : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    UiaHostProviderFromHwnd(hwnd.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn UiaIAccessibleFromProvider<P0>(pprovider: P0, dwflags: u32, ppaccessible: *mut ::core::option::Option<IAccessible>, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaIAccessibleFromProvider ( pprovider : * mut::core::ffi::c_void , dwflags : u32 , ppaccessible : *mut * mut::core::ffi::c_void , pvarchild : *mut super::super::System::Com:: VARIANT ) -> :: windows::core::HRESULT );
    UiaIAccessibleFromProvider(pprovider.into().abi(), dwflags, ::core::mem::transmute(ppaccessible), pvarchild).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaLookupId(r#type: AutomationIdentifierType, pguid: *const ::windows::core::GUID) -> i32 {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaLookupId ( r#type : AutomationIdentifierType , pguid : *const :: windows::core::GUID ) -> i32 );
    UiaLookupId(r#type, pguid)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaNavigate<P0>(hnode: P0, direction: NavigateDirection, pcondition: *mut UiaCondition, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaNavigate ( hnode : HUIANODE , direction : NavigateDirection , pcondition : *mut UiaCondition , prequest : *mut UiaCacheRequest , pprequesteddata : *mut *mut super::super::System::Com:: SAFEARRAY , pptreestructure : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    UiaNavigate(hnode.into(), direction, pcondition, prequest, pprequesteddata, ::core::mem::transmute(pptreestructure)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaNodeFromFocus(prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaNodeFromFocus ( prequest : *mut UiaCacheRequest , pprequesteddata : *mut *mut super::super::System::Com:: SAFEARRAY , pptreestructure : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    UiaNodeFromFocus(prequest, pprequesteddata, ::core::mem::transmute(pptreestructure)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaNodeFromHandle<P0>(hwnd: P0, phnode: *mut HUIANODE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaNodeFromHandle ( hwnd : super::super::Foundation:: HWND , phnode : *mut HUIANODE ) -> :: windows::core::HRESULT );
    UiaNodeFromHandle(hwnd.into(), phnode).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaNodeFromPoint(x: f64, y: f64, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaNodeFromPoint ( x : f64 , y : f64 , prequest : *mut UiaCacheRequest , pprequesteddata : *mut *mut super::super::System::Com:: SAFEARRAY , pptreestructure : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    UiaNodeFromPoint(x, y, prequest, pprequesteddata, ::core::mem::transmute(pptreestructure)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaNodeFromProvider<P0>(pprovider: P0, phnode: *mut HUIANODE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaNodeFromProvider ( pprovider : * mut::core::ffi::c_void , phnode : *mut HUIANODE ) -> :: windows::core::HRESULT );
    UiaNodeFromProvider(pprovider.into().abi(), phnode).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaNodeRelease<P0>(hnode: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaNodeRelease ( hnode : HUIANODE ) -> super::super::Foundation:: BOOL );
    UiaNodeRelease(hnode.into())
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaPatternRelease<P0>(hobj: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaPatternRelease ( hobj : HUIAPATTERNOBJECT ) -> super::super::Foundation:: BOOL );
    UiaPatternRelease(hobj.into())
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaProviderForNonClient<P0>(hwnd: P0, idobject: i32, idchild: i32) -> ::windows::core::Result<IRawElementProviderSimple>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaProviderForNonClient ( hwnd : super::super::Foundation:: HWND , idobject : i32 , idchild : i32 , ppprovider : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    UiaProviderForNonClient(hwnd.into(), idobject, idchild, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaProviderFromIAccessible<P0>(paccessible: P0, idchild: i32, dwflags: u32) -> ::windows::core::Result<IRawElementProviderSimple>
where
    P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaProviderFromIAccessible ( paccessible : * mut::core::ffi::c_void , idchild : i32 , dwflags : u32 , ppprovider : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    UiaProviderFromIAccessible(paccessible.into().abi(), idchild, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaRaiseActiveTextPositionChangedEvent<P0, P1>(provider: P0, textrange: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    P1: ::std::convert::Into<::windows::core::InParam<ITextRangeProvider>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaRaiseActiveTextPositionChangedEvent ( provider : * mut::core::ffi::c_void , textrange : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    UiaRaiseActiveTextPositionChangedEvent(provider.into().abi(), textrange.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaRaiseAsyncContentLoadedEvent<P0>(pprovider: P0, asynccontentloadedstate: AsyncContentLoadedState, percentcomplete: f64) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaRaiseAsyncContentLoadedEvent ( pprovider : * mut::core::ffi::c_void , asynccontentloadedstate : AsyncContentLoadedState , percentcomplete : f64 ) -> :: windows::core::HRESULT );
    UiaRaiseAsyncContentLoadedEvent(pprovider.into().abi(), asynccontentloadedstate, percentcomplete).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaRaiseAutomationEvent<P0>(pprovider: P0, id: UIA_EVENT_ID) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaRaiseAutomationEvent ( pprovider : * mut::core::ffi::c_void , id : UIA_EVENT_ID ) -> :: windows::core::HRESULT );
    UiaRaiseAutomationEvent(pprovider.into().abi(), id).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn UiaRaiseAutomationPropertyChangedEvent<P0>(pprovider: P0, id: UIA_PROPERTY_ID, oldvalue: super::super::System::Com::VARIANT, newvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaRaiseAutomationPropertyChangedEvent ( pprovider : * mut::core::ffi::c_void , id : UIA_PROPERTY_ID , oldvalue : super::super::System::Com:: VARIANT , newvalue : super::super::System::Com:: VARIANT ) -> :: windows::core::HRESULT );
    UiaRaiseAutomationPropertyChangedEvent(pprovider.into().abi(), id, ::core::mem::transmute(oldvalue), ::core::mem::transmute(newvalue)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn UiaRaiseChangesEvent<P0>(pprovider: P0, eventidcount: i32, puiachanges: *mut UiaChangeInfo) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaRaiseChangesEvent ( pprovider : * mut::core::ffi::c_void , eventidcount : i32 , puiachanges : *mut UiaChangeInfo ) -> :: windows::core::HRESULT );
    UiaRaiseChangesEvent(pprovider.into().abi(), eventidcount, puiachanges).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaRaiseNotificationEvent<P0>(provider: P0, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: &::windows::core::BSTR, activityid: &::windows::core::BSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaRaiseNotificationEvent ( provider : * mut::core::ffi::c_void , notificationkind : NotificationKind , notificationprocessing : NotificationProcessing , displaystring : * mut::core::ffi::c_void , activityid : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    UiaRaiseNotificationEvent(provider.into().abi(), notificationkind, notificationprocessing, ::core::mem::transmute_copy(displaystring), ::core::mem::transmute_copy(activityid)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaRaiseStructureChangedEvent<P0>(pprovider: P0, structurechangetype: StructureChangeType, pruntimeid: *mut i32, cruntimeidlen: i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaRaiseStructureChangedEvent ( pprovider : * mut::core::ffi::c_void , structurechangetype : StructureChangeType , pruntimeid : *mut i32 , cruntimeidlen : i32 ) -> :: windows::core::HRESULT );
    UiaRaiseStructureChangedEvent(pprovider.into().abi(), structurechangetype, pruntimeid, cruntimeidlen).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UiaRaiseTextEditTextChangedEvent<P0>(pprovider: P0, texteditchangetype: TextEditChangeType, pchangeddata: *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaRaiseTextEditTextChangedEvent ( pprovider : * mut::core::ffi::c_void , texteditchangetype : TextEditChangeType , pchangeddata : *mut super::super::System::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    UiaRaiseTextEditTextChangedEvent(pprovider.into().abi(), texteditchangetype, pchangeddata).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn UiaRegisterProviderCallback(pcallback: *mut UiaProviderCallback) {
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaRegisterProviderCallback ( pcallback : *mut UiaProviderCallback ) -> ( ) );
    UiaRegisterProviderCallback(pcallback)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaRemoveEvent<P0>(hevent: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAEVENT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaRemoveEvent ( hevent : HUIAEVENT ) -> :: windows::core::HRESULT );
    UiaRemoveEvent(hevent.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaReturnRawElementProvider<P0, P1, P2, P3>(hwnd: P0, wparam: P1, lparam: P2, el: P3) -> super::super::Foundation::LRESULT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    P3: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaReturnRawElementProvider ( hwnd : super::super::Foundation:: HWND , wparam : super::super::Foundation:: WPARAM , lparam : super::super::Foundation:: LPARAM , el : * mut::core::ffi::c_void ) -> super::super::Foundation:: LRESULT );
    UiaReturnRawElementProvider(hwnd.into(), wparam.into(), lparam.into(), el.into().abi())
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn UiaSetFocus<P0>(hnode: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIANODE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaSetFocus ( hnode : HUIANODE ) -> :: windows::core::HRESULT );
    UiaSetFocus(hnode.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UiaTextRangeRelease<P0>(hobj: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HUIATEXTRANGE>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn UiaTextRangeRelease ( hobj : HUIATEXTRANGE ) -> super::super::Foundation:: BOOL );
    UiaTextRangeRelease(hobj.into())
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnhookWinEvent<P0>(hwineventhook: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HWINEVENTHOOK>,
{
    ::windows::core::link ! ( "user32.dll""system" fn UnhookWinEvent ( hwineventhook : HWINEVENTHOOK ) -> super::super::Foundation:: BOOL );
    UnhookWinEvent(hwineventhook.into())
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn UnregisterPointerInputTarget<P0>(hwnd: P0, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn UnregisterPointerInputTarget ( hwnd : super::super::Foundation:: HWND , pointertype : super::WindowsAndMessaging:: POINTER_INPUT_TYPE ) -> super::super::Foundation:: BOOL );
    UnregisterPointerInputTarget(hwnd.into(), pointertype)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn UnregisterPointerInputTargetEx<P0>(hwnd: P0, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn UnregisterPointerInputTargetEx ( hwnd : super::super::Foundation:: HWND , pointertype : super::WindowsAndMessaging:: POINTER_INPUT_TYPE ) -> super::super::Foundation:: BOOL );
    UnregisterPointerInputTargetEx(hwnd.into(), pointertype)
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn ValuePattern_SetValue<P0, P1>(hobj: P0, pval: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn ValuePattern_SetValue ( hobj : HUIAPATTERNOBJECT , pval : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    ValuePattern_SetValue(hobj.into(), pval.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn VirtualizedItemPattern_Realize<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn VirtualizedItemPattern_Realize ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    VirtualizedItemPattern_Realize(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn WindowFromAccessibleObject<P0>(param0: P0, phwnd: ::core::option::Option<*mut super::super::Foundation::HWND>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
{
    ::windows::core::link ! ( "oleacc.dll""system" fn WindowFromAccessibleObject ( param0 : * mut::core::ffi::c_void , phwnd : *mut super::super::Foundation:: HWND ) -> :: windows::core::HRESULT );
    WindowFromAccessibleObject(param0.into().abi(), ::core::mem::transmute(phwnd.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn WindowPattern_Close<P0>(hobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn WindowPattern_Close ( hobj : HUIAPATTERNOBJECT ) -> :: windows::core::HRESULT );
    WindowPattern_Close(hobj.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[inline]
pub unsafe fn WindowPattern_SetWindowVisualState<P0>(hobj: P0, state: WindowVisualState) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn WindowPattern_SetWindowVisualState ( hobj : HUIAPATTERNOBJECT , state : WindowVisualState ) -> :: windows::core::HRESULT );
    WindowPattern_SetWindowVisualState(hobj.into(), state).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WindowPattern_WaitForInputIdle<P0>(hobj: P0, milliseconds: i32, presult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HUIAPATTERNOBJECT>,
{
    ::windows::core::link ! ( "uiautomationcore.dll""system" fn WindowPattern_WaitForInputIdle ( hobj : HUIAPATTERNOBJECT , milliseconds : i32 , presult : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    WindowPattern_WaitForInputIdle(hobj.into(), milliseconds, presult).ok()
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IAccIdentity(::windows::core::IUnknown);
impl IAccIdentity {
    pub unsafe fn GetIdentityString(&self, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetIdentityString)(::windows::core::Vtable::as_raw(self), dwidchild, ppidstring, pdwidstringlen).ok()
    }
}
::windows::core::interface_hierarchy!(IAccIdentity, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAccIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAccIdentity {
    type Vtable = IAccIdentity_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccIdentity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7852b78d_1cfd_41c1_a615_9c0c85960b5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccIdentity_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetIdentityString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IAccPropServer(::windows::core::IUnknown);
impl IAccPropServer {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPropValue(&self, pidstring: &[u8], idprop: ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pidstring.as_ptr()), pidstring.len() as _, ::core::mem::transmute(idprop), pvarvalue, pfhasprop).ok()
    }
}
::windows::core::interface_hierarchy!(IAccPropServer, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAccPropServer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAccPropServer {
    type Vtable = IAccPropServer_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccPropServer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76c0dbbb_15e0_4e7b_b61b_20eeea2001e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccPropServer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPropValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPropValue: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IAccPropServices(::windows::core::IUnknown);
impl IAccPropServices {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetPropValue(&self, pidstring: &[u8], idprop: ::windows::core::GUID, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPropValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pidstring.as_ptr()), pidstring.len() as _, ::core::mem::transmute(idprop), ::core::mem::transmute(var)).ok()
    }
    pub unsafe fn SetPropServer<P0>(&self, pidstring: &[u8], paprops: &[::windows::core::GUID], pserver: P0, annoscope: AnnoScope) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccPropServer>>,
    {
        (::windows::core::Vtable::vtable(self).SetPropServer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pidstring.as_ptr()), pidstring.len() as _, ::core::mem::transmute(paprops.as_ptr()), paprops.len() as _, pserver.into().abi(), annoscope).ok()
    }
    pub unsafe fn ClearProps(&self, pidstring: &[u8], paprops: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ClearProps)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pidstring.as_ptr()), pidstring.len() as _, ::core::mem::transmute(paprops.as_ptr()), paprops.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetHwndProp<P0>(&self, hwnd: P0, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).SetHwndProp)(::windows::core::Vtable::as_raw(self), hwnd.into(), idobject, idchild, ::core::mem::transmute(idprop), ::core::mem::transmute(var)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHwndPropStr<P0, P1>(&self, hwnd: P0, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, str: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetHwndPropStr)(::windows::core::Vtable::as_raw(self), hwnd.into(), idobject, idchild, ::core::mem::transmute(idprop), str.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHwndPropServer<P0, P1>(&self, hwnd: P0, idobject: u32, idchild: u32, paprops: &[::windows::core::GUID], pserver: P1, annoscope: AnnoScope) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IAccPropServer>>,
    {
        (::windows::core::Vtable::vtable(self).SetHwndPropServer)(::windows::core::Vtable::as_raw(self), hwnd.into(), idobject, idchild, ::core::mem::transmute(paprops.as_ptr()), paprops.len() as _, pserver.into().abi(), annoscope).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearHwndProps<P0>(&self, hwnd: P0, idobject: u32, idchild: u32, paprops: &[::windows::core::GUID]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).ClearHwndProps)(::windows::core::Vtable::as_raw(self), hwnd.into(), idobject, idchild, ::core::mem::transmute(paprops.as_ptr()), paprops.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComposeHwndIdentityString<P0>(&self, hwnd: P0, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).ComposeHwndIdentityString)(::windows::core::Vtable::as_raw(self), hwnd.into(), idobject, idchild, ppidstring, pdwidstringlen).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DecomposeHwndIdentityString(&self, pidstring: &[u8], phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DecomposeHwndIdentityString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pidstring.as_ptr()), pidstring.len() as _, phwnd, pidobject, pidchild).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn SetHmenuProp<P0>(&self, hmenu: P0, idchild: u32, idprop: ::windows::core::GUID, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HMENU>,
    {
        (::windows::core::Vtable::vtable(self).SetHmenuProp)(::windows::core::Vtable::as_raw(self), hmenu.into(), idchild, ::core::mem::transmute(idprop), ::core::mem::transmute(var)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetHmenuPropStr<P0, P1>(&self, hmenu: P0, idchild: u32, idprop: ::windows::core::GUID, str: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HMENU>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetHmenuPropStr)(::windows::core::Vtable::as_raw(self), hmenu.into(), idchild, ::core::mem::transmute(idprop), str.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetHmenuPropServer<P0, P1>(&self, hmenu: P0, idchild: u32, paprops: &[::windows::core::GUID], pserver: P1, annoscope: AnnoScope) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HMENU>,
        P1: ::std::convert::Into<::windows::core::InParam<IAccPropServer>>,
    {
        (::windows::core::Vtable::vtable(self).SetHmenuPropServer)(::windows::core::Vtable::as_raw(self), hmenu.into(), idchild, ::core::mem::transmute(paprops.as_ptr()), paprops.len() as _, pserver.into().abi(), annoscope).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn ClearHmenuProps<P0>(&self, hmenu: P0, idchild: u32, paprops: &[::windows::core::GUID]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HMENU>,
    {
        (::windows::core::Vtable::vtable(self).ClearHmenuProps)(::windows::core::Vtable::as_raw(self), hmenu.into(), idchild, ::core::mem::transmute(paprops.as_ptr()), paprops.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn ComposeHmenuIdentityString<P0>(&self, hmenu: P0, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HMENU>,
    {
        (::windows::core::Vtable::vtable(self).ComposeHmenuIdentityString)(::windows::core::Vtable::as_raw(self), hmenu.into(), idchild, ppidstring, pdwidstringlen).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn DecomposeHmenuIdentityString(&self, pidstring: &[u8], phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DecomposeHmenuIdentityString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pidstring.as_ptr()), pidstring.len() as _, phmenu, pidchild).ok()
    }
}
::windows::core::interface_hierarchy!(IAccPropServices, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAccPropServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAccPropServices {
    type Vtable = IAccPropServices_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccPropServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e26e776_04f0_495d_80e4_3330352e3169);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccPropServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPropValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, var: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPropValue: usize,
    pub SetPropServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: *mut ::core::ffi::c_void, annoscope: AnnoScope) -> ::windows::core::HRESULT,
    pub ClearProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetHwndProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, var: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetHwndProp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHwndPropStr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, str: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHwndPropStr: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHwndPropServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: *mut ::core::ffi::c_void, annoscope: AnnoScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHwndPropServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClearHwndProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClearHwndProps: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ComposeHwndIdentityString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ComposeHwndIdentityString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DecomposeHwndIdentityString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DecomposeHwndIdentityString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub SetHmenuProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, var: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging")))]
    SetHmenuProp: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetHmenuPropStr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, str: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetHmenuPropStr: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetHmenuPropServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: *mut ::core::ffi::c_void, annoscope: AnnoScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetHmenuPropServer: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub ClearHmenuProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    ClearHmenuProps: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub ComposeHmenuIdentityString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    ComposeHmenuIdentityString: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub DecomposeHmenuIdentityString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    DecomposeHmenuIdentityString: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAccessible(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAccessible {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn accParent(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).accParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn accChildCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).accChildCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_accChild(&self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_accChild)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_accName(&self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_accName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_accValue(&self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_accValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_accDescription(&self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_accDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_accRole(&self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_accRole)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_accState(&self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_accState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_accHelp(&self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_accHelp)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_accHelpTopic(&self, pszhelpfile: *mut ::windows::core::BSTR, varchild: super::super::System::Com::VARIANT, pidtopic: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).get_accHelpTopic)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszhelpfile), ::core::mem::transmute(varchild), pidtopic).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_accKeyboardShortcut(&self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_accKeyboardShortcut)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn accFocus(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).accFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn accSelection(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).accSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_accDefaultAction(&self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_accDefaultAction)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn accSelect(&self, flagsselect: i32, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).accSelect)(::windows::core::Vtable::as_raw(self), flagsselect, ::core::mem::transmute(varchild)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn accLocation(&self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).accLocation)(::windows::core::Vtable::as_raw(self), pxleft, pytop, pcxwidth, pcyheight, ::core::mem::transmute(varchild)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn accNavigate(&self, navdir: i32, varstart: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).accNavigate)(::windows::core::Vtable::as_raw(self), navdir, ::core::mem::transmute(varstart), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn accHitTest(&self, xleft: i32, ytop: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).accHitTest)(::windows::core::Vtable::as_raw(self), xleft, ytop, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn accDoDefaultAction(&self, varchild: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).accDoDefaultAction)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn put_accName(&self, varchild: super::super::System::Com::VARIANT, szname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_accName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), ::core::mem::transmute_copy(szname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn put_accValue(&self, varchild: super::super::System::Com::VARIANT, szvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).put_accValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(varchild), ::core::mem::transmute_copy(szvalue)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IAccessible, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAccessible {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IAccessible {
    type Vtable = IAccessible_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAccessible {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x618736e0_3c3d_11cf_810c_00aa00389b71);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAccessible_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub accParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdispparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    accParent: usize,
    pub accChildCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcountchildren: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_accChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, ppdispchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_accChild: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_accName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, pszname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_accName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_accValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, pszvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_accValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_accDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, pszdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_accDescription: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_accRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, pvarrole: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_accRole: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_accState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, pvarstate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_accState: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_accHelp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, pszhelp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_accHelp: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_accHelpTopic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszhelpfile: *mut *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, pidtopic: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_accHelpTopic: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_accKeyboardShortcut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, pszkeyboardshortcut: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_accKeyboardShortcut: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub accFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    accFocus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub accSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarchildren: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    accSelection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_accDefaultAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, pszdefaultaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_accDefaultAction: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub accSelect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flagsselect: i32, varchild: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    accSelect: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub accLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    accLocation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub accNavigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, navdir: i32, varstart: super::super::System::Com::VARIANT, pvarendupat: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    accNavigate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub accHitTest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xleft: i32, ytop: i32, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    accHitTest: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub accDoDefaultAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    accDoDefaultAction: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_accName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, szname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_accName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_accValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varchild: super::super::System::Com::VARIANT, szvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_accValue: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IAccessibleEx(::windows::core::IUnknown);
impl IAccessibleEx {
    pub unsafe fn GetObjectForChild(&self, idchild: i32) -> ::windows::core::Result<IAccessibleEx> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetObjectForChild)(::windows::core::Vtable::as_raw(self), idchild, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetIAccessiblePair(&self, ppacc: *mut ::core::option::Option<IAccessible>, pidchild: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetIAccessiblePair)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppacc), pidchild).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ConvertReturnedElement<P0>(&self, pin: P0) -> ::windows::core::Result<IAccessibleEx>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ConvertReturnedElement)(::windows::core::Vtable::as_raw(self), pin.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAccessibleEx, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAccessibleEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAccessibleEx {
    type Vtable = IAccessibleEx_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccessibleEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8b80ada_2c44_48d0_89be_5ff23c9cd875);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetObjectForChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idchild: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetIAccessiblePair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppacc: *mut *mut ::core::ffi::c_void, pidchild: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetIAccessiblePair: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRuntimeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRuntimeId: usize,
    pub ConvertReturnedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, ppretvalout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IAccessibleHandler(::windows::core::IUnknown);
impl IAccessibleHandler {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AccessibleObjectFromID(&self, hwnd: i32, lobjectid: i32) -> ::windows::core::Result<IAccessible> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AccessibleObjectFromID)(::windows::core::Vtable::as_raw(self), hwnd, lobjectid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAccessibleHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAccessibleHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAccessibleHandler {
    type Vtable = IAccessibleHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccessibleHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03022430_abc4_11d0_bde2_00aa001a1953);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AccessibleObjectFromID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: i32, lobjectid: i32, piaccessible: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AccessibleObjectFromID: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IAccessibleHostingElementProviders(::windows::core::IUnknown);
impl IAccessibleHostingElementProviders {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEmbeddedFragmentRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEmbeddedFragmentRoots)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetObjectIdForProvider<P0>(&self, pprovider: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetObjectIdForProvider)(::windows::core::Vtable::as_raw(self), pprovider.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAccessibleHostingElementProviders, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAccessibleHostingElementProviders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAccessibleHostingElementProviders {
    type Vtable = IAccessibleHostingElementProviders_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccessibleHostingElementProviders {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ac331b_943e_4020_b295_db37784974a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleHostingElementProviders_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEmbeddedFragmentRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEmbeddedFragmentRoots: usize,
    pub GetObjectIdForProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, pidobject: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IAccessibleWindowlessSite(::windows::core::IUnknown);
impl IAccessibleWindowlessSite {
    pub unsafe fn AcquireObjectIdRange<P0>(&self, rangesize: i32, prangeowner: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessibleHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AcquireObjectIdRange)(::windows::core::Vtable::as_raw(self), rangesize, prangeowner.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReleaseObjectIdRange<P0>(&self, rangebase: i32, prangeowner: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessibleHandler>>,
    {
        (::windows::core::Vtable::vtable(self).ReleaseObjectIdRange)(::windows::core::Vtable::as_raw(self), rangebase, prangeowner.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryObjectIdRanges<P0>(&self, prangesowner: P0) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessibleHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QueryObjectIdRanges)(::windows::core::Vtable::as_raw(self), prangesowner.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetParentAccessible(&self) -> ::windows::core::Result<IAccessible> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetParentAccessible)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAccessibleWindowlessSite, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAccessibleWindowlessSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAccessibleWindowlessSite {
    type Vtable = IAccessibleWindowlessSite_Vtbl;
}
unsafe impl ::windows::core::Interface for IAccessibleWindowlessSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf3abd9c_76da_4389_9eb6_1427d25abab7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleWindowlessSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AcquireObjectIdRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangesize: i32, prangeowner: *mut ::core::ffi::c_void, prangebase: *mut i32) -> ::windows::core::HRESULT,
    pub ReleaseObjectIdRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangebase: i32, prangeowner: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryObjectIdRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangesowner: *mut ::core::ffi::c_void, psaranges: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryObjectIdRanges: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParentAccessible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParentAccessible: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IAnnotationProvider(::windows::core::IUnknown);
impl IAnnotationProvider {
    pub unsafe fn AnnotationTypeId(&self) -> ::windows::core::Result<UIA_ANNOTATIONTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AnnotationTypeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AnnotationTypeName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AnnotationTypeName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Author(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Author)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DateTime(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DateTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Target(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Target)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAnnotationProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAnnotationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IAnnotationProvider {
    type Vtable = IAnnotationProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IAnnotationProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf95c7e80_bd63_4601_9782_445ebff011fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AnnotationTypeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_ANNOTATIONTYPE) -> ::windows::core::HRESULT,
    pub AnnotationTypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ICustomNavigationProvider(::windows::core::IUnknown);
impl ICustomNavigationProvider {
    pub unsafe fn Navigate(&self, direction: NavigateDirection) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Navigate)(::windows::core::Vtable::as_raw(self), direction, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ICustomNavigationProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICustomNavigationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ICustomNavigationProvider {
    type Vtable = ICustomNavigationProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ICustomNavigationProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2062a28a_8c07_4b94_8e12_7037c622aeb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomNavigationProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Navigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IDockProvider(::windows::core::IUnknown);
impl IDockProvider {
    pub unsafe fn SetDockPosition(&self, dockposition: DockPosition) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDockPosition)(::windows::core::Vtable::as_raw(self), dockposition).ok()
    }
    pub unsafe fn DockPosition(&self) -> ::windows::core::Result<DockPosition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DockPosition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDockProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDockProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDockProvider {
    type Vtable = IDockProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IDockProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x159bc72c_4ad3_485e_9637_d7052edf0146);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetDockPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dockposition: DockPosition) -> ::windows::core::HRESULT,
    pub DockPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut DockPosition) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IDragProvider(::windows::core::IUnknown);
impl IDragProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGrabbed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsGrabbed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DropEffect(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DropEffect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DropEffects(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DropEffects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGrabbedItems(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGrabbedItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDragProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDragProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDragProvider {
    type Vtable = IDragProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IDragProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aa7bbbb_7ff9_497d_904f_d20b897929d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsGrabbed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsGrabbed: usize,
    pub DropEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DropEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DropEffects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGrabbedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGrabbedItems: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IDropTargetProvider(::windows::core::IUnknown);
impl IDropTargetProvider {
    pub unsafe fn DropTargetEffect(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DropTargetEffect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DropTargetEffects(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DropTargetEffects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDropTargetProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDropTargetProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IDropTargetProvider {
    type Vtable = IDropTargetProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IDropTargetProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbae82bfd_358a_481c_85a0_d8b4d90a5d61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DropTargetEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DropTargetEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DropTargetEffects: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IExpandCollapseProvider(::windows::core::IUnknown);
impl IExpandCollapseProvider {
    pub unsafe fn Expand(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Expand)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Collapse(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Collapse)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ExpandCollapseState(&self) -> ::windows::core::Result<ExpandCollapseState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExpandCollapseState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IExpandCollapseProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IExpandCollapseProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IExpandCollapseProvider {
    type Vtable = IExpandCollapseProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IExpandCollapseProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd847d3a5_cab0_4a98_8c32_ecb45c59ad24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapseProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Expand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Collapse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExpandCollapseState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ExpandCollapseState) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IGridItemProvider(::windows::core::IUnknown);
impl IGridItemProvider {
    pub unsafe fn Row(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Row)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Column(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Column)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RowSpan(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RowSpan)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ColumnSpan(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ColumnSpan)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContainingGrid(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ContainingGrid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IGridItemProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGridItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IGridItemProvider {
    type Vtable = IGridItemProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IGridItemProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd02541f1_fb81_4d64_ae32_f520f8a6dbd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Row: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub Column: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub RowSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub ColumnSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub ContainingGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IGridProvider(::windows::core::IUnknown);
impl IGridProvider {
    pub unsafe fn GetItem(&self, row: i32, column: i32) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItem)(::windows::core::Vtable::as_raw(self), row, column, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RowCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RowCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ColumnCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ColumnCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IGridProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGridProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IGridProvider {
    type Vtable = IGridProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IGridProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb17d6187_0907_464b_a168_0ef17a1572b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RowCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub ColumnCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IInvokeProvider(::windows::core::IUnknown);
impl IInvokeProvider {
    pub unsafe fn Invoke(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Invoke)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IInvokeProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IInvokeProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IInvokeProvider {
    type Vtable = IInvokeProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IInvokeProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54fcb24b_e18e_47a2_b4d3_eccbe77599a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInvokeProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IItemContainerProvider(::windows::core::IUnknown);
impl IItemContainerProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FindItemByProperty<P0>(&self, pstartafter: P0, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IRawElementProviderSimple>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindItemByProperty)(::windows::core::Vtable::as_raw(self), pstartafter.into().abi(), propertyid, ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IItemContainerProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IItemContainerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IItemContainerProvider {
    type Vtable = IItemContainerProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IItemContainerProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe747770b_39ce_4382_ab30_d8fb3f336f24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemContainerProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FindItemByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstartafter: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT, pfound: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FindItemByProperty: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ILegacyIAccessibleProvider(::windows::core::IUnknown);
impl ILegacyIAccessibleProvider {
    pub unsafe fn Select(&self, flagsselect: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Select)(::windows::core::Vtable::as_raw(self), flagsselect).ok()
    }
    pub unsafe fn DoDefaultAction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DoDefaultAction)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetValue<P0>(&self, szvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), szvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetIAccessible(&self) -> ::windows::core::Result<IAccessible> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIAccessible)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ChildId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ChildId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Role(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Role)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Help(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Help)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn KeyboardShortcut(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).KeyboardShortcut)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DefaultAction(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DefaultAction)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ILegacyIAccessibleProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ILegacyIAccessibleProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ILegacyIAccessibleProvider {
    type Vtable = ILegacyIAccessibleProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ILegacyIAccessibleProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe44c3566_915d_4070_99c6_047bff5a08f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILegacyIAccessibleProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows::core::HRESULT,
    pub DoDefaultAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetIAccessible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaccessible: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetIAccessible: usize,
    pub ChildId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Role: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT,
    pub Help: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszhelp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub KeyboardShortcut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelection: usize,
    pub DefaultAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IMultipleViewProvider(::windows::core::IUnknown);
impl IMultipleViewProvider {
    pub unsafe fn GetViewName(&self, viewid: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetViewName)(::windows::core::Vtable::as_raw(self), viewid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCurrentView(&self, viewid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCurrentView)(::windows::core::Vtable::as_raw(self), viewid).ok()
    }
    pub unsafe fn CurrentView(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentView)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSupportedViews(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSupportedViews)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IMultipleViewProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IMultipleViewProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IMultipleViewProvider {
    type Vtable = IMultipleViewProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IMultipleViewProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6278cab1_b556_4a1a_b4e0_418acc523201);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetViewName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32) -> ::windows::core::HRESULT,
    pub CurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSupportedViews: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSupportedViews: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IObjectModelProvider(::windows::core::IUnknown);
impl IObjectModelProvider {
    pub unsafe fn GetUnderlyingObjectModel(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUnderlyingObjectModel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IObjectModelProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IObjectModelProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IObjectModelProvider {
    type Vtable = IObjectModelProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IObjectModelProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ad86ebd_f5ef_483d_bb18_b1042a475d64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectModelProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUnderlyingObjectModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IProxyProviderWinEventHandler(::windows::core::IUnknown);
impl IProxyProviderWinEventHandler {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RespondToWinEvent<P0, P1>(&self, idwinevent: u32, hwnd: P0, idobject: i32, idchild: i32, psink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IProxyProviderWinEventSink>>,
    {
        (::windows::core::Vtable::vtable(self).RespondToWinEvent)(::windows::core::Vtable::as_raw(self), idwinevent, hwnd.into(), idobject, idchild, psink.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IProxyProviderWinEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProxyProviderWinEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IProxyProviderWinEventHandler {
    type Vtable = IProxyProviderWinEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IProxyProviderWinEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89592ad4_f4e0_43d5_a3b6_bad7e111b435);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProxyProviderWinEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RespondToWinEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RespondToWinEvent: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IProxyProviderWinEventSink(::windows::core::IUnknown);
impl IProxyProviderWinEventSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAutomationPropertyChangedEvent<P0>(&self, pprovider: P0, id: UIA_PROPERTY_ID, newvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    {
        (::windows::core::Vtable::vtable(self).AddAutomationPropertyChangedEvent)(::windows::core::Vtable::as_raw(self), pprovider.into().abi(), id, ::core::mem::transmute(newvalue)).ok()
    }
    pub unsafe fn AddAutomationEvent<P0>(&self, pprovider: P0, id: UIA_EVENT_ID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    {
        (::windows::core::Vtable::vtable(self).AddAutomationEvent)(::windows::core::Vtable::as_raw(self), pprovider.into().abi(), id).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddStructureChangedEvent<P0>(&self, pprovider: P0, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    {
        (::windows::core::Vtable::vtable(self).AddStructureChangedEvent)(::windows::core::Vtable::as_raw(self), pprovider.into().abi(), structurechangetype, runtimeid).ok()
    }
}
::windows::core::interface_hierarchy!(IProxyProviderWinEventSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProxyProviderWinEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IProxyProviderWinEventSink {
    type Vtable = IProxyProviderWinEventSink_Vtbl;
}
unsafe impl ::windows::core::Interface for IProxyProviderWinEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fd82b78_a43e_46ac_9803_0a6969c7c183);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProxyProviderWinEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddAutomationPropertyChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, id: UIA_PROPERTY_ID, newvalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddAutomationPropertyChangedEvent: usize,
    pub AddAutomationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, id: UIA_EVENT_ID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddStructureChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddStructureChangedEvent: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRangeValueProvider(::windows::core::IUnknown);
impl IRangeValueProvider {
    pub unsafe fn SetValue(&self, val: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), val).ok()
    }
    pub unsafe fn Value(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsReadOnly(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsReadOnly)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Maximum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Maximum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Minimum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Minimum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LargeChange(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LargeChange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SmallChange(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SmallChange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRangeValueProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRangeValueProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRangeValueProvider {
    type Vtable = IRangeValueProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IRangeValueProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36dc7aef_33e6_4691_afe1_2be7274b3d33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValueProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: f64) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsReadOnly: usize,
    pub Maximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    pub Minimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    pub LargeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    pub SmallChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRawElementProviderAdviseEvents(::windows::core::IUnknown);
impl IRawElementProviderAdviseEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdviseEventAdded(&self, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AdviseEventAdded)(::windows::core::Vtable::as_raw(self), eventid, propertyids).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdviseEventRemoved(&self, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AdviseEventRemoved)(::windows::core::Vtable::as_raw(self), eventid, propertyids).ok()
    }
}
::windows::core::interface_hierarchy!(IRawElementProviderAdviseEvents, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRawElementProviderAdviseEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRawElementProviderAdviseEvents {
    type Vtable = IRawElementProviderAdviseEvents_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawElementProviderAdviseEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa407b27b_0f6d_4427_9292_473c7bf93258);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderAdviseEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AdviseEventAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdviseEventAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdviseEventRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdviseEventRemoved: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRawElementProviderFragment(::windows::core::IUnknown);
impl IRawElementProviderFragment {
    pub unsafe fn Navigate(&self, direction: NavigateDirection) -> ::windows::core::Result<IRawElementProviderFragment> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Navigate)(::windows::core::Vtable::as_raw(self), direction, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BoundingRectangle(&self) -> ::windows::core::Result<UiaRect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEmbeddedFragmentRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEmbeddedFragmentRoots)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FragmentRoot(&self) -> ::windows::core::Result<IRawElementProviderFragmentRoot> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FragmentRoot)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRawElementProviderFragment, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRawElementProviderFragment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRawElementProviderFragment {
    type Vtable = IRawElementProviderFragment_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawElementProviderFragment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7063da8_8359_439c_9297_bbc5299a7d87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderFragment_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Navigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRuntimeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRuntimeId: usize,
    pub BoundingRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut UiaRect) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEmbeddedFragmentRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEmbeddedFragmentRoots: usize,
    pub SetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FragmentRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRawElementProviderFragmentRoot(::windows::core::IUnknown);
impl IRawElementProviderFragmentRoot {
    pub unsafe fn ElementProviderFromPoint(&self, x: f64, y: f64) -> ::windows::core::Result<IRawElementProviderFragment> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElementProviderFromPoint)(::windows::core::Vtable::as_raw(self), x, y, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocus(&self) -> ::windows::core::Result<IRawElementProviderFragment> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRawElementProviderFragmentRoot, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRawElementProviderFragmentRoot {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRawElementProviderFragmentRoot {
    type Vtable = IRawElementProviderFragmentRoot_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawElementProviderFragmentRoot {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x620ce2a5_ab8f_40a9_86cb_de3c75599b58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderFragmentRoot_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ElementProviderFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f64, y: f64, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRawElementProviderHostingAccessibles(::windows::core::IUnknown);
impl IRawElementProviderHostingAccessibles {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEmbeddedAccessibles(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEmbeddedAccessibles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRawElementProviderHostingAccessibles, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRawElementProviderHostingAccessibles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRawElementProviderHostingAccessibles {
    type Vtable = IRawElementProviderHostingAccessibles_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawElementProviderHostingAccessibles {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24be0b07_d37d_487a_98cf_a13ed465e9b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderHostingAccessibles_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEmbeddedAccessibles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEmbeddedAccessibles: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRawElementProviderHwndOverride(::windows::core::IUnknown);
impl IRawElementProviderHwndOverride {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOverrideProviderForHwnd<P0>(&self, hwnd: P0) -> ::windows::core::Result<IRawElementProviderSimple>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOverrideProviderForHwnd)(::windows::core::Vtable::as_raw(self), hwnd.into(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRawElementProviderHwndOverride, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRawElementProviderHwndOverride {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRawElementProviderHwndOverride {
    type Vtable = IRawElementProviderHwndOverride_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawElementProviderHwndOverride {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d5df27c_8947_4425_b8d9_79787bb460b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderHwndOverride_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOverrideProviderForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOverrideProviderForHwnd: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRawElementProviderSimple(::windows::core::IUnknown);
impl IRawElementProviderSimple {
    pub unsafe fn ProviderOptions(&self) -> ::windows::core::Result<ProviderOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ProviderOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPatternProvider(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPatternProvider)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HostRawElementProvider(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HostRawElementProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRawElementProviderSimple, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRawElementProviderSimple {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRawElementProviderSimple {
    type Vtable = IRawElementProviderSimple_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawElementProviderSimple {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6dd68d1_86fd_4332_8666_9abedea2d24c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderSimple_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ProviderOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ProviderOptions) -> ::windows::core::HRESULT,
    pub GetPatternProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPropertyValue: usize,
    pub HostRawElementProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRawElementProviderSimple2(::windows::core::IUnknown);
impl IRawElementProviderSimple2 {
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IRawElementProviderSimple2, ::windows::core::IUnknown, IRawElementProviderSimple);
impl ::core::clone::Clone for IRawElementProviderSimple2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRawElementProviderSimple2 {
    type Vtable = IRawElementProviderSimple2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawElementProviderSimple2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0a839a9_8da1_4a82_806a_8e0d44e79f56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderSimple2_Vtbl {
    pub base__: IRawElementProviderSimple_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRawElementProviderSimple3(::windows::core::IUnknown);
impl IRawElementProviderSimple3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetMetadataValue(&self, targetid: i32, metadataid: UIA_METADATA_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMetadataValue)(::windows::core::Vtable::as_raw(self), targetid, metadataid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRawElementProviderSimple3, ::windows::core::IUnknown, IRawElementProviderSimple, IRawElementProviderSimple2);
impl ::core::clone::Clone for IRawElementProviderSimple3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRawElementProviderSimple3 {
    type Vtable = IRawElementProviderSimple3_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawElementProviderSimple3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcf5d820_d7ec_4613_bdf6_42a84ce7daaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderSimple3_Vtbl {
    pub base__: IRawElementProviderSimple2_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetMetadataValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: UIA_METADATA_ID, returnval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetMetadataValue: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRawElementProviderWindowlessSite(::windows::core::IUnknown);
impl IRawElementProviderWindowlessSite {
    pub unsafe fn GetAdjacentFragment(&self, direction: NavigateDirection) -> ::windows::core::Result<IRawElementProviderFragment> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAdjacentFragment)(::windows::core::Vtable::as_raw(self), direction, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeIdPrefix(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRuntimeIdPrefix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRawElementProviderWindowlessSite, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRawElementProviderWindowlessSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRawElementProviderWindowlessSite {
    type Vtable = IRawElementProviderWindowlessSite_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawElementProviderWindowlessSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a2a93cc_bfad_42ac_9b2e_0991fb0d3ea0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderWindowlessSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAdjacentFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: NavigateDirection, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRuntimeIdPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRuntimeIdPrefix: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRichEditUiaInformation(::windows::core::IUnknown);
impl IRichEditUiaInformation {
    pub unsafe fn GetBoundaryRectangle(&self, puiarect: *mut UiaRect) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetBoundaryRectangle)(::windows::core::Vtable::as_raw(self), puiarect).ok()
    }
    pub unsafe fn IsVisible(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsVisible)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IRichEditUiaInformation, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRichEditUiaInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRichEditUiaInformation {
    type Vtable = IRichEditUiaInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for IRichEditUiaInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditUiaInformation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBoundaryRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiarect: *mut UiaRect) -> ::windows::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IRicheditWindowlessAccessibility(::windows::core::IUnknown);
impl IRicheditWindowlessAccessibility {
    pub unsafe fn CreateProvider<P0>(&self, psite: P0) -> ::windows::core::Result<IRawElementProviderSimple>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderWindowlessSite>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateProvider)(::windows::core::Vtable::as_raw(self), psite.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IRicheditWindowlessAccessibility, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRicheditWindowlessAccessibility {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IRicheditWindowlessAccessibility {
    type Vtable = IRicheditWindowlessAccessibility_Vtbl;
}
unsafe impl ::windows::core::Interface for IRicheditWindowlessAccessibility {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRicheditWindowlessAccessibility_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psite: *mut ::core::ffi::c_void, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IScrollItemProvider(::windows::core::IUnknown);
impl IScrollItemProvider {
    pub unsafe fn ScrollIntoView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ScrollIntoView)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IScrollItemProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IScrollItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IScrollItemProvider {
    type Vtable = IScrollItemProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IScrollItemProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2360c714_4bf1_4b26_ba65_9b21316127eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollItemProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IScrollProvider(::windows::core::IUnknown);
impl IScrollProvider {
    pub unsafe fn Scroll(&self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Scroll)(::windows::core::Vtable::as_raw(self), horizontalamount, verticalamount).ok()
    }
    pub unsafe fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetScrollPercent)(::windows::core::Vtable::as_raw(self), horizontalpercent, verticalpercent).ok()
    }
    pub unsafe fn HorizontalScrollPercent(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HorizontalScrollPercent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn VerticalScrollPercent(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).VerticalScrollPercent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HorizontalViewSize(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HorizontalViewSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn VerticalViewSize(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).VerticalViewSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HorizontallyScrollable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HorizontallyScrollable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VerticallyScrollable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).VerticallyScrollable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IScrollProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IScrollProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IScrollProvider {
    type Vtable = IScrollProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IScrollProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb38b8077_1fc3_42a5_8cae_d40c2215055a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::HRESULT,
    pub SetScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT,
    pub HorizontalScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    pub VerticalScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    pub HorizontalViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    pub VerticalViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HorizontallyScrollable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HorizontallyScrollable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VerticallyScrollable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VerticallyScrollable: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ISelectionItemProvider(::windows::core::IUnknown);
impl ISelectionItemProvider {
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Select)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddToSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveFromSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSelected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsSelected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SelectionContainer(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SelectionContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ISelectionItemProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISelectionItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ISelectionItemProvider {
    type Vtable = ISelectionItemProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ISelectionItemProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2acad808_b2d4_452d_a407_91ff1ad167b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSelected: usize,
    pub SelectionContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ISelectionProvider(::windows::core::IUnknown);
impl ISelectionProvider {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSelectMultiple(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CanSelectMultiple)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSelectionRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsSelectionRequired)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ISelectionProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISelectionProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ISelectionProvider {
    type Vtable = ISelectionProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ISelectionProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb8b03af_3bdf_48d4_bd36_1a65793be168);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanSelectMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanSelectMultiple: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSelectionRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSelectionRequired: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ISelectionProvider2(::windows::core::IUnknown);
impl ISelectionProvider2 {
    pub unsafe fn FirstSelectedItem(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FirstSelectedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastSelectedItem(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastSelectedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentSelectedItem(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentSelectedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ISelectionProvider2, ::windows::core::IUnknown, ISelectionProvider);
impl ::core::clone::Clone for ISelectionProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ISelectionProvider2 {
    type Vtable = ISelectionProvider2_Vtbl;
}
unsafe impl ::windows::core::Interface for ISelectionProvider2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14f68475_ee1c_44f6_a869_d239381f0fe7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionProvider2_Vtbl {
    pub base__: ISelectionProvider_Vtbl,
    pub FirstSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LastSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ISpreadsheetItemProvider(::windows::core::IUnknown);
impl ISpreadsheetItemProvider {
    pub unsafe fn Formula(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Formula)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAnnotationObjects(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ISpreadsheetItemProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISpreadsheetItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ISpreadsheetItemProvider {
    type Vtable = ISpreadsheetItemProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpreadsheetItemProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeaed4660_7b3d_4879_a2e6_365ce603f3d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Formula: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAnnotationObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAnnotationObjects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAnnotationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAnnotationTypes: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ISpreadsheetProvider(::windows::core::IUnknown);
impl ISpreadsheetProvider {
    pub unsafe fn GetItemByName<P0>(&self, name: P0) -> ::windows::core::Result<IRawElementProviderSimple>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItemByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ISpreadsheetProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISpreadsheetProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ISpreadsheetProvider {
    type Vtable = ISpreadsheetProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpreadsheetProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f6b5d35_5525_4f80_b758_85473832ffc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IStylesProvider(::windows::core::IUnknown);
impl IStylesProvider {
    pub unsafe fn StyleId(&self) -> ::windows::core::Result<UIA_STYLE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StyleId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StyleName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StyleName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FillColor(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FillColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FillPatternStyle(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FillPatternStyle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Shape(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Shape)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FillPatternColor(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FillPatternColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ExtendedProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExtendedProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IStylesProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IStylesProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IStylesProvider {
    type Vtable = IStylesProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IStylesProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19b6b649_f5d7_4a6d_bdcb_129252be588a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StyleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_STYLE_ID) -> ::windows::core::HRESULT,
    pub StyleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FillColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub FillPatternStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FillPatternColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ISynchronizedInputProvider(::windows::core::IUnknown);
impl ISynchronizedInputProvider {
    pub unsafe fn StartListening(&self, inputtype: SynchronizedInputType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StartListening)(::windows::core::Vtable::as_raw(self), inputtype).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ISynchronizedInputProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISynchronizedInputProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ISynchronizedInputProvider {
    type Vtable = ISynchronizedInputProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ISynchronizedInputProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29db1a06_02ce_4cf7_9b42_565d4fab20ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizedInputProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartListening: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ITableItemProvider(::windows::core::IUnknown);
impl ITableItemProvider {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRowHeaderItems(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRowHeaderItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetColumnHeaderItems(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetColumnHeaderItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITableItemProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITableItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITableItemProvider {
    type Vtable = ITableItemProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ITableItemProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9734fa6_771f_4d78_9c90_2517999349cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRowHeaderItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRowHeaderItems: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetColumnHeaderItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetColumnHeaderItems: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ITableProvider(::windows::core::IUnknown);
impl ITableProvider {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRowHeaders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRowHeaders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetColumnHeaders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetColumnHeaders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RowOrColumnMajor(&self) -> ::windows::core::Result<RowOrColumnMajor> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RowOrColumnMajor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITableProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITableProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITableProvider {
    type Vtable = ITableProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ITableProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c860395_97b3_490a_b52a_858cc22af166);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRowHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRowHeaders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetColumnHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetColumnHeaders: usize,
    pub RowOrColumnMajor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ITextChildProvider(::windows::core::IUnknown);
impl ITextChildProvider {
    pub unsafe fn TextContainer(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TextContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TextRange(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TextRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITextChildProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITextChildProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITextChildProvider {
    type Vtable = ITextChildProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextChildProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c2de2b9_c88f_4f88_a111_f1d336b7d1a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextChildProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub TextContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ITextEditProvider(::windows::core::IUnknown);
impl ITextEditProvider {
    pub unsafe fn GetActiveComposition(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetActiveComposition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetConversionTarget(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetConversionTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITextEditProvider, ::windows::core::IUnknown, ITextProvider);
impl ::core::clone::Clone for ITextEditProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITextEditProvider {
    type Vtable = ITextEditProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextEditProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea3605b4_3a05_400e_b5f9_4e91b40f6176);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextEditProvider_Vtbl {
    pub base__: ITextProvider_Vtbl,
    pub GetActiveComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConversionTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ITextProvider(::windows::core::IUnknown);
impl ITextProvider {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleRanges(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVisibleRanges)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeFromChild<P0>(&self, childelement: P0) -> ::windows::core::Result<ITextRangeProvider>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RangeFromChild)(::windows::core::Vtable::as_raw(self), childelement.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeFromPoint(&self, point: UiaPoint) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RangeFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DocumentRange(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DocumentRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SupportedTextSelection(&self) -> ::windows::core::Result<SupportedTextSelection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SupportedTextSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITextProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITextProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITextProvider {
    type Vtable = ITextProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3589c92c_63f3_4367_99bb_ada653b77cf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetVisibleRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetVisibleRanges: usize,
    pub RangeFromChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childelement: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RangeFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point: UiaPoint, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SupportedTextSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut SupportedTextSelection) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ITextProvider2(::windows::core::IUnknown);
impl ITextProvider2 {
    pub unsafe fn RangeFromAnnotation<P0>(&self, annotationelement: P0) -> ::windows::core::Result<ITextRangeProvider>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RangeFromAnnotation)(::windows::core::Vtable::as_raw(self), annotationelement.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCaretRange(&self, isactive: *mut super::super::Foundation::BOOL, pretval: *mut ::core::option::Option<ITextRangeProvider>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCaretRange)(::windows::core::Vtable::as_raw(self), isactive, ::core::mem::transmute(pretval)).ok()
    }
}
::windows::core::interface_hierarchy!(ITextProvider2, ::windows::core::IUnknown, ITextProvider);
impl ::core::clone::Clone for ITextProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITextProvider2 {
    type Vtable = ITextProvider2_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextProvider2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dc5e6ed_3e16_4bf1_8f9a_a979878bc195);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider2_Vtbl {
    pub base__: ITextProvider_Vtbl,
    pub RangeFromAnnotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotationelement: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCaretRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCaretRange: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ITextRangeProvider(::windows::core::IUnknown);
impl ITextRangeProvider {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, range: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRangeProvider>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Compare)(::windows::core::Vtable::as_raw(self), range.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareEndpoints<P0>(&self, endpoint: TextPatternRangeEndpoint, targetrange: P0, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRangeProvider>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CompareEndpoints)(::windows::core::Vtable::as_raw(self), endpoint, targetrange.into().abi(), targetendpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ExpandToEnclosingUnit(&self, unit: TextUnit) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ExpandToEnclosingUnit)(::windows::core::Vtable::as_raw(self), unit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FindAttribute<P0>(&self, attributeid: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Com::VARIANT, backward: P0) -> ::windows::core::Result<ITextRangeProvider>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindAttribute)(::windows::core::Vtable::as_raw(self), attributeid, ::core::mem::transmute(val), backward.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindText<P0, P1>(&self, text: &::windows::core::BSTR, backward: P0, ignorecase: P1) -> ::windows::core::Result<ITextRangeProvider>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(text), backward.into(), ignorecase.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttributeValue(&self, attributeid: UIA_TEXTATTRIBUTE_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAttributeValue)(::windows::core::Vtable::as_raw(self), attributeid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBoundingRectangles(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBoundingRectangles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEnclosingElement(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEnclosingElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetText(&self, maxlength: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetText)(::windows::core::Vtable::as_raw(self), maxlength, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Move)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveEndpointByUnit)(::windows::core::Vtable::as_raw(self), endpoint, unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEndpointByRange<P0>(&self, endpoint: TextPatternRangeEndpoint, targetrange: P0, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRangeProvider>>,
    {
        (::windows::core::Vtable::vtable(self).MoveEndpointByRange)(::windows::core::Vtable::as_raw(self), endpoint, targetrange.into().abi(), targetendpoint).ok()
    }
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Select)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddToSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveFromSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScrollIntoView<P0>(&self, aligntotop: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).ScrollIntoView)(::windows::core::Vtable::as_raw(self), aligntotop.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITextRangeProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITextRangeProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITextRangeProvider {
    type Vtable = ITextRangeProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextRangeProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5347ad7b_c355_46f8_aff5_909033582f63);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRangeProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Compare: usize,
    pub CompareEndpoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextUnit) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FindAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributeid: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Com::VARIANT, backward: super::super::Foundation::BOOL, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FindAttribute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FindText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindText: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttributeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributeid: UIA_TEXTATTRIBUTE_ID, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttributeValue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBoundingRectangles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBoundingRectangles: usize,
    pub GetEnclosingElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlength: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub MoveEndpointByUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub MoveEndpointByRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::HRESULT,
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScrollIntoView: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetChildren: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ITextRangeProvider2(::windows::core::IUnknown);
impl ITextRangeProvider2 {
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ITextRangeProvider2, ::windows::core::IUnknown, ITextRangeProvider);
impl ::core::clone::Clone for ITextRangeProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITextRangeProvider2 {
    type Vtable = ITextRangeProvider2_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextRangeProvider2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bbce42c_1921_4f18_89ca_dba1910a0386);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRangeProvider2_Vtbl {
    pub base__: ITextRangeProvider_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IToggleProvider(::windows::core::IUnknown);
impl IToggleProvider {
    pub unsafe fn Toggle(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Toggle)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ToggleState(&self) -> ::windows::core::Result<ToggleState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ToggleState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IToggleProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IToggleProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IToggleProvider {
    type Vtable = IToggleProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IToggleProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56d00bd0_c4f4_433c_a836_1a52a57e0892);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Toggle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ToggleState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ToggleState) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ITransformProvider(::windows::core::IUnknown);
impl ITransformProvider {
    pub unsafe fn Move(&self, x: f64, y: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Move)(::windows::core::Vtable::as_raw(self), x, y).ok()
    }
    pub unsafe fn Resize(&self, width: f64, height: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Resize)(::windows::core::Vtable::as_raw(self), width, height).ok()
    }
    pub unsafe fn Rotate(&self, degrees: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Rotate)(::windows::core::Vtable::as_raw(self), degrees).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanMove(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CanMove)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanResize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CanResize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanRotate(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CanRotate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITransformProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITransformProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITransformProvider {
    type Vtable = ITransformProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for ITransformProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6829ddc4_4f91_4ffa_b86f_bd3e2987cb4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanMove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanResize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanResize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanRotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanRotate: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct ITransformProvider2(::windows::core::IUnknown);
impl ITransformProvider2 {
    pub unsafe fn Zoom(&self, zoom: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Zoom)(::windows::core::Vtable::as_raw(self), zoom).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanZoom(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CanZoom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ZoomLevel(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ZoomLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ZoomMinimum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ZoomMinimum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ZoomMaximum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ZoomMaximum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ZoomByUnit(&self, zoomunit: ZoomUnit) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ZoomByUnit)(::windows::core::Vtable::as_raw(self), zoomunit).ok()
    }
}
::windows::core::interface_hierarchy!(ITransformProvider2, ::windows::core::IUnknown, ITransformProvider);
impl ::core::clone::Clone for ITransformProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for ITransformProvider2 {
    type Vtable = ITransformProvider2_Vtbl;
}
unsafe impl ::windows::core::Interface for ITransformProvider2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4758742f_7ac2_460c_bc48_09fc09308a93);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformProvider2_Vtbl {
    pub base__: ITransformProvider_Vtbl,
    pub Zoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zoom: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanZoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanZoom: usize,
    pub ZoomLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    pub ZoomMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    pub ZoomMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT,
    pub ZoomByUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomation(::windows::core::IUnknown);
impl IUIAutomation {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompareElements<P0, P1>(&self, el1: P0, el2: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CompareElements)(::windows::core::Vtable::as_raw(self), el1.into().abi(), el2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareRuntimeIds(&self, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CompareRuntimeIds)(::windows::core::Vtable::as_raw(self), runtimeid1, runtimeid2, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRootElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandle<P0>(&self, hwnd: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElementFromHandle)(::windows::core::Vtable::as_raw(self), hwnd.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPoint(&self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElementFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFocusedElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRootElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandleBuildCache<P0, P1>(&self, hwnd: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElementFromHandleBuildCache)(::windows::core::Vtable::as_raw(self), hwnd.into(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPointBuildCache<P0>(&self, pt: super::super::Foundation::POINT, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElementFromPointBuildCache)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFocusedElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTreeWalker<P0>(&self, pcondition: P0) -> ::windows::core::Result<IUIAutomationTreeWalker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTreeWalker)(::windows::core::Vtable::as_raw(self), pcondition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ControlViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ContentViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RawViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RawViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ControlViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ContentViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCacheRequest(&self) -> ::windows::core::Result<IUIAutomationCacheRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCacheRequest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTrueCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTrueCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFalseCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFalseCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyCondition(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePropertyCondition)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyConditionEx(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT, flags: PropertyConditionFlags) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePropertyConditionEx)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAndCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAndConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAndConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAndConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateOrCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateOrConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateOrConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateOrConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateNotCondition<P0>(&self, condition: P0) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateNotCondition)(::windows::core::Vtable::as_raw(self), condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddAutomationEventHandler<P0, P1, P2>(&self, eventid: UIA_EVENT_ID, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAutomationEventHandler<P0, P1>(&self, eventid: UIA_EVENT_ID, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddPropertyChangedEventHandlerNativeArray<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: &[UIA_PROPERTY_ID]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddPropertyChangedEventHandlerNativeArray)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), ::core::mem::transmute(propertyarray.as_ptr()), propertyarray.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPropertyChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddPropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), propertyarray).ok()
    }
    pub unsafe fn RemovePropertyChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).RemovePropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveStructureChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddFocusChangedEventHandler<P0, P1>(&self, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveFocusChangedEventHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAllEventHandlers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAllEventHandlers)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntNativeArrayToSafeArray(&self, array: &[i32]) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IntNativeArrayToSafeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(array.as_ptr()), array.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntSafeArrayToNativeArray(&self, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IntSafeArrayToNativeArray)(::windows::core::Vtable::as_raw(self), intarray, array, arraycount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RectToVariant(&self, rc: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RectToVariant)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rc), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VariantToRect(&self, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).VariantToRect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(var), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SafeArrayToRectNativeArray(&self, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SafeArrayToRectNativeArray)(::windows::core::Vtable::as_raw(self), rects, rectarray, rectarraycount).ok()
    }
    pub unsafe fn CreateProxyFactoryEntry<P0>(&self, factory: P0) -> ::windows::core::Result<IUIAutomationProxyFactoryEntry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationProxyFactory>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateProxyFactoryEntry)(::windows::core::Vtable::as_raw(self), factory.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ProxyFactoryMapping(&self) -> ::windows::core::Result<IUIAutomationProxyFactoryMapping> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ProxyFactoryMapping)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyProgrammaticName(&self, property: UIA_PROPERTY_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPropertyProgrammaticName)(::windows::core::Vtable::as_raw(self), property, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPatternProgrammaticName(&self, pattern: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPatternProgrammaticName)(::windows::core::Vtable::as_raw(self), pattern, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedPatterns<P0>(&self, pelement: P0, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).PollForPotentialSupportedPatterns)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), patternids, patternnames).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedProperties<P0>(&self, pelement: P0, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).PollForPotentialSupportedProperties)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), propertyids, propertynames).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CheckNotSupported(&self, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CheckNotSupported)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedNotSupportedValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReservedNotSupportedValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedMixedAttributeValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReservedMixedAttributeValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessible<P0>(&self, accessible: P0, childid: i32) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElementFromIAccessible)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessibleBuildCache<P0, P1>(&self, accessible: P0, childid: i32, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElementFromIAccessibleBuildCache)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomation, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomation {
    type Vtable = IUIAutomation_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30cbe57d_d9d0_452a_ab13_7ac5ac4825ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CompareElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, el1: *mut ::core::ffi::c_void, el2: *mut ::core::ffi::c_void, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CompareElements: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CompareRuntimeIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CompareRuntimeIds: usize,
    pub GetRootElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ElementFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ElementFromHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ElementFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ElementFromPoint: usize,
    pub GetFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRootElementBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ElementFromHandleBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ElementFromHandleBuildCache: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ElementFromPointBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ElementFromPointBuildCache: usize,
    pub GetFocusedElementBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTreeWalker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcondition: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ControlViewWalker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ContentViewWalker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RawViewWalker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RawViewCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ControlViewCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ContentViewCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCacheRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cacherequest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTrueCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFalseCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreatePropertyCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreatePropertyCondition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreatePropertyConditionEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT, flags: PropertyConditionFlags, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreatePropertyConditionEx: usize,
    pub CreateAndCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition1: *mut ::core::ffi::c_void, condition2: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAndConditionFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAndConditionFromArray: usize,
    pub CreateAndConditionFromNativeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditions: *const *mut ::core::ffi::c_void, conditioncount: i32, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateOrCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition1: *mut ::core::ffi::c_void, condition2: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateOrConditionFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateOrConditionFromArray: usize,
    pub CreateOrConditionFromNativeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditions: *const *mut ::core::ffi::c_void, conditioncount: i32, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNotCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddAutomationEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAutomationEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddPropertyChangedEventHandlerNativeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, propertyarray: *const UIA_PROPERTY_ID, propertycount: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPropertyChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPropertyChangedEventHandler: usize,
    pub RemovePropertyChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddStructureChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveStructureChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddFocusChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveFocusChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAllEventHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IntNativeArrayToSafeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, array: *const i32, arraycount: i32, safearray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IntNativeArrayToSafeArray: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IntSafeArrayToNativeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IntSafeArrayToNativeArray: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RectToVariant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, var: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RectToVariant: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub VariantToRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, var: super::super::System::Com::VARIANT, rc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    VariantToRect: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SafeArrayToRectNativeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SafeArrayToRectNativeArray: usize,
    pub CreateProxyFactoryEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, factoryentry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProxyFactoryMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factorymapping: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertyProgrammaticName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: UIA_PROPERTY_ID, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPatternProgrammaticName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: UIA_PATTERN_ID, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PollForPotentialSupportedPatterns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pelement: *mut ::core::ffi::c_void, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PollForPotentialSupportedPatterns: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PollForPotentialSupportedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pelement: *mut ::core::ffi::c_void, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PollForPotentialSupportedProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CheckNotSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::Com::VARIANT, isnotsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CheckNotSupported: usize,
    pub ReservedNotSupportedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notsupportedvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReservedMixedAttributeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mixedattributevalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ElementFromIAccessible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessible: *mut ::core::ffi::c_void, childid: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ElementFromIAccessible: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ElementFromIAccessibleBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessible: *mut ::core::ffi::c_void, childid: i32, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ElementFromIAccessibleBuildCache: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomation2(::windows::core::IUnknown);
impl IUIAutomation2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoSetFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutoSetFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoSetFocus<P0>(&self, autosetfocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAutoSetFocus)(::windows::core::Vtable::as_raw(self), autosetfocus.into()).ok()
    }
    pub unsafe fn ConnectionTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ConnectionTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetConnectionTimeout(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetConnectionTimeout)(::windows::core::Vtable::as_raw(self), timeout).ok()
    }
    pub unsafe fn TransactionTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TransactionTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransactionTimeout(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTransactionTimeout)(::windows::core::Vtable::as_raw(self), timeout).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomation2, ::windows::core::IUnknown, IUIAutomation);
impl ::core::clone::Clone for IUIAutomation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomation2 {
    type Vtable = IUIAutomation2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34723aff_0c9d_49d0_9896_7ab52df8cd8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation2_Vtbl {
    pub base__: IUIAutomation_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AutoSetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autosetfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AutoSetFocus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoSetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autosetfocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoSetFocus: usize,
    pub ConnectionTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows::core::HRESULT,
    pub SetConnectionTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT,
    pub TransactionTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows::core::HRESULT,
    pub SetTransactionTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomation3(::windows::core::IUnknown);
impl IUIAutomation3 {
    pub unsafe fn AddTextEditTextChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextEditTextChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddTextEditTextChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, texteditchangetype, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveTextEditTextChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextEditTextChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveTextEditTextChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomation3, ::windows::core::IUnknown, IUIAutomation, IUIAutomation2);
impl ::core::clone::Clone for IUIAutomation3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomation3 {
    type Vtable = IUIAutomation3_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomation3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73d768da_9b51_4b89_936e_c209290973e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation3_Vtbl {
    pub base__: IUIAutomation2_Vtbl,
    pub AddTextEditTextChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveTextEditTextChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomation4(::windows::core::IUnknown);
impl IUIAutomation4 {
    pub unsafe fn AddChangesEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, changetypes: &[i32], pcacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationChangesEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddChangesEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, ::core::mem::transmute(changetypes.as_ptr()), changetypes.len() as _, pcacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveChangesEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationChangesEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveChangesEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomation4, ::windows::core::IUnknown, IUIAutomation, IUIAutomation2, IUIAutomation3);
impl ::core::clone::Clone for IUIAutomation4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomation4 {
    type Vtable = IUIAutomation4_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomation4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1189c02a_05f8_4319_8e21_e817e3db2860);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation4_Vtbl {
    pub base__: IUIAutomation3_Vtbl,
    pub AddChangesEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveChangesEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomation5(::windows::core::IUnknown);
impl IUIAutomation5 {
    pub unsafe fn AddNotificationEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationNotificationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddNotificationEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveNotificationEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationNotificationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveNotificationEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomation5, ::windows::core::IUnknown, IUIAutomation, IUIAutomation2, IUIAutomation3, IUIAutomation4);
impl ::core::clone::Clone for IUIAutomation5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomation5 {
    type Vtable = IUIAutomation5_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomation5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25f700c8_d816_4057_a9dc_3cbdee77e256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation5_Vtbl {
    pub base__: IUIAutomation4_Vtbl,
    pub AddNotificationEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveNotificationEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomation6(::windows::core::IUnknown);
impl IUIAutomation6 {
    pub unsafe fn CreateEventHandlerGroup(&self) -> ::windows::core::Result<IUIAutomationEventHandlerGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEventHandlerGroup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddEventHandlerGroup<P0, P1>(&self, element: P0, handlergroup: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandlerGroup>>,
    {
        (::windows::core::Vtable::vtable(self).AddEventHandlerGroup)(::windows::core::Vtable::as_raw(self), element.into().abi(), handlergroup.into().abi()).ok()
    }
    pub unsafe fn RemoveEventHandlerGroup<P0, P1>(&self, element: P0, handlergroup: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandlerGroup>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveEventHandlerGroup)(::windows::core::Vtable::as_raw(self), element.into().abi(), handlergroup.into().abi()).ok()
    }
    pub unsafe fn ConnectionRecoveryBehavior(&self) -> ::windows::core::Result<ConnectionRecoveryBehaviorOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ConnectionRecoveryBehavior)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetConnectionRecoveryBehavior(&self, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetConnectionRecoveryBehavior)(::windows::core::Vtable::as_raw(self), connectionrecoverybehavioroptions).ok()
    }
    pub unsafe fn CoalesceEvents(&self) -> ::windows::core::Result<CoalesceEventsOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CoalesceEvents)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCoalesceEvents(&self, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCoalesceEvents)(::windows::core::Vtable::as_raw(self), coalesceeventsoptions).ok()
    }
    pub unsafe fn AddActiveTextPositionChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationActiveTextPositionChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddActiveTextPositionChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveActiveTextPositionChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationActiveTextPositionChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).RemoveActiveTextPositionChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomation6, ::windows::core::IUnknown, IUIAutomation, IUIAutomation2, IUIAutomation3, IUIAutomation4, IUIAutomation5);
impl ::core::clone::Clone for IUIAutomation6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomation6 {
    type Vtable = IUIAutomation6_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomation6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaae072da_29e3_413d_87a7_192dbf81ed10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation6_Vtbl {
    pub base__: IUIAutomation5_Vtbl,
    pub CreateEventHandlerGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handlergroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddEventHandlerGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handlergroup: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveEventHandlerGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handlergroup: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConnectionRecoveryBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: *mut ConnectionRecoveryBehaviorOptions) -> ::windows::core::HRESULT,
    pub SetConnectionRecoveryBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows::core::HRESULT,
    pub CoalesceEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coalesceeventsoptions: *mut CoalesceEventsOptions) -> ::windows::core::HRESULT,
    pub SetCoalesceEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows::core::HRESULT,
    pub AddActiveTextPositionChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveActiveTextPositionChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationActiveTextPositionChangedEventHandler(::windows::core::IUnknown);
impl IUIAutomationActiveTextPositionChangedEventHandler {
    pub unsafe fn HandleActiveTextPositionChangedEvent<P0, P1>(&self, sender: P0, range: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextRange>>,
    {
        (::windows::core::Vtable::vtable(self).HandleActiveTextPositionChangedEvent)(::windows::core::Vtable::as_raw(self), sender.into().abi(), range.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationActiveTextPositionChangedEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationActiveTextPositionChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationActiveTextPositionChangedEventHandler {
    type Vtable = IUIAutomationActiveTextPositionChangedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationActiveTextPositionChangedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf97933b0_8dae_4496_8997_5ba015fe0d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub HandleActiveTextPositionChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationAndCondition(::windows::core::IUnknown);
impl IUIAutomationAndCondition {
    pub unsafe fn ChildCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ChildCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetChildrenAsNativeArray(&self, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetChildrenAsNativeArray)(::windows::core::Vtable::as_raw(self), childarray, childarraycount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationAndCondition, ::windows::core::IUnknown, IUIAutomationCondition);
impl ::core::clone::Clone for IUIAutomationAndCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationAndCondition {
    type Vtable = IUIAutomationAndCondition_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationAndCondition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7d0af36_b912_45fe_9855_091ddc174aec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationAndCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub ChildCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows::core::HRESULT,
    pub GetChildrenAsNativeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetChildren: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationAnnotationPattern(::windows::core::IUnknown);
impl IUIAutomationAnnotationPattern {
    pub unsafe fn CurrentAnnotationTypeId(&self) -> ::windows::core::Result<UIA_ANNOTATIONTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentAnnotationTypeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAnnotationTypeName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentAnnotationTypeName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAuthor(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentAuthor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDateTime(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentDateTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentTarget(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAnnotationTypeId(&self) -> ::windows::core::Result<UIA_ANNOTATIONTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedAnnotationTypeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAnnotationTypeName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedAnnotationTypeName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAuthor(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedAuthor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDateTime(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedDateTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedTarget(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationAnnotationPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationAnnotationPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationAnnotationPattern {
    type Vtable = IUIAutomationAnnotationPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationAnnotationPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a175b21_339e_41b1_8e8b_623f6b681098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationAnnotationPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CurrentAnnotationTypeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_ANNOTATIONTYPE) -> ::windows::core::HRESULT,
    pub CurrentAnnotationTypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentAuthor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedAnnotationTypeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_ANNOTATIONTYPE) -> ::windows::core::HRESULT,
    pub CachedAnnotationTypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedAuthor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationBoolCondition(::windows::core::IUnknown);
impl IUIAutomationBoolCondition {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BooleanValue(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BooleanValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationBoolCondition, ::windows::core::IUnknown, IUIAutomationCondition);
impl ::core::clone::Clone for IUIAutomationBoolCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationBoolCondition {
    type Vtable = IUIAutomationBoolCondition_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationBoolCondition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b4e1f2e_75eb_4d0b_8952_5a69988e2307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationBoolCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BooleanValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boolval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BooleanValue: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationCacheRequest(::windows::core::IUnknown);
impl IUIAutomationCacheRequest {
    pub unsafe fn AddProperty(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddProperty)(::windows::core::Vtable::as_raw(self), propertyid).ok()
    }
    pub unsafe fn AddPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddPattern)(::windows::core::Vtable::as_raw(self), patternid).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IUIAutomationCacheRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TreeScope(&self) -> ::windows::core::Result<TreeScope> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TreeScope)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTreeScope(&self, scope: TreeScope) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTreeScope)(::windows::core::Vtable::as_raw(self), scope).ok()
    }
    pub unsafe fn TreeFilter(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TreeFilter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTreeFilter<P0>(&self, filter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        (::windows::core::Vtable::vtable(self).SetTreeFilter)(::windows::core::Vtable::as_raw(self), filter.into().abi()).ok()
    }
    pub unsafe fn AutomationElementMode(&self) -> ::windows::core::Result<AutomationElementMode> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AutomationElementMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAutomationElementMode(&self, mode: AutomationElementMode) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAutomationElementMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationCacheRequest, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationCacheRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationCacheRequest {
    type Vtable = IUIAutomationCacheRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationCacheRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb32a92b5_bc25_4078_9c08_d7ee95c48e03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationCacheRequest_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID) -> ::windows::core::HRESULT,
    pub AddPattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clonedrequest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TreeScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut TreeScope) -> ::windows::core::HRESULT,
    pub SetTreeScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope) -> ::windows::core::HRESULT,
    pub TreeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTreeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AutomationElementMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut AutomationElementMode) -> ::windows::core::HRESULT,
    pub SetAutomationElementMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: AutomationElementMode) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationChangesEventHandler(::windows::core::IUnknown);
impl IUIAutomationChangesEventHandler {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn HandleChangesEvent<P0>(&self, sender: P0, uiachanges: &[UiaChangeInfo]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).HandleChangesEvent)(::windows::core::Vtable::as_raw(self), sender.into().abi(), ::core::mem::transmute(uiachanges.as_ptr()), uiachanges.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationChangesEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationChangesEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationChangesEventHandler {
    type Vtable = IUIAutomationChangesEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationChangesEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58edca55_2c3e_4980_b1b9_56c17f27a2a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationChangesEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub HandleChangesEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    HandleChangesEvent: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationCondition(::windows::core::IUnknown);
impl IUIAutomationCondition {}
::windows::core::interface_hierarchy!(IUIAutomationCondition, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationCondition {
    type Vtable = IUIAutomationCondition_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationCondition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x352ffba8_0973_437c_a61f_f64cafd81df9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationCondition_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationCustomNavigationPattern(::windows::core::IUnknown);
impl IUIAutomationCustomNavigationPattern {
    pub unsafe fn Navigate(&self, direction: NavigateDirection) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Navigate)(::windows::core::Vtable::as_raw(self), direction, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationCustomNavigationPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationCustomNavigationPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationCustomNavigationPattern {
    type Vtable = IUIAutomationCustomNavigationPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationCustomNavigationPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01ea217a_1766_47ed_a6cc_acf492854b1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationCustomNavigationPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Navigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationDockPattern(::windows::core::IUnknown);
impl IUIAutomationDockPattern {
    pub unsafe fn SetDockPosition(&self, dockpos: DockPosition) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDockPosition)(::windows::core::Vtable::as_raw(self), dockpos).ok()
    }
    pub unsafe fn CurrentDockPosition(&self) -> ::windows::core::Result<DockPosition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentDockPosition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDockPosition(&self) -> ::windows::core::Result<DockPosition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedDockPosition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationDockPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationDockPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationDockPattern {
    type Vtable = IUIAutomationDockPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationDockPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfde5ef97_1464_48f6_90bf_43d0948e86ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationDockPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetDockPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dockpos: DockPosition) -> ::windows::core::HRESULT,
    pub CurrentDockPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows::core::HRESULT,
    pub CachedDockPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationDragPattern(::windows::core::IUnknown);
impl IUIAutomationDragPattern {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsGrabbed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsGrabbed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsGrabbed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsGrabbed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDropEffect(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentDropEffect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDropEffect(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedDropEffect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentDropEffects(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentDropEffects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedDropEffects(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedDropEffects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentGrabbedItems(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentGrabbedItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedGrabbedItems(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedGrabbedItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationDragPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationDragPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationDragPattern {
    type Vtable = IUIAutomationDragPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationDragPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dc7b570_1f54_4bad_bcda_d36a722fb7bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationDragPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsGrabbed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsGrabbed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsGrabbed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsGrabbed: usize,
    pub CurrentDropEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedDropEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentDropEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentDropEffects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CachedDropEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CachedDropEffects: usize,
    pub GetCurrentGrabbedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedGrabbedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationDropTargetPattern(::windows::core::IUnknown);
impl IUIAutomationDropTargetPattern {
    pub unsafe fn CurrentDropTargetEffect(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentDropTargetEffect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDropTargetEffect(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedDropTargetEffect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentDropTargetEffects(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentDropTargetEffects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedDropTargetEffects(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedDropTargetEffects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationDropTargetPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationDropTargetPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationDropTargetPattern {
    type Vtable = IUIAutomationDropTargetPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationDropTargetPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69a095f7_eee4_430e_a46b_fb73b1ae39a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationDropTargetPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CurrentDropTargetEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedDropTargetEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentDropTargetEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentDropTargetEffects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CachedDropTargetEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CachedDropTargetEffects: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationElement(::windows::core::IUnknown);
impl IUIAutomationElement {
    pub unsafe fn SetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirst<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirst)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAll<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindAll)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindAllBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BuildUpdatedCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedParent(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetClickablePoint)(::windows::core::Vtable::as_raw(self), clickable, gotclickable).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationElement, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationElement {
    type Vtable = IUIAutomationElement_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd22108aa_8ac5_49a5_837b_37bbb3d7591e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRuntimeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runtimeid: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRuntimeId: usize,
    pub FindFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFirstBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindAllBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BuildUpdatedCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, updatedelement: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCurrentPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCurrentPropertyValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCurrentPropertyValueEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCurrentPropertyValueEx: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCachedPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCachedPropertyValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCachedPropertyValueEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCachedPropertyValueEx: usize,
    pub GetCurrentPatternAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, riid: *const ::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedPatternAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, riid: *const ::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentPattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedPattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentControlType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_CONTROLTYPE_ID) -> ::windows::core::HRESULT,
    pub CurrentLocalizedControlType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentAcceleratorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentAccessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentHasKeyboardFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentHasKeyboardFocus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsKeyboardFocusable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsKeyboardFocusable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsEnabled: usize,
    pub CurrentAutomationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentHelpText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentCulture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsControlElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsControlElement: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsContentElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsContentElement: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsPassword: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentNativeWindowHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentNativeWindowHandle: usize,
    pub CurrentItemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsOffscreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsOffscreen: usize,
    pub CurrentOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows::core::HRESULT,
    pub CurrentFrameworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsRequiredForForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsRequiredForForm: usize,
    pub CurrentItemStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentBoundingRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentBoundingRectangle: usize,
    pub CurrentLabeledBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentAriaRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentAriaProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsDataValidForForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsDataValidForForm: usize,
    pub CurrentControllerFor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentDescribedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentFlowsTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentProviderDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedControlType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_CONTROLTYPE_ID) -> ::windows::core::HRESULT,
    pub CachedLocalizedControlType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedAcceleratorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedAccessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedHasKeyboardFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedHasKeyboardFocus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsKeyboardFocusable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsKeyboardFocusable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsEnabled: usize,
    pub CachedAutomationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedHelpText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedCulture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsControlElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsControlElement: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsContentElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsContentElement: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsPassword: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedNativeWindowHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedNativeWindowHandle: usize,
    pub CachedItemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsOffscreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsOffscreen: usize,
    pub CachedOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows::core::HRESULT,
    pub CachedFrameworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsRequiredForForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsRequiredForForm: usize,
    pub CachedItemStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedBoundingRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedBoundingRectangle: usize,
    pub CachedLabeledBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedAriaRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedAriaProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsDataValidForForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsDataValidForForm: usize,
    pub CachedControllerFor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedDescribedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedFlowsTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedProviderDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetClickablePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetClickablePoint: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationElement2(::windows::core::IUnknown);
impl IUIAutomationElement2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationElement2, ::windows::core::IUnknown, IUIAutomationElement);
impl ::core::clone::Clone for IUIAutomationElement2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationElement2 {
    type Vtable = IUIAutomationElement2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationElement2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6749c683_f70d_4487_a698_5f79d55290d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement2_Vtbl {
    pub base__: IUIAutomationElement_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentOptimizeForVisualContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentOptimizeForVisualContent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedOptimizeForVisualContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedOptimizeForVisualContent: usize,
    pub CurrentLiveSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows::core::HRESULT,
    pub CachedLiveSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows::core::HRESULT,
    pub CurrentFlowsFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedFlowsFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationElement3(::windows::core::IUnknown);
impl IUIAutomationElement3 {
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationElement3, ::windows::core::IUnknown, IUIAutomationElement, IUIAutomationElement2);
impl ::core::clone::Clone for IUIAutomationElement3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationElement3 {
    type Vtable = IUIAutomationElement3_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationElement3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8471df34_aee0_4a01_a7de_7db9af12c296);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement3_Vtbl {
    pub base__: IUIAutomationElement2_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsPeripheral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsPeripheral: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsPeripheral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsPeripheral: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationElement4(::windows::core::IUnknown);
impl IUIAutomationElement4 {
    pub unsafe fn CurrentPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationElement4, ::windows::core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3);
impl ::core::clone::Clone for IUIAutomationElement4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationElement4 {
    type Vtable = IUIAutomationElement4_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationElement4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b6e233c_52fb_4063_a4c9_77c075c2a06b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement4_Vtbl {
    pub base__: IUIAutomationElement3_Vtbl,
    pub CurrentPositionInSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentSizeOfSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentAnnotationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentAnnotationTypes: usize,
    pub CurrentAnnotationObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedPositionInSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedSizeOfSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CachedAnnotationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CachedAnnotationTypes: usize,
    pub CachedAnnotationObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationElement5(::windows::core::IUnknown);
impl IUIAutomationElement5 {
    pub unsafe fn CurrentLandmarkType(&self) -> ::windows::core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentLocalizedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLandmarkType(&self) -> ::windows::core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedLocalizedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationElement5, ::windows::core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4);
impl ::core::clone::Clone for IUIAutomationElement5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationElement5 {
    type Vtable = IUIAutomationElement5_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationElement5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98141c1d_0d0e_4175_bbe2_6bff455842a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement5_Vtbl {
    pub base__: IUIAutomationElement4_Vtbl,
    pub CurrentLandmarkType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_LANDMARKTYPE_ID) -> ::windows::core::HRESULT,
    pub CurrentLocalizedLandmarkType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedLandmarkType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_LANDMARKTYPE_ID) -> ::windows::core::HRESULT,
    pub CachedLocalizedLandmarkType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationElement6(::windows::core::IUnknown);
impl IUIAutomationElement6 {
    pub unsafe fn CurrentFullDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentFullDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFullDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedFullDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationElement6, ::windows::core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5);
impl ::core::clone::Clone for IUIAutomationElement6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationElement6 {
    type Vtable = IUIAutomationElement6_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationElement6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4780d450_8bca_4977_afa5_a4a517f555e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement6_Vtbl {
    pub base__: IUIAutomationElement5_Vtbl,
    pub CurrentFullDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedFullDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationElement7(::windows::core::IUnknown);
impl IUIAutomationElement7 {
    pub unsafe fn FindFirstWithOptions<P0, P1>(&self, scope: TreeScope, condition: P0, traversaloptions: TreeTraversalOptions, root: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstWithOptions)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllWithOptions<P0, P1>(&self, scope: TreeScope, condition: P0, traversaloptions: TreeTraversalOptions, root: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindAllWithOptions)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstWithOptionsBuildCache<P0, P1, P2>(&self, scope: TreeScope, condition: P0, cacherequest: P1, traversaloptions: TreeTraversalOptions, root: P2) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindFirstWithOptionsBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllWithOptionsBuildCache<P0, P1, P2>(&self, scope: TreeScope, condition: P0, cacherequest: P1, traversaloptions: TreeTraversalOptions, root: P2) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindAllWithOptionsBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentMetadataValue(&self, targetid: i32, metadataid: UIA_METADATA_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentMetadataValue)(::windows::core::Vtable::as_raw(self), targetid, metadataid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationElement7, ::windows::core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5, IUIAutomationElement6);
impl ::core::clone::Clone for IUIAutomationElement7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationElement7 {
    type Vtable = IUIAutomationElement7_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationElement7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x204e8572_cfc3_4c11_b0c8_7da7420750b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement7_Vtbl {
    pub base__: IUIAutomationElement6_Vtbl,
    pub FindFirstWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindAllWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindFirstWithOptionsBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindAllWithOptionsBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCurrentMetadataValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: UIA_METADATA_ID, returnval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCurrentMetadataValue: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationElement8(::windows::core::IUnknown);
impl IUIAutomationElement8 {
    pub unsafe fn CurrentHeadingLevel(&self) -> ::windows::core::Result<UIA_HEADINGLEVEL_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentHeadingLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHeadingLevel(&self) -> ::windows::core::Result<UIA_HEADINGLEVEL_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedHeadingLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationElement8, ::windows::core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5, IUIAutomationElement6, IUIAutomationElement7);
impl ::core::clone::Clone for IUIAutomationElement8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationElement8 {
    type Vtable = IUIAutomationElement8_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationElement8 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c60217d_5411_4cde_bcc0_1ceda223830c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement8_Vtbl {
    pub base__: IUIAutomationElement7_Vtbl,
    pub CurrentHeadingLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_HEADINGLEVEL_ID) -> ::windows::core::HRESULT,
    pub CachedHeadingLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_HEADINGLEVEL_ID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationElement9(::windows::core::IUnknown);
impl IUIAutomationElement9 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsDialog(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsDialog)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsDialog(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsDialog)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationElement9, ::windows::core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5, IUIAutomationElement6, IUIAutomationElement7, IUIAutomationElement8);
impl ::core::clone::Clone for IUIAutomationElement9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationElement9 {
    type Vtable = IUIAutomationElement9_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationElement9 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39325fac_039d_440e_a3a3_5eb81a5cecc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement9_Vtbl {
    pub base__: IUIAutomationElement8_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsDialog: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsDialog: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationElementArray(::windows::core::IUnknown);
impl IUIAutomationElementArray {
    pub unsafe fn Length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Length)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetElement(&self, index: i32) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetElement)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationElementArray, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationElementArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationElementArray {
    type Vtable = IUIAutomationElementArray_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationElementArray {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14314595_b4bc_4055_95f2_58f2e42c9855);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElementArray_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT,
    pub GetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationEventHandler(::windows::core::IUnknown);
impl IUIAutomationEventHandler {
    pub unsafe fn HandleAutomationEvent<P0>(&self, sender: P0, eventid: UIA_EVENT_ID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).HandleAutomationEvent)(::windows::core::Vtable::as_raw(self), sender.into().abi(), eventid).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationEventHandler {
    type Vtable = IUIAutomationEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x146c3c17_f12e_4e22_8c27_f894b9b79c69);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub HandleAutomationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationEventHandlerGroup(::windows::core::IUnknown);
impl IUIAutomationEventHandlerGroup {
    pub unsafe fn AddActiveTextPositionChangedEventHandler<P0, P1>(&self, scope: TreeScope, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationActiveTextPositionChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddActiveTextPositionChangedEventHandler)(::windows::core::Vtable::as_raw(self), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddAutomationEventHandler<P0, P1>(&self, eventid: UIA_EVENT_ID, scope: TreeScope, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddChangesEventHandler<P0, P1>(&self, scope: TreeScope, changetypes: &[i32], cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationChangesEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddChangesEventHandler)(::windows::core::Vtable::as_raw(self), scope, ::core::mem::transmute(changetypes.as_ptr()), changetypes.len() as _, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddNotificationEventHandler<P0, P1>(&self, scope: TreeScope, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationNotificationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddNotificationEventHandler)(::windows::core::Vtable::as_raw(self), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddPropertyChangedEventHandler<P0, P1>(&self, scope: TreeScope, cacherequest: P0, handler: P1, propertyarray: &[UIA_PROPERTY_ID]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddPropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), scope, cacherequest.into().abi(), handler.into().abi(), ::core::mem::transmute(propertyarray.as_ptr()), propertyarray.len() as _).ok()
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P1>(&self, scope: TreeScope, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddTextEditTextChangedEventHandler<P0, P1>(&self, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextEditTextChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).AddTextEditTextChangedEventHandler)(::windows::core::Vtable::as_raw(self), scope, texteditchangetype, cacherequest.into().abi(), handler.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationEventHandlerGroup, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationEventHandlerGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationEventHandlerGroup {
    type Vtable = IUIAutomationEventHandlerGroup_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationEventHandlerGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9ee12f2_c13b_4408_997c_639914377f4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationEventHandlerGroup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddActiveTextPositionChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddAutomationEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddChangesEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddNotificationEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddPropertyChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, propertyarray: *const UIA_PROPERTY_ID, propertycount: i32) -> ::windows::core::HRESULT,
    pub AddStructureChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddTextEditTextChangedEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationExpandCollapsePattern(::windows::core::IUnknown);
impl IUIAutomationExpandCollapsePattern {
    pub unsafe fn Expand(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Expand)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Collapse(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Collapse)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CurrentExpandCollapseState(&self) -> ::windows::core::Result<ExpandCollapseState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentExpandCollapseState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedExpandCollapseState(&self) -> ::windows::core::Result<ExpandCollapseState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedExpandCollapseState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationExpandCollapsePattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationExpandCollapsePattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationExpandCollapsePattern {
    type Vtable = IUIAutomationExpandCollapsePattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationExpandCollapsePattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x619be086_1f4e_4ee4_bafa_210128738730);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationExpandCollapsePattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Expand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Collapse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentExpandCollapseState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows::core::HRESULT,
    pub CachedExpandCollapseState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationFocusChangedEventHandler(::windows::core::IUnknown);
impl IUIAutomationFocusChangedEventHandler {
    pub unsafe fn HandleFocusChangedEvent<P0>(&self, sender: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).HandleFocusChangedEvent)(::windows::core::Vtable::as_raw(self), sender.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationFocusChangedEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationFocusChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationFocusChangedEventHandler {
    type Vtable = IUIAutomationFocusChangedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationFocusChangedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc270f6b5_5c69_4290_9745_7a7f97169468);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationFocusChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub HandleFocusChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationGridItemPattern(::windows::core::IUnknown);
impl IUIAutomationGridItemPattern {
    pub unsafe fn CurrentContainingGrid(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentContainingGrid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentRow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentRow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentColumn(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentColumn)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentRowSpan(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentRowSpan)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentColumnSpan(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentColumnSpan)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedContainingGrid(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedContainingGrid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedRow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedRow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedColumn(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedColumn)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedRowSpan(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedRowSpan)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedColumnSpan(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedColumnSpan)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationGridItemPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationGridItemPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationGridItemPattern {
    type Vtable = IUIAutomationGridItemPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationGridItemPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78f8ef57_66c3_4e09_bd7c_e79b2004894d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationGridItemPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CurrentContainingGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentRowSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentColumnSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedContainingGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedRowSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedColumnSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationGridPattern(::windows::core::IUnknown);
impl IUIAutomationGridPattern {
    pub unsafe fn GetItem(&self, row: i32, column: i32) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItem)(::windows::core::Vtable::as_raw(self), row, column, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentRowCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentRowCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentColumnCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentColumnCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedRowCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedRowCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedColumnCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedColumnCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationGridPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationGridPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationGridPattern {
    type Vtable = IUIAutomationGridPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationGridPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x414c3cdc_856b_4f5b_8538_3131c6302550);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationGridPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentRowCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentColumnCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedRowCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedColumnCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationInvokePattern(::windows::core::IUnknown);
impl IUIAutomationInvokePattern {
    pub unsafe fn Invoke(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Invoke)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationInvokePattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationInvokePattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationInvokePattern {
    type Vtable = IUIAutomationInvokePattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationInvokePattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb377fbe_8ea6_46d5_9c73_6499642d3059);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationInvokePattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationItemContainerPattern(::windows::core::IUnknown);
impl IUIAutomationItemContainerPattern {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FindItemByProperty<P0>(&self, pstartafter: P0, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindItemByProperty)(::windows::core::Vtable::as_raw(self), pstartafter.into().abi(), propertyid, ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationItemContainerPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationItemContainerPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationItemContainerPattern {
    type Vtable = IUIAutomationItemContainerPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationItemContainerPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc690fdb2_27a8_423c_812d_429773c9084e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationItemContainerPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FindItemByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstartafter: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT, pfound: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FindItemByProperty: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationLegacyIAccessiblePattern(::windows::core::IUnknown);
impl IUIAutomationLegacyIAccessiblePattern {
    pub unsafe fn Select(&self, flagsselect: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Select)(::windows::core::Vtable::as_raw(self), flagsselect).ok()
    }
    pub unsafe fn DoDefaultAction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DoDefaultAction)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetValue<P0>(&self, szvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), szvalue.into().abi()).ok()
    }
    pub unsafe fn CurrentChildId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentChildId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentValue(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentRole(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHelp(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentHelp)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentKeyboardShortcut(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentKeyboardShortcut)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentSelection(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDefaultAction(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentDefaultAction)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedChildId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedChildId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedValue(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedRole(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHelp(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedHelp)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedKeyboardShortcut(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedKeyboardShortcut)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedSelection(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDefaultAction(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedDefaultAction)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetIAccessible(&self) -> ::windows::core::Result<IAccessible> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIAccessible)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationLegacyIAccessiblePattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationLegacyIAccessiblePattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationLegacyIAccessiblePattern {
    type Vtable = IUIAutomationLegacyIAccessiblePattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationLegacyIAccessiblePattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x828055ad_355b_4435_86d5_3b51c14a9b1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationLegacyIAccessiblePattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows::core::HRESULT,
    pub DoDefaultAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub CurrentChildId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT,
    pub CurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT,
    pub CurrentHelp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszhelp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentKeyboardShortcut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentDefaultAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedChildId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT,
    pub CachedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT,
    pub CachedHelp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszhelp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedKeyboardShortcut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedDefaultAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetIAccessible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaccessible: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetIAccessible: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationMultipleViewPattern(::windows::core::IUnknown);
impl IUIAutomationMultipleViewPattern {
    pub unsafe fn GetViewName(&self, view: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetViewName)(::windows::core::Vtable::as_raw(self), view, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCurrentView(&self, view: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCurrentView)(::windows::core::Vtable::as_raw(self), view).ok()
    }
    pub unsafe fn CurrentCurrentView(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCurrentView)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCurrentSupportedViews(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentSupportedViews)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCurrentView(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedCurrentView)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCachedSupportedViews(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedSupportedViews)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationMultipleViewPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationMultipleViewPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationMultipleViewPattern {
    type Vtable = IUIAutomationMultipleViewPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationMultipleViewPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d253c91_1dc5_4bb5_b18f_ade16fa495e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationMultipleViewPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetViewName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: i32, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: i32) -> ::windows::core::HRESULT,
    pub CurrentCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCurrentSupportedViews: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCurrentSupportedViews: usize,
    pub CachedCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCachedSupportedViews: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCachedSupportedViews: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationNotCondition(::windows::core::IUnknown);
impl IUIAutomationNotCondition {
    pub unsafe fn GetChild(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetChild)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationNotCondition, ::windows::core::IUnknown, IUIAutomationCondition);
impl ::core::clone::Clone for IUIAutomationNotCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationNotCondition {
    type Vtable = IUIAutomationNotCondition_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationNotCondition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf528b657_847b_498c_8896_d52b565407a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationNotCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub GetChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationNotificationEventHandler(::windows::core::IUnknown);
impl IUIAutomationNotificationEventHandler {
    pub unsafe fn HandleNotificationEvent<P0>(&self, sender: P0, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: &::windows::core::BSTR, activityid: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).HandleNotificationEvent)(::windows::core::Vtable::as_raw(self), sender.into().abi(), notificationkind, notificationprocessing, ::core::mem::transmute_copy(displaystring), ::core::mem::transmute_copy(activityid)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationNotificationEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationNotificationEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationNotificationEventHandler {
    type Vtable = IUIAutomationNotificationEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationNotificationEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7cb2637_e6c2_4d0c_85de_4948c02175c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationNotificationEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub HandleNotificationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: *mut ::core::ffi::c_void, activityid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationObjectModelPattern(::windows::core::IUnknown);
impl IUIAutomationObjectModelPattern {
    pub unsafe fn GetUnderlyingObjectModel(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUnderlyingObjectModel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationObjectModelPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationObjectModelPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationObjectModelPattern {
    type Vtable = IUIAutomationObjectModelPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationObjectModelPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71c284b3_c14d_4d14_981e_19751b0d756d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationObjectModelPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUnderlyingObjectModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationOrCondition(::windows::core::IUnknown);
impl IUIAutomationOrCondition {
    pub unsafe fn ChildCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ChildCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetChildrenAsNativeArray(&self, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetChildrenAsNativeArray)(::windows::core::Vtable::as_raw(self), childarray, childarraycount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationOrCondition, ::windows::core::IUnknown, IUIAutomationCondition);
impl ::core::clone::Clone for IUIAutomationOrCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationOrCondition {
    type Vtable = IUIAutomationOrCondition_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationOrCondition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8753f032_3db1_47b5_a1fc_6e34a266c712);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationOrCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub ChildCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows::core::HRESULT,
    pub GetChildrenAsNativeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetChildren: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationPatternHandler(::windows::core::IUnknown);
impl IUIAutomationPatternHandler {
    pub unsafe fn CreateClientWrapper<P0>(&self, ppatterninstance: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationPatternInstance>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateClientWrapper)(::windows::core::Vtable::as_raw(self), ppatterninstance.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Dispatch<P0>(&self, ptarget: P0, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).Dispatch)(::windows::core::Vtable::as_raw(self), ptarget.into().abi(), index, pparams, cparams).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationPatternHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationPatternHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationPatternHandler {
    type Vtable = IUIAutomationPatternHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationPatternHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd97022f3_a947_465e_8b2a_ac4315fa54e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationPatternHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateClientWrapper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppatterninstance: *mut ::core::ffi::c_void, pclientwrapper: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Dispatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationPatternInstance(::windows::core::IUnknown);
impl IUIAutomationPatternInstance {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty<P0>(&self, index: u32, cached: P0, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).GetProperty)(::windows::core::Vtable::as_raw(self), index, cached.into(), r#type, pptr).ok()
    }
    pub unsafe fn CallMethod(&self, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CallMethod)(::windows::core::Vtable::as_raw(self), index, pparams, cparams).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationPatternInstance, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationPatternInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationPatternInstance {
    type Vtable = IUIAutomationPatternInstance_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationPatternInstance {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc03a7fe4_9431_409f_bed8_ae7c2299bc8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationPatternInstance_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, cached: super::super::Foundation::BOOL, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProperty: usize,
    pub CallMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationPropertyChangedEventHandler(::windows::core::IUnknown);
impl IUIAutomationPropertyChangedEventHandler {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn HandlePropertyChangedEvent<P0>(&self, sender: P0, propertyid: UIA_PROPERTY_ID, newvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).HandlePropertyChangedEvent)(::windows::core::Vtable::as_raw(self), sender.into().abi(), propertyid, ::core::mem::transmute(newvalue)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationPropertyChangedEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationPropertyChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationPropertyChangedEventHandler {
    type Vtable = IUIAutomationPropertyChangedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationPropertyChangedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40cd37d4_c756_4b0c_8c6f_bddfeeb13b50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationPropertyChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub HandlePropertyChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, newvalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    HandlePropertyChangedEvent: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationPropertyCondition(::windows::core::IUnknown);
impl IUIAutomationPropertyCondition {
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<UIA_PROPERTY_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PropertyValue(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PropertyConditionFlags(&self) -> ::windows::core::Result<PropertyConditionFlags> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyConditionFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationPropertyCondition, ::windows::core::IUnknown, IUIAutomationCondition);
impl ::core::clone::Clone for IUIAutomationPropertyCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationPropertyCondition {
    type Vtable = IUIAutomationPropertyCondition_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationPropertyCondition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99ebf2cb_5578_4267_9ad4_afd6ea77e94b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationPropertyCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub PropertyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: *mut UIA_PROPERTY_ID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PropertyValue: usize,
    pub PropertyConditionFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut PropertyConditionFlags) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationProxyFactory(::windows::core::IUnknown);
impl IUIAutomationProxyFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateProvider<P0>(&self, hwnd: P0, idobject: i32, idchild: i32) -> ::windows::core::Result<IRawElementProviderSimple>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateProvider)(::windows::core::Vtable::as_raw(self), hwnd.into(), idobject, idchild, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ProxyFactoryId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ProxyFactoryId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationProxyFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationProxyFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationProxyFactory {
    type Vtable = IUIAutomationProxyFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationProxyFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85b94ecd_849d_42b6_b94d_d6db23fdf5a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationProxyFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, provider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateProvider: usize,
    pub ProxyFactoryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factoryid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationProxyFactoryEntry(::windows::core::IUnknown);
impl IUIAutomationProxyFactoryEntry {
    pub unsafe fn ProxyFactory(&self) -> ::windows::core::Result<IUIAutomationProxyFactory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ProxyFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ImageName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ImageName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowSubstringMatch(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AllowSubstringMatch)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanCheckBaseClass(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CanCheckBaseClass)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NeedsAdviseEvents(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NeedsAdviseEvents)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClassName<P0>(&self, classname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetClassName)(::windows::core::Vtable::as_raw(self), classname.into().abi()).ok()
    }
    pub unsafe fn SetImageName<P0>(&self, imagename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetImageName)(::windows::core::Vtable::as_raw(self), imagename.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowSubstringMatch<P0>(&self, allowsubstringmatch: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetAllowSubstringMatch)(::windows::core::Vtable::as_raw(self), allowsubstringmatch.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCanCheckBaseClass<P0>(&self, cancheckbaseclass: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetCanCheckBaseClass)(::windows::core::Vtable::as_raw(self), cancheckbaseclass.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNeedsAdviseEvents<P0>(&self, adviseevents: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetNeedsAdviseEvents)(::windows::core::Vtable::as_raw(self), adviseevents.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWinEventsForAutomationEvent(&self, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetWinEventsForAutomationEvent)(::windows::core::Vtable::as_raw(self), eventid, propertyid, winevents).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWinEventsForAutomationEvent(&self, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWinEventsForAutomationEvent)(::windows::core::Vtable::as_raw(self), eventid, propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationProxyFactoryEntry, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationProxyFactoryEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationProxyFactoryEntry {
    type Vtable = IUIAutomationProxyFactoryEntry_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationProxyFactoryEntry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd50e472e_b64b_490c_bca1_d30696f9f289);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationProxyFactoryEntry_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ProxyFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ImageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowSubstringMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowsubstringmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowSubstringMatch: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanCheckBaseClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cancheckbaseclass: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanCheckBaseClass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NeedsAdviseEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adviseevents: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NeedsAdviseEvents: usize,
    pub SetClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetImageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowSubstringMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowsubstringmatch: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowSubstringMatch: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCanCheckBaseClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cancheckbaseclass: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCanCheckBaseClass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNeedsAdviseEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adviseevents: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNeedsAdviseEvents: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWinEventsForAutomationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWinEventsForAutomationEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWinEventsForAutomationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID, winevents: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWinEventsForAutomationEvent: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationProxyFactoryMapping(::windows::core::IUnknown);
impl IUIAutomationProxyFactoryMapping {
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTable(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEntry(&self, index: u32) -> ::windows::core::Result<IUIAutomationProxyFactoryEntry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEntry)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTable(&self, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTable)(::windows::core::Vtable::as_raw(self), factorylist).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEntries(&self, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InsertEntries)(::windows::core::Vtable::as_raw(self), before, factorylist).ok()
    }
    pub unsafe fn InsertEntry<P0>(&self, before: u32, factory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationProxyFactoryEntry>>,
    {
        (::windows::core::Vtable::vtable(self).InsertEntry)(::windows::core::Vtable::as_raw(self), before, factory.into().abi()).ok()
    }
    pub unsafe fn RemoveEntry(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveEntry)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn ClearTable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ClearTable)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RestoreDefaultTable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RestoreDefaultTable)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationProxyFactoryMapping, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationProxyFactoryMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationProxyFactoryMapping {
    type Vtable = IUIAutomationProxyFactoryMapping_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationProxyFactoryMapping {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09e31e18_872d_4873_93d1_1e541ec133fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationProxyFactoryMapping_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, table: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTable: usize,
    pub GetEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, entry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEntries: usize,
    pub InsertEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, before: u32, factory: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub ClearTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RestoreDefaultTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationRangeValuePattern(::windows::core::IUnknown);
impl IUIAutomationRangeValuePattern {
    pub unsafe fn SetValue(&self, val: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), val).ok()
    }
    pub unsafe fn CurrentValue(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsReadOnly(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsReadOnly)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentMaximum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentMaximum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentMinimum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentMinimum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLargeChange(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentLargeChange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentSmallChange(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentSmallChange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedValue(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsReadOnly(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsReadOnly)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedMaximum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedMaximum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedMinimum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedMinimum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLargeChange(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedLargeChange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedSmallChange(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedSmallChange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationRangeValuePattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationRangeValuePattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationRangeValuePattern {
    type Vtable = IUIAutomationRangeValuePattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationRangeValuePattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59213f4f_7346_49e5_b120_80555987a148);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationRangeValuePattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: f64) -> ::windows::core::HRESULT,
    pub CurrentValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsReadOnly: usize,
    pub CurrentMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CurrentMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CurrentLargeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CurrentSmallChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CachedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsReadOnly: usize,
    pub CachedMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CachedMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CachedLargeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CachedSmallChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationRegistrar(::windows::core::IUnknown);
impl IUIAutomationRegistrar {
    pub unsafe fn RegisterProperty(&self, property: *const UIAutomationPropertyInfo) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterProperty)(::windows::core::Vtable::as_raw(self), property, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterEvent(&self, event: *const UIAutomationEventInfo) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RegisterEvent)(::windows::core::Vtable::as_raw(self), event, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterPattern(&self, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, ppropertyids: &mut [i32], peventids: &mut [i32]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RegisterPattern)(::windows::core::Vtable::as_raw(self), pattern, ppatternid, ppatternavailablepropertyid, ppropertyids.len() as _, ::core::mem::transmute(ppropertyids.as_ptr()), peventids.len() as _, ::core::mem::transmute(peventids.as_ptr())).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationRegistrar, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationRegistrar {
    type Vtable = IUIAutomationRegistrar_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationRegistrar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8609c4ec_4a1a_4d88_a357_5a66e060e1cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationRegistrar_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *const UIAutomationPropertyInfo, propertyid: *mut i32) -> ::windows::core::HRESULT,
    pub RegisterEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *const UIAutomationEventInfo, eventid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterPattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, propertyidcount: u32, ppropertyids: *mut i32, eventidcount: u32, peventids: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterPattern: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationScrollItemPattern(::windows::core::IUnknown);
impl IUIAutomationScrollItemPattern {
    pub unsafe fn ScrollIntoView(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ScrollIntoView)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationScrollItemPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationScrollItemPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationScrollItemPattern {
    type Vtable = IUIAutomationScrollItemPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationScrollItemPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb488300f_d015_4f19_9c29_bb595e3645ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationScrollItemPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationScrollPattern(::windows::core::IUnknown);
impl IUIAutomationScrollPattern {
    pub unsafe fn Scroll(&self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Scroll)(::windows::core::Vtable::as_raw(self), horizontalamount, verticalamount).ok()
    }
    pub unsafe fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetScrollPercent)(::windows::core::Vtable::as_raw(self), horizontalpercent, verticalpercent).ok()
    }
    pub unsafe fn CurrentHorizontalScrollPercent(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentHorizontalScrollPercent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentVerticalScrollPercent(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentVerticalScrollPercent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHorizontalViewSize(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentHorizontalViewSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentVerticalViewSize(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentVerticalViewSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentHorizontallyScrollable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentHorizontallyScrollable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentVerticallyScrollable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentVerticallyScrollable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHorizontalScrollPercent(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedHorizontalScrollPercent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedVerticalScrollPercent(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedVerticalScrollPercent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHorizontalViewSize(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedHorizontalViewSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedVerticalViewSize(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedVerticalViewSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedHorizontallyScrollable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedHorizontallyScrollable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedVerticallyScrollable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedVerticallyScrollable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationScrollPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationScrollPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationScrollPattern {
    type Vtable = IUIAutomationScrollPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationScrollPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88f4d42a_e881_459d_a77c_73bbbb7e02dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationScrollPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::HRESULT,
    pub SetScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT,
    pub CurrentHorizontalScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CurrentVerticalScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CurrentHorizontalViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CurrentVerticalViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentHorizontallyScrollable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentHorizontallyScrollable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentVerticallyScrollable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentVerticallyScrollable: usize,
    pub CachedHorizontalScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CachedVerticalScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CachedHorizontalViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CachedVerticalViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedHorizontallyScrollable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedHorizontallyScrollable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedVerticallyScrollable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedVerticallyScrollable: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationSelectionItemPattern(::windows::core::IUnknown);
impl IUIAutomationSelectionItemPattern {
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Select)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddToSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveFromSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsSelected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsSelected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentSelectionContainer(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentSelectionContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsSelected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsSelected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedSelectionContainer(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedSelectionContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationSelectionItemPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationSelectionItemPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationSelectionItemPattern {
    type Vtable = IUIAutomationSelectionItemPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationSelectionItemPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8efa66a_0fda_421a_9194_38021f3578ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSelectionItemPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsSelected: usize,
    pub CurrentSelectionContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsSelected: usize,
    pub CachedSelectionContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationSelectionPattern(::windows::core::IUnknown);
impl IUIAutomationSelectionPattern {
    pub unsafe fn GetCurrentSelection(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanSelectMultiple(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCanSelectMultiple)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsSelectionRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsSelectionRequired)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedSelection(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanSelectMultiple(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedCanSelectMultiple)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsSelectionRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsSelectionRequired)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationSelectionPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationSelectionPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationSelectionPattern {
    type Vtable = IUIAutomationSelectionPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationSelectionPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ed5202e_b2ac_47a6_b638_4b0bf140d78e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSelectionPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrentSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentCanSelectMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentCanSelectMultiple: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsSelectionRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsSelectionRequired: usize,
    pub GetCachedSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedCanSelectMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedCanSelectMultiple: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsSelectionRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsSelectionRequired: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationSelectionPattern2(::windows::core::IUnknown);
impl IUIAutomationSelectionPattern2 {
    pub unsafe fn CurrentFirstSelectedItem(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentFirstSelectedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLastSelectedItem(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentLastSelectedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCurrentSelectedItem(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCurrentSelectedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFirstSelectedItem(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedFirstSelectedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLastSelectedItem(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedLastSelectedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCurrentSelectedItem(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedCurrentSelectedItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationSelectionPattern2, ::windows::core::IUnknown, IUIAutomationSelectionPattern);
impl ::core::clone::Clone for IUIAutomationSelectionPattern2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationSelectionPattern2 {
    type Vtable = IUIAutomationSelectionPattern2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationSelectionPattern2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0532bfae_c011_4e32_a343_6d642d798555);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSelectionPattern2_Vtbl {
    pub base__: IUIAutomationSelectionPattern_Vtbl,
    pub CurrentFirstSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentLastSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentCurrentSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedFirstSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedLastSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedCurrentSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationSpreadsheetItemPattern(::windows::core::IUnknown);
impl IUIAutomationSpreadsheetItemPattern {
    pub unsafe fn CurrentFormula(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentFormula)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCurrentAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFormula(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedFormula)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCachedAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationSpreadsheetItemPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationSpreadsheetItemPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationSpreadsheetItemPattern {
    type Vtable = IUIAutomationSpreadsheetItemPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationSpreadsheetItemPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d4fb86c_8d34_40e1_8e83_62c15204e335);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSpreadsheetItemPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CurrentFormula: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentAnnotationObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCurrentAnnotationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCurrentAnnotationTypes: usize,
    pub CachedFormula: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedAnnotationObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCachedAnnotationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCachedAnnotationTypes: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationSpreadsheetPattern(::windows::core::IUnknown);
impl IUIAutomationSpreadsheetPattern {
    pub unsafe fn GetItemByName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItemByName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationSpreadsheetPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationSpreadsheetPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationSpreadsheetPattern {
    type Vtable = IUIAutomationSpreadsheetPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationSpreadsheetPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7517a7c8_faae_4de9_9f08_29b91e8595c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSpreadsheetPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationStructureChangedEventHandler(::windows::core::IUnknown);
impl IUIAutomationStructureChangedEventHandler {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HandleStructureChangedEvent<P0>(&self, sender: P0, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).HandleStructureChangedEvent)(::windows::core::Vtable::as_raw(self), sender.into().abi(), changetype, runtimeid).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationStructureChangedEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationStructureChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationStructureChangedEventHandler {
    type Vtable = IUIAutomationStructureChangedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationStructureChangedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe81d1b4e_11c5_42f8_9754_e7036c79f054);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationStructureChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub HandleStructureChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HandleStructureChangedEvent: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationStylesPattern(::windows::core::IUnknown);
impl IUIAutomationStylesPattern {
    pub unsafe fn CurrentStyleId(&self) -> ::windows::core::Result<UIA_STYLE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentStyleId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentStyleName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentStyleName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFillColor(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentFillColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFillPatternStyle(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentFillPatternStyle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentShape(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentShape)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFillPatternColor(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentFillPatternColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentExtendedProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentExtendedProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCurrentExtendedPropertiesAsArray)(::windows::core::Vtable::as_raw(self), propertyarray, propertycount).ok()
    }
    pub unsafe fn CachedStyleId(&self) -> ::windows::core::Result<UIA_STYLE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedStyleId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedStyleName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedStyleName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFillColor(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedFillColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFillPatternStyle(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedFillPatternStyle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedShape(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedShape)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFillPatternColor(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedFillPatternColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedExtendedProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedExtendedProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCachedExtendedPropertiesAsArray)(::windows::core::Vtable::as_raw(self), propertyarray, propertycount).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationStylesPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationStylesPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationStylesPattern {
    type Vtable = IUIAutomationStylesPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationStylesPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85b5f0a2_bd79_484a_ad2b_388c9838d5fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationStylesPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CurrentStyleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_STYLE_ID) -> ::windows::core::HRESULT,
    pub CurrentStyleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentFillColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentFillPatternStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentFillPatternColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentExtendedPropertiesAsArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::HRESULT,
    pub CachedStyleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UIA_STYLE_ID) -> ::windows::core::HRESULT,
    pub CachedStyleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedFillColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedFillPatternStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedFillPatternColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT,
    pub CachedExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedExtendedPropertiesAsArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationSynchronizedInputPattern(::windows::core::IUnknown);
impl IUIAutomationSynchronizedInputPattern {
    pub unsafe fn StartListening(&self, inputtype: SynchronizedInputType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).StartListening)(::windows::core::Vtable::as_raw(self), inputtype).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationSynchronizedInputPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationSynchronizedInputPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationSynchronizedInputPattern {
    type Vtable = IUIAutomationSynchronizedInputPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationSynchronizedInputPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2233be0b_afb7_448b_9fda_3b378aa5eae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSynchronizedInputPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartListening: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTableItemPattern(::windows::core::IUnknown);
impl IUIAutomationTableItemPattern {
    pub unsafe fn GetCurrentRowHeaderItems(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentRowHeaderItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentColumnHeaderItems(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentColumnHeaderItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedRowHeaderItems(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedRowHeaderItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedColumnHeaderItems(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedColumnHeaderItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTableItemPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationTableItemPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTableItemPattern {
    type Vtable = IUIAutomationTableItemPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTableItemPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b964eb3_ef2e_4464_9c79_61d61737a27e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTableItemPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrentRowHeaderItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentColumnHeaderItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedRowHeaderItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedColumnHeaderItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTablePattern(::windows::core::IUnknown);
impl IUIAutomationTablePattern {
    pub unsafe fn GetCurrentRowHeaders(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentRowHeaders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentColumnHeaders(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrentColumnHeaders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentRowOrColumnMajor(&self) -> ::windows::core::Result<RowOrColumnMajor> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentRowOrColumnMajor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedRowHeaders(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedRowHeaders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedColumnHeaders(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCachedColumnHeaders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedRowOrColumnMajor(&self) -> ::windows::core::Result<RowOrColumnMajor> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedRowOrColumnMajor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTablePattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationTablePattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTablePattern {
    type Vtable = IUIAutomationTablePattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTablePattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x620e691c_ea96_4710_a850_754b24ce2417);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTablePattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrentRowHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentColumnHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentRowOrColumnMajor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT,
    pub GetCachedRowHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCachedColumnHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CachedRowOrColumnMajor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTextChildPattern(::windows::core::IUnknown);
impl IUIAutomationTextChildPattern {
    pub unsafe fn TextContainer(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TextContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TextRange(&self) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).TextRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTextChildPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationTextChildPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTextChildPattern {
    type Vtable = IUIAutomationTextChildPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTextChildPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6552b038_ae05_40c8_abfd_aa08352aab86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextChildPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub TextContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, container: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTextEditPattern(::windows::core::IUnknown);
impl IUIAutomationTextEditPattern {
    pub unsafe fn GetActiveComposition(&self) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetActiveComposition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetConversionTarget(&self) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetConversionTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTextEditPattern, ::windows::core::IUnknown, IUIAutomationTextPattern);
impl ::core::clone::Clone for IUIAutomationTextEditPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTextEditPattern {
    type Vtable = IUIAutomationTextEditPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTextEditPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17e21576_996c_4870_99d9_bff323380c06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextEditPattern_Vtbl {
    pub base__: IUIAutomationTextPattern_Vtbl,
    pub GetActiveComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConversionTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTextEditTextChangedEventHandler(::windows::core::IUnknown);
impl IUIAutomationTextEditTextChangedEventHandler {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HandleTextEditTextChangedEvent<P0>(&self, sender: P0, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).HandleTextEditTextChangedEvent)(::windows::core::Vtable::as_raw(self), sender.into().abi(), texteditchangetype, eventstrings).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTextEditTextChangedEventHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationTextEditTextChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTextEditTextChangedEventHandler {
    type Vtable = IUIAutomationTextEditTextChangedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTextEditTextChangedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92faa680_e704_4156_931a_e32d5bb38f3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextEditTextChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub HandleTextEditTextChangedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HandleTextEditTextChangedEvent: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTextPattern(::windows::core::IUnknown);
impl IUIAutomationTextPattern {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RangeFromPoint(&self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RangeFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeFromChild<P0>(&self, child: P0) -> ::windows::core::Result<IUIAutomationTextRange>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RangeFromChild)(::windows::core::Vtable::as_raw(self), child.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<IUIAutomationTextRangeArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVisibleRanges(&self) -> ::windows::core::Result<IUIAutomationTextRangeArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVisibleRanges)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DocumentRange(&self) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DocumentRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SupportedTextSelection(&self) -> ::windows::core::Result<SupportedTextSelection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SupportedTextSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTextPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationTextPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTextPattern {
    type Vtable = IUIAutomationTextPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTextPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32eba289_3583_42c9_9c59_3b6d9a1e9b6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RangeFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, range: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RangeFromPoint: usize,
    pub RangeFromChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, child: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ranges: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVisibleRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ranges: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SupportedTextSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedtextselection: *mut SupportedTextSelection) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTextPattern2(::windows::core::IUnknown);
impl IUIAutomationTextPattern2 {
    pub unsafe fn RangeFromAnnotation<P0>(&self, annotation: P0) -> ::windows::core::Result<IUIAutomationTextRange>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RangeFromAnnotation)(::windows::core::Vtable::as_raw(self), annotation.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCaretRange(&self, isactive: *mut super::super::Foundation::BOOL, range: *mut ::core::option::Option<IUIAutomationTextRange>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCaretRange)(::windows::core::Vtable::as_raw(self), isactive, ::core::mem::transmute(range)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTextPattern2, ::windows::core::IUnknown, IUIAutomationTextPattern);
impl ::core::clone::Clone for IUIAutomationTextPattern2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTextPattern2 {
    type Vtable = IUIAutomationTextPattern2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTextPattern2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x506a921a_fcc9_409f_b23b_37eb74106872);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextPattern2_Vtbl {
    pub base__: IUIAutomationTextPattern_Vtbl,
    pub RangeFromAnnotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotation: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCaretRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, range: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCaretRange: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTextRange(::windows::core::IUnknown);
impl IUIAutomationTextRange {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, range: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Compare)(::windows::core::Vtable::as_raw(self), range.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareEndpoints<P0>(&self, srcendpoint: TextPatternRangeEndpoint, range: P0, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CompareEndpoints)(::windows::core::Vtable::as_raw(self), srcendpoint, range.into().abi(), targetendpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ExpandToEnclosingUnit(&self, textunit: TextUnit) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ExpandToEnclosingUnit)(::windows::core::Vtable::as_raw(self), textunit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FindAttribute<P0>(&self, attr: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Com::VARIANT, backward: P0) -> ::windows::core::Result<IUIAutomationTextRange>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindAttribute)(::windows::core::Vtable::as_raw(self), attr, ::core::mem::transmute(val), backward.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindText<P0, P1>(&self, text: &::windows::core::BSTR, backward: P0, ignorecase: P1) -> ::windows::core::Result<IUIAutomationTextRange>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FindText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(text), backward.into(), ignorecase.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttributeValue(&self, attr: UIA_TEXTATTRIBUTE_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAttributeValue)(::windows::core::Vtable::as_raw(self), attr, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBoundingRectangles(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBoundingRectangles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEnclosingElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEnclosingElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetText(&self, maxlength: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetText)(::windows::core::Vtable::as_raw(self), maxlength, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Move)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveEndpointByUnit)(::windows::core::Vtable::as_raw(self), endpoint, unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEndpointByRange<P0>(&self, srcendpoint: TextPatternRangeEndpoint, range: P0, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextRange>>,
    {
        (::windows::core::Vtable::vtable(self).MoveEndpointByRange)(::windows::core::Vtable::as_raw(self), srcendpoint, range.into().abi(), targetendpoint).ok()
    }
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Select)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddToSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveFromSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScrollIntoView<P0>(&self, aligntotop: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).ScrollIntoView)(::windows::core::Vtable::as_raw(self), aligntotop.into()).ok()
    }
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTextRange, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationTextRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTextRange {
    type Vtable = IUIAutomationTextRange_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTextRange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa543cc6a_f4ae_494b_8239_c814481187a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextRange_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clonedrange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Compare: usize,
    pub CompareEndpoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint, compvalue: *mut i32) -> ::windows::core::HRESULT,
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textunit: TextUnit) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FindAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attr: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Com::VARIANT, backward: super::super::Foundation::BOOL, found: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FindAttribute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FindText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, found: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindText: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttributeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attr: UIA_TEXTATTRIBUTE_ID, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttributeValue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBoundingRectangles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundingrects: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBoundingRectangles: usize,
    pub GetEnclosingElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enclosingelement: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlength: i32, text: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows::core::HRESULT,
    pub MoveEndpointByUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows::core::HRESULT,
    pub MoveEndpointByRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::HRESULT,
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScrollIntoView: usize,
    pub GetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTextRange2(::windows::core::IUnknown);
impl IUIAutomationTextRange2 {
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTextRange2, ::windows::core::IUnknown, IUIAutomationTextRange);
impl ::core::clone::Clone for IUIAutomationTextRange2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTextRange2 {
    type Vtable = IUIAutomationTextRange2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTextRange2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb9b40e0_5e04_46bd_9be0_4b601b9afad4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextRange2_Vtbl {
    pub base__: IUIAutomationTextRange_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTextRange3(::windows::core::IUnknown);
impl IUIAutomationTextRange3 {
    pub unsafe fn GetEnclosingElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEnclosingElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetChildrenBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetChildrenBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAttributeValues(&self, attributeids: &[UIA_TEXTATTRIBUTE_ID]) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAttributeValues)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(attributeids.as_ptr()), attributeids.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTextRange3, ::windows::core::IUnknown, IUIAutomationTextRange, IUIAutomationTextRange2);
impl ::core::clone::Clone for IUIAutomationTextRange3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTextRange3 {
    type Vtable = IUIAutomationTextRange3_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTextRange3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a315d69_5512_4c2e_85f0_53fce6dd4bc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextRange3_Vtbl {
    pub base__: IUIAutomationTextRange2_Vtbl,
    pub GetEnclosingElementBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, enclosingelement: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetChildrenBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAttributeValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributeids: *const UIA_TEXTATTRIBUTE_ID, attributeidcount: i32, attributevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAttributeValues: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTextRangeArray(::windows::core::IUnknown);
impl IUIAutomationTextRangeArray {
    pub unsafe fn Length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Length)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetElement(&self, index: i32) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetElement)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTextRangeArray, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationTextRangeArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTextRangeArray {
    type Vtable = IUIAutomationTextRangeArray_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTextRangeArray {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce4ae76a_e717_4c98_81ea_47371d028eb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextRangeArray_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT,
    pub GetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTogglePattern(::windows::core::IUnknown);
impl IUIAutomationTogglePattern {
    pub unsafe fn Toggle(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Toggle)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CurrentToggleState(&self) -> ::windows::core::Result<ToggleState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentToggleState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedToggleState(&self) -> ::windows::core::Result<ToggleState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedToggleState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTogglePattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationTogglePattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTogglePattern {
    type Vtable = IUIAutomationTogglePattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTogglePattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94cf8058_9b8d_4ab9_8bfd_4cd0a33c8c70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTogglePattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Toggle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentToggleState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows::core::HRESULT,
    pub CachedToggleState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTransformPattern(::windows::core::IUnknown);
impl IUIAutomationTransformPattern {
    pub unsafe fn Move(&self, x: f64, y: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Move)(::windows::core::Vtable::as_raw(self), x, y).ok()
    }
    pub unsafe fn Resize(&self, width: f64, height: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Resize)(::windows::core::Vtable::as_raw(self), width, height).ok()
    }
    pub unsafe fn Rotate(&self, degrees: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Rotate)(::windows::core::Vtable::as_raw(self), degrees).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanMove(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCanMove)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanResize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCanResize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanRotate(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCanRotate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanMove(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedCanMove)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanResize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedCanResize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanRotate(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedCanRotate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTransformPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationTransformPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTransformPattern {
    type Vtable = IUIAutomationTransformPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTransformPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b55844_a55d_4ef0_926d_569c16ff89bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTransformPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentCanMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentCanMove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentCanResize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentCanResize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentCanRotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentCanRotate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedCanMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedCanMove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedCanResize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedCanResize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedCanRotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedCanRotate: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTransformPattern2(::windows::core::IUnknown);
impl IUIAutomationTransformPattern2 {
    pub unsafe fn Zoom(&self, zoomvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Zoom)(::windows::core::Vtable::as_raw(self), zoomvalue).ok()
    }
    pub unsafe fn ZoomByUnit(&self, zoomunit: ZoomUnit) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ZoomByUnit)(::windows::core::Vtable::as_raw(self), zoomunit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanZoom(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCanZoom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanZoom(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedCanZoom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentZoomLevel(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentZoomLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedZoomLevel(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedZoomLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentZoomMinimum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentZoomMinimum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedZoomMinimum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedZoomMinimum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentZoomMaximum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentZoomMaximum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedZoomMaximum(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedZoomMaximum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTransformPattern2, ::windows::core::IUnknown, IUIAutomationTransformPattern);
impl ::core::clone::Clone for IUIAutomationTransformPattern2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTransformPattern2 {
    type Vtable = IUIAutomationTransformPattern2_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTransformPattern2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d74d017_6ecb_4381_b38b_3c17a48ff1c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTransformPattern2_Vtbl {
    pub base__: IUIAutomationTransformPattern_Vtbl,
    pub Zoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zoomvalue: f64) -> ::windows::core::HRESULT,
    pub ZoomByUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentCanZoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentCanZoom: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedCanZoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedCanZoom: usize,
    pub CurrentZoomLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CachedZoomLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CurrentZoomMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CachedZoomMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CurrentZoomMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
    pub CachedZoomMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationTreeWalker(::windows::core::IUnknown);
impl IUIAutomationTreeWalker {
    pub unsafe fn GetParentElement<P0>(&self, element: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetParentElement)(::windows::core::Vtable::as_raw(self), element.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFirstChildElement<P0>(&self, element: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFirstChildElement)(::windows::core::Vtable::as_raw(self), element.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLastChildElement<P0>(&self, element: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLastChildElement)(::windows::core::Vtable::as_raw(self), element.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNextSiblingElement<P0>(&self, element: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNextSiblingElement)(::windows::core::Vtable::as_raw(self), element.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPreviousSiblingElement<P0>(&self, element: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPreviousSiblingElement)(::windows::core::Vtable::as_raw(self), element.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NormalizeElement<P0>(&self, element: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NormalizeElement)(::windows::core::Vtable::as_raw(self), element.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetParentElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetParentElementBuildCache)(::windows::core::Vtable::as_raw(self), element.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFirstChildElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFirstChildElementBuildCache)(::windows::core::Vtable::as_raw(self), element.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLastChildElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLastChildElementBuildCache)(::windows::core::Vtable::as_raw(self), element.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNextSiblingElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNextSiblingElementBuildCache)(::windows::core::Vtable::as_raw(self), element.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPreviousSiblingElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPreviousSiblingElementBuildCache)(::windows::core::Vtable::as_raw(self), element.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NormalizeElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NormalizeElementBuildCache)(::windows::core::Vtable::as_raw(self), element.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Condition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Condition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationTreeWalker, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationTreeWalker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationTreeWalker {
    type Vtable = IUIAutomationTreeWalker_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationTreeWalker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4042c624_389c_4afc_a630_9df854a541fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTreeWalker_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetParentElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFirstChildElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, first: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLastChildElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, last: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNextSiblingElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, next: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPreviousSiblingElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, previous: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NormalizeElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, normalized: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParentElementBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFirstChildElementBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, first: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLastChildElementBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, last: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNextSiblingElementBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, next: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPreviousSiblingElementBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, previous: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NormalizeElementBuildCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, normalized: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationValuePattern(::windows::core::IUnknown);
impl IUIAutomationValuePattern {
    pub unsafe fn SetValue(&self, val: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(val)).ok()
    }
    pub unsafe fn CurrentValue(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsReadOnly(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsReadOnly)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedValue(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsReadOnly(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsReadOnly)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationValuePattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationValuePattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationValuePattern {
    type Vtable = IUIAutomationValuePattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationValuePattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa94cd8b1_0844_4cd6_9d2d_640537ab39e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationValuePattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsReadOnly: usize,
    pub CachedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsReadOnly: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationVirtualizedItemPattern(::windows::core::IUnknown);
impl IUIAutomationVirtualizedItemPattern {
    pub unsafe fn Realize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Realize)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUIAutomationVirtualizedItemPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationVirtualizedItemPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationVirtualizedItemPattern {
    type Vtable = IUIAutomationVirtualizedItemPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationVirtualizedItemPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ba3d7a6_04cf_4f11_8793_a8d1cde9969f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationVirtualizedItemPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Realize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IUIAutomationWindowPattern(::windows::core::IUnknown);
impl IUIAutomationWindowPattern {
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForInputIdle(&self, milliseconds: i32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WaitForInputIdle)(::windows::core::Vtable::as_raw(self), milliseconds, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetWindowVisualState(&self, state: WindowVisualState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetWindowVisualState)(::windows::core::Vtable::as_raw(self), state).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanMaximize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCanMaximize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanMinimize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentCanMinimize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsModal(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsModal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsTopmost(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentIsTopmost)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentWindowVisualState(&self) -> ::windows::core::Result<WindowVisualState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentWindowVisualState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentWindowInteractionState(&self) -> ::windows::core::Result<WindowInteractionState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurrentWindowInteractionState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanMaximize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedCanMaximize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanMinimize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedCanMinimize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsModal(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsModal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsTopmost(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedIsTopmost)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedWindowVisualState(&self) -> ::windows::core::Result<WindowVisualState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedWindowVisualState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedWindowInteractionState(&self) -> ::windows::core::Result<WindowInteractionState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CachedWindowInteractionState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IUIAutomationWindowPattern, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUIAutomationWindowPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IUIAutomationWindowPattern {
    type Vtable = IUIAutomationWindowPattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IUIAutomationWindowPattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0faef453_9208_43ef_bbb2_3b485177864f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationWindowPattern_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitForInputIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, milliseconds: i32, success: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitForInputIdle: usize,
    pub SetWindowVisualState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentCanMaximize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentCanMaximize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentCanMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentCanMinimize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsModal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentIsTopmost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentIsTopmost: usize,
    pub CurrentWindowVisualState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows::core::HRESULT,
    pub CurrentWindowInteractionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedCanMaximize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedCanMaximize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedCanMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedCanMinimize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsModal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CachedIsTopmost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CachedIsTopmost: usize,
    pub CachedWindowVisualState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows::core::HRESULT,
    pub CachedWindowInteractionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IValueProvider(::windows::core::IUnknown);
impl IValueProvider {
    pub unsafe fn SetValue<P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), val.into().abi()).ok()
    }
    pub unsafe fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsReadOnly(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsReadOnly)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IValueProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IValueProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IValueProvider {
    type Vtable = IValueProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IValueProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7935180_6fb3_4201_b174_7df73adbf64a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsReadOnly: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IVirtualizedItemProvider(::windows::core::IUnknown);
impl IVirtualizedItemProvider {
    pub unsafe fn Realize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Realize)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IVirtualizedItemProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IVirtualizedItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IVirtualizedItemProvider {
    type Vtable = IVirtualizedItemProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IVirtualizedItemProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb98b665_2d35_4fac_ad35_f3c60d0c0b8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualizedItemProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Realize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
pub struct IWindowProvider(::windows::core::IUnknown);
impl IWindowProvider {
    pub unsafe fn SetVisualState(&self, state: WindowVisualState) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVisualState)(::windows::core::Vtable::as_raw(self), state).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForInputIdle(&self, milliseconds: i32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WaitForInputIdle)(::windows::core::Vtable::as_raw(self), milliseconds, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanMaximize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CanMaximize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanMinimize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CanMinimize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsModal(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsModal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn WindowVisualState(&self) -> ::windows::core::Result<WindowVisualState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WindowVisualState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn WindowInteractionState(&self) -> ::windows::core::Result<WindowInteractionState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WindowInteractionState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTopmost(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsTopmost)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IWindowProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWindowProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IWindowProvider {
    type Vtable = IWindowProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IWindowProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x987df77b_db06_4d77_8f8a_86a9c3bb90b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetVisualState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitForInputIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, milliseconds: i32, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitForInputIdle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanMaximize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanMaximize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanMinimize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsModal: usize,
    pub WindowVisualState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut WindowVisualState) -> ::windows::core::HRESULT,
    pub WindowInteractionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut WindowInteractionState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTopmost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTopmost: usize,
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ANRUS_PRIORITY_AUDIO_DYNAMIC_DUCK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AcceleratorKey_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x514865df_2557_4cb9_aeed_6ced084ce52c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AccessKey_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06827b12_a7f9_4a15_917c_ffa5ad3eb0a7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ActiveTextPositionChanged_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5c09e9c_c77d_4f25_b491_e5bb7017cbd4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationObjects_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x310910c8_7c6e_4f20_becd_4aaf6d191156);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationTypes_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64b71f76_53c4_4696_a219_20e940c9a176);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_AdvancedProofingIssue_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdac7b72c_c0f2_4b84_b90d_5fafc0f0ef1c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_AnnotationTypeId_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20ae484f_69ef_4c48_8f5b_c4938b206ac7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_AnnotationTypeName_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b818892_5ac9_4af9_aa96_f58a77b058e3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Author_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf161d3a7_f81b_4128_b17f_71f690914520);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Author_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a528462_9c5c_4a03_a974_8b307a9937f2);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_CircularReferenceError_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25bd9cf4_1745_4659_ba67_727f0318c616);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Comment_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd2fda30_26b3_4c06_8bc7_98f1532e46fd);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_ConflictingChange_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98af8802_517c_459f_af13_016d3fab877e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Custom_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ec82750_3931_4952_85bc_1dbff78a43e3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_DataValidationError_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8649fa8_9775_437e_ad46_e709d93c2343);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_DateTime_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99b5ca5d_1acf_414b_a4d0_6b350b047578);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_DeletionChange_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe3d5b05_951d_42e7_901d_adc8c2cf34d0);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_EditingLockedChange_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc31f3e1c_7423_4dac_8348_41f099ff6f64);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Endnote_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7565725c_2d99_4839_960d_33d3b866aba5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_ExternalChange_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75a05b31_5f11_42fd_887d_dfa010db2392);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Footer_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcceab046_1833_47aa_8080_701ed0b0c832);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Footnote_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3de10e21_4125_42db_8620_be8083080624);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_FormatChange_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb247345_d4f1_41ce_8e52_f79b69635e48);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_FormulaError_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95611982_0cab_46d5_a2f0_e30d1905f8bf);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_GrammarError_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x757a048d_4518_41c6_854c_dc009b7cfb53);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Header_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x867b409b_b216_4472_a219_525e310681f8);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Highlighted_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x757c884e_8083_4081_8b9c_e87f5072f0e4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_InsertionChange_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dbeb3a6_df15_4164_a3c0_e21a8ce931c4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Mathematics_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeaab634b_26d0_40c1_8073_57ca1c633c9b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_MoveChange_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9da587eb_23e5_4490_b385_1a22ddc8b187);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6c72ad7_356c_4850_9291_316f608a8c84);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Sensitive_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37f4c04f_0f12_4464_929c_828fd15292e3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_SpellingError_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae85567e_9ece_423f_81b7_96c43d53e50e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_Target_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb71b302d_2104_44ad_9c5c_092b4907d70f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_TrackChanges_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21e6e888_dc14_4016_ac27_190553c8c470);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Annotation_UnsyncedChange_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1851116a_0e47_4b30_8cb5_d7dae4fbcd1b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AppBar_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6114908d_cc02_4d37_875b_b530c7139554);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AriaProperties_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4213678c_e025_4922_beb5_e43ba08e6221);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AriaRole_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd207b95_be4a_4e0d_b727_63ace94b6916);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AsyncContentLoaded_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fdee11c_d2fa_4fb9_904e_5cbee894d5ef);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationFocusChanged_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb68a1f17_f60d_41a7_a3cc_b05292155fe0);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationId_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc82c0500_b60e_4310_a267_303c531f8ee5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationPropertyChanged_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2527fba1_8d7a_4630_a4cc_e66315942f52);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const BoundingRectangle_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bbfe8b2_3bfc_48dd_b729_c794b846e9a1);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Button_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a78e369_c6a1_4f33_a9d7_79f20d0c788e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CAccPropServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5f8350b_0548_48b1_a6ee_88bd00b4a5e7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CLSID_AccPropServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5f8350b_0548_48b1_a6ee_88bd00b4a5e7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CUIAutomation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff48dba4_60ef_4201_aa87_54103eef594e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CUIAutomation8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe22ad333_b25f_460c_83d0_0581107395c9);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CUIAutomationRegistrar: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e29fabf_9977_42d1_8d0e_ca7e61ad87e6);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Calendar_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8913eb88_00e5_46bc_8e4e_14a786e165a1);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CenterPoint_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cb00c08_540c_4edb_9445_26359ea69785);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Changes_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7df26714_614f_4e05_9488_716c5ba19436);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Changes_Summary_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x313d65a6_e60f_4d62_9861_55afd728d207);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CheckBox_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb50f922_a3db_49c0_8bc3_06dad55778e2);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ClassName_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x157b7215_894f_4b65_84e2_aac0da08b16b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ClickablePoint_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0196903b_b203_4818_a9f3_f08e675f2341);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ComboBox_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54cb426c_2f33_4fff_aaa1_aef60dac5deb);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ControlType_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca774fea_28ac_4bc2_94ca_acec6d6c10a3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ControllerFor_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51124c8a_a5d2_4f13_9be6_7fa8ba9d3a90);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Culture_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2d74f27_3d79_4dc2_b88b_3044963a8afb);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CustomNavigation_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafea938a_621e_4054_bb2c_2f46114dac3f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Custom_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf29ea0c3_adb7_430a_ba90_e52c7313e6ed);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_CHILD: i32 = -5002i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_CHILDCOUNT: i32 = -5001i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_DEFAULTACTION: i32 = -5013i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_DESCRIPTION: i32 = -5005i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_DODEFAULTACTION: i32 = -5018i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_FOCUS: i32 = -5011i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_HELP: i32 = -5008i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_HELPTOPIC: i32 = -5009i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_HITTEST: i32 = -5017i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_KEYBOARDSHORTCUT: i32 = -5010i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_LOCATION: i32 = -5015i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_NAME: i32 = -5003i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_NAVIGATE: i32 = -5016i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_PARENT: i32 = -5000i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_ROLE: i32 = -5006i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_SELECT: i32 = -5014i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_SELECTION: i32 = -5012i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_STATE: i32 = -5007i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DISPID_ACC_VALUE: i32 = -5004i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DataGrid_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b783af_d103_4b0a_8415_e73942410f4b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DataItem_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0177842_d94f_42a5_814b_6068addc8da5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DescribedBy_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c5865b8_9992_40fd_8db0_6bf1d317f998);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Dock_DockPosition_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d67f02e_c0b0_4b10_b5b9_18d6ecf98760);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Dock_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cbaa846_83c8_428d_827f_7e6063fe0620);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Document_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cd6bb6f_6f08_4562_b229_e4e2fc7a9eb4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Drag_DragCancel_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3ede6fa_3451_4e0f_9e71_df9c280a4657);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Drag_DragComplete_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38e96188_ef1f_463e_91ca_3a7792c29caf);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Drag_DragStart_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x883a480b_3aa9_429d_95e4_d9c8d011f0dd);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Drag_DropEffect_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x646f2779_48d3_4b23_8902_4bf100005df3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Drag_DropEffects_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5d61156_7ce6_49be_a836_9269dcec920f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Drag_GrabbedItems_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77c1562c_7b86_4b21_9ed7_3cefda6f4c43);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Drag_IsGrabbed_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45f206f3_75cc_4cca_a9b9_fcdfb982d8a2);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Drag_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0bee21f_ccb3_4fed_995b_114f6e3d2728);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DropTarget_DragEnter_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaad9319b_032c_4a88_961d_1cf579581e34);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DropTarget_DragLeave_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f82eb15_24a2_4988_9217_de162aee272b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DropTarget_DropTargetEffect_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bb75975_a0ca_4981_b818_87fc66e9509d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DropTarget_DropTargetEffects_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc1dd4ed_cb89_45f1_a592_e03b08ae790f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DropTarget_Dropped_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x622cead8_1edb_4a3d_abbc_be2211ff68b5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DropTarget_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bcbec56_bd34_4b7b_9fd5_2659905ea3dc);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Edit_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6504a5c8_2c86_4f87_ae7b_1abddc810cf9);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ExpandCollapse_ExpandCollapseState_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x275a4c48_85a7_4f69_aba0_af157610002b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ExpandCollapse_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae05efa2_f9d1_428a_834c_53a5c52f9b8b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FillColor_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e0ec4d0_e2a8_4a56_9de7_953389933b39);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FillType_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6fc74e4_8cb9_429c_a9e1_9bc4ac372b62);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FlowsFrom_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05c6844f_19de_48f8_95fa_880d5b0fd615);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FlowsTo_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4f33d20_559a_47fb_a830_f9cb4ff1a70a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FrameworkId_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbfd9900_7e1a_4f58_b61b_7063120f773b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FullDescription_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d4450ff_6aef_4f33_95dd_7befa72a4391);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const GridItem_ColumnSpan_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x583ea3f5_86d0_4b08_a6ec_2c5463ffc109);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const GridItem_Column_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc774c15c_62c0_4519_8bdc_47be573c8ad5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const GridItem_Parent_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d912252_b97f_4ecc_8510_ea0e33427c72);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const GridItem_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2d5c877_a462_4957_a2a5_2c96b303bc63);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const GridItem_RowSpan_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4582291c_466b_4e93_8e83_3d1715ec0c5e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const GridItem_Row_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6223972a_c945_4563_9329_fdc974af2553);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Grid_ColumnCount_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe96f375_44aa_4536_ac7a_2a75d71a3efc);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Grid_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x260a2ccb_93a8_4e44_a4c1_3df397f2b02b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Grid_RowCount_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a9505bf_c2eb_4fb6_b356_8245ae53703e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Group_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad50aa1c_e8c8_4774_ae1b_dd86df0b3bdc);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HasKeyboardFocus_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf8afd39_3f46_4800_9656_b2bf12529905);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeaderItem_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6bc12cb_7c8e_49cf_b168_4a93a32bebb0);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Header_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b90cbce_78fb_4614_82b6_554d74718e67);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29084272_aaaf_4a30_8796_3c12f62b6bbb);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HelpText_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08555685_0977_45c7_a7a6_abaf5684121a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HostedFragmentRootsInvalidated_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6bdb03e_0921_4ec5_8dcf_eae877b0426b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Hyperlink_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a56022c_b00d_4d15_8ff0_5b6b266e5e02);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IIS_ControlAccessible: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38c682a6_9731_43f2_9fae_e901e641b101);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IIS_IsOleaccProxy: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x902697fa_80e4_4560_802a_a13f22a64709);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Image_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d3736e4_6b16_4c57_a962_f93260a75243);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const InputDiscarded_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f36c367_7b18_417c_97e3_9d58ddc944ab);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const InputReachedOtherElement_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed201d8a_4e6c_415e_a874_2460c9b66ba8);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const InputReachedTarget_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93ed549a_0549_40f0_bedb_28e44f7de2a3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Invoke_Invoked_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfd699f0_c915_49dd_b422_dde785c3d24b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Invoke_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd976c2fc_66ea_4a6e_b28f_c24c7546ad37);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsAnnotationPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b5b3238_6d5c_41b6_bcc4_5e807f6551c4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsContentElement_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bda64a8_f5d8_480b_8155_ef2e89adb672);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsControlElement_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95f35085_abcc_4afd_a5f4_dbb46c230fdb);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsCustomNavigationPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f8e80d4_2351_48e0_874a_54aa7313889a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsDataValidForForm_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x445ac684_c3fc_4dd9_acf8_845a579296ba);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsDialog_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d0dfb9b_8436_4501_bbbb_e534a4fb3b3f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsDockPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2600a4c4_2ff8_4c96_ae31_8fe619a13c6c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsDragPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe997a7b7_1d39_4ca7_be0f_277fcf5605cc);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsDropTargetPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0686b62e_8e19_4aaf_873d_384f6d3b92be);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsEnabled_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2109427f_da60_4fed_bf1b_264bdce6eb3a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsExpandCollapsePatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x929d3806_5287_4725_aa16_222afc63d595);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsGridItemPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a43e524_f9a2_4b12_84c8_b48a3efedd34);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsGridPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5622c26c_f0ef_4f3b_97cb_714c0868588b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsInvokePatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e725738_8364_4679_aa6c_f3f41931f750);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsItemContainerPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x624b5ca7_fe40_4957_a019_20c4cf11920f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsKeyboardFocusable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7b8552a_0859_4b37_b9cb_51e72092f29f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsLegacyIAccessiblePatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8ebd0c7_929a_4ee7_8d3a_d3d94413027b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsMultipleViewPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff0a31eb_8e25_469d_8d6e_e771a27c1b90);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsObjectModelPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b21d89b_2841_412f_8ef2_15ca952318ba);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsOffscreen_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03c3d160_db79_42db_a2ef_1c231eede507);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsPassword_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8482eb1_687c_497b_bebc_03be53ec1454);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsPeripheral_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda758276_7ed5_49d4_8e68_ecc9a2d300dd);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsRangeValuePatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfda4244a_eb4d_43ff_b5ad_ed36d373ec4c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsRequiredForForm_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f5f43cf_59fb_4bde_a270_602e5e1141e9);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsScrollItemPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cad1a05_0927_4b76_97e1_0fcdb209b98a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsScrollPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ebb7b4a_828a_4b57_9d22_2fea1632ed0d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsSelectionItemPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8becd62d_0bc3_4109_bee2_8e6715290e68);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsSelectionPattern2Available_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x490806fb_6e89_4a47_8319_d266e511f021);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsSelectionPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf588acbe_c769_4838_9a60_2686dc1188c4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsSpreadsheetItemPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fe79b2a_2f94_43fd_996b_549e316f4acd);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsSpreadsheetPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ff43732_e4b4_4555_97bc_ecdbbc4d1888);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsStructuredMarkupPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0d4c196_2c0b_489c_b165_a405928c6f3d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsStylesPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27f353d3_459c_4b59_a490_50611dacafb5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsSynchronizedInputPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d69cc5_d2bf_4943_876e_b45b62a6cc66);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsTableItemPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb36b40d_8ea4_489b_a013_e60d5951fe34);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsTablePatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb83575f_45c2_4048_9c76_159715a139df);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsTextChildPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x559e65df_30ff_43b5_b5ed_5b283b80c7e9);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsTextEditPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7843425c_8b32_484c_9ab5_e3200571ffda);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsTextPattern2Available_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41cf921d_e3f1_4b22_9c81_e1c3ed331c22);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsTextPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbe2d69d_aff6_4a45_82e2_fc92a82f5917);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsTogglePatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78686d53_fcd0_4b83_9b78_5832ce63bb5b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsTransformPattern2Available_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25980b4b_be04_4710_ab4a_fda31dbd2895);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsTransformPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7f78804_d68b_4077_a5c6_7a5ea1ac31c5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsValuePatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b5020a7_2119_473b_be37_5ceb98bbfb22);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsVirtualizedItemPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x302cb151_2ac8_45d6_977b_d2b3a5a53f20);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const IsWindowPatternAvailable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7a57bb1_5888_4155_98dc_b422fd57f2bc);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ItemContainer_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d13da0f_8b9a_4a99_85fa_c5c9a69f1ed4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ItemStatus_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51de0321_3973_43e7_8913_0b08e813c37f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ItemType_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdda434d_6222_413b_a68a_325dd1d40f39);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LIBID_Accessibility: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ea4dbf0_3c3b_11cf_810c_00aa00389b71);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LabeledBy_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5b8924b_fc8a_4a35_8031_cf78ac43e55e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LandmarkType_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x454045f2_6f61_49f7_a4f8_b5f0cf82da1e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LayoutInvalidated_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed7d6544_a6bd_4595_9bae_3d28946cc715);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_ChildId_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a191b5d_9ef2_4787_a459_dcde885dd4e8);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_DefaultAction_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b331729_eaad_4502_b85f_92615622913c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_Description_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46448418_7d70_4ea9_9d27_b7e775cf2ad7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_Help_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94402352_161c_4b77_a98d_a872cc33947a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_KeyboardShortcut_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f6909ac_00b8_4259_a41c_966266d43a8a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_Name_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaeb063d_40ae_4869_aa5a_1b8e5d666739);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54cc0a9f_3395_48af_ba8d_73f85690f3e0);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_Role_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6856e59f_cbaf_4e31_93e8_bcbf6f7e491c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_Selection_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8aa8b1e0_0891_40cc_8b06_90d7d4166219);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_State_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf985854_2281_4340_ab9c_c60e2c5803f6);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LegacyIAccessible_Value_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5c5b0b6_8217_4a77_97a5_190a85ed0156);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Level_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x242ac529_cd36_400f_aad9_7876ef3af627);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ListItem_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b3717f2_44d1_4a58_98a8_f12a9b8f78e2);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const List_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b149ee1_7cca_4cfc_9af1_cac7bddd3031);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LiveRegionChanged_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x102d5e90_e6a9_41b6_b1c5_a9b1929d9510);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LiveSetting_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12bcd8e_2a8e_4950_8ae7_3625111d58eb);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LocalizedControlType_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8763404f_a1bd_452a_89c4_3f01d3833806);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const LocalizedLandmarkType_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ac81980_eafb_4fb2_bf91_f485bef5e8e1);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const MSAA_MENU_SIG: i32 = -1441927155i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const MenuBar_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc384250_0e7b_4ae8_95ae_a08f261b52ee);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const MenuClosed_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cf1266e_1582_4041_acd7_88a35a965297);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const MenuItem_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf45225d3_d0a0_49d8_9834_9a000d2aeddc);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const MenuModeEnd_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ecd4c9f_80dd_47b8_8267_5aec06bb2cff);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const MenuModeStart_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18d7c631_166a_4ac9_ae3b_ef4b5420e681);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const MenuOpened_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebe2e945_66ca_4ed1_9ff8_2ad7df0a1b08);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Menu_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e9b1440_0ea8_41fd_b374_c1ea6f503cd1);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const MultipleView_CurrentView_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a81a67a_b94f_4875_918b_65c8d2f998e5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const MultipleView_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x547a6ae4_113f_47c4_850f_db4dfa466b1d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const MultipleView_SupportedViews_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d5db9fd_ce3c_4ae7_b788_400a3c645547);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NAVDIR_DOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NAVDIR_FIRSTCHILD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NAVDIR_LASTCHILD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NAVDIR_LEFT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NAVDIR_MAX: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NAVDIR_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NAVDIR_NEXT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NAVDIR_PREVIOUS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NAVDIR_RIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NAVDIR_UP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Name_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3a6921b_4a99_44f1_bca6_61187052c431);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NewNativeWindowHandle_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5196b33b_380a_4982_95e1_91f3ef60e024);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Notification_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72c5a2f7_9788_480f_b8eb_4dee00f6186f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ObjectModel_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e04acfe_08fc_47ec_96bc_353fa3b34aa7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OptimizeForVisualContent_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a852250_c75a_4e5d_b858_e381b0f78861);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Orientation_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa01eee62_3884_4415_887e_678ec21e39ba);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OutlineColor_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc395d6c0_4b55_4762_a073_fd303a634f52);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OutlineThickness_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13e67cc7_dac2_4888_bdd3_375c62fa9618);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_DEFAULTACTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x180c072b_c27f_43c7_9922_f63562a4632b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_DESCRIPTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d48dfe4_bd3f_491f_a648_492d6f20c588);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_DESCRIPTIONMAP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ff1435f_8a14_477b_b226_a0abe279975d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_DODEFAULTACTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ba09523_2e3b_49a6_a059_59682a3c48fd);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_FOCUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6eb335df_1c29_4127_b12c_dee9fd157f2b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_HELP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc831e11f_44db_4a99_9768_cb8f978b7231);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_HELPTOPIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x787d1379_8ede_440b_8aec_11f7bf9030b3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_KEYBOARDSHORTCUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d9bceee_7d1e_4979_9382_5180f4172c34);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_NAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x608d3df8_8128_4aa7_a428_f55e49267291);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_NAV_DOWN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x031670ed_3cdf_48d2_9613_138f2dd8a668);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_NAV_FIRSTCHILD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfd02558_557b_4c67_84f9_2a09fce40749);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_NAV_LASTCHILD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x302ecaa5_48d5_4f8d_b671_1a8d20a77832);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_NAV_LEFT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x228086cb_82f1_4a39_8705_dcdc0fff92f5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_NAV_NEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cdc5455_8cd9_4c92_a371_3939a2fe3eee);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_NAV_PREV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x776d3891_c73b_4480_b3f6_076a16a15af6);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_NAV_RIGHT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd211d9f_e1cb_4fe5_a77c_920b884d095b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_NAV_UP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x016e1a2b_1a4e_4767_8612_3386f66935ec);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_PARENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x474c22b6_ffc2_467a_b1b5_e958b4657330);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_ROLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb905ff2_7bd1_4c05_b3c8_e6c241364d70);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_ROLEMAP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf79acda2_140d_4fe6_8914_208476328269);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_SELECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb99d073c_d731_405b_9061_d95e8f842984);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_STATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8d4d5b0_0a21_42d0_a5c0_514e984f457b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_STATEMAP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43946c5e_0ac0_4042_b525_07bbdbe17fa7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_VALUE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x123fe443_211a_4615_9527_c45a7e93717a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PROPID_ACC_VALUEMAP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda1c3d79_fc5c_420e_b399_9d1533549e75);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Pane_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c2b3f5b_9182_42a3_8dec_8c04c1ee634d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PositionInSet_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33d1dc54_641e_4d76_a6b1_13f341c1f896);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProcessId_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40499998_9c31_4245_a403_87320e59eaf6);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProgressBar_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x228c9f86_c36c_47bb_9fb6_a5834bfc53a4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderDescription_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdca5708a_c16b_4cd9_b889_beb16a804904);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_ALERT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_ANIMATION: u32 = 54u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_APPLICATION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_BORDER: u32 = 19u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_BUTTONDROPDOWN: u32 = 56u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_BUTTONDROPDOWNGRID: u32 = 58u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_BUTTONMENU: u32 = 57u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_CARET: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_CELL: u32 = 29u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_CHARACTER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_CHART: u32 = 17u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_CHECKBUTTON: u32 = 44u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_CLIENT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_CLOCK: u32 = 61u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_COLUMN: u32 = 27u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_COLUMNHEADER: u32 = 25u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_COMBOBOX: u32 = 46u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_CURSOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_DIAGRAM: u32 = 53u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_DIAL: u32 = 49u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_DIALOG: u32 = 18u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_DOCUMENT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_DROPLIST: u32 = 47u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_EQUATION: u32 = 55u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_GRAPHIC: u32 = 40u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_GRIP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_GROUPING: u32 = 20u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_HELPBALLOON: u32 = 31u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_HOTKEYFIELD: u32 = 50u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_INDICATOR: u32 = 39u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_IPADDRESS: u32 = 63u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_LINK: u32 = 30u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_LIST: u32 = 33u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_LISTITEM: u32 = 34u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_MENUBAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_MENUITEM: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_MENUPOPUP: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_OUTLINE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_OUTLINEBUTTON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_OUTLINEITEM: u32 = 36u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_PAGETAB: u32 = 37u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_PAGETABLIST: u32 = 60u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_PANE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_PROGRESSBAR: u32 = 48u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_PROPERTYPAGE: u32 = 38u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_PUSHBUTTON: u32 = 43u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_RADIOBUTTON: u32 = 45u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_ROW: u32 = 28u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_ROWHEADER: u32 = 26u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_SCROLLBAR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_SEPARATOR: u32 = 21u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_SLIDER: u32 = 51u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_SOUND: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_SPINBUTTON: u32 = 52u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_SPLITBUTTON: u32 = 62u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_STATICTEXT: u32 = 41u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_STATUSBAR: u32 = 23u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_TABLE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_TEXT: u32 = 42u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_TITLEBAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_TOOLBAR: u32 = 22u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_TOOLTIP: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_WHITESPACE: u32 = 59u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ROLE_SYSTEM_WINDOW: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RadioButton_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bdb49db_fe2c_4483_b3e1_e57f219440c6);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RangeValue_IsReadOnly_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25fa1055_debf_4373_a79e_1f1a1908d3c4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RangeValue_LargeChange_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1f96325_3a3d_4b44_8e1f_4a46d9844019);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RangeValue_Maximum_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19319914_f979_4b35_a1a6_d37e05433473);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RangeValue_Minimum_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78cbd3b2_684d_4860_af93_d1f95cb022fd);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RangeValue_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18b00d87_b1c9_476a_bfbd_5f0bdb926f63);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RangeValue_SmallChange_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c2c457_3941_4107_9975_139760f7c072);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RangeValue_Value_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x131f5d98_c50c_489d_abe5_ae220898c5f7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Rotation_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x767cdc7d_aec0_4110_ad32_30edd403492e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RuntimeId_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa39eebfa_7fba_4c89_b4d4_b99e2de7d160);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SELFLAG_ADDSELECTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SELFLAG_EXTENDSELECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SELFLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SELFLAG_REMOVESELECTION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SELFLAG_TAKEFOCUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SELFLAG_TAKESELECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SELFLAG_VALID: u32 = 31u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SID_ControlElementProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4791d68_e254_4ba3_9a53_26a5c5497946);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SID_IsUIAutomationObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96fdb85_7204_4724_842b_c7059dedb9d0);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const STATE_SYSTEM_HASPOPUP: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const STATE_SYSTEM_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ScrollBar_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaf34b36_5065_4946_b22f_92595fc0751a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ScrollItem_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4591d005_a803_4d5c_b4d5_8d2800f906a7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Scroll_HorizontalScrollPercent_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7c13c0e_eb21_47ff_acc4_b5a3350f5191);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Scroll_HorizontalViewSize_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70c2e5d4_fcb0_4713_a9aa_af92ff79e4cd);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Scroll_HorizontallyScrollable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b925147_28cd_49ae_bd63_f44118d2e719);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Scroll_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x895fa4b4_759d_4c50_8e15_03460672003c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Scroll_VerticalScrollPercent_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c8d7099_b2a8_4948_bff7_3cf9058bfefb);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Scroll_VerticalViewSize_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde6a2e22_d8c7_40c5_83ba_e5f681d53108);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Scroll_VerticallyScrollable_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89164798_0068_4315_b89a_1e7cfbbc3dfc);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Selection2_CurrentSelectedItem_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34257c26_83b5_41a6_939c_ae841c136236);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Selection2_FirstSelectedItem_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc24ea67_369c_4e55_9ff7_38da69540c29);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Selection2_ItemCount_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb49eb9f_456d_4048_b591_9c2026b84636);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Selection2_LastSelectedItem_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf7bda90_2d83_49f8_860c_9ce394cf89b4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SelectionItem_ElementAddedToSelectionEvent_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c822dd1_c407_4dba_91dd_79d4aed0aec6);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SelectionItem_ElementRemovedFromSelectionEvent_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x097fa8a9_7079_41af_8b9c_0934d8305e5c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SelectionItem_ElementSelectedEvent_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9c7dbfb_4ebe_4532_aaf4_008cf647233c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SelectionItem_IsSelected_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf122835f_cd5f_43df_b79d_4b849e9e6020);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SelectionItem_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bc64eeb_87c7_4b28_94bb_4d9fa437b6ef);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SelectionItem_SelectionContainer_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4365b6e_9c1e_4b63_8b53_c2421dd1e8fb);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Selection_CanSelectMultiple_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d73da5_c883_4500_883d_8fcf8daf6cbe);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Selection_InvalidatedEvent_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcac14904_16b4_4b53_8e47_4cb1df267bb7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Selection_IsSelectionRequired_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1ae4422_63fe_44e7_a5a5_a738c829b19a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Selection_Pattern2_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfba25cab_ab98_49f7_a7dc_fe539dc15be7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Selection_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66e3b7e8_d821_4d25_8761_435d2c8b253f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Selection_Selection_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa6dc2a2_0e2b_4d38_96d5_34e470b81853);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SemanticZoom_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fd34a43_061e_42c8_b589_9dccf74bc43a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Separator_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8767eba3_2a63_4ab0_ac8d_aa50e23de978);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SizeOfSet_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1600d33c_3b9f_4369_9431_aa293f344cf1);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Size_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b5f761d_f885_4404_973f_9b1d98e36d8f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Slider_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb033c24b_3b35_4cea_b609_763682fa660b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Spinner_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60cc4b38_3cb1_4161_b442_c6b726c17825);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SplitButton_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7011f01f_4ace_4901_b461_920a6f1ca650);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SpreadsheetItem_AnnotationObjects_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3194c38_c9bc_4604_9396_ae3f9f457f7b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SpreadsheetItem_AnnotationTypes_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc70c51d0_d602_4b45_afbc_b4712b96d72b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SpreadsheetItem_Formula_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe602e47d_1b47_4bea_87cf_3b0b0b5c15b6);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SpreadsheetItem_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32cf83ff_f1a8_4a8c_8658_d47ba74e20ba);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Spreadsheet_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a5b24c9_9d1e_4b85_9e44_c02e3169b10b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StatusBar_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45e7d1b_5873_475f_95a4_0433e1f1b00a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructureChanged_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59977961_3edd_4b11_b13b_676b2a2a6ca9);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructuredMarkup_CompositionComplete_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc48a3c17_677a_4047_a68d_fc1257528aef);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructuredMarkup_Deleted_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9d0a020_e1c1_4ecf_b9aa_52efde7e41e1);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructuredMarkup_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabbd0878_8665_4f5c_94fc_36e7d8bb706b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructuredMarkup_SelectionChanged_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7c815f7_ff9f_41c7_a3a7_ab6cbfdb4903);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_BulletedList_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5963ed64_6426_4632_8caf_a32ad402d91a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Custom_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef2edd3e_a999_4b7c_a378_09bbd52a3516);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Emphasis_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca6e7dbe_355e_4820_95a0_925f041d3470);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading1_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f7e8f69_6866_4621_930c_9a5d0ca5961c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading2_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbaa9b241_5c69_469d_85ad_474737b52b14);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading3_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf8be9d2_d8b8_4ec5_8c52_9cfb0d035970);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading4_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8436ffc0_9578_45fc_83a4_ff40053315dd);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading5_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x909f424d_0dbf_406e_97bb_4e773d9798f7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading6_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89d23459_5d5b_4824_a420_11d3ed82e40f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading7_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3790473_e9ae_422d_b8e3_3b675c6181a4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading8_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bc14145_a40c_4881_84ae_f2235685380c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading9_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc70d9133_bb2a_43d3_8ac6_33657884b0f0);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Normal_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd14d429_e45e_4475_a1c5_7f9e6be96eba);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_NumberedList_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e96dbd5_64c3_43d0_b1ee_b53b06e3eddf);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Quote_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d1c21ea_8195_4f6c_87ea_5dabece64c1d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Subtitle_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5d9fc17_5d6f_4420_b439_7cb19ad434e2);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Title_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15d8201a_ffcf_481f_b0a1_30b63be98f07);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Styles_ExtendedProperties_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf451cda0_ba0a_4681_b0b0_0dbdb53e58f3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Styles_FillColor_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63eff97a_a1c5_4b1d_84eb_b765f2edd632);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Styles_FillPatternColor_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x939a59fe_8fbd_4e75_a271_ac4595195163);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Styles_FillPatternStyle_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81cf651f_482b_4451_a30a_e1545e554fb8);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Styles_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ae62655_da72_4d60_a153_e5aa6988e3bf);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Styles_Shape_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc71a23f8_778c_400d_8458_3b543e526984);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Styles_StyleId_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda82852f_3817_4233_82af_02279e72cc77);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Styles_StyleName_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c12b035_05d1_4f55_9e8e_1489f3ff550d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SynchronizedInput_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05c288a6_c47b_488b_b653_33977a551b8b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SystemAlert_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd271545d_7a3a_47a7_8474_81d29a2451c9);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TabItem_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c6a634f_921b_4e6e_b26e_08fcb0798f4c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Tab_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38cd1f2d_337a_4bd2_a5e3_adb469e30bd3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TableItem_ColumnHeaderItems_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x967a56a3_74b6_431e_8de6_99c411031c58);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TableItem_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf1343bd_1888_4a29_a50c_b92e6de37f6f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TableItem_RowHeaderItems_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3f853a0_0574_4cd8_bcd7_ed5923572d97);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Table_ColumnHeaders_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaff1d72b_968d_42b1_b459_150b299da664);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Table_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x773bfa0e_5bc4_4deb_921b_de7b3206229e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Table_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc415218e_a028_461e_aa92_8f925cf79351);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Table_RowHeaders_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9e35b87_6eb8_4562_aac6_a8a9075236a8);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Table_RowOrColumnMajor_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83be75c3_29fe_4a30_85e1_2a6277fd106e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextChild_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7533cab7_3bfe_41ef_9e85_e2638cbe169e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextEdit_ConversionTargetChanged_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3388c183_ed4f_4c8b_9baa_364d51d8847f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextEdit_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69f3ff89_5af9_4c75_9340_f2de292e4591);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextEdit_TextChanged_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x120b0308_ec22_4eb8_9c98_9867cda1b165);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_AfterParagraphSpacing_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x588cbb38_e62f_497c_b5d1_ccdf0ee823d8);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_AfterSpacing_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x588cbb38_e62f_497c_b5d1_ccdf0ee823d8);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_AnimationStyle_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x628209f0_7c9a_4d57_be64_1f1836571ff5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_AnnotationObjects_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff41cf68_e7ab_40b9_8c72_72a8ed94017d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_AnnotationTypes_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad2eb431_ee4e_4be1_a7ba_5559155a73ef);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_BackgroundColor_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdc49a07_583d_4f17_ad27_77fc832a3c0b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_BeforeParagraphSpacing_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe7b0ab1_c822_4a24_85e9_c8f2650fc79c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_BeforeSpacing_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe7b0ab1_c822_4a24_85e9_c8f2650fc79c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_BulletStyle_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1097c90_d5c4_4237_9781_3bec8ba54e48);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_CapStyle_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb059c50_92cc_49a5_ba8f_0aa872bba2f3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_CaretBidiMode_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x929ee7a6_51d3_4715_96dc_b694fa24a168);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_CaretPosition_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb227b131_9889_4752_a91b_733efdc5c5a0);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae9772dc_d331_4f09_be20_7e6dfaf07b0a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_Culture_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2025af9_a42d_4ced_a1fb_c6746315222e);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_FontName_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64e63ba8_f2e5_476e_a477_1734feaaf726);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_FontSize_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc5eeeff_0506_4673_93f2_377e4a8e01f1);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_FontWeight_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fc02359_b316_4f5f_b401_f1ce55741853);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_ForegroundColor_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d1c95d_5e60_471a_96b1_6c1b3b77a436);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_HorizontalTextAlignment_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04ea6161_fba3_477a_952a_bb326d026a5b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_IndentationFirstLine_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x206f9ad5_c1d3_424a_8182_6da9a7f3d632);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_IndentationLeading_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cf66bac_2d45_4a4b_b6c9_f7221d2815b0);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_IndentationTrailing_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97ff6c0f_1ce4_408a_b67b_94d83eb69bf2);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_IsActive_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5a4e533_e1b8_436b_935d_b57aa3f558c4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_IsHidden_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x360182fb_bdd7_47f6_ab69_19e33f8a3344);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_IsItalic_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfce12a56_1336_4a34_9663_1bab47239320);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_IsReadOnly_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa738156b_ca3e_495e_9514_833c440feb11);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_IsSubscript_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0ead858_8f53_413c_873f_1a7d7f5e0de4);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_IsSuperscript_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda706ee4_b3aa_4645_a41f_cd25157dea76);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_LineSpacing_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63ff70ae_d943_4b47_8ab7_a7a033d3214b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_Link_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb38ef51d_9e8d_4e46_9144_56ebe177329b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_MarginBottom_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ee593c4_72b4_4cac_9271_3ed24b0e4d42);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_MarginLeading_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e9242d0_5ed0_4900_8e8a_eecc03835afc);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_MarginTop_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x683d936f_c9b9_4a9a_b3d9_d20d33311e2a);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_MarginTrailing_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf522f98_999d_40af_a5b2_0169d0342002);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_OutlineStyles_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b675b27_db89_46fe_970c_614d523bb97d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_OverlineColor_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83ab383a_fd43_40da_ab3e_ecf8165cbb6d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_OverlineStyle_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a234d66_617e_427f_871d_e1ff1e0c213f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_Pattern2_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x498479a2_5b22_448d_b6e4_647490860698);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8615f05d_7de5_44fd_a679_2ca4b46033a8);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_SayAsInterpretAs_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb38ad6ac_eee1_4b6e_88cc_014cefa93fcb);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_SelectionActiveEnd_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f668cc3_9bbf_416b_b0a2_f89f86f6612c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_StrikethroughColor_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfe15a18_8c41_4c5a_9a0b_04af0e07f487);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_StrikethroughStyle_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72913ef1_da00_4f01_899c_ac5a8577a307);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_StyleId_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14c300de_c32b_449b_ab7c_b0e0789aea5d);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_StyleName_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22c9e091_4d66_45d8_a828_737bab4c98a7);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_Tabs_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e68d00b_92fe_42d8_899a_a784aa4454a1);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_TextChangedEvent_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a342082_f483_48c4_ac11_a84b435e2a84);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_TextFlowDirections_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bdf8739_f420_423e_af77_20a5d973a907);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_TextSelectionChangedEvent_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x918edaa1_71b3_49ae_9741_79beb8d358f3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_UnderlineColor_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfa12c73_fde2_4473_bf64_1036d6aa0f45);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Text_UnderlineStyle_Attribute_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f3b21c0_ede4_44bd_9c36_3853038cbfeb);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Thumb_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x701ca877_e310_4dd6_b644_797e4faea213);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TitleBar_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98aa55bf_3bb0_4b65_836e_2ea30dbc171f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Toggle_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b419760_e2f4_43ff_8c5f_9457c82b56e9);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Toggle_ToggleState_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb23cdc52_22c2_4c6c_9ded_f5c422479ede);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ToolBar_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f06b751_e182_4e98_8893_2284543a7dce);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ToolTipClosed_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x276d71ef_24a9_49b6_8e97_da98b401bbcd);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ToolTipOpened_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f4b97ff_2edc_451d_bca4_95a3188d5b03);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ToolTip_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05ddc6d1_2137_4768_98ea_73f52f7134f3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Tranform_Pattern2_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8afcfd07_a369_44de_988b_2f7ff49fb8a8);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Transform2_CanZoom_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf357e890_a756_4359_9ca6_86702bf8f381);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Transform2_ZoomLevel_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeee29f1a_f4a2_4b5b_ac65_95cf93283387);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Transform2_ZoomMaximum_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42ab6b77_ceb0_4eca_b82a_6cfa5fa1fc08);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Transform2_ZoomMinimum_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x742ccc16_4ad1_4e07_96fe_b122c6e6b22b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Transform_CanMove_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b75824d_208b_4fdf_bccd_f1f4e5741f4f);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Transform_CanResize_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb98dca5_4c1a_41d4_a4f6_ebc128644180);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Transform_CanRotate_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10079b48_3849_476f_ac96_44a95c8440d9);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Transform_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24b46fdb_587e_49f1_9c4a_d8e98b664b7b);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeItem_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62c9feb9_8ffc_4878_a3a4_96b030315c18);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Tree_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7561349c_d241_43f4_9908_b5f091bee611);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_E_ELEMENTNOTAVAILABLE: u32 = 2147746305u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_E_ELEMENTNOTENABLED: u32 = 2147746304u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_E_INVALIDOPERATION: u32 = 2148734217u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_E_NOCLICKABLEPOINT: u32 = 2147746306u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_E_NOTSUPPORTED: u32 = 2147746308u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_E_PROXYASSEMBLYNOTLOADED: u32 = 2147746307u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_E_TIMEOUT: u32 = 2148734213u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IAFP_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IAFP_UNWRAP_BRIDGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_PFIA_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_PFIA_UNWRAP_BRIDGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ScrollPatternNoScroll: f64 = -1f64;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UiaAppendRuntimeId: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UiaRootObjectId: i32 = -25i32;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Value_IsReadOnly_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb090f30_e24c_4799_a705_0d247bc037f8);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Value_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17faad9e_c877_475b_b933_77332779b637);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Value_Value_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe95f5e64_269f_4a85_ba99_4092c3ea2986);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const VirtualizedItem_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf510173e_2e71_45e9_a6e5_62f6ed8289d5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const VisualEffects_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe61a8565_aad9_46d7_9e70_4e8a8420d420);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Window_CanMaximize_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64fff53f_635d_41c1_950c_cb5adfbe28e3);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Window_CanMinimize_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb73b4625_5988_4b97_b4c2_a6fe6e78c8c6);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Window_Control_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe13a7242_f462_4f4d_aec1_53b28d6c3290);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Window_IsModal_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff4e6892_37b9_4fca_8532_ffe674ecfeed);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Window_IsTopmost_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef7d85d3_0937_4962_9241_b62345f24041);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Window_Pattern_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27901735_c760_4994_ad11_5919e606b110);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Window_WindowClosed_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedf141f8_fa67_4e22_bbf7_944e05735ee2);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Window_WindowInteractionState_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fed26a4_0455_4fa2_b21c_c4da2db1ff9c);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Window_WindowOpened_Event_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3e81d06_de45_4f2f_9633_de9e02fb65af);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Window_WindowVisualState_Property_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ab7905f_e860_453e_a30a_f6431e5daad5);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACC_UTILITY_STATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ANRUS_ON_SCREEN_KEYBOARD_ACTIVE: ACC_UTILITY_STATE_FLAGS = ACC_UTILITY_STATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ANRUS_TOUCH_MODIFICATION_ACTIVE: ACC_UTILITY_STATE_FLAGS = ACC_UTILITY_STATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ANRUS_PRIORITY_AUDIO_ACTIVE: ACC_UTILITY_STATE_FLAGS = ACC_UTILITY_STATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ANRUS_PRIORITY_AUDIO_ACTIVE_NODUCK: ACC_UTILITY_STATE_FLAGS = ACC_UTILITY_STATE_FLAGS(8u32);
impl ::core::marker::Copy for ACC_UTILITY_STATE_FLAGS {}
impl ::core::clone::Clone for ACC_UTILITY_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACC_UTILITY_STATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActiveEnd(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ActiveEnd_None: ActiveEnd = ActiveEnd(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ActiveEnd_Start: ActiveEnd = ActiveEnd(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ActiveEnd_End: ActiveEnd = ActiveEnd(2i32);
impl ::core::marker::Copy for ActiveEnd {}
impl ::core::clone::Clone for ActiveEnd {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ActiveEnd {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AnimationStyle(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnimationStyle_None: AnimationStyle = AnimationStyle(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnimationStyle_LasVegasLights: AnimationStyle = AnimationStyle(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnimationStyle_BlinkingBackground: AnimationStyle = AnimationStyle(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnimationStyle_SparkleText: AnimationStyle = AnimationStyle(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnimationStyle_MarchingBlackAnts: AnimationStyle = AnimationStyle(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnimationStyle_MarchingRedAnts: AnimationStyle = AnimationStyle(5i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnimationStyle_Shimmer: AnimationStyle = AnimationStyle(6i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnimationStyle_Other: AnimationStyle = AnimationStyle(-1i32);
impl ::core::marker::Copy for AnimationStyle {}
impl ::core::clone::Clone for AnimationStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AnimationStyle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AnnoScope(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ANNO_THIS: AnnoScope = AnnoScope(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ANNO_CONTAINER: AnnoScope = AnnoScope(1i32);
impl ::core::marker::Copy for AnnoScope {}
impl ::core::clone::Clone for AnnoScope {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AnnoScope {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AsyncContentLoadedState(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AsyncContentLoadedState_Beginning: AsyncContentLoadedState = AsyncContentLoadedState(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AsyncContentLoadedState_Progress: AsyncContentLoadedState = AsyncContentLoadedState(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AsyncContentLoadedState_Completed: AsyncContentLoadedState = AsyncContentLoadedState(2i32);
impl ::core::marker::Copy for AsyncContentLoadedState {}
impl ::core::clone::Clone for AsyncContentLoadedState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AsyncContentLoadedState {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationElementMode(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationElementMode_None: AutomationElementMode = AutomationElementMode(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationElementMode_Full: AutomationElementMode = AutomationElementMode(1i32);
impl ::core::marker::Copy for AutomationElementMode {}
impl ::core::clone::Clone for AutomationElementMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationElementMode {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationIdentifierType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationIdentifierType_Property: AutomationIdentifierType = AutomationIdentifierType(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationIdentifierType_Pattern: AutomationIdentifierType = AutomationIdentifierType(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationIdentifierType_Event: AutomationIdentifierType = AutomationIdentifierType(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationIdentifierType_ControlType: AutomationIdentifierType = AutomationIdentifierType(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationIdentifierType_TextAttribute: AutomationIdentifierType = AutomationIdentifierType(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationIdentifierType_LandmarkType: AutomationIdentifierType = AutomationIdentifierType(5i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationIdentifierType_Annotation: AutomationIdentifierType = AutomationIdentifierType(6i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationIdentifierType_Changes: AutomationIdentifierType = AutomationIdentifierType(7i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AutomationIdentifierType_Style: AutomationIdentifierType = AutomationIdentifierType(8i32);
impl ::core::marker::Copy for AutomationIdentifierType {}
impl ::core::clone::Clone for AutomationIdentifierType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AutomationIdentifierType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BulletStyle(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const BulletStyle_None: BulletStyle = BulletStyle(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const BulletStyle_HollowRoundBullet: BulletStyle = BulletStyle(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const BulletStyle_FilledRoundBullet: BulletStyle = BulletStyle(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const BulletStyle_HollowSquareBullet: BulletStyle = BulletStyle(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const BulletStyle_FilledSquareBullet: BulletStyle = BulletStyle(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const BulletStyle_DashBullet: BulletStyle = BulletStyle(5i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const BulletStyle_Other: BulletStyle = BulletStyle(-1i32);
impl ::core::marker::Copy for BulletStyle {}
impl ::core::clone::Clone for BulletStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BulletStyle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CapStyle(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CapStyle_None: CapStyle = CapStyle(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CapStyle_SmallCap: CapStyle = CapStyle(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CapStyle_AllCap: CapStyle = CapStyle(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CapStyle_AllPetiteCaps: CapStyle = CapStyle(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CapStyle_PetiteCaps: CapStyle = CapStyle(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CapStyle_Unicase: CapStyle = CapStyle(5i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CapStyle_Titling: CapStyle = CapStyle(6i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CapStyle_Other: CapStyle = CapStyle(-1i32);
impl ::core::marker::Copy for CapStyle {}
impl ::core::clone::Clone for CapStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CapStyle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CaretBidiMode(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CaretBidiMode_LTR: CaretBidiMode = CaretBidiMode(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CaretBidiMode_RTL: CaretBidiMode = CaretBidiMode(1i32);
impl ::core::marker::Copy for CaretBidiMode {}
impl ::core::clone::Clone for CaretBidiMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CaretBidiMode {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CaretPosition(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CaretPosition_Unknown: CaretPosition = CaretPosition(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CaretPosition_EndOfLine: CaretPosition = CaretPosition(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CaretPosition_BeginningOfLine: CaretPosition = CaretPosition(2i32);
impl ::core::marker::Copy for CaretPosition {}
impl ::core::clone::Clone for CaretPosition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CaretPosition {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoalesceEventsOptions(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CoalesceEventsOptions_Disabled: CoalesceEventsOptions = CoalesceEventsOptions(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const CoalesceEventsOptions_Enabled: CoalesceEventsOptions = CoalesceEventsOptions(1i32);
impl ::core::marker::Copy for CoalesceEventsOptions {}
impl ::core::clone::Clone for CoalesceEventsOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CoalesceEventsOptions {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConditionType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ConditionType_True: ConditionType = ConditionType(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ConditionType_False: ConditionType = ConditionType(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ConditionType_Property: ConditionType = ConditionType(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ConditionType_And: ConditionType = ConditionType(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ConditionType_Or: ConditionType = ConditionType(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ConditionType_Not: ConditionType = ConditionType(5i32);
impl ::core::marker::Copy for ConditionType {}
impl ::core::clone::Clone for ConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ConditionType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConnectionRecoveryBehaviorOptions(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ConnectionRecoveryBehaviorOptions_Disabled: ConnectionRecoveryBehaviorOptions = ConnectionRecoveryBehaviorOptions(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ConnectionRecoveryBehaviorOptions_Enabled: ConnectionRecoveryBehaviorOptions = ConnectionRecoveryBehaviorOptions(1i32);
impl ::core::marker::Copy for ConnectionRecoveryBehaviorOptions {}
impl ::core::clone::Clone for ConnectionRecoveryBehaviorOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ConnectionRecoveryBehaviorOptions {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DockPosition(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DockPosition_Top: DockPosition = DockPosition(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DockPosition_Left: DockPosition = DockPosition(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DockPosition_Bottom: DockPosition = DockPosition(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DockPosition_Right: DockPosition = DockPosition(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DockPosition_Fill: DockPosition = DockPosition(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const DockPosition_None: DockPosition = DockPosition(5i32);
impl ::core::marker::Copy for DockPosition {}
impl ::core::clone::Clone for DockPosition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DockPosition {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EventArgsType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const EventArgsType_Simple: EventArgsType = EventArgsType(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const EventArgsType_PropertyChanged: EventArgsType = EventArgsType(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const EventArgsType_StructureChanged: EventArgsType = EventArgsType(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const EventArgsType_AsyncContentLoaded: EventArgsType = EventArgsType(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const EventArgsType_WindowClosed: EventArgsType = EventArgsType(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const EventArgsType_TextEditTextChanged: EventArgsType = EventArgsType(5i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const EventArgsType_Changes: EventArgsType = EventArgsType(6i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const EventArgsType_Notification: EventArgsType = EventArgsType(7i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const EventArgsType_ActiveTextPositionChanged: EventArgsType = EventArgsType(8i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const EventArgsType_StructuredMarkup: EventArgsType = EventArgsType(9i32);
impl ::core::marker::Copy for EventArgsType {}
impl ::core::clone::Clone for EventArgsType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EventArgsType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ExpandCollapseState(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ExpandCollapseState_Collapsed: ExpandCollapseState = ExpandCollapseState(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ExpandCollapseState_Expanded: ExpandCollapseState = ExpandCollapseState(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ExpandCollapseState_PartiallyExpanded: ExpandCollapseState = ExpandCollapseState(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ExpandCollapseState_LeafNode: ExpandCollapseState = ExpandCollapseState(3i32);
impl ::core::marker::Copy for ExpandCollapseState {}
impl ::core::clone::Clone for ExpandCollapseState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ExpandCollapseState {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FillType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FillType_None: FillType = FillType(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FillType_Color: FillType = FillType(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FillType_Gradient: FillType = FillType(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FillType_Picture: FillType = FillType(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FillType_Pattern: FillType = FillType(4i32);
impl ::core::marker::Copy for FillType {}
impl ::core::clone::Clone for FillType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FillType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FlowDirections(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FlowDirections_Default: FlowDirections = FlowDirections(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FlowDirections_RightToLeft: FlowDirections = FlowDirections(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FlowDirections_BottomToTop: FlowDirections = FlowDirections(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const FlowDirections_Vertical: FlowDirections = FlowDirections(4i32);
impl ::core::marker::Copy for FlowDirections {}
impl ::core::clone::Clone for FlowDirections {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FlowDirections {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HIGHCONTRASTW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HCF_HIGHCONTRASTON: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HCF_AVAILABLE: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HCF_HOTKEYACTIVE: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HCF_CONFIRMHOTKEY: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HCF_HOTKEYSOUND: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HCF_INDICATOR: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HCF_HOTKEYAVAILABLE: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HCF_OPTION_NOTHEMECHANGE: HIGHCONTRASTW_FLAGS = HIGHCONTRASTW_FLAGS(4096u32);
impl ::core::marker::Copy for HIGHCONTRASTW_FLAGS {}
impl ::core::clone::Clone for HIGHCONTRASTW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HIGHCONTRASTW_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HorizontalTextAlignment(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HorizontalTextAlignment_Left: HorizontalTextAlignment = HorizontalTextAlignment(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HorizontalTextAlignment_Centered: HorizontalTextAlignment = HorizontalTextAlignment(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HorizontalTextAlignment_Right: HorizontalTextAlignment = HorizontalTextAlignment(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HorizontalTextAlignment_Justified: HorizontalTextAlignment = HorizontalTextAlignment(3i32);
impl ::core::marker::Copy for HorizontalTextAlignment {}
impl ::core::clone::Clone for HorizontalTextAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HorizontalTextAlignment {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LiveSetting(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Off: LiveSetting = LiveSetting(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Polite: LiveSetting = LiveSetting(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const Assertive: LiveSetting = LiveSetting(2i32);
impl ::core::marker::Copy for LiveSetting {}
impl ::core::clone::Clone for LiveSetting {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LiveSetting {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NavigateDirection(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NavigateDirection_Parent: NavigateDirection = NavigateDirection(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NavigateDirection_NextSibling: NavigateDirection = NavigateDirection(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NavigateDirection_PreviousSibling: NavigateDirection = NavigateDirection(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NavigateDirection_FirstChild: NavigateDirection = NavigateDirection(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NavigateDirection_LastChild: NavigateDirection = NavigateDirection(4i32);
impl ::core::marker::Copy for NavigateDirection {}
impl ::core::clone::Clone for NavigateDirection {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NavigateDirection {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NormalizeState(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NormalizeState_None: NormalizeState = NormalizeState(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NormalizeState_View: NormalizeState = NormalizeState(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NormalizeState_Custom: NormalizeState = NormalizeState(2i32);
impl ::core::marker::Copy for NormalizeState {}
impl ::core::clone::Clone for NormalizeState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NormalizeState {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NotificationKind(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NotificationKind_ItemAdded: NotificationKind = NotificationKind(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NotificationKind_ItemRemoved: NotificationKind = NotificationKind(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NotificationKind_ActionCompleted: NotificationKind = NotificationKind(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NotificationKind_ActionAborted: NotificationKind = NotificationKind(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NotificationKind_Other: NotificationKind = NotificationKind(4i32);
impl ::core::marker::Copy for NotificationKind {}
impl ::core::clone::Clone for NotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NotificationKind {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NotificationProcessing(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NotificationProcessing_ImportantAll: NotificationProcessing = NotificationProcessing(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NotificationProcessing_ImportantMostRecent: NotificationProcessing = NotificationProcessing(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NotificationProcessing_All: NotificationProcessing = NotificationProcessing(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NotificationProcessing_MostRecent: NotificationProcessing = NotificationProcessing(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const NotificationProcessing_CurrentThenMostRecent: NotificationProcessing = NotificationProcessing(4i32);
impl ::core::marker::Copy for NotificationProcessing {}
impl ::core::clone::Clone for NotificationProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NotificationProcessing {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OrientationType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OrientationType_None: OrientationType = OrientationType(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OrientationType_Horizontal: OrientationType = OrientationType(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OrientationType_Vertical: OrientationType = OrientationType(2i32);
impl ::core::marker::Copy for OrientationType {}
impl ::core::clone::Clone for OrientationType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OrientationType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OutlineStyles(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OutlineStyles_None: OutlineStyles = OutlineStyles(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OutlineStyles_Outline: OutlineStyles = OutlineStyles(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OutlineStyles_Shadow: OutlineStyles = OutlineStyles(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OutlineStyles_Engraved: OutlineStyles = OutlineStyles(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const OutlineStyles_Embossed: OutlineStyles = OutlineStyles(8i32);
impl ::core::marker::Copy for OutlineStyles {}
impl ::core::clone::Clone for OutlineStyles {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OutlineStyles {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PropertyConditionFlags(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PropertyConditionFlags_None: PropertyConditionFlags = PropertyConditionFlags(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PropertyConditionFlags_IgnoreCase: PropertyConditionFlags = PropertyConditionFlags(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const PropertyConditionFlags_MatchSubstring: PropertyConditionFlags = PropertyConditionFlags(2i32);
impl ::core::marker::Copy for PropertyConditionFlags {}
impl ::core::clone::Clone for PropertyConditionFlags {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PropertyConditionFlags {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderOptions(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderOptions_ClientSideProvider: ProviderOptions = ProviderOptions(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderOptions_ServerSideProvider: ProviderOptions = ProviderOptions(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderOptions_NonClientAreaProvider: ProviderOptions = ProviderOptions(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderOptions_OverrideProvider: ProviderOptions = ProviderOptions(8i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderOptions_ProviderOwnsSetFocus: ProviderOptions = ProviderOptions(16i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderOptions_UseComThreading: ProviderOptions = ProviderOptions(32i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderOptions_RefuseNonClientSupport: ProviderOptions = ProviderOptions(64i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderOptions_HasNativeIAccessible: ProviderOptions = ProviderOptions(128i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderOptions_UseClientCoordinates: ProviderOptions = ProviderOptions(256i32);
impl ::core::marker::Copy for ProviderOptions {}
impl ::core::clone::Clone for ProviderOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ProviderOptions {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderType_BaseHwnd: ProviderType = ProviderType(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderType_Proxy: ProviderType = ProviderType(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ProviderType_NonClientArea: ProviderType = ProviderType(2i32);
impl ::core::marker::Copy for ProviderType {}
impl ::core::clone::Clone for ProviderType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ProviderType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RowOrColumnMajor(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RowOrColumnMajor_RowMajor: RowOrColumnMajor = RowOrColumnMajor(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RowOrColumnMajor_ColumnMajor: RowOrColumnMajor = RowOrColumnMajor(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const RowOrColumnMajor_Indeterminate: RowOrColumnMajor = RowOrColumnMajor(2i32);
impl ::core::marker::Copy for RowOrColumnMajor {}
impl ::core::clone::Clone for RowOrColumnMajor {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RowOrColumnMajor {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERIALKEYS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SERKF_AVAILABLE: SERIALKEYS_FLAGS = SERIALKEYS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SERKF_INDICATOR: SERIALKEYS_FLAGS = SERIALKEYS_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SERKF_SERIALKEYSON: SERIALKEYS_FLAGS = SERIALKEYS_FLAGS(1u32);
impl ::core::marker::Copy for SERIALKEYS_FLAGS {}
impl ::core::clone::Clone for SERIALKEYS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERIALKEYS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SOUNDSENTRY_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSF_SOUNDSENTRYON: SOUNDSENTRY_FLAGS = SOUNDSENTRY_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSF_AVAILABLE: SOUNDSENTRY_FLAGS = SOUNDSENTRY_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSF_INDICATOR: SOUNDSENTRY_FLAGS = SOUNDSENTRY_FLAGS(4u32);
impl ::core::marker::Copy for SOUNDSENTRY_FLAGS {}
impl ::core::clone::Clone for SOUNDSENTRY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SOUNDSENTRY_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SOUNDSENTRY_TEXT_EFFECT(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSTF_BORDER: SOUNDSENTRY_TEXT_EFFECT = SOUNDSENTRY_TEXT_EFFECT(2u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSTF_CHARS: SOUNDSENTRY_TEXT_EFFECT = SOUNDSENTRY_TEXT_EFFECT(1u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSTF_DISPLAY: SOUNDSENTRY_TEXT_EFFECT = SOUNDSENTRY_TEXT_EFFECT(3u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSTF_NONE: SOUNDSENTRY_TEXT_EFFECT = SOUNDSENTRY_TEXT_EFFECT(0u32);
impl ::core::marker::Copy for SOUNDSENTRY_TEXT_EFFECT {}
impl ::core::clone::Clone for SOUNDSENTRY_TEXT_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SOUNDSENTRY_TEXT_EFFECT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SOUNDSENTRY_WINDOWS_EFFECT(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSWF_CUSTOM: SOUNDSENTRY_WINDOWS_EFFECT = SOUNDSENTRY_WINDOWS_EFFECT(4u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSWF_DISPLAY: SOUNDSENTRY_WINDOWS_EFFECT = SOUNDSENTRY_WINDOWS_EFFECT(3u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSWF_NONE: SOUNDSENTRY_WINDOWS_EFFECT = SOUNDSENTRY_WINDOWS_EFFECT(0u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSWF_TITLE: SOUNDSENTRY_WINDOWS_EFFECT = SOUNDSENTRY_WINDOWS_EFFECT(1u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSWF_WINDOW: SOUNDSENTRY_WINDOWS_EFFECT = SOUNDSENTRY_WINDOWS_EFFECT(2u32);
impl ::core::marker::Copy for SOUNDSENTRY_WINDOWS_EFFECT {}
impl ::core::clone::Clone for SOUNDSENTRY_WINDOWS_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SOUNDSENTRY_WINDOWS_EFFECT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SOUND_SENTRY_GRAPHICS_EFFECT(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSGF_DISPLAY: SOUND_SENTRY_GRAPHICS_EFFECT = SOUND_SENTRY_GRAPHICS_EFFECT(3u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SSGF_NONE: SOUND_SENTRY_GRAPHICS_EFFECT = SOUND_SENTRY_GRAPHICS_EFFECT(0u32);
impl ::core::marker::Copy for SOUND_SENTRY_GRAPHICS_EFFECT {}
impl ::core::clone::Clone for SOUND_SENTRY_GRAPHICS_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SOUND_SENTRY_GRAPHICS_EFFECT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STICKYKEYS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_STICKYKEYSON: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_AVAILABLE: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_HOTKEYACTIVE: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_CONFIRMHOTKEY: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_HOTKEYSOUND: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_INDICATOR: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_AUDIBLEFEEDBACK: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_TRISTATE: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_TWOKEYSOFF: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_LALTLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_LCTLLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_LSHIFTLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_RALTLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_RCTLLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(134217728u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_RSHIFTLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_LWINLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_RWINLATCHED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_LALTLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_LCTLLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_LSHIFTLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_RALTLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_RCTLLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_RSHIFTLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_LWINLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SKF_RWINLOCKED: STICKYKEYS_FLAGS = STICKYKEYS_FLAGS(8388608u32);
impl ::core::marker::Copy for STICKYKEYS_FLAGS {}
impl ::core::clone::Clone for STICKYKEYS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for STICKYKEYS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SayAsInterpretAs(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_None: SayAsInterpretAs = SayAsInterpretAs(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Spell: SayAsInterpretAs = SayAsInterpretAs(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Cardinal: SayAsInterpretAs = SayAsInterpretAs(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Ordinal: SayAsInterpretAs = SayAsInterpretAs(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Number: SayAsInterpretAs = SayAsInterpretAs(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Date: SayAsInterpretAs = SayAsInterpretAs(5i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Time: SayAsInterpretAs = SayAsInterpretAs(6i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Telephone: SayAsInterpretAs = SayAsInterpretAs(7i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Currency: SayAsInterpretAs = SayAsInterpretAs(8i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Net: SayAsInterpretAs = SayAsInterpretAs(9i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Url: SayAsInterpretAs = SayAsInterpretAs(10i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Address: SayAsInterpretAs = SayAsInterpretAs(11i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Alphanumeric: SayAsInterpretAs = SayAsInterpretAs(12i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Name: SayAsInterpretAs = SayAsInterpretAs(13i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Media: SayAsInterpretAs = SayAsInterpretAs(14i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Date_MonthDayYear: SayAsInterpretAs = SayAsInterpretAs(15i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Date_DayMonthYear: SayAsInterpretAs = SayAsInterpretAs(16i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Date_YearMonthDay: SayAsInterpretAs = SayAsInterpretAs(17i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Date_YearMonth: SayAsInterpretAs = SayAsInterpretAs(18i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Date_MonthYear: SayAsInterpretAs = SayAsInterpretAs(19i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Date_DayMonth: SayAsInterpretAs = SayAsInterpretAs(20i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Date_MonthDay: SayAsInterpretAs = SayAsInterpretAs(21i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Date_Year: SayAsInterpretAs = SayAsInterpretAs(22i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Time_HoursMinutesSeconds12: SayAsInterpretAs = SayAsInterpretAs(23i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Time_HoursMinutes12: SayAsInterpretAs = SayAsInterpretAs(24i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Time_HoursMinutesSeconds24: SayAsInterpretAs = SayAsInterpretAs(25i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SayAsInterpretAs_Time_HoursMinutes24: SayAsInterpretAs = SayAsInterpretAs(26i32);
impl ::core::marker::Copy for SayAsInterpretAs {}
impl ::core::clone::Clone for SayAsInterpretAs {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SayAsInterpretAs {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ScrollAmount(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ScrollAmount_LargeDecrement: ScrollAmount = ScrollAmount(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ScrollAmount_SmallDecrement: ScrollAmount = ScrollAmount(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ScrollAmount_NoAmount: ScrollAmount = ScrollAmount(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ScrollAmount_LargeIncrement: ScrollAmount = ScrollAmount(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ScrollAmount_SmallIncrement: ScrollAmount = ScrollAmount(4i32);
impl ::core::marker::Copy for ScrollAmount {}
impl ::core::clone::Clone for ScrollAmount {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ScrollAmount {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StructureChangeType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructureChangeType_ChildAdded: StructureChangeType = StructureChangeType(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructureChangeType_ChildRemoved: StructureChangeType = StructureChangeType(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructureChangeType_ChildrenInvalidated: StructureChangeType = StructureChangeType(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructureChangeType_ChildrenBulkAdded: StructureChangeType = StructureChangeType(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructureChangeType_ChildrenBulkRemoved: StructureChangeType = StructureChangeType(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StructureChangeType_ChildrenReordered: StructureChangeType = StructureChangeType(5i32);
impl ::core::marker::Copy for StructureChangeType {}
impl ::core::clone::Clone for StructureChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for StructureChangeType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SupportedTextSelection(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SupportedTextSelection_None: SupportedTextSelection = SupportedTextSelection(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SupportedTextSelection_Single: SupportedTextSelection = SupportedTextSelection(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SupportedTextSelection_Multiple: SupportedTextSelection = SupportedTextSelection(2i32);
impl ::core::marker::Copy for SupportedTextSelection {}
impl ::core::clone::Clone for SupportedTextSelection {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SupportedTextSelection {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SynchronizedInputType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SynchronizedInputType_KeyUp: SynchronizedInputType = SynchronizedInputType(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SynchronizedInputType_KeyDown: SynchronizedInputType = SynchronizedInputType(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SynchronizedInputType_LeftMouseUp: SynchronizedInputType = SynchronizedInputType(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SynchronizedInputType_LeftMouseDown: SynchronizedInputType = SynchronizedInputType(8i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SynchronizedInputType_RightMouseUp: SynchronizedInputType = SynchronizedInputType(16i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const SynchronizedInputType_RightMouseDown: SynchronizedInputType = SynchronizedInputType(32i32);
impl ::core::marker::Copy for SynchronizedInputType {}
impl ::core::clone::Clone for SynchronizedInputType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SynchronizedInputType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TextDecorationLineStyle(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_None: TextDecorationLineStyle = TextDecorationLineStyle(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_Single: TextDecorationLineStyle = TextDecorationLineStyle(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_WordsOnly: TextDecorationLineStyle = TextDecorationLineStyle(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_Double: TextDecorationLineStyle = TextDecorationLineStyle(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_Dot: TextDecorationLineStyle = TextDecorationLineStyle(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_Dash: TextDecorationLineStyle = TextDecorationLineStyle(5i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_DashDot: TextDecorationLineStyle = TextDecorationLineStyle(6i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_DashDotDot: TextDecorationLineStyle = TextDecorationLineStyle(7i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_Wavy: TextDecorationLineStyle = TextDecorationLineStyle(8i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_ThickSingle: TextDecorationLineStyle = TextDecorationLineStyle(9i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_DoubleWavy: TextDecorationLineStyle = TextDecorationLineStyle(11i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_ThickWavy: TextDecorationLineStyle = TextDecorationLineStyle(12i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_LongDash: TextDecorationLineStyle = TextDecorationLineStyle(13i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_ThickDash: TextDecorationLineStyle = TextDecorationLineStyle(14i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_ThickDashDot: TextDecorationLineStyle = TextDecorationLineStyle(15i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_ThickDashDotDot: TextDecorationLineStyle = TextDecorationLineStyle(16i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_ThickDot: TextDecorationLineStyle = TextDecorationLineStyle(17i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_ThickLongDash: TextDecorationLineStyle = TextDecorationLineStyle(18i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextDecorationLineStyle_Other: TextDecorationLineStyle = TextDecorationLineStyle(-1i32);
impl ::core::marker::Copy for TextDecorationLineStyle {}
impl ::core::clone::Clone for TextDecorationLineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TextDecorationLineStyle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TextEditChangeType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextEditChangeType_None: TextEditChangeType = TextEditChangeType(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextEditChangeType_AutoCorrect: TextEditChangeType = TextEditChangeType(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextEditChangeType_Composition: TextEditChangeType = TextEditChangeType(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextEditChangeType_CompositionFinalized: TextEditChangeType = TextEditChangeType(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextEditChangeType_AutoComplete: TextEditChangeType = TextEditChangeType(4i32);
impl ::core::marker::Copy for TextEditChangeType {}
impl ::core::clone::Clone for TextEditChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TextEditChangeType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TextPatternRangeEndpoint(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextPatternRangeEndpoint_Start: TextPatternRangeEndpoint = TextPatternRangeEndpoint(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextPatternRangeEndpoint_End: TextPatternRangeEndpoint = TextPatternRangeEndpoint(1i32);
impl ::core::marker::Copy for TextPatternRangeEndpoint {}
impl ::core::clone::Clone for TextPatternRangeEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TextPatternRangeEndpoint {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TextUnit(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextUnit_Character: TextUnit = TextUnit(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextUnit_Format: TextUnit = TextUnit(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextUnit_Word: TextUnit = TextUnit(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextUnit_Line: TextUnit = TextUnit(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextUnit_Paragraph: TextUnit = TextUnit(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextUnit_Page: TextUnit = TextUnit(5i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TextUnit_Document: TextUnit = TextUnit(6i32);
impl ::core::marker::Copy for TextUnit {}
impl ::core::clone::Clone for TextUnit {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TextUnit {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ToggleState(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ToggleState_Off: ToggleState = ToggleState(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ToggleState_On: ToggleState = ToggleState(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ToggleState_Indeterminate: ToggleState = ToggleState(2i32);
impl ::core::marker::Copy for ToggleState {}
impl ::core::clone::Clone for ToggleState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ToggleState {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TreeScope(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeScope_None: TreeScope = TreeScope(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeScope_Element: TreeScope = TreeScope(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeScope_Children: TreeScope = TreeScope(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeScope_Descendants: TreeScope = TreeScope(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeScope_Parent: TreeScope = TreeScope(8i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeScope_Ancestors: TreeScope = TreeScope(16i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeScope_Subtree: TreeScope = TreeScope(7i32);
impl ::core::marker::Copy for TreeScope {}
impl ::core::clone::Clone for TreeScope {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TreeScope {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TreeTraversalOptions(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeTraversalOptions_Default: TreeTraversalOptions = TreeTraversalOptions(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeTraversalOptions_PostOrder: TreeTraversalOptions = TreeTraversalOptions(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const TreeTraversalOptions_LastToFirstOrder: TreeTraversalOptions = TreeTraversalOptions(2i32);
impl ::core::marker::Copy for TreeTraversalOptions {}
impl ::core::clone::Clone for TreeTraversalOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TreeTraversalOptions {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_ANNOTATIONTYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_Unknown: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60000u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_SpellingError: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60001u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_GrammarError: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60002u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_Comment: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60003u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_FormulaError: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60004u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_TrackChanges: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60005u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_Header: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60006u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_Footer: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60007u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_Highlighted: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60008u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_Endnote: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60009u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_Footnote: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60010u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_InsertionChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60011u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_DeletionChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60012u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_MoveChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60013u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_FormatChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60014u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_UnsyncedChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60015u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_EditingLockedChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60016u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_ExternalChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60017u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_ConflictingChange: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60018u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_Author: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60019u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_AdvancedProofingIssue: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60020u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_DataValidationError: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60021u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_CircularReferenceError: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60022u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_Mathematics: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60023u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const AnnotationType_Sensitive: UIA_ANNOTATIONTYPE = UIA_ANNOTATIONTYPE(60024u32);
impl ::core::marker::Copy for UIA_ANNOTATIONTYPE {}
impl ::core::clone::Clone for UIA_ANNOTATIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_ANNOTATIONTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_CHANGE_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SummaryChangeId: UIA_CHANGE_ID = UIA_CHANGE_ID(90000u32);
impl ::core::marker::Copy for UIA_CHANGE_ID {}
impl ::core::clone::Clone for UIA_CHANGE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_CHANGE_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_CONTROLTYPE_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ButtonControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50000u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CalendarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50001u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CheckBoxControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50002u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ComboBoxControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50003u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_EditControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50004u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_HyperlinkControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50005u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ImageControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50006u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ListItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50007u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ListControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50008u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MenuControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50009u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MenuBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50010u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MenuItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50011u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ProgressBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50012u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_RadioButtonControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50013u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ScrollBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50014u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SliderControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50015u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SpinnerControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50016u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StatusBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50017u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TabControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50018u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TabItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50019u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TextControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50020u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ToolBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50021u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ToolTipControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50022u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TreeControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50023u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TreeItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50024u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CustomControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50025u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_GroupControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50026u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ThumbControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50027u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DataGridControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50028u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DataItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50029u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DocumentControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50030u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SplitButtonControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50031u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_WindowControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50032u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_PaneControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50033u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_HeaderControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50034u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_HeaderItemControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50035u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TableControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50036u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TitleBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50037u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SeparatorControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50038u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SemanticZoomControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50039u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AppBarControlTypeId: UIA_CONTROLTYPE_ID = UIA_CONTROLTYPE_ID(50040u32);
impl ::core::marker::Copy for UIA_CONTROLTYPE_ID {}
impl ::core::clone::Clone for UIA_CONTROLTYPE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_CONTROLTYPE_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_EVENT_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ToolTipOpenedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20000u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ToolTipClosedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20001u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StructureChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20002u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MenuOpenedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20003u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AutomationPropertyChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20004u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AutomationFocusChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20005u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AsyncContentLoadedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20006u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MenuClosedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20007u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LayoutInvalidatedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20008u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Invoke_InvokedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20009u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionItem_ElementAddedToSelectionEventId: UIA_EVENT_ID = UIA_EVENT_ID(20010u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionItem_ElementRemovedFromSelectionEventId: UIA_EVENT_ID = UIA_EVENT_ID(20011u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionItem_ElementSelectedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20012u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Selection_InvalidatedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20013u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Text_TextSelectionChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20014u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Text_TextChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20015u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Window_WindowOpenedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20016u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Window_WindowClosedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20017u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MenuModeStartEventId: UIA_EVENT_ID = UIA_EVENT_ID(20018u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MenuModeEndEventId: UIA_EVENT_ID = UIA_EVENT_ID(20019u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_InputReachedTargetEventId: UIA_EVENT_ID = UIA_EVENT_ID(20020u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_InputReachedOtherElementEventId: UIA_EVENT_ID = UIA_EVENT_ID(20021u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_InputDiscardedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20022u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SystemAlertEventId: UIA_EVENT_ID = UIA_EVENT_ID(20023u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LiveRegionChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20024u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_HostedFragmentRootsInvalidatedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20025u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Drag_DragStartEventId: UIA_EVENT_ID = UIA_EVENT_ID(20026u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Drag_DragCancelEventId: UIA_EVENT_ID = UIA_EVENT_ID(20027u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Drag_DragCompleteEventId: UIA_EVENT_ID = UIA_EVENT_ID(20028u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DropTarget_DragEnterEventId: UIA_EVENT_ID = UIA_EVENT_ID(20029u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DropTarget_DragLeaveEventId: UIA_EVENT_ID = UIA_EVENT_ID(20030u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DropTarget_DroppedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20031u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TextEdit_TextChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20032u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TextEdit_ConversionTargetChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20033u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ChangesEventId: UIA_EVENT_ID = UIA_EVENT_ID(20034u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_NotificationEventId: UIA_EVENT_ID = UIA_EVENT_ID(20035u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ActiveTextPositionChangedEventId: UIA_EVENT_ID = UIA_EVENT_ID(20036u32);
impl ::core::marker::Copy for UIA_EVENT_ID {}
impl ::core::clone::Clone for UIA_EVENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_EVENT_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_HEADINGLEVEL_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel_None: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80050u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel1: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80051u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel2: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80052u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel3: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80053u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel4: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80054u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel5: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80055u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel6: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80056u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel7: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80057u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel8: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80058u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const HeadingLevel9: UIA_HEADINGLEVEL_ID = UIA_HEADINGLEVEL_ID(80059u32);
impl ::core::marker::Copy for UIA_HEADINGLEVEL_ID {}
impl ::core::clone::Clone for UIA_HEADINGLEVEL_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_HEADINGLEVEL_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_LANDMARKTYPE_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CustomLandmarkTypeId: UIA_LANDMARKTYPE_ID = UIA_LANDMARKTYPE_ID(80000u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_FormLandmarkTypeId: UIA_LANDMARKTYPE_ID = UIA_LANDMARKTYPE_ID(80001u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MainLandmarkTypeId: UIA_LANDMARKTYPE_ID = UIA_LANDMARKTYPE_ID(80002u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_NavigationLandmarkTypeId: UIA_LANDMARKTYPE_ID = UIA_LANDMARKTYPE_ID(80003u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SearchLandmarkTypeId: UIA_LANDMARKTYPE_ID = UIA_LANDMARKTYPE_ID(80004u32);
impl ::core::marker::Copy for UIA_LANDMARKTYPE_ID {}
impl ::core::clone::Clone for UIA_LANDMARKTYPE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_LANDMARKTYPE_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_METADATA_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SayAsInterpretAsMetadataId: UIA_METADATA_ID = UIA_METADATA_ID(100000u32);
impl ::core::marker::Copy for UIA_METADATA_ID {}
impl ::core::clone::Clone for UIA_METADATA_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_METADATA_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_PATTERN_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_InvokePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10000u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10001u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ValuePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10002u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_RangeValuePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10003u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ScrollPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10004u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ExpandCollapsePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10005u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_GridPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10006u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_GridItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10007u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MultipleViewPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10008u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_WindowPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10009u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10010u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DockPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10011u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TablePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10012u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TableItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10013u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TextPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10014u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TogglePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10015u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TransformPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10016u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ScrollItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10017u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessiblePatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10018u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ItemContainerPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10019u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_VirtualizedItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10020u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SynchronizedInputPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10021u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ObjectModelPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10022u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnnotationPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10023u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TextPattern2Id: UIA_PATTERN_ID = UIA_PATTERN_ID(10024u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StylesPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10025u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SpreadsheetPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10026u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SpreadsheetItemPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10027u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TransformPattern2Id: UIA_PATTERN_ID = UIA_PATTERN_ID(10028u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TextChildPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10029u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DragPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10030u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DropTargetPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10031u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TextEditPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10032u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CustomNavigationPatternId: UIA_PATTERN_ID = UIA_PATTERN_ID(10033u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionPattern2Id: UIA_PATTERN_ID = UIA_PATTERN_ID(10034u32);
impl ::core::marker::Copy for UIA_PATTERN_ID {}
impl ::core::clone::Clone for UIA_PATTERN_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_PATTERN_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_PROPERTY_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_RuntimeIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30000u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_BoundingRectanglePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30001u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ProcessIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30002u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ControlTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30003u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LocalizedControlTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30004u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_NamePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30005u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AcceleratorKeyPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30006u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AccessKeyPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30007u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_HasKeyboardFocusPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30008u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsKeyboardFocusablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30009u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsEnabledPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30010u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AutomationIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30011u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ClassNamePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30012u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_HelpTextPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30013u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ClickablePointPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30014u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CulturePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30015u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsControlElementPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30016u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsContentElementPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30017u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LabeledByPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30018u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsPasswordPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30019u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_NativeWindowHandlePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30020u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ItemTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30021u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsOffscreenPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30022u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_OrientationPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30023u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_FrameworkIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30024u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsRequiredForFormPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30025u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ItemStatusPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30026u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsDockPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30027u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsExpandCollapsePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30028u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsGridItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30029u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsGridPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30030u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsInvokePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30031u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsMultipleViewPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30032u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsRangeValuePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30033u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsScrollPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30034u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsScrollItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30035u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsSelectionItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30036u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsSelectionPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30037u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsTablePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30038u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsTableItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30039u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsTextPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30040u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsTogglePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30041u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsTransformPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30042u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsValuePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30043u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsWindowPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30044u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ValueValuePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30045u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ValueIsReadOnlyPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30046u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_RangeValueValuePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30047u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_RangeValueIsReadOnlyPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30048u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_RangeValueMinimumPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30049u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_RangeValueMaximumPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30050u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_RangeValueLargeChangePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30051u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_RangeValueSmallChangePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30052u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ScrollHorizontalScrollPercentPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30053u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ScrollHorizontalViewSizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30054u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ScrollVerticalScrollPercentPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30055u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ScrollVerticalViewSizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30056u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ScrollHorizontallyScrollablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30057u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ScrollVerticallyScrollablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30058u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionSelectionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30059u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionCanSelectMultiplePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30060u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionIsSelectionRequiredPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30061u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_GridRowCountPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30062u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_GridColumnCountPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30063u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_GridItemRowPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30064u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_GridItemColumnPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30065u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_GridItemRowSpanPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30066u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_GridItemColumnSpanPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30067u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_GridItemContainingGridPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30068u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DockDockPositionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30069u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ExpandCollapseExpandCollapseStatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30070u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MultipleViewCurrentViewPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30071u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MultipleViewSupportedViewsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30072u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_WindowCanMaximizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30073u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_WindowCanMinimizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30074u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_WindowWindowVisualStatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30075u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_WindowWindowInteractionStatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30076u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_WindowIsModalPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30077u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_WindowIsTopmostPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30078u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionItemIsSelectedPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30079u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionItemSelectionContainerPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30080u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TableRowHeadersPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30081u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TableColumnHeadersPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30082u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TableRowOrColumnMajorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30083u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TableItemRowHeaderItemsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30084u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TableItemColumnHeaderItemsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30085u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ToggleToggleStatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30086u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TransformCanMovePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30087u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TransformCanResizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30088u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TransformCanRotatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30089u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsLegacyIAccessiblePatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30090u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessibleChildIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30091u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessibleNamePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30092u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessibleValuePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30093u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessibleDescriptionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30094u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessibleRolePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30095u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessibleStatePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30096u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessibleHelpPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30097u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessibleKeyboardShortcutPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30098u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessibleSelectionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30099u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LegacyIAccessibleDefaultActionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30100u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AriaRolePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30101u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AriaPropertiesPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30102u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsDataValidForFormPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30103u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ControllerForPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30104u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DescribedByPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30105u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_FlowsToPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30106u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ProviderDescriptionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30107u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsItemContainerPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30108u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsVirtualizedItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30109u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsSynchronizedInputPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30110u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_OptimizeForVisualContentPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30111u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsObjectModelPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30112u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnnotationAnnotationTypeIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30113u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnnotationAnnotationTypeNamePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30114u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnnotationAuthorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30115u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnnotationDateTimePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30116u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnnotationTargetPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30117u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsAnnotationPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30118u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsTextPattern2AvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30119u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StylesStyleIdPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30120u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StylesStyleNamePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30121u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StylesFillColorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30122u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StylesFillPatternStylePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30123u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StylesShapePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30124u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StylesFillPatternColorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30125u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StylesExtendedPropertiesPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30126u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsStylesPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30127u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsSpreadsheetPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30128u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SpreadsheetItemFormulaPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30129u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SpreadsheetItemAnnotationObjectsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30130u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SpreadsheetItemAnnotationTypesPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30131u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsSpreadsheetItemPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30132u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Transform2CanZoomPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30133u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsTransformPattern2AvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30134u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LiveSettingPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30135u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsTextChildPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30136u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsDragPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30137u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DragIsGrabbedPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30138u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DragDropEffectPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30139u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DragDropEffectsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30140u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsDropTargetPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30141u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DropTargetDropTargetEffectPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30142u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DropTargetDropTargetEffectsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30143u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_DragGrabbedItemsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30144u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Transform2ZoomLevelPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30145u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Transform2ZoomMinimumPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30146u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Transform2ZoomMaximumPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30147u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_FlowsFromPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30148u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsTextEditPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30149u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsPeripheralPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30150u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsCustomNavigationPatternAvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30151u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_PositionInSetPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30152u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SizeOfSetPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30153u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LevelPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30154u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnnotationTypesPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30155u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnnotationObjectsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30156u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LandmarkTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30157u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LocalizedLandmarkTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30158u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_FullDescriptionPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30159u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_FillColorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30160u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_OutlineColorPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30161u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_FillTypePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30162u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_VisualEffectsPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30163u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_OutlineThicknessPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30164u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CenterPointPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30165u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_RotationPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30166u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SizePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30167u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsSelectionPattern2AvailablePropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30168u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Selection2FirstSelectedItemPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30169u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Selection2LastSelectedItemPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30170u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Selection2CurrentSelectedItemPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30171u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_Selection2ItemCountPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30172u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_HeadingLevelPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30173u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsDialogPropertyId: UIA_PROPERTY_ID = UIA_PROPERTY_ID(30174u32);
impl ::core::marker::Copy for UIA_PROPERTY_ID {}
impl ::core::clone::Clone for UIA_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_PROPERTY_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_STYLE_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Custom: UIA_STYLE_ID = UIA_STYLE_ID(70000u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading1: UIA_STYLE_ID = UIA_STYLE_ID(70001u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading2: UIA_STYLE_ID = UIA_STYLE_ID(70002u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading3: UIA_STYLE_ID = UIA_STYLE_ID(70003u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading4: UIA_STYLE_ID = UIA_STYLE_ID(70004u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading5: UIA_STYLE_ID = UIA_STYLE_ID(70005u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading6: UIA_STYLE_ID = UIA_STYLE_ID(70006u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading7: UIA_STYLE_ID = UIA_STYLE_ID(70007u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading8: UIA_STYLE_ID = UIA_STYLE_ID(70008u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Heading9: UIA_STYLE_ID = UIA_STYLE_ID(70009u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Title: UIA_STYLE_ID = UIA_STYLE_ID(70010u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Subtitle: UIA_STYLE_ID = UIA_STYLE_ID(70011u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Normal: UIA_STYLE_ID = UIA_STYLE_ID(70012u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Emphasis: UIA_STYLE_ID = UIA_STYLE_ID(70013u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_Quote: UIA_STYLE_ID = UIA_STYLE_ID(70014u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_BulletedList: UIA_STYLE_ID = UIA_STYLE_ID(70015u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const StyleId_NumberedList: UIA_STYLE_ID = UIA_STYLE_ID(70016u32);
impl ::core::marker::Copy for UIA_STYLE_ID {}
impl ::core::clone::Clone for UIA_STYLE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_STYLE_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIA_TEXTATTRIBUTE_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnimationStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40000u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_BackgroundColorAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40001u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_BulletStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40002u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CapStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40003u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CultureAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40004u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_FontNameAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40005u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_FontSizeAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40006u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_FontWeightAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40007u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_ForegroundColorAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40008u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_HorizontalTextAlignmentAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40009u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IndentationFirstLineAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40010u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IndentationLeadingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40011u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IndentationTrailingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40012u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsHiddenAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40013u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsItalicAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40014u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsReadOnlyAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40015u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsSubscriptAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40016u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsSuperscriptAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40017u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MarginBottomAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40018u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MarginLeadingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40019u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MarginTopAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40020u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_MarginTrailingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40021u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_OutlineStylesAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40022u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_OverlineColorAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40023u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_OverlineStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40024u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StrikethroughColorAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40025u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StrikethroughStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40026u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TabsAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40027u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_TextFlowDirectionsAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40028u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_UnderlineColorAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40029u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_UnderlineStyleAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40030u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnnotationTypesAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40031u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AnnotationObjectsAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40032u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StyleNameAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40033u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_StyleIdAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40034u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LinkAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40035u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_IsActiveAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40036u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SelectionActiveEndAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40037u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CaretPositionAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40038u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_CaretBidiModeAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40039u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_LineSpacingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40040u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_BeforeParagraphSpacingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40041u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_AfterParagraphSpacingAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40042u32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIA_SayAsInterpretAsAttributeId: UIA_TEXTATTRIBUTE_ID = UIA_TEXTATTRIBUTE_ID(40043u32);
impl ::core::marker::Copy for UIA_TEXTATTRIBUTE_ID {}
impl ::core::clone::Clone for UIA_TEXTATTRIBUTE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIA_TEXTATTRIBUTE_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UIAutomationType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_Int: UIAutomationType = UIAutomationType(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_Bool: UIAutomationType = UIAutomationType(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_String: UIAutomationType = UIAutomationType(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_Double: UIAutomationType = UIAutomationType(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_Point: UIAutomationType = UIAutomationType(5i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_Rect: UIAutomationType = UIAutomationType(6i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_Element: UIAutomationType = UIAutomationType(7i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_Array: UIAutomationType = UIAutomationType(65536i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_Out: UIAutomationType = UIAutomationType(131072i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_IntArray: UIAutomationType = UIAutomationType(65537i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_BoolArray: UIAutomationType = UIAutomationType(65538i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_StringArray: UIAutomationType = UIAutomationType(65539i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_DoubleArray: UIAutomationType = UIAutomationType(65540i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_PointArray: UIAutomationType = UIAutomationType(65541i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_RectArray: UIAutomationType = UIAutomationType(65542i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_ElementArray: UIAutomationType = UIAutomationType(65543i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutInt: UIAutomationType = UIAutomationType(131073i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutBool: UIAutomationType = UIAutomationType(131074i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutString: UIAutomationType = UIAutomationType(131075i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutDouble: UIAutomationType = UIAutomationType(131076i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutPoint: UIAutomationType = UIAutomationType(131077i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutRect: UIAutomationType = UIAutomationType(131078i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutElement: UIAutomationType = UIAutomationType(131079i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutIntArray: UIAutomationType = UIAutomationType(196609i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutBoolArray: UIAutomationType = UIAutomationType(196610i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutStringArray: UIAutomationType = UIAutomationType(196611i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutDoubleArray: UIAutomationType = UIAutomationType(196612i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutPointArray: UIAutomationType = UIAutomationType(196613i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutRectArray: UIAutomationType = UIAutomationType(196614i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const UIAutomationType_OutElementArray: UIAutomationType = UIAutomationType(196615i32);
impl ::core::marker::Copy for UIAutomationType {}
impl ::core::clone::Clone for UIAutomationType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIAutomationType {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VisualEffects(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const VisualEffects_None: VisualEffects = VisualEffects(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const VisualEffects_Shadow: VisualEffects = VisualEffects(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const VisualEffects_Reflection: VisualEffects = VisualEffects(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const VisualEffects_Glow: VisualEffects = VisualEffects(4i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const VisualEffects_SoftEdges: VisualEffects = VisualEffects(8i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const VisualEffects_Bevel: VisualEffects = VisualEffects(16i32);
impl ::core::marker::Copy for VisualEffects {}
impl ::core::clone::Clone for VisualEffects {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VisualEffects {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowInteractionState(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const WindowInteractionState_Running: WindowInteractionState = WindowInteractionState(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const WindowInteractionState_Closing: WindowInteractionState = WindowInteractionState(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const WindowInteractionState_ReadyForUserInteraction: WindowInteractionState = WindowInteractionState(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const WindowInteractionState_BlockedByModalWindow: WindowInteractionState = WindowInteractionState(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const WindowInteractionState_NotResponding: WindowInteractionState = WindowInteractionState(4i32);
impl ::core::marker::Copy for WindowInteractionState {}
impl ::core::clone::Clone for WindowInteractionState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WindowInteractionState {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowVisualState(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const WindowVisualState_Normal: WindowVisualState = WindowVisualState(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const WindowVisualState_Maximized: WindowVisualState = WindowVisualState(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const WindowVisualState_Minimized: WindowVisualState = WindowVisualState(2i32);
impl ::core::marker::Copy for WindowVisualState {}
impl ::core::clone::Clone for WindowVisualState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WindowVisualState {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ZoomUnit(pub i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ZoomUnit_NoAmount: ZoomUnit = ZoomUnit(0i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ZoomUnit_LargeDecrement: ZoomUnit = ZoomUnit(1i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ZoomUnit_SmallDecrement: ZoomUnit = ZoomUnit(2i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ZoomUnit_LargeIncrement: ZoomUnit = ZoomUnit(3i32);
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub const ZoomUnit_SmallIncrement: ZoomUnit = ZoomUnit(4i32);
impl ::core::marker::Copy for ZoomUnit {}
impl ::core::clone::Clone for ZoomUnit {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ZoomUnit {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct ACCESSTIMEOUT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iTimeOutMSec: u32,
}
impl ::core::marker::Copy for ACCESSTIMEOUT {}
impl ::core::clone::Clone for ACCESSTIMEOUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACCESSTIMEOUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct ExtendedProperty {
    pub PropertyName: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
    pub PropertyValue: ::windows::core::ManuallyDrop<::windows::core::BSTR>,
}
impl ::core::clone::Clone for ExtendedProperty {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for ExtendedProperty {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct FILTERKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iWaitMSec: u32,
    pub iDelayMSec: u32,
    pub iRepeatMSec: u32,
    pub iBounceMSec: u32,
}
impl ::core::marker::Copy for FILTERKEYS {}
impl ::core::clone::Clone for FILTERKEYS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILTERKEYS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct HIGHCONTRASTA {
    pub cbSize: u32,
    pub dwFlags: HIGHCONTRASTW_FLAGS,
    pub lpszDefaultScheme: ::windows::core::PSTR,
}
impl ::core::marker::Copy for HIGHCONTRASTA {}
impl ::core::clone::Clone for HIGHCONTRASTA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HIGHCONTRASTA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct HIGHCONTRASTW {
    pub cbSize: u32,
    pub dwFlags: HIGHCONTRASTW_FLAGS,
    pub lpszDefaultScheme: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for HIGHCONTRASTW {}
impl ::core::clone::Clone for HIGHCONTRASTW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HIGHCONTRASTW {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HUIAEVENT(pub isize);
impl HUIAEVENT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HUIAEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HUIAEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HUIAEVENT {}
impl ::core::fmt::Debug for HUIAEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HUIAEVENT").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HUIAEVENT>> for HUIAEVENT {
    fn from(optional: ::core::option::Option<HUIAEVENT>) -> HUIAEVENT {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HUIAEVENT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HUIANODE(pub isize);
impl HUIANODE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HUIANODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HUIANODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HUIANODE {}
impl ::core::fmt::Debug for HUIANODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HUIANODE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HUIANODE>> for HUIANODE {
    fn from(optional: ::core::option::Option<HUIANODE>) -> HUIANODE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HUIANODE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HUIAPATTERNOBJECT(pub isize);
impl HUIAPATTERNOBJECT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HUIAPATTERNOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HUIAPATTERNOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HUIAPATTERNOBJECT {}
impl ::core::fmt::Debug for HUIAPATTERNOBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HUIAPATTERNOBJECT").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HUIAPATTERNOBJECT>> for HUIAPATTERNOBJECT {
    fn from(optional: ::core::option::Option<HUIAPATTERNOBJECT>) -> HUIAPATTERNOBJECT {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HUIAPATTERNOBJECT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HUIATEXTRANGE(pub isize);
impl HUIATEXTRANGE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HUIATEXTRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HUIATEXTRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HUIATEXTRANGE {}
impl ::core::fmt::Debug for HUIATEXTRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HUIATEXTRANGE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HUIATEXTRANGE>> for HUIATEXTRANGE {
    fn from(optional: ::core::option::Option<HUIATEXTRANGE>) -> HUIATEXTRANGE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HUIATEXTRANGE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HWINEVENTHOOK(pub isize);
impl HWINEVENTHOOK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HWINEVENTHOOK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWINEVENTHOOK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWINEVENTHOOK {}
impl ::core::fmt::Debug for HWINEVENTHOOK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWINEVENTHOOK").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HWINEVENTHOOK>> for HWINEVENTHOOK {
    fn from(optional: ::core::option::Option<HWINEVENTHOOK>) -> HWINEVENTHOOK {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HWINEVENTHOOK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct MOUSEKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iMaxSpeed: u32,
    pub iTimeToMaxSpeed: u32,
    pub iCtrlSpeed: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for MOUSEKEYS {}
impl ::core::clone::Clone for MOUSEKEYS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MOUSEKEYS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct MSAAMENUINFO {
    pub dwMSAASignature: u32,
    pub cchWText: u32,
    pub pszWText: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MSAAMENUINFO {}
impl ::core::clone::Clone for MSAAMENUINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSAAMENUINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct SERIALKEYSA {
    pub cbSize: u32,
    pub dwFlags: SERIALKEYS_FLAGS,
    pub lpszActivePort: ::windows::core::PSTR,
    pub lpszPort: ::windows::core::PSTR,
    pub iBaudRate: u32,
    pub iPortState: u32,
    pub iActive: u32,
}
impl ::core::marker::Copy for SERIALKEYSA {}
impl ::core::clone::Clone for SERIALKEYSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERIALKEYSA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct SERIALKEYSW {
    pub cbSize: u32,
    pub dwFlags: SERIALKEYS_FLAGS,
    pub lpszActivePort: ::windows::core::PWSTR,
    pub lpszPort: ::windows::core::PWSTR,
    pub iBaudRate: u32,
    pub iPortState: u32,
    pub iActive: u32,
}
impl ::core::marker::Copy for SERIALKEYSW {}
impl ::core::clone::Clone for SERIALKEYSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERIALKEYSW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct SOUNDSENTRYA {
    pub cbSize: u32,
    pub dwFlags: SOUNDSENTRY_FLAGS,
    pub iFSTextEffect: SOUNDSENTRY_TEXT_EFFECT,
    pub iFSTextEffectMSec: u32,
    pub iFSTextEffectColorBits: u32,
    pub iFSGrafEffect: SOUND_SENTRY_GRAPHICS_EFFECT,
    pub iFSGrafEffectMSec: u32,
    pub iFSGrafEffectColor: u32,
    pub iWindowsEffect: SOUNDSENTRY_WINDOWS_EFFECT,
    pub iWindowsEffectMSec: u32,
    pub lpszWindowsEffectDLL: ::windows::core::PSTR,
    pub iWindowsEffectOrdinal: u32,
}
impl ::core::marker::Copy for SOUNDSENTRYA {}
impl ::core::clone::Clone for SOUNDSENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SOUNDSENTRYA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct SOUNDSENTRYW {
    pub cbSize: u32,
    pub dwFlags: SOUNDSENTRY_FLAGS,
    pub iFSTextEffect: SOUNDSENTRY_TEXT_EFFECT,
    pub iFSTextEffectMSec: u32,
    pub iFSTextEffectColorBits: u32,
    pub iFSGrafEffect: SOUND_SENTRY_GRAPHICS_EFFECT,
    pub iFSGrafEffectMSec: u32,
    pub iFSGrafEffectColor: u32,
    pub iWindowsEffect: SOUNDSENTRY_WINDOWS_EFFECT,
    pub iWindowsEffectMSec: u32,
    pub lpszWindowsEffectDLL: ::windows::core::PWSTR,
    pub iWindowsEffectOrdinal: u32,
}
impl ::core::marker::Copy for SOUNDSENTRYW {}
impl ::core::clone::Clone for SOUNDSENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SOUNDSENTRYW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct STICKYKEYS {
    pub cbSize: u32,
    pub dwFlags: STICKYKEYS_FLAGS,
}
impl ::core::marker::Copy for STICKYKEYS {}
impl ::core::clone::Clone for STICKYKEYS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for STICKYKEYS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct TOGGLEKEYS {
    pub cbSize: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for TOGGLEKEYS {}
impl ::core::clone::Clone for TOGGLEKEYS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOGGLEKEYS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UIAutomationEventInfo {
    pub guid: ::windows::core::GUID,
    pub pProgrammaticName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for UIAutomationEventInfo {}
impl ::core::clone::Clone for UIAutomationEventInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIAutomationEventInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct UIAutomationMethodInfo {
    pub pProgrammaticName: ::windows::core::PCWSTR,
    pub doSetFocus: super::super::Foundation::BOOL,
    pub cInParameters: u32,
    pub cOutParameters: u32,
    pub pParameterTypes: *mut UIAutomationType,
    pub pParameterNames: *mut ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UIAutomationMethodInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UIAutomationMethodInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for UIAutomationMethodInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UIAutomationParameter {
    pub r#type: UIAutomationType,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for UIAutomationParameter {}
impl ::core::clone::Clone for UIAutomationParameter {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIAutomationParameter {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct UIAutomationPatternInfo {
    pub guid: ::windows::core::GUID,
    pub pProgrammaticName: ::windows::core::PCWSTR,
    pub providerInterfaceId: ::windows::core::GUID,
    pub clientInterfaceId: ::windows::core::GUID,
    pub cProperties: u32,
    pub pProperties: *mut UIAutomationPropertyInfo,
    pub cMethods: u32,
    pub pMethods: *mut UIAutomationMethodInfo,
    pub cEvents: u32,
    pub pEvents: *mut UIAutomationEventInfo,
    pub pPatternHandler: ::windows::core::ManuallyDrop<IUIAutomationPatternHandler>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UIAutomationPatternInfo {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for UIAutomationPatternInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UIAutomationPropertyInfo {
    pub guid: ::windows::core::GUID,
    pub pProgrammaticName: ::windows::core::PCWSTR,
    pub r#type: UIAutomationType,
}
impl ::core::marker::Copy for UIAutomationPropertyInfo {}
impl ::core::clone::Clone for UIAutomationPropertyInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UIAutomationPropertyInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UiaAndOrCondition {
    pub ConditionType: ConditionType,
    pub ppConditions: *mut *mut UiaCondition,
    pub cConditions: i32,
}
impl ::core::marker::Copy for UiaAndOrCondition {}
impl ::core::clone::Clone for UiaAndOrCondition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UiaAndOrCondition {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UiaAsyncContentLoadedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub AsyncContentLoadedState: AsyncContentLoadedState,
    pub PercentComplete: f64,
}
impl ::core::marker::Copy for UiaAsyncContentLoadedEventArgs {}
impl ::core::clone::Clone for UiaAsyncContentLoadedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UiaAsyncContentLoadedEventArgs {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UiaCacheRequest {
    pub pViewCondition: *mut UiaCondition,
    pub Scope: TreeScope,
    pub pProperties: *mut i32,
    pub cProperties: i32,
    pub pPatterns: *mut i32,
    pub cPatterns: i32,
    pub automationElementMode: AutomationElementMode,
}
impl ::core::marker::Copy for UiaCacheRequest {}
impl ::core::clone::Clone for UiaCacheRequest {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UiaCacheRequest {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct UiaChangeInfo {
    pub uiaId: i32,
    pub payload: super::super::System::Com::VARIANT,
    pub extraInfo: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for UiaChangeInfo {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for UiaChangeInfo {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct UiaChangesEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub EventIdCount: i32,
    pub pUiaChanges: *mut UiaChangeInfo,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for UiaChangesEventArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for UiaChangesEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for UiaChangesEventArgs {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UiaCondition {
    pub ConditionType: ConditionType,
}
impl ::core::marker::Copy for UiaCondition {}
impl ::core::clone::Clone for UiaCondition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UiaCondition {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UiaEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
}
impl ::core::marker::Copy for UiaEventArgs {}
impl ::core::clone::Clone for UiaEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UiaEventArgs {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct UiaFindParams {
    pub MaxDepth: i32,
    pub FindFirst: super::super::Foundation::BOOL,
    pub ExcludeRoot: super::super::Foundation::BOOL,
    pub pFindCondition: *mut UiaCondition,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UiaFindParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UiaFindParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for UiaFindParams {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UiaNotCondition {
    pub ConditionType: ConditionType,
    pub pCondition: *mut UiaCondition,
}
impl ::core::marker::Copy for UiaNotCondition {}
impl ::core::clone::Clone for UiaNotCondition {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UiaNotCondition {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UiaPoint {
    pub x: f64,
    pub y: f64,
}
impl ::core::marker::Copy for UiaPoint {}
impl ::core::clone::Clone for UiaPoint {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UiaPoint {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct UiaPropertyChangedEventArgs {
    pub Type: EventArgsType,
    pub EventId: UIA_EVENT_ID,
    pub PropertyId: i32,
    pub OldValue: super::super::System::Com::VARIANT,
    pub NewValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for UiaPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for UiaPropertyChangedEventArgs {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct UiaPropertyCondition {
    pub ConditionType: ConditionType,
    pub PropertyId: UIA_PROPERTY_ID,
    pub Value: super::super::System::Com::VARIANT,
    pub Flags: PropertyConditionFlags,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for UiaPropertyCondition {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for UiaPropertyCondition {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UiaRect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}
impl ::core::marker::Copy for UiaRect {}
impl ::core::clone::Clone for UiaRect {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UiaRect {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UiaStructureChangedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub StructureChangeType: StructureChangeType,
    pub pRuntimeId: *mut i32,
    pub cRuntimeIdLen: i32,
}
impl ::core::marker::Copy for UiaStructureChangedEventArgs {}
impl ::core::clone::Clone for UiaStructureChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UiaStructureChangedEventArgs {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct UiaTextEditTextChangedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub TextEditChangeType: TextEditChangeType,
    pub pTextChange: *mut super::super::System::Com::SAFEARRAY,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for UiaTextEditTextChangedEventArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for UiaTextEditTextChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for UiaTextEditTextChangedEventArgs {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`*"]
pub struct UiaWindowClosedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub pRuntimeId: *mut i32,
    pub cRuntimeIdLen: i32,
}
impl ::core::marker::Copy for UiaWindowClosedEventArgs {}
impl ::core::clone::Clone for UiaWindowClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UiaWindowClosedEventArgs {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type LPFNACCESSIBLECHILDREN = ::core::option::Option<unsafe extern "system" fn(pacccontainer: ::core::option::Option<IAccessible>, ichildstart: i32, cchildren: i32, rgvarchildren: *mut super::super::System::Com::VARIANT, pcobtained: *mut i32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub type LPFNACCESSIBLEOBJECTFROMPOINT = ::core::option::Option<unsafe extern "system" fn(ptscreen: super::super::Foundation::POINT, ppacc: *mut ::core::option::Option<IAccessible>, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNACCESSIBLEOBJECTFROMWINDOW = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, dwid: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNCREATESTDACCESSIBLEOBJECT = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, idobject: i32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNLRESULTFROMOBJECT = ::core::option::Option<unsafe extern "system" fn(riid: *const ::windows::core::GUID, wparam: super::super::Foundation::WPARAM, punk: ::core::option::Option<::windows::core::IUnknown>) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNOBJECTFROMLRESULT = ::core::option::Option<unsafe extern "system" fn(lresult: super::super::Foundation::LRESULT, riid: *const ::windows::core::GUID, wparam: super::super::Foundation::WPARAM, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type UiaEventCallback = ::core::option::Option<unsafe extern "system" fn(pargs: *mut UiaEventArgs, prequesteddata: *mut super::super::System::Com::SAFEARRAY, ptreestructure: ::windows::core::BSTR) -> ()>;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type UiaProviderCallback = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, providertype: ProviderType) -> *mut super::super::System::Com::SAFEARRAY>;
#[doc = "*Required features: `\"Win32_UI_Accessibility\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WINEVENTPROC = ::core::option::Option<unsafe extern "system" fn(hwineventhook: HWINEVENTHOOK, event: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, ideventthread: u32, dwmseventtime: u32) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
