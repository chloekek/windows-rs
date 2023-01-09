#[cfg(feature = "Win32_UI_Controls_Dialogs")]
pub mod Dialogs;
#[cfg(feature = "Win32_UI_Controls_RichEdit")]
pub mod RichEdit;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn BeginBufferedAnimation<P0, P1>(hwnd: P0, hdctarget: P1, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: ::core::option::Option<*const BP_PAINTPARAMS>, panimationparams: *const BP_ANIMATIONPARAMS, phdcfrom: *mut super::super::Graphics::Gdi::HDC, phdcto: *mut super::super::Graphics::Gdi::HDC) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn BeginBufferedAnimation ( hwnd : super::super::Foundation:: HWND , hdctarget : super::super::Graphics::Gdi:: HDC , prctarget : *const super::super::Foundation:: RECT , dwformat : BP_BUFFERFORMAT , ppaintparams : *const BP_PAINTPARAMS , panimationparams : *const BP_ANIMATIONPARAMS , phdcfrom : *mut super::super::Graphics::Gdi:: HDC , phdcto : *mut super::super::Graphics::Gdi:: HDC ) -> isize );
    BeginBufferedAnimation(hwnd.into(), hdctarget.into(), prctarget, dwformat, ::core::mem::transmute(ppaintparams.unwrap_or(::std::ptr::null())), panimationparams, phdcfrom, phdcto)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn BeginBufferedPaint<P0>(hdctarget: P0, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: ::core::option::Option<*const BP_PAINTPARAMS>, phdc: *mut super::super::Graphics::Gdi::HDC) -> isize
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn BeginBufferedPaint ( hdctarget : super::super::Graphics::Gdi:: HDC , prctarget : *const super::super::Foundation:: RECT , dwformat : BP_BUFFERFORMAT , ppaintparams : *const BP_PAINTPARAMS , phdc : *mut super::super::Graphics::Gdi:: HDC ) -> isize );
    BeginBufferedPaint(hdctarget.into(), prctarget, dwformat, ::core::mem::transmute(ppaintparams.unwrap_or(::std::ptr::null())), phdc)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginPanningFeedback<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn BeginPanningFeedback ( hwnd : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    BeginPanningFeedback(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BufferedPaintClear(hbufferedpaint: isize, prc: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uxtheme.dll""system" fn BufferedPaintClear ( hbufferedpaint : isize , prc : *const super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    BufferedPaintClear(hbufferedpaint, ::core::mem::transmute(prc.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn BufferedPaintInit() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uxtheme.dll""system" fn BufferedPaintInit ( ) -> :: windows::core::HRESULT );
    BufferedPaintInit().ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn BufferedPaintRenderAnimation<P0, P1>(hwnd: P0, hdctarget: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn BufferedPaintRenderAnimation ( hwnd : super::super::Foundation:: HWND , hdctarget : super::super::Graphics::Gdi:: HDC ) -> super::super::Foundation:: BOOL );
    BufferedPaintRenderAnimation(hwnd.into(), hdctarget.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BufferedPaintSetAlpha(hbufferedpaint: isize, prc: ::core::option::Option<*const super::super::Foundation::RECT>, alpha: u8) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uxtheme.dll""system" fn BufferedPaintSetAlpha ( hbufferedpaint : isize , prc : *const super::super::Foundation:: RECT , alpha : u8 ) -> :: windows::core::HRESULT );
    BufferedPaintSetAlpha(hbufferedpaint, ::core::mem::transmute(prc.unwrap_or(::std::ptr::null())), alpha).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BufferedPaintStopAllAnimations<P0>(hwnd: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn BufferedPaintStopAllAnimations ( hwnd : super::super::Foundation:: HWND ) -> :: windows::core::HRESULT );
    BufferedPaintStopAllAnimations(hwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn BufferedPaintUnInit() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uxtheme.dll""system" fn BufferedPaintUnInit ( ) -> :: windows::core::HRESULT );
    BufferedPaintUnInit().ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckDlgButton<P0>(hdlg: P0, nidbutton: i32, ucheck: DLG_BUTTON_CHECK_STATE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn CheckDlgButton ( hdlg : super::super::Foundation:: HWND , nidbutton : i32 , ucheck : DLG_BUTTON_CHECK_STATE ) -> super::super::Foundation:: BOOL );
    CheckDlgButton(hdlg.into(), nidbutton, ucheck)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckRadioButton<P0>(hdlg: P0, nidfirstbutton: i32, nidlastbutton: i32, nidcheckbutton: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn CheckRadioButton ( hdlg : super::super::Foundation:: HWND , nidfirstbutton : i32 , nidlastbutton : i32 , nidcheckbutton : i32 ) -> super::super::Foundation:: BOOL );
    CheckRadioButton(hdlg.into(), nidfirstbutton, nidlastbutton, nidcheckbutton)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn CloseThemeData<P0>(htheme: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn CloseThemeData ( htheme : HTHEME ) -> :: windows::core::HRESULT );
    CloseThemeData(htheme.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CreateMappedBitmap<P0>(hinstance: P0, idbitmap: isize, wflags: u32, lpcolormap: ::core::option::Option<*const COLORMAP>, inummaps: i32) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn CreateMappedBitmap ( hinstance : super::super::Foundation:: HINSTANCE , idbitmap : isize , wflags : u32 , lpcolormap : *const COLORMAP , inummaps : i32 ) -> super::super::Graphics::Gdi:: HBITMAP );
    let result__ = CreateMappedBitmap(hinstance.into(), idbitmap, wflags, ::core::mem::transmute(lpcolormap.unwrap_or(::std::ptr::null())), inummaps);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn CreatePropertySheetPageA(constpropsheetpagepointer: *mut PROPSHEETPAGEA) -> HPROPSHEETPAGE {
    ::windows::core::link ! ( "comctl32.dll""system" fn CreatePropertySheetPageA ( constpropsheetpagepointer : *mut PROPSHEETPAGEA ) -> HPROPSHEETPAGE );
    CreatePropertySheetPageA(constpropsheetpagepointer)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn CreatePropertySheetPageW(constpropsheetpagepointer: *mut PROPSHEETPAGEW) -> HPROPSHEETPAGE {
    ::windows::core::link ! ( "comctl32.dll""system" fn CreatePropertySheetPageW ( constpropsheetpagepointer : *mut PROPSHEETPAGEW ) -> HPROPSHEETPAGE );
    CreatePropertySheetPageW(constpropsheetpagepointer)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStatusWindowA<P0, P1>(style: i32, lpsztext: P0, hwndparent: P1, wid: u32) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn CreateStatusWindowA ( style : i32 , lpsztext : :: windows::core::PCSTR , hwndparent : super::super::Foundation:: HWND , wid : u32 ) -> super::super::Foundation:: HWND );
    CreateStatusWindowA(style, lpsztext.into().abi(), hwndparent.into(), wid)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStatusWindowW<P0, P1>(style: i32, lpsztext: P0, hwndparent: P1, wid: u32) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn CreateStatusWindowW ( style : i32 , lpsztext : :: windows::core::PCWSTR , hwndparent : super::super::Foundation:: HWND , wid : u32 ) -> super::super::Foundation:: HWND );
    CreateStatusWindowW(style, lpsztext.into().abi(), hwndparent.into(), wid)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn CreateSyntheticPointerDevice(pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, maxcount: u32, mode: POINTER_FEEDBACK_MODE) -> ::windows::core::Result<HSYNTHETICPOINTERDEVICE> {
    ::windows::core::link ! ( "user32.dll""system" fn CreateSyntheticPointerDevice ( pointertype : super::WindowsAndMessaging:: POINTER_INPUT_TYPE , maxcount : u32 , mode : POINTER_FEEDBACK_MODE ) -> HSYNTHETICPOINTERDEVICE );
    let result__ = CreateSyntheticPointerDevice(pointertype, maxcount, mode);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateToolbarEx<P0, P1>(hwnd: P0, ws: u32, wid: u32, nbitmaps: i32, hbminst: P1, wbmid: usize, lpbuttons: *mut TBBUTTON, inumbuttons: i32, dxbutton: i32, dybutton: i32, dxbitmap: i32, dybitmap: i32, ustructsize: u32) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn CreateToolbarEx ( hwnd : super::super::Foundation:: HWND , ws : u32 , wid : u32 , nbitmaps : i32 , hbminst : super::super::Foundation:: HINSTANCE , wbmid : usize , lpbuttons : *mut TBBUTTON , inumbuttons : i32 , dxbutton : i32 , dybutton : i32 , dxbitmap : i32 , dybitmap : i32 , ustructsize : u32 ) -> super::super::Foundation:: HWND );
    CreateToolbarEx(hwnd.into(), ws, wid, nbitmaps, hbminst.into(), wbmid, lpbuttons, inumbuttons, dxbutton, dybutton, dxbitmap, dybitmap, ustructsize)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUpDownControl<P0, P1, P2>(dwstyle: u32, x: i32, y: i32, cx: i32, cy: i32, hparent: P0, nid: i32, hinst: P1, hbuddy: P2, nupper: i32, nlower: i32, npos: i32) -> super::super::Foundation::HWND
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn CreateUpDownControl ( dwstyle : u32 , x : i32 , y : i32 , cx : i32 , cy : i32 , hparent : super::super::Foundation:: HWND , nid : i32 , hinst : super::super::Foundation:: HINSTANCE , hbuddy : super::super::Foundation:: HWND , nupper : i32 , nlower : i32 , npos : i32 ) -> super::super::Foundation:: HWND );
    CreateUpDownControl(dwstyle, x, y, cx, cy, hparent.into(), nid, hinst.into(), hbuddy.into(), nupper, nlower, npos)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DPA_Clone<P0, P1>(hdpa: P0, hdpanew: P1) -> HDPA
where
    P0: ::std::convert::Into<HDPA>,
    P1: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_Clone ( hdpa : HDPA , hdpanew : HDPA ) -> HDPA );
    DPA_Clone(hdpa.into(), hdpanew.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DPA_Create(citemgrow: i32) -> HDPA {
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_Create ( citemgrow : i32 ) -> HDPA );
    DPA_Create(citemgrow)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_CreateEx<P0>(cpgrow: i32, hheap: P0) -> HDPA
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_CreateEx ( cpgrow : i32 , hheap : super::super::Foundation:: HANDLE ) -> HDPA );
    DPA_CreateEx(cpgrow, hheap.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_DeleteAllPtrs<P0>(hdpa: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_DeleteAllPtrs ( hdpa : HDPA ) -> super::super::Foundation:: BOOL );
    DPA_DeleteAllPtrs(hdpa.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DPA_DeletePtr<P0>(hdpa: P0, i: i32) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_DeletePtr ( hdpa : HDPA , i : i32 ) -> *mut ::core::ffi::c_void );
    DPA_DeletePtr(hdpa.into(), i)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_Destroy<P0>(hdpa: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_Destroy ( hdpa : HDPA ) -> super::super::Foundation:: BOOL );
    DPA_Destroy(hdpa.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DPA_DestroyCallback<P0>(hdpa: P0, pfncb: PFNDAENUMCALLBACK, pdata: ::core::option::Option<*const ::core::ffi::c_void>)
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_DestroyCallback ( hdpa : HDPA , pfncb : PFNDAENUMCALLBACK , pdata : *const ::core::ffi::c_void ) -> ( ) );
    DPA_DestroyCallback(hdpa.into(), pfncb, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DPA_EnumCallback<P0>(hdpa: P0, pfncb: PFNDAENUMCALLBACK, pdata: ::core::option::Option<*const ::core::ffi::c_void>)
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_EnumCallback ( hdpa : HDPA , pfncb : PFNDAENUMCALLBACK , pdata : *const ::core::ffi::c_void ) -> ( ) );
    DPA_EnumCallback(hdpa.into(), pfncb, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DPA_GetPtr<P0>(hdpa: P0, i: isize) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_GetPtr ( hdpa : HDPA , i : isize ) -> *mut ::core::ffi::c_void );
    DPA_GetPtr(hdpa.into(), i)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DPA_GetPtrIndex<P0>(hdpa: P0, p: ::core::option::Option<*const ::core::ffi::c_void>) -> i32
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_GetPtrIndex ( hdpa : HDPA , p : *const ::core::ffi::c_void ) -> i32 );
    DPA_GetPtrIndex(hdpa.into(), ::core::mem::transmute(p.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DPA_GetSize<P0>(hdpa: P0) -> u64
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_GetSize ( hdpa : HDPA ) -> u64 );
    DPA_GetSize(hdpa.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_Grow<P0>(pdpa: P0, cp: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_Grow ( pdpa : HDPA , cp : i32 ) -> super::super::Foundation:: BOOL );
    DPA_Grow(pdpa.into(), cp)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DPA_InsertPtr<P0>(hdpa: P0, i: i32, p: ::core::option::Option<*const ::core::ffi::c_void>) -> i32
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_InsertPtr ( hdpa : HDPA , i : i32 , p : *const ::core::ffi::c_void ) -> i32 );
    DPA_InsertPtr(hdpa.into(), i, ::core::mem::transmute(p.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DPA_LoadStream<P0>(phdpa: *mut HDPA, pfn: PFNDPASTREAM, pstream: P0, pvinstdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_LoadStream ( phdpa : *mut HDPA , pfn : PFNDPASTREAM , pstream : * mut::core::ffi::c_void , pvinstdata : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DPA_LoadStream(phdpa, pfn, pstream.into().abi(), ::core::mem::transmute(pvinstdata.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_Merge<P0, P1, P2>(hdpadest: P0, hdpasrc: P1, dwflags: u32, pfncompare: PFNDACOMPARE, pfnmerge: PFNDPAMERGE, lparam: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDPA>,
    P1: ::std::convert::Into<HDPA>,
    P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_Merge ( hdpadest : HDPA , hdpasrc : HDPA , dwflags : u32 , pfncompare : PFNDACOMPARE , pfnmerge : PFNDPAMERGE , lparam : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: BOOL );
    DPA_Merge(hdpadest.into(), hdpasrc.into(), dwflags, pfncompare, pfnmerge, lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DPA_SaveStream<P0, P1>(hdpa: P0, pfn: PFNDPASTREAM, pstream: P1, pvinstdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HDPA>,
    P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_SaveStream ( hdpa : HDPA , pfn : PFNDPASTREAM , pstream : * mut::core::ffi::c_void , pvinstdata : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DPA_SaveStream(hdpa.into(), pfn, pstream.into().abi(), ::core::mem::transmute(pvinstdata.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_Search<P0, P1>(hdpa: P0, pfind: ::core::option::Option<*const ::core::ffi::c_void>, istart: i32, pfncompare: PFNDACOMPARE, lparam: P1, options: u32) -> i32
where
    P0: ::std::convert::Into<HDPA>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_Search ( hdpa : HDPA , pfind : *const ::core::ffi::c_void , istart : i32 , pfncompare : PFNDACOMPARE , lparam : super::super::Foundation:: LPARAM , options : u32 ) -> i32 );
    DPA_Search(hdpa.into(), ::core::mem::transmute(pfind.unwrap_or(::std::ptr::null())), istart, pfncompare, lparam.into(), options)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_SetPtr<P0>(hdpa: P0, i: i32, p: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDPA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_SetPtr ( hdpa : HDPA , i : i32 , p : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DPA_SetPtr(hdpa.into(), i, ::core::mem::transmute(p.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_Sort<P0, P1>(hdpa: P0, pfncompare: PFNDACOMPARE, lparam: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDPA>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DPA_Sort ( hdpa : HDPA , pfncompare : PFNDACOMPARE , lparam : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: BOOL );
    DPA_Sort(hdpa.into(), pfncompare, lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DSA_Clone<P0>(hdsa: P0) -> HDSA
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_Clone ( hdsa : HDSA ) -> HDSA );
    DSA_Clone(hdsa.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DSA_Create(cbitem: i32, citemgrow: i32) -> HDSA {
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_Create ( cbitem : i32 , citemgrow : i32 ) -> HDSA );
    DSA_Create(cbitem, citemgrow)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_DeleteAllItems<P0>(hdsa: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_DeleteAllItems ( hdsa : HDSA ) -> super::super::Foundation:: BOOL );
    DSA_DeleteAllItems(hdsa.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_DeleteItem<P0>(hdsa: P0, i: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_DeleteItem ( hdsa : HDSA , i : i32 ) -> super::super::Foundation:: BOOL );
    DSA_DeleteItem(hdsa.into(), i)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_Destroy<P0>(hdsa: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_Destroy ( hdsa : HDSA ) -> super::super::Foundation:: BOOL );
    DSA_Destroy(hdsa.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DSA_DestroyCallback<P0>(hdsa: P0, pfncb: PFNDAENUMCALLBACK, pdata: ::core::option::Option<*const ::core::ffi::c_void>)
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_DestroyCallback ( hdsa : HDSA , pfncb : PFNDAENUMCALLBACK , pdata : *const ::core::ffi::c_void ) -> ( ) );
    DSA_DestroyCallback(hdsa.into(), pfncb, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DSA_EnumCallback<P0>(hdsa: P0, pfncb: PFNDAENUMCALLBACK, pdata: ::core::option::Option<*const ::core::ffi::c_void>)
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_EnumCallback ( hdsa : HDSA , pfncb : PFNDAENUMCALLBACK , pdata : *const ::core::ffi::c_void ) -> ( ) );
    DSA_EnumCallback(hdsa.into(), pfncb, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_GetItem<P0>(hdsa: P0, i: i32, pitem: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_GetItem ( hdsa : HDSA , i : i32 , pitem : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DSA_GetItem(hdsa.into(), i, pitem)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DSA_GetItemPtr<P0>(hdsa: P0, i: i32) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_GetItemPtr ( hdsa : HDSA , i : i32 ) -> *mut ::core::ffi::c_void );
    DSA_GetItemPtr(hdsa.into(), i)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DSA_GetSize<P0>(hdsa: P0) -> u64
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_GetSize ( hdsa : HDSA ) -> u64 );
    DSA_GetSize(hdsa.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DSA_InsertItem<P0>(hdsa: P0, i: i32, pitem: *const ::core::ffi::c_void) -> i32
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_InsertItem ( hdsa : HDSA , i : i32 , pitem : *const ::core::ffi::c_void ) -> i32 );
    DSA_InsertItem(hdsa.into(), i, pitem)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_SetItem<P0>(hdsa: P0, i: i32, pitem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDSA>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_SetItem ( hdsa : HDSA , i : i32 , pitem : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DSA_SetItem(hdsa.into(), i, pitem)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_Sort<P0, P1>(pdsa: P0, pfncompare: PFNDACOMPARE, lparam: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HDSA>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DSA_Sort ( pdsa : HDSA , pfncompare : PFNDACOMPARE , lparam : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: BOOL );
    DSA_Sort(pdsa.into(), pfncompare, lparam.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyPropertySheetPage<P0>(param0: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HPROPSHEETPAGE>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DestroyPropertySheetPage ( param0 : HPROPSHEETPAGE ) -> super::super::Foundation:: BOOL );
    DestroyPropertySheetPage(param0.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn DestroySyntheticPointerDevice<P0>(device: P0)
where
    P0: ::std::convert::Into<HSYNTHETICPOINTERDEVICE>,
{
    ::windows::core::link ! ( "user32.dll""system" fn DestroySyntheticPointerDevice ( device : HSYNTHETICPOINTERDEVICE ) -> ( ) );
    DestroySyntheticPointerDevice(device.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirListA<P0>(hdlg: P0, lppathspec: ::windows::core::PSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn DlgDirListA ( hdlg : super::super::Foundation:: HWND , lppathspec : :: windows::core::PSTR , nidlistbox : i32 , nidstaticpath : i32 , ufiletype : DLG_DIR_LIST_FILE_TYPE ) -> i32 );
    DlgDirListA(hdlg.into(), ::core::mem::transmute(lppathspec), nidlistbox, nidstaticpath, ufiletype)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirListComboBoxA<P0>(hdlg: P0, lppathspec: ::windows::core::PSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn DlgDirListComboBoxA ( hdlg : super::super::Foundation:: HWND , lppathspec : :: windows::core::PSTR , nidcombobox : i32 , nidstaticpath : i32 , ufiletype : DLG_DIR_LIST_FILE_TYPE ) -> i32 );
    DlgDirListComboBoxA(hdlg.into(), ::core::mem::transmute(lppathspec), nidcombobox, nidstaticpath, ufiletype)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirListComboBoxW<P0>(hdlg: P0, lppathspec: ::windows::core::PWSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn DlgDirListComboBoxW ( hdlg : super::super::Foundation:: HWND , lppathspec : :: windows::core::PWSTR , nidcombobox : i32 , nidstaticpath : i32 , ufiletype : DLG_DIR_LIST_FILE_TYPE ) -> i32 );
    DlgDirListComboBoxW(hdlg.into(), ::core::mem::transmute(lppathspec), nidcombobox, nidstaticpath, ufiletype)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirListW<P0>(hdlg: P0, lppathspec: ::windows::core::PWSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn DlgDirListW ( hdlg : super::super::Foundation:: HWND , lppathspec : :: windows::core::PWSTR , nidlistbox : i32 , nidstaticpath : i32 , ufiletype : DLG_DIR_LIST_FILE_TYPE ) -> i32 );
    DlgDirListW(hdlg.into(), ::core::mem::transmute(lppathspec), nidlistbox, nidstaticpath, ufiletype)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirSelectComboBoxExA<P0>(hwnddlg: P0, lpstring: &mut [u8], idcombobox: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn DlgDirSelectComboBoxExA ( hwnddlg : super::super::Foundation:: HWND , lpstring : :: windows::core::PSTR , cchout : i32 , idcombobox : i32 ) -> super::super::Foundation:: BOOL );
    DlgDirSelectComboBoxExA(hwnddlg.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, idcombobox)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirSelectComboBoxExW<P0>(hwnddlg: P0, lpstring: &mut [u16], idcombobox: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn DlgDirSelectComboBoxExW ( hwnddlg : super::super::Foundation:: HWND , lpstring : :: windows::core::PWSTR , cchout : i32 , idcombobox : i32 ) -> super::super::Foundation:: BOOL );
    DlgDirSelectComboBoxExW(hwnddlg.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, idcombobox)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirSelectExA<P0>(hwnddlg: P0, lpstring: &mut [u8], idlistbox: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn DlgDirSelectExA ( hwnddlg : super::super::Foundation:: HWND , lpstring : :: windows::core::PSTR , chcount : i32 , idlistbox : i32 ) -> super::super::Foundation:: BOOL );
    DlgDirSelectExA(hwnddlg.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, idlistbox)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirSelectExW<P0>(hwnddlg: P0, lpstring: &mut [u16], idlistbox: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn DlgDirSelectExW ( hwnddlg : super::super::Foundation:: HWND , lpstring : :: windows::core::PWSTR , chcount : i32 , idlistbox : i32 ) -> super::super::Foundation:: BOOL );
    DlgDirSelectExW(hwnddlg.into(), ::core::mem::transmute(lpstring.as_ptr()), lpstring.len() as _, idlistbox)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawInsert<P0, P1>(handparent: P0, hlb: P1, nitem: i32)
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DrawInsert ( handparent : super::super::Foundation:: HWND , hlb : super::super::Foundation:: HWND , nitem : i32 ) -> ( ) );
    DrawInsert(handparent.into(), hlb.into(), nitem)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawShadowText<P0, P1, P2>(hdc: P0, psztext: &[u16], prc: *const super::super::Foundation::RECT, dwflags: u32, crtext: P1, crshadow: P2, ixoffset: i32, iyoffset: i32) -> i32
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
    P2: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DrawShadowText ( hdc : super::super::Graphics::Gdi:: HDC , psztext : :: windows::core::PCWSTR , cch : u32 , prc : *const super::super::Foundation:: RECT , dwflags : u32 , crtext : super::super::Foundation:: COLORREF , crshadow : super::super::Foundation:: COLORREF , ixoffset : i32 , iyoffset : i32 ) -> i32 );
    DrawShadowText(hdc.into(), ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _, prc, dwflags, crtext.into(), crshadow.into(), ixoffset, iyoffset)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawStatusTextA<P0, P1>(hdc: P0, lprc: *mut super::super::Foundation::RECT, psztext: P1, uflags: u32)
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DrawStatusTextA ( hdc : super::super::Graphics::Gdi:: HDC , lprc : *mut super::super::Foundation:: RECT , psztext : :: windows::core::PCSTR , uflags : u32 ) -> ( ) );
    DrawStatusTextA(hdc.into(), lprc, psztext.into().abi(), uflags)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawStatusTextW<P0, P1>(hdc: P0, lprc: *mut super::super::Foundation::RECT, psztext: P1, uflags: u32)
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn DrawStatusTextW ( hdc : super::super::Graphics::Gdi:: HDC , lprc : *mut super::super::Foundation:: RECT , psztext : :: windows::core::PCWSTR , uflags : u32 ) -> ( ) );
    DrawStatusTextW(hdc.into(), lprc, psztext.into().abi(), uflags)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeBackground<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, pcliprect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn DrawThemeBackground ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , prect : *const super::super::Foundation:: RECT , pcliprect : *const super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    DrawThemeBackground(htheme.into(), hdc.into(), ipartid, istateid, prect, ::core::mem::transmute(pcliprect.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeBackgroundEx<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, poptions: ::core::option::Option<*const DTBGOPTS>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn DrawThemeBackgroundEx ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , prect : *const super::super::Foundation:: RECT , poptions : *const DTBGOPTS ) -> :: windows::core::HRESULT );
    DrawThemeBackgroundEx(htheme.into(), hdc.into(), ipartid, istateid, prect, ::core::mem::transmute(poptions.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeEdge<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, pdestrect: *const super::super::Foundation::RECT, uedge: super::super::Graphics::Gdi::DRAWEDGE_FLAGS, uflags: super::super::Graphics::Gdi::DRAW_EDGE_FLAGS, pcontentrect: ::core::option::Option<*mut super::super::Foundation::RECT>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn DrawThemeEdge ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , pdestrect : *const super::super::Foundation:: RECT , uedge : super::super::Graphics::Gdi:: DRAWEDGE_FLAGS , uflags : super::super::Graphics::Gdi:: DRAW_EDGE_FLAGS , pcontentrect : *mut super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    DrawThemeEdge(htheme.into(), hdc.into(), ipartid, istateid, pdestrect, uedge, uflags, ::core::mem::transmute(pcontentrect.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeIcon<P0, P1, P2>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, himl: P2, iimageindex: i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    P2: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn DrawThemeIcon ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , prect : *const super::super::Foundation:: RECT , himl : HIMAGELIST , iimageindex : i32 ) -> :: windows::core::HRESULT );
    DrawThemeIcon(htheme.into(), hdc.into(), ipartid, istateid, prect, himl.into(), iimageindex).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeParentBackground<P0, P1>(hwnd: P0, hdc: P1, prc: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn DrawThemeParentBackground ( hwnd : super::super::Foundation:: HWND , hdc : super::super::Graphics::Gdi:: HDC , prc : *const super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    DrawThemeParentBackground(hwnd.into(), hdc.into(), ::core::mem::transmute(prc.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeParentBackgroundEx<P0, P1>(hwnd: P0, hdc: P1, dwflags: DRAW_THEME_PARENT_BACKGROUND_FLAGS, prc: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn DrawThemeParentBackgroundEx ( hwnd : super::super::Foundation:: HWND , hdc : super::super::Graphics::Gdi:: HDC , dwflags : DRAW_THEME_PARENT_BACKGROUND_FLAGS , prc : *const super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    DrawThemeParentBackgroundEx(hwnd.into(), hdc.into(), dwflags, ::core::mem::transmute(prc.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeText<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, psztext: &[u16], dwtextflags: super::super::Graphics::Gdi::DRAW_TEXT_FORMAT, dwtextflags2: u32, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn DrawThemeText ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , psztext : :: windows::core::PCWSTR , cchtext : i32 , dwtextflags : super::super::Graphics::Gdi:: DRAW_TEXT_FORMAT , dwtextflags2 : u32 , prect : *const super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    DrawThemeText(htheme.into(), hdc.into(), ipartid, istateid, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _, dwtextflags, dwtextflags2, prect).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeTextEx<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, psztext: &[u16], dwtextflags: super::super::Graphics::Gdi::DRAW_TEXT_FORMAT, prect: *mut super::super::Foundation::RECT, poptions: ::core::option::Option<*const DTTOPTS>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn DrawThemeTextEx ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , psztext : :: windows::core::PCWSTR , cchtext : i32 , dwtextflags : super::super::Graphics::Gdi:: DRAW_TEXT_FORMAT , prect : *mut super::super::Foundation:: RECT , poptions : *const DTTOPTS ) -> :: windows::core::HRESULT );
    DrawThemeTextEx(htheme.into(), hdc.into(), ipartid, istateid, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _, dwtextflags, prect, ::core::mem::transmute(poptions.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn EnableScrollBar<P0>(hwnd: P0, wsbflags: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, warrows: ENABLE_SCROLL_BAR_ARROWS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn EnableScrollBar ( hwnd : super::super::Foundation:: HWND , wsbflags : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , warrows : ENABLE_SCROLL_BAR_ARROWS ) -> super::super::Foundation:: BOOL );
    EnableScrollBar(hwnd.into(), wsbflags, warrows)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableThemeDialogTexture<P0>(hwnd: P0, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn EnableThemeDialogTexture ( hwnd : super::super::Foundation:: HWND , dwflags : u32 ) -> :: windows::core::HRESULT );
    EnableThemeDialogTexture(hwnd.into(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableTheming<P0>(fenable: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn EnableTheming ( fenable : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    EnableTheming(fenable.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndBufferedAnimation<P0>(hbpanimation: isize, fupdatetarget: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn EndBufferedAnimation ( hbpanimation : isize , fupdatetarget : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    EndBufferedAnimation(hbpanimation, fupdatetarget.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndBufferedPaint<P0>(hbufferedpaint: isize, fupdatetarget: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn EndBufferedPaint ( hbufferedpaint : isize , fupdatetarget : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    EndBufferedPaint(hbufferedpaint, fupdatetarget.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndPanningFeedback<P0, P1>(hwnd: P0, fanimateback: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn EndPanningFeedback ( hwnd : super::super::Foundation:: HWND , fanimateback : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    EndPanningFeedback(hwnd.into(), fanimateback.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvaluateProximityToPolygon(controlpolygon: &[super::super::Foundation::POINT], phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "user32.dll""system" fn EvaluateProximityToPolygon ( numvertices : u32 , controlpolygon : *const super::super::Foundation:: POINT , phittestinginput : *const TOUCH_HIT_TESTING_INPUT , pproximityeval : *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION ) -> super::super::Foundation:: BOOL );
    EvaluateProximityToPolygon(controlpolygon.len() as _, ::core::mem::transmute(controlpolygon.as_ptr()), phittestinginput, pproximityeval)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvaluateProximityToRect(controlboundingbox: *const super::super::Foundation::RECT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "user32.dll""system" fn EvaluateProximityToRect ( controlboundingbox : *const super::super::Foundation:: RECT , phittestinginput : *const TOUCH_HIT_TESTING_INPUT , pproximityeval : *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION ) -> super::super::Foundation:: BOOL );
    EvaluateProximityToRect(controlboundingbox, phittestinginput, pproximityeval)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlatSB_EnableScrollBar<P0>(param0: P0, param1: i32, param2: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn FlatSB_EnableScrollBar ( param0 : super::super::Foundation:: HWND , param1 : i32 , param2 : u32 ) -> super::super::Foundation:: BOOL );
    FlatSB_EnableScrollBar(param0.into(), param1, param2)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_GetScrollInfo<P0>(param0: P0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut super::WindowsAndMessaging::SCROLLINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn FlatSB_GetScrollInfo ( param0 : super::super::Foundation:: HWND , code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , param2 : *mut super::WindowsAndMessaging:: SCROLLINFO ) -> super::super::Foundation:: BOOL );
    FlatSB_GetScrollInfo(param0.into(), code, param2)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_GetScrollPos<P0>(param0: P0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn FlatSB_GetScrollPos ( param0 : super::super::Foundation:: HWND , code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS ) -> i32 );
    FlatSB_GetScrollPos(param0.into(), code)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlatSB_GetScrollProp<P0>(param0: P0, propindex: WSB_PROP, param2: *mut i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn FlatSB_GetScrollProp ( param0 : super::super::Foundation:: HWND , propindex : WSB_PROP , param2 : *mut i32 ) -> super::super::Foundation:: BOOL );
    FlatSB_GetScrollProp(param0.into(), propindex, param2)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_GetScrollRange<P0>(param0: P0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut i32, param3: *mut i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn FlatSB_GetScrollRange ( param0 : super::super::Foundation:: HWND , code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , param2 : *mut i32 , param3 : *mut i32 ) -> super::super::Foundation:: BOOL );
    FlatSB_GetScrollRange(param0.into(), code, param2, param3)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_SetScrollInfo<P0, P1>(param0: P0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, psi: *mut super::WindowsAndMessaging::SCROLLINFO, fredraw: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn FlatSB_SetScrollInfo ( param0 : super::super::Foundation:: HWND , code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , psi : *mut super::WindowsAndMessaging:: SCROLLINFO , fredraw : super::super::Foundation:: BOOL ) -> i32 );
    FlatSB_SetScrollInfo(param0.into(), code, psi, fredraw.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_SetScrollPos<P0, P1>(param0: P0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, pos: i32, fredraw: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn FlatSB_SetScrollPos ( param0 : super::super::Foundation:: HWND , code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , pos : i32 , fredraw : super::super::Foundation:: BOOL ) -> i32 );
    FlatSB_SetScrollPos(param0.into(), code, pos, fredraw.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlatSB_SetScrollProp<P0, P1>(param0: P0, index: WSB_PROP, newvalue: isize, param3: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn FlatSB_SetScrollProp ( param0 : super::super::Foundation:: HWND , index : WSB_PROP , newvalue : isize , param3 : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    FlatSB_SetScrollProp(param0.into(), index, newvalue, param3.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_SetScrollRange<P0, P1>(param0: P0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, min: i32, max: i32, fredraw: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn FlatSB_SetScrollRange ( param0 : super::super::Foundation:: HWND , code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , min : i32 , max : i32 , fredraw : super::super::Foundation:: BOOL ) -> i32 );
    FlatSB_SetScrollRange(param0.into(), code, min, max, fredraw.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_ShowScrollBar<P0, P1>(param0: P0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn FlatSB_ShowScrollBar ( param0 : super::super::Foundation:: HWND , code : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , param2 : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    FlatSB_ShowScrollBar(param0.into(), code, param2.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintBits(hbufferedpaint: isize, ppbbuffer: *mut *mut super::super::Graphics::Gdi::RGBQUAD, pcxrow: *mut i32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetBufferedPaintBits ( hbufferedpaint : isize , ppbbuffer : *mut *mut super::super::Graphics::Gdi:: RGBQUAD , pcxrow : *mut i32 ) -> :: windows::core::HRESULT );
    GetBufferedPaintBits(hbufferedpaint, ppbbuffer, pcxrow).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC {
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetBufferedPaintDC ( hbufferedpaint : isize ) -> super::super::Graphics::Gdi:: HDC );
    GetBufferedPaintDC(hbufferedpaint)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintTargetDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC {
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetBufferedPaintTargetDC ( hbufferedpaint : isize ) -> super::super::Graphics::Gdi:: HDC );
    GetBufferedPaintTargetDC(hbufferedpaint)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetBufferedPaintTargetRect(hbufferedpaint: isize) -> ::windows::core::Result<super::super::Foundation::RECT> {
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetBufferedPaintTargetRect ( hbufferedpaint : isize , prc : *mut super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetBufferedPaintTargetRect(hbufferedpaint, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComboBoxInfo<P0>(hwndcombo: P0, pcbi: *mut COMBOBOXINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn GetComboBoxInfo ( hwndcombo : super::super::Foundation:: HWND , pcbi : *mut COMBOBOXINFO ) -> super::super::Foundation:: BOOL );
    GetComboBoxInfo(hwndcombo.into(), pcbi)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetCurrentThemeName(pszthemefilename: &mut [u16], pszcolorbuff: ::core::option::Option<&mut [u16]>, pszsizebuff: ::core::option::Option<&mut [u16]>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetCurrentThemeName ( pszthemefilename : :: windows::core::PWSTR , cchmaxnamechars : i32 , pszcolorbuff : :: windows::core::PWSTR , cchmaxcolorchars : i32 , pszsizebuff : :: windows::core::PWSTR , cchmaxsizechars : i32 ) -> :: windows::core::HRESULT );
    GetCurrentThemeName(::core::mem::transmute(pszthemefilename.as_ptr()), pszthemefilename.len() as _, ::core::mem::transmute(pszcolorbuff.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszcolorbuff.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pszsizebuff.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszsizebuff.as_deref().map_or(0, |slice| slice.len() as _)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEffectiveClientRect<P0>(hwnd: P0, lprc: *mut super::super::Foundation::RECT, lpinfo: *const i32)
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn GetEffectiveClientRect ( hwnd : super::super::Foundation:: HWND , lprc : *mut super::super::Foundation:: RECT , lpinfo : *const i32 ) -> ( ) );
    GetEffectiveClientRect(hwnd.into(), lprc, lpinfo)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetListBoxInfo<P0>(hwnd: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn GetListBoxInfo ( hwnd : super::super::Foundation:: HWND ) -> u32 );
    GetListBoxInfo(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetMUILanguage() -> u16 {
    ::windows::core::link ! ( "comctl32.dll""system" fn GetMUILanguage ( ) -> u16 );
    GetMUILanguage()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeAnimationProperty<P0>(htheme: P0, istoryboardid: i32, itargetid: i32, eproperty: TA_PROPERTY, pvproperty: ::core::option::Option<*mut ::core::ffi::c_void>, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeAnimationProperty ( htheme : HTHEME , istoryboardid : i32 , itargetid : i32 , eproperty : TA_PROPERTY , pvproperty : *mut ::core::ffi::c_void , cbsize : u32 , pcbsizeout : *mut u32 ) -> :: windows::core::HRESULT );
    GetThemeAnimationProperty(htheme.into(), istoryboardid, itargetid, eproperty, ::core::mem::transmute(pvproperty.unwrap_or(::std::ptr::null_mut())), cbsize, pcbsizeout).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeAnimationTransform<P0>(htheme: P0, istoryboardid: i32, itargetid: i32, dwtransformindex: u32, ptransform: ::core::option::Option<*mut TA_TRANSFORM>, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeAnimationTransform ( htheme : HTHEME , istoryboardid : i32 , itargetid : i32 , dwtransformindex : u32 , ptransform : *mut TA_TRANSFORM , cbsize : u32 , pcbsizeout : *mut u32 ) -> :: windows::core::HRESULT );
    GetThemeAnimationTransform(htheme.into(), istoryboardid, itargetid, dwtransformindex, ::core::mem::transmute(ptransform.unwrap_or(::std::ptr::null_mut())), cbsize, pcbsizeout).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeAppProperties() -> SET_THEME_APP_PROPERTIES_FLAGS {
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeAppProperties ( ) -> SET_THEME_APP_PROPERTIES_FLAGS );
    GetThemeAppProperties()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemeBackgroundContentRect<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, pboundingrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::RECT>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeBackgroundContentRect ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , pboundingrect : *const super::super::Foundation:: RECT , pcontentrect : *mut super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeBackgroundContentRect(htheme.into(), hdc.into(), ipartid, istateid, pboundingrect, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemeBackgroundExtent<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, pcontentrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::RECT>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeBackgroundExtent ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , pcontentrect : *const super::super::Foundation:: RECT , pextentrect : *mut super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeBackgroundExtent(htheme.into(), hdc.into(), ipartid, istateid, pcontentrect, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemeBackgroundRegion<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Graphics::Gdi::HRGN>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeBackgroundRegion ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , prect : *const super::super::Foundation:: RECT , pregion : *mut super::super::Graphics::Gdi:: HRGN ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeBackgroundRegion(htheme.into(), hdc.into(), ipartid, istateid, prect, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeBitmap<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, dwflags: GET_THEME_BITMAP_FLAGS) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeBitmap ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : THEME_PROPERTY_SYMBOL_ID , dwflags : GET_THEME_BITMAP_FLAGS , phbitmap : *mut super::super::Graphics::Gdi:: HBITMAP ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeBitmap(htheme.into(), ipartid, istateid, ipropid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeBool<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> ::windows::core::Result<super::super::Foundation::BOOL>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeBool ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : THEME_PROPERTY_SYMBOL_ID , pfval : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeBool(htheme.into(), ipartid, istateid, ipropid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeColor<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> ::windows::core::Result<super::super::Foundation::COLORREF>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeColor ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : THEME_PROPERTY_SYMBOL_ID , pcolor : *mut super::super::Foundation:: COLORREF ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeColor(htheme.into(), ipartid, istateid, ipropid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeDocumentationProperty<P0, P1>(pszthemename: P0, pszpropertyname: P1, pszvaluebuff: &mut [u16]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeDocumentationProperty ( pszthemename : :: windows::core::PCWSTR , pszpropertyname : :: windows::core::PCWSTR , pszvaluebuff : :: windows::core::PWSTR , cchmaxvalchars : i32 ) -> :: windows::core::HRESULT );
    GetThemeDocumentationProperty(pszthemename.into().abi(), pszpropertyname.into().abi(), ::core::mem::transmute(pszvaluebuff.as_ptr()), pszvaluebuff.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeEnumValue<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> ::windows::core::Result<i32>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeEnumValue ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : THEME_PROPERTY_SYMBOL_ID , pival : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeEnumValue(htheme.into(), ipartid, istateid, ipropid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeFilename<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pszthemefilename: &mut [u16]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeFilename ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : THEME_PROPERTY_SYMBOL_ID , pszthemefilename : :: windows::core::PWSTR , cchmaxbuffchars : i32 ) -> :: windows::core::HRESULT );
    GetThemeFilename(htheme.into(), ipartid, istateid, ipropid, ::core::mem::transmute(pszthemefilename.as_ptr()), pszthemefilename.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeFont<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, ipropid: i32, pfont: *mut super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeFont ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , ipropid : i32 , pfont : *mut super::super::Graphics::Gdi:: LOGFONTW ) -> :: windows::core::HRESULT );
    GetThemeFont(htheme.into(), hdc.into(), ipartid, istateid, ipropid, pfont).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeInt<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> ::windows::core::Result<i32>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeInt ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : THEME_PROPERTY_SYMBOL_ID , pival : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeInt(htheme.into(), ipartid, istateid, ipropid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeIntList<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pintlist: *mut INTLIST) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeIntList ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : THEME_PROPERTY_SYMBOL_ID , pintlist : *mut INTLIST ) -> :: windows::core::HRESULT );
    GetThemeIntList(htheme.into(), ipartid, istateid, ipropid, pintlist).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemeMargins<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, prc: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<MARGINS>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeMargins ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , ipropid : THEME_PROPERTY_SYMBOL_ID , prc : *const super::super::Foundation:: RECT , pmargins : *mut MARGINS ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeMargins(htheme.into(), hdc.into(), ipartid, istateid, ipropid, ::core::mem::transmute(prc.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeMetric<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> ::windows::core::Result<i32>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeMetric ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , ipropid : THEME_PROPERTY_SYMBOL_ID , pival : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeMetric(htheme.into(), hdc.into(), ipartid, istateid, ipropid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemePartSize<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, prc: ::core::option::Option<*const super::super::Foundation::RECT>, esize: THEMESIZE) -> ::windows::core::Result<super::super::Foundation::SIZE>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemePartSize ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , prc : *const super::super::Foundation:: RECT , esize : THEMESIZE , psz : *mut super::super::Foundation:: SIZE ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemePartSize(htheme.into(), hdc.into(), ipartid, istateid, ::core::mem::transmute(prc.unwrap_or(::std::ptr::null())), esize, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemePosition<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> ::windows::core::Result<super::super::Foundation::POINT>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemePosition ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : THEME_PROPERTY_SYMBOL_ID , ppoint : *mut super::super::Foundation:: POINT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemePosition(htheme.into(), ipartid, istateid, ipropid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemePropertyOrigin<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows::core::Result<PROPERTYORIGIN>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemePropertyOrigin ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : i32 , porigin : *mut PROPERTYORIGIN ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemePropertyOrigin(htheme.into(), ipartid, istateid, ipropid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeRect<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows::core::Result<super::super::Foundation::RECT>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeRect ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : i32 , prect : *mut super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeRect(htheme.into(), ipartid, istateid, ipropid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeStream<P0, P1>(htheme: P0, ipartid: i32, istateid: i32, ipropid: i32, ppvstream: *mut *mut ::core::ffi::c_void, pcbstream: ::core::option::Option<*mut u32>, hinst: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeStream ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : i32 , ppvstream : *mut *mut ::core::ffi::c_void , pcbstream : *mut u32 , hinst : super::super::Foundation:: HINSTANCE ) -> :: windows::core::HRESULT );
    GetThemeStream(htheme.into(), ipartid, istateid, ipropid, ppvstream, ::core::mem::transmute(pcbstream.unwrap_or(::std::ptr::null_mut())), hinst.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeString<P0>(htheme: P0, ipartid: i32, istateid: i32, ipropid: i32, pszbuff: &mut [u16]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeString ( htheme : HTHEME , ipartid : i32 , istateid : i32 , ipropid : i32 , pszbuff : :: windows::core::PWSTR , cchmaxbuffchars : i32 ) -> :: windows::core::HRESULT );
    GetThemeString(htheme.into(), ipartid, istateid, ipropid, ::core::mem::transmute(pszbuff.as_ptr()), pszbuff.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeSysBool<P0>(htheme: P0, iboolid: THEME_PROPERTY_SYMBOL_ID) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeSysBool ( htheme : HTHEME , iboolid : THEME_PROPERTY_SYMBOL_ID ) -> super::super::Foundation:: BOOL );
    GetThemeSysBool(htheme.into(), iboolid)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeSysColor<P0>(htheme: P0, icolorid: i32) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeSysColor ( htheme : HTHEME , icolorid : i32 ) -> super::super::Foundation:: COLORREF );
    GetThemeSysColor(htheme.into(), icolorid)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeSysColorBrush<P0>(htheme: P0, icolorid: THEME_PROPERTY_SYMBOL_ID) -> super::super::Graphics::Gdi::HBRUSH
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeSysColorBrush ( htheme : HTHEME , icolorid : THEME_PROPERTY_SYMBOL_ID ) -> super::super::Graphics::Gdi:: HBRUSH );
    GetThemeSysColorBrush(htheme.into(), icolorid)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeSysFont<P0>(htheme: P0, ifontid: THEME_PROPERTY_SYMBOL_ID, plf: *mut super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeSysFont ( htheme : HTHEME , ifontid : THEME_PROPERTY_SYMBOL_ID , plf : *mut super::super::Graphics::Gdi:: LOGFONTW ) -> :: windows::core::HRESULT );
    GetThemeSysFont(htheme.into(), ifontid, plf).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeSysInt<P0>(htheme: P0, iintid: THEME_PROPERTY_SYMBOL_ID) -> ::windows::core::Result<i32>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeSysInt ( htheme : HTHEME , iintid : THEME_PROPERTY_SYMBOL_ID , pivalue : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeSysInt(htheme.into(), iintid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeSysSize<P0>(htheme: P0, isizeid: i32) -> i32
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeSysSize ( htheme : HTHEME , isizeid : i32 ) -> i32 );
    GetThemeSysSize(htheme.into(), isizeid)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeSysString<P0>(htheme: P0, istringid: THEME_PROPERTY_SYMBOL_ID, pszstringbuff: &mut [u16]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeSysString ( htheme : HTHEME , istringid : THEME_PROPERTY_SYMBOL_ID , pszstringbuff : :: windows::core::PWSTR , cchmaxstringchars : i32 ) -> :: windows::core::HRESULT );
    GetThemeSysString(htheme.into(), istringid, ::core::mem::transmute(pszstringbuff.as_ptr()), pszstringbuff.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemeTextExtent<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, psztext: &[u16], dwtextflags: super::super::Graphics::Gdi::DRAW_TEXT_FORMAT, pboundingrect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<super::super::Foundation::RECT>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeTextExtent ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , psztext : :: windows::core::PCWSTR , cchcharcount : i32 , dwtextflags : super::super::Graphics::Gdi:: DRAW_TEXT_FORMAT , pboundingrect : *const super::super::Foundation:: RECT , pextentrect : *mut super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeTextExtent(htheme.into(), hdc.into(), ipartid, istateid, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _, dwtextflags, ::core::mem::transmute(pboundingrect.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeTextMetrics<P0, P1>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeTextMetrics ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , ptm : *mut super::super::Graphics::Gdi:: TEXTMETRICW ) -> :: windows::core::HRESULT );
    GetThemeTextMetrics(htheme.into(), hdc.into(), ipartid, istateid, ptm).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeTimingFunction<P0>(htheme: P0, itimingfunctionid: i32, ptimingfunction: ::core::option::Option<*mut TA_TIMINGFUNCTION>, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeTimingFunction ( htheme : HTHEME , itimingfunctionid : i32 , ptimingfunction : *mut TA_TIMINGFUNCTION , cbsize : u32 , pcbsizeout : *mut u32 ) -> :: windows::core::HRESULT );
    GetThemeTimingFunction(htheme.into(), itimingfunctionid, ::core::mem::transmute(ptimingfunction.unwrap_or(::std::ptr::null_mut())), cbsize, pcbsizeout).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn GetThemeTransitionDuration<P0>(htheme: P0, ipartid: i32, istateidfrom: i32, istateidto: i32, ipropid: i32) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetThemeTransitionDuration ( htheme : HTHEME , ipartid : i32 , istateidfrom : i32 , istateidto : i32 , ipropid : i32 , pdwduration : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetThemeTransitionDuration(htheme.into(), ipartid, istateidfrom, istateidto, ipropid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowFeedbackSetting<P0>(hwnd: P0, feedback: FEEDBACK_TYPE, dwflags: u32, psize: *mut u32, config: ::core::option::Option<*mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn GetWindowFeedbackSetting ( hwnd : super::super::Foundation:: HWND , feedback : FEEDBACK_TYPE , dwflags : u32 , psize : *mut u32 , config : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetWindowFeedbackSetting(hwnd.into(), feedback, dwflags, psize, ::core::mem::transmute(config.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowTheme<P0>(hwnd: P0) -> HTHEME
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn GetWindowTheme ( hwnd : super::super::Foundation:: HWND ) -> HTHEME );
    GetWindowTheme(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn HIMAGELIST_QueryInterface<P0>(himl: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn HIMAGELIST_QueryInterface ( himl : HIMAGELIST , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HIMAGELIST_QueryInterface(himl.into(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn HitTestThemeBackground<P0, P1, P2>(htheme: P0, hdc: P1, ipartid: i32, istateid: i32, dwoptions: HIT_TEST_BACKGROUND_OPTIONS, prect: *const super::super::Foundation::RECT, hrgn: P2, pttest: super::super::Foundation::POINT) -> ::windows::core::Result<u16>
where
    P0: ::std::convert::Into<HTHEME>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    P2: ::std::convert::Into<super::super::Graphics::Gdi::HRGN>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn HitTestThemeBackground ( htheme : HTHEME , hdc : super::super::Graphics::Gdi:: HDC , ipartid : i32 , istateid : i32 , dwoptions : HIT_TEST_BACKGROUND_OPTIONS , prect : *const super::super::Foundation:: RECT , hrgn : super::super::Graphics::Gdi:: HRGN , pttest : super::super::Foundation:: POINT , pwhittestcode : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HitTestThemeBackground(htheme.into(), hdc.into(), ipartid, istateid, dwoptions, prect, hrgn.into(), ::core::mem::transmute(pttest), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_Add<P0, P1, P2>(himl: P0, hbmimage: P1, hbmmask: P2) -> i32
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
    P2: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Add ( himl : HIMAGELIST , hbmimage : super::super::Graphics::Gdi:: HBITMAP , hbmmask : super::super::Graphics::Gdi:: HBITMAP ) -> i32 );
    ImageList_Add(himl.into(), hbmimage.into(), hbmmask.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_AddMasked<P0, P1, P2>(himl: P0, hbmimage: P1, crmask: P2) -> i32
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
    P2: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_AddMasked ( himl : HIMAGELIST , hbmimage : super::super::Graphics::Gdi:: HBITMAP , crmask : super::super::Foundation:: COLORREF ) -> i32 );
    ImageList_AddMasked(himl.into(), hbmimage.into(), crmask.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_BeginDrag<P0>(himltrack: P0, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_BeginDrag ( himltrack : HIMAGELIST , itrack : i32 , dxhotspot : i32 , dyhotspot : i32 ) -> super::super::Foundation:: BOOL );
    ImageList_BeginDrag(himltrack.into(), itrack, dxhotspot, dyhotspot)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn ImageList_CoCreateInstance<P0, T>(rclsid: *const ::windows::core::GUID, punkouter: P0) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_CoCreateInstance ( rclsid : *const :: windows::core::GUID , punkouter : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    ImageList_CoCreateInstance(rclsid, punkouter.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_Copy<P0, P1>(himldst: P0, idst: i32, himlsrc: P1, isrc: i32, uflags: IMAGE_LIST_COPY_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Copy ( himldst : HIMAGELIST , idst : i32 , himlsrc : HIMAGELIST , isrc : i32 , uflags : IMAGE_LIST_COPY_FLAGS ) -> super::super::Foundation:: BOOL );
    ImageList_Copy(himldst.into(), idst, himlsrc.into(), isrc, uflags)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn ImageList_Create(cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> HIMAGELIST {
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Create ( cx : i32 , cy : i32 , flags : IMAGELIST_CREATION_FLAGS , cinitial : i32 , cgrow : i32 ) -> HIMAGELIST );
    ImageList_Create(cx, cy, flags, cinitial, cgrow)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_Destroy<P0>(himl: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Destroy ( himl : HIMAGELIST ) -> super::super::Foundation:: BOOL );
    ImageList_Destroy(himl.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_DragEnter<P0>(hwndlock: P0, x: i32, y: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_DragEnter ( hwndlock : super::super::Foundation:: HWND , x : i32 , y : i32 ) -> super::super::Foundation:: BOOL );
    ImageList_DragEnter(hwndlock.into(), x, y)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_DragLeave<P0>(hwndlock: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_DragLeave ( hwndlock : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    ImageList_DragLeave(hwndlock.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_DragMove(x: i32, y: i32) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_DragMove ( x : i32 , y : i32 ) -> super::super::Foundation:: BOOL );
    ImageList_DragMove(x, y)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_DragShowNolock<P0>(fshow: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_DragShowNolock ( fshow : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    ImageList_DragShowNolock(fshow.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_Draw<P0, P1>(himl: P0, i: i32, hdcdst: P1, x: i32, y: i32, fstyle: IMAGE_LIST_DRAW_STYLE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Draw ( himl : HIMAGELIST , i : i32 , hdcdst : super::super::Graphics::Gdi:: HDC , x : i32 , y : i32 , fstyle : IMAGE_LIST_DRAW_STYLE ) -> super::super::Foundation:: BOOL );
    ImageList_Draw(himl.into(), i, hdcdst.into(), x, y, fstyle)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_DrawEx<P0, P1, P2, P3>(himl: P0, i: i32, hdcdst: P1, x: i32, y: i32, dx: i32, dy: i32, rgbbk: P2, rgbfg: P3, fstyle: IMAGE_LIST_DRAW_STYLE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    P2: ::std::convert::Into<super::super::Foundation::COLORREF>,
    P3: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_DrawEx ( himl : HIMAGELIST , i : i32 , hdcdst : super::super::Graphics::Gdi:: HDC , x : i32 , y : i32 , dx : i32 , dy : i32 , rgbbk : super::super::Foundation:: COLORREF , rgbfg : super::super::Foundation:: COLORREF , fstyle : IMAGE_LIST_DRAW_STYLE ) -> super::super::Foundation:: BOOL );
    ImageList_DrawEx(himl.into(), i, hdcdst.into(), x, y, dx, dy, rgbbk.into(), rgbfg.into(), fstyle)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_DrawIndirect(pimldp: *const IMAGELISTDRAWPARAMS) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_DrawIndirect ( pimldp : *const IMAGELISTDRAWPARAMS ) -> super::super::Foundation:: BOOL );
    ImageList_DrawIndirect(pimldp)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn ImageList_Duplicate<P0>(himl: P0) -> HIMAGELIST
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Duplicate ( himl : HIMAGELIST ) -> HIMAGELIST );
    ImageList_Duplicate(himl.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn ImageList_EndDrag() {
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_EndDrag ( ) -> ( ) );
    ImageList_EndDrag()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_GetBkColor<P0>(himl: P0) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_GetBkColor ( himl : HIMAGELIST ) -> super::super::Foundation:: COLORREF );
    ImageList_GetBkColor(himl.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_GetDragImage(ppt: ::core::option::Option<*mut super::super::Foundation::POINT>, ppthotspot: ::core::option::Option<*mut super::super::Foundation::POINT>) -> HIMAGELIST {
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_GetDragImage ( ppt : *mut super::super::Foundation:: POINT , ppthotspot : *mut super::super::Foundation:: POINT ) -> HIMAGELIST );
    ImageList_GetDragImage(::core::mem::transmute(ppt.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppthotspot.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_GetIcon<P0>(himl: P0, i: i32, flags: u32) -> super::WindowsAndMessaging::HICON
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_GetIcon ( himl : HIMAGELIST , i : i32 , flags : u32 ) -> super::WindowsAndMessaging:: HICON );
    ImageList_GetIcon(himl.into(), i, flags)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_GetIconSize<P0>(himl: P0, cx: ::core::option::Option<*mut i32>, cy: ::core::option::Option<*mut i32>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_GetIconSize ( himl : HIMAGELIST , cx : *mut i32 , cy : *mut i32 ) -> super::super::Foundation:: BOOL );
    ImageList_GetIconSize(himl.into(), ::core::mem::transmute(cx.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(cy.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn ImageList_GetImageCount<P0>(himl: P0) -> i32
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_GetImageCount ( himl : HIMAGELIST ) -> i32 );
    ImageList_GetImageCount(himl.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_GetImageInfo<P0>(himl: P0, i: i32, pimageinfo: *mut IMAGEINFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_GetImageInfo ( himl : HIMAGELIST , i : i32 , pimageinfo : *mut IMAGEINFO ) -> super::super::Foundation:: BOOL );
    ImageList_GetImageInfo(himl.into(), i, pimageinfo)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn ImageList_LoadImageA<P0, P1, P2>(hi: P0, lpbmp: P1, cx: i32, cgrow: i32, crmask: P2, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_LoadImageA ( hi : super::super::Foundation:: HINSTANCE , lpbmp : :: windows::core::PCSTR , cx : i32 , cgrow : i32 , crmask : super::super::Foundation:: COLORREF , utype : u32 , uflags : super::WindowsAndMessaging:: IMAGE_FLAGS ) -> HIMAGELIST );
    ImageList_LoadImageA(hi.into(), lpbmp.into().abi(), cx, cgrow, crmask.into(), utype, uflags)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn ImageList_LoadImageW<P0, P1, P2>(hi: P0, lpbmp: P1, cx: i32, cgrow: i32, crmask: P2, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_LoadImageW ( hi : super::super::Foundation:: HINSTANCE , lpbmp : :: windows::core::PCWSTR , cx : i32 , cgrow : i32 , crmask : super::super::Foundation:: COLORREF , utype : u32 , uflags : super::WindowsAndMessaging:: IMAGE_FLAGS ) -> HIMAGELIST );
    ImageList_LoadImageW(hi.into(), lpbmp.into().abi(), cx, cgrow, crmask.into(), utype, uflags)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn ImageList_Merge<P0, P1>(himl1: P0, i1: i32, himl2: P1, i2: i32, dx: i32, dy: i32) -> HIMAGELIST
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Merge ( himl1 : HIMAGELIST , i1 : i32 , himl2 : HIMAGELIST , i2 : i32 , dx : i32 , dy : i32 ) -> HIMAGELIST );
    ImageList_Merge(himl1.into(), i1, himl2.into(), i2, dx, dy)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_Read<P0>(pstm: P0) -> HIMAGELIST
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Read ( pstm : * mut::core::ffi::c_void ) -> HIMAGELIST );
    ImageList_Read(pstm.into().abi())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_ReadEx<P0>(dwflags: u32, pstm: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_ReadEx ( dwflags : u32 , pstm : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    ImageList_ReadEx(dwflags, pstm.into().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_Remove<P0>(himl: P0, i: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Remove ( himl : HIMAGELIST , i : i32 ) -> super::super::Foundation:: BOOL );
    ImageList_Remove(himl.into(), i)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_Replace<P0, P1, P2>(himl: P0, i: i32, hbmimage: P1, hbmmask: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
    P2: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Replace ( himl : HIMAGELIST , i : i32 , hbmimage : super::super::Graphics::Gdi:: HBITMAP , hbmmask : super::super::Graphics::Gdi:: HBITMAP ) -> super::super::Foundation:: BOOL );
    ImageList_Replace(himl.into(), i, hbmimage.into(), hbmmask.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_ReplaceIcon<P0, P1>(himl: P0, i: i32, hicon: P1) -> i32
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<super::WindowsAndMessaging::HICON>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_ReplaceIcon ( himl : HIMAGELIST , i : i32 , hicon : super::WindowsAndMessaging:: HICON ) -> i32 );
    ImageList_ReplaceIcon(himl.into(), i, hicon.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_SetBkColor<P0, P1>(himl: P0, clrbk: P1) -> super::super::Foundation::COLORREF
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_SetBkColor ( himl : HIMAGELIST , clrbk : super::super::Foundation:: COLORREF ) -> super::super::Foundation:: COLORREF );
    ImageList_SetBkColor(himl.into(), clrbk.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_SetDragCursorImage<P0>(himldrag: P0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_SetDragCursorImage ( himldrag : HIMAGELIST , idrag : i32 , dxhotspot : i32 , dyhotspot : i32 ) -> super::super::Foundation:: BOOL );
    ImageList_SetDragCursorImage(himldrag.into(), idrag, dxhotspot, dyhotspot)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_SetIconSize<P0>(himl: P0, cx: i32, cy: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_SetIconSize ( himl : HIMAGELIST , cx : i32 , cy : i32 ) -> super::super::Foundation:: BOOL );
    ImageList_SetIconSize(himl.into(), cx, cy)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_SetImageCount<P0>(himl: P0, unewcount: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_SetImageCount ( himl : HIMAGELIST , unewcount : u32 ) -> super::super::Foundation:: BOOL );
    ImageList_SetImageCount(himl.into(), unewcount)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_SetOverlayImage<P0>(himl: P0, iimage: i32, ioverlay: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_SetOverlayImage ( himl : HIMAGELIST , iimage : i32 , ioverlay : i32 ) -> super::super::Foundation:: BOOL );
    ImageList_SetOverlayImage(himl.into(), iimage, ioverlay)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ImageList_Write<P0, P1>(himl: P0, pstm: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_Write ( himl : HIMAGELIST , pstm : * mut::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ImageList_Write(himl.into(), pstm.into().abi())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_WriteEx<P0, P1>(himl: P0, dwflags: IMAGE_LIST_WRITE_STREAM_FLAGS, pstm: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HIMAGELIST>,
    P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ImageList_WriteEx ( himl : HIMAGELIST , dwflags : IMAGE_LIST_WRITE_STREAM_FLAGS , pstm : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    ImageList_WriteEx(himl.into(), dwflags, pstm.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn InitCommonControls() {
    ::windows::core::link ! ( "comctl32.dll""system" fn InitCommonControls ( ) -> ( ) );
    InitCommonControls()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "comctl32.dll""system" fn InitCommonControlsEx ( picce : *const INITCOMMONCONTROLSEX ) -> super::super::Foundation:: BOOL );
    InitCommonControlsEx(picce)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn InitMUILanguage(uilang: u16) {
    ::windows::core::link ! ( "comctl32.dll""system" fn InitMUILanguage ( uilang : u16 ) -> ( ) );
    InitMUILanguage(uilang)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeFlatSB<P0>(param0: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn InitializeFlatSB ( param0 : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    InitializeFlatSB(param0.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsAppThemed() -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "uxtheme.dll""system" fn IsAppThemed ( ) -> super::super::Foundation:: BOOL );
    IsAppThemed()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCharLowerW(ch: u16) -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "user32.dll""system" fn IsCharLowerW ( ch : u16 ) -> super::super::Foundation:: BOOL );
    IsCharLowerW(ch)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCompositionActive() -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "uxtheme.dll""system" fn IsCompositionActive ( ) -> super::super::Foundation:: BOOL );
    IsCompositionActive()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDlgButtonChecked<P0>(hdlg: P0, nidbutton: i32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn IsDlgButtonChecked ( hdlg : super::super::Foundation:: HWND , nidbutton : i32 ) -> u32 );
    IsDlgButtonChecked(hdlg.into(), nidbutton)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThemeActive() -> super::super::Foundation::BOOL {
    ::windows::core::link ! ( "uxtheme.dll""system" fn IsThemeActive ( ) -> super::super::Foundation:: BOOL );
    IsThemeActive()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThemeBackgroundPartiallyTransparent<P0>(htheme: P0, ipartid: i32, istateid: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn IsThemeBackgroundPartiallyTransparent ( htheme : HTHEME , ipartid : i32 , istateid : i32 ) -> super::super::Foundation:: BOOL );
    IsThemeBackgroundPartiallyTransparent(htheme.into(), ipartid, istateid)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThemeDialogTextureEnabled<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn IsThemeDialogTextureEnabled ( hwnd : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    IsThemeDialogTextureEnabled(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThemePartDefined<P0>(htheme: P0, ipartid: i32, istateid: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HTHEME>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn IsThemePartDefined ( htheme : HTHEME , ipartid : i32 , istateid : i32 ) -> super::super::Foundation:: BOOL );
    IsThemePartDefined(htheme.into(), ipartid, istateid)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LBItemFromPt<P0, P1>(hlb: P0, pt: super::super::Foundation::POINT, bautoscroll: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn LBItemFromPt ( hlb : super::super::Foundation:: HWND , pt : super::super::Foundation:: POINT , bautoscroll : super::super::Foundation:: BOOL ) -> i32 );
    LBItemFromPt(hlb.into(), ::core::mem::transmute(pt), bautoscroll.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn LoadIconMetric<P0, P1>(hinst: P0, pszname: P1, lims: _LI_METRIC) -> ::windows::core::Result<super::WindowsAndMessaging::HICON>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn LoadIconMetric ( hinst : super::super::Foundation:: HINSTANCE , pszname : :: windows::core::PCWSTR , lims : _LI_METRIC , phico : *mut super::WindowsAndMessaging:: HICON ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    LoadIconMetric(hinst.into(), pszname.into().abi(), lims, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn LoadIconWithScaleDown<P0, P1>(hinst: P0, pszname: P1, cx: i32, cy: i32) -> ::windows::core::Result<super::WindowsAndMessaging::HICON>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn LoadIconWithScaleDown ( hinst : super::super::Foundation:: HINSTANCE , pszname : :: windows::core::PCWSTR , cx : i32 , cy : i32 , phico : *mut super::WindowsAndMessaging:: HICON ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    LoadIconWithScaleDown(hinst.into(), pszname.into().abi(), cx, cy, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MakeDragList<P0>(hlb: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn MakeDragList ( hlb : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    MakeDragList(hlb.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn MenuHelp<P0, P1, P2, P3, P4>(umsg: u32, wparam: P0, lparam: P1, hmainmenu: P2, hinst: P3, hwndstatus: P4, lpwids: *const u32)
where
    P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
    P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    P2: ::std::convert::Into<super::WindowsAndMessaging::HMENU>,
    P3: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P4: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn MenuHelp ( umsg : u32 , wparam : super::super::Foundation:: WPARAM , lparam : super::super::Foundation:: LPARAM , hmainmenu : super::WindowsAndMessaging:: HMENU , hinst : super::super::Foundation:: HINSTANCE , hwndstatus : super::super::Foundation:: HWND , lpwids : *const u32 ) -> ( ) );
    MenuHelp(umsg, wparam.into(), lparam.into(), hmainmenu.into(), hinst.into(), hwndstatus.into(), lpwids)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenThemeData<P0, P1>(hwnd: P0, pszclasslist: P1) -> HTHEME
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn OpenThemeData ( hwnd : super::super::Foundation:: HWND , pszclasslist : :: windows::core::PCWSTR ) -> HTHEME );
    OpenThemeData(hwnd.into(), pszclasslist.into().abi())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenThemeDataEx<P0, P1>(hwnd: P0, pszclasslist: P1, dwflags: OPEN_THEME_DATA_FLAGS) -> HTHEME
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn OpenThemeDataEx ( hwnd : super::super::Foundation:: HWND , pszclasslist : :: windows::core::PCWSTR , dwflags : OPEN_THEME_DATA_FLAGS ) -> HTHEME );
    OpenThemeDataEx(hwnd.into(), pszclasslist.into().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackTouchHitTestingProximityEvaluation(phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::LRESULT {
    ::windows::core::link ! ( "user32.dll""system" fn PackTouchHitTestingProximityEvaluation ( phittestinginput : *const TOUCH_HIT_TESTING_INPUT , pproximityeval : *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION ) -> super::super::Foundation:: LRESULT );
    PackTouchHitTestingProximityEvaluation(phittestinginput, pproximityeval)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn PropertySheetA(param0: *mut PROPSHEETHEADERA_V2) -> isize {
    ::windows::core::link ! ( "comctl32.dll""system" fn PropertySheetA ( param0 : *mut PROPSHEETHEADERA_V2 ) -> isize );
    PropertySheetA(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn PropertySheetW(param0: *mut PROPSHEETHEADERW_V2) -> isize {
    ::windows::core::link ! ( "comctl32.dll""system" fn PropertySheetW ( param0 : *mut PROPSHEETHEADERW_V2 ) -> isize );
    PropertySheetW(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterPointerDeviceNotifications<P0, P1>(window: P0, notifyrange: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "user32.dll""system" fn RegisterPointerDeviceNotifications ( window : super::super::Foundation:: HWND , notifyrange : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    RegisterPointerDeviceNotifications(window.into(), notifyrange.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTouchHitTestingWindow<P0>(hwnd: P0, value: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn RegisterTouchHitTestingWindow ( hwnd : super::super::Foundation:: HWND , value : u32 ) -> super::super::Foundation:: BOOL );
    RegisterTouchHitTestingWindow(hwnd.into(), value)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetScrollInfo<P0, P1>(hwnd: P0, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, lpsi: *const super::WindowsAndMessaging::SCROLLINFO, redraw: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "user32.dll""system" fn SetScrollInfo ( hwnd : super::super::Foundation:: HWND , nbar : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , lpsi : *const super::WindowsAndMessaging:: SCROLLINFO , redraw : super::super::Foundation:: BOOL ) -> i32 );
    SetScrollInfo(hwnd.into(), nbar, lpsi, redraw.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetScrollPos<P0, P1>(hwnd: P0, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, npos: i32, bredraw: P1) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "user32.dll""system" fn SetScrollPos ( hwnd : super::super::Foundation:: HWND , nbar : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , npos : i32 , bredraw : super::super::Foundation:: BOOL ) -> i32 );
    SetScrollPos(hwnd.into(), nbar, npos, bredraw.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetScrollRange<P0, P1>(hwnd: P0, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, nminpos: i32, nmaxpos: i32, bredraw: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "user32.dll""system" fn SetScrollRange ( hwnd : super::super::Foundation:: HWND , nbar : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , nminpos : i32 , nmaxpos : i32 , bredraw : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    SetScrollRange(hwnd.into(), nbar, nminpos, nmaxpos, bredraw.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[inline]
pub unsafe fn SetThemeAppProperties(dwflags: SET_THEME_APP_PROPERTIES_FLAGS) {
    ::windows::core::link ! ( "uxtheme.dll""system" fn SetThemeAppProperties ( dwflags : SET_THEME_APP_PROPERTIES_FLAGS ) -> ( ) );
    SetThemeAppProperties(dwflags)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowFeedbackSetting<P0>(hwnd: P0, feedback: FEEDBACK_TYPE, dwflags: u32, size: u32, configuration: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "user32.dll""system" fn SetWindowFeedbackSetting ( hwnd : super::super::Foundation:: HWND , feedback : FEEDBACK_TYPE , dwflags : u32 , size : u32 , configuration : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetWindowFeedbackSetting(hwnd.into(), feedback, dwflags, size, ::core::mem::transmute(configuration.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowTheme<P0, P1, P2>(hwnd: P0, pszsubappname: P1, pszsubidlist: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn SetWindowTheme ( hwnd : super::super::Foundation:: HWND , pszsubappname : :: windows::core::PCWSTR , pszsubidlist : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    SetWindowTheme(hwnd.into(), pszsubappname.into().abi(), pszsubidlist.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowThemeAttribute<P0>(hwnd: P0, eattribute: WINDOWTHEMEATTRIBUTETYPE, pvattribute: *const ::core::ffi::c_void, cbattribute: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn SetWindowThemeAttribute ( hwnd : super::super::Foundation:: HWND , eattribute : WINDOWTHEMEATTRIBUTETYPE , pvattribute : *const ::core::ffi::c_void , cbattribute : u32 ) -> :: windows::core::HRESULT );
    SetWindowThemeAttribute(hwnd.into(), eattribute, pvattribute, cbattribute).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowHideMenuCtl<P0>(hwnd: P0, uflags: usize, lpinfo: *const i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn ShowHideMenuCtl ( hwnd : super::super::Foundation:: HWND , uflags : usize , lpinfo : *const i32 ) -> super::super::Foundation:: BOOL );
    ShowHideMenuCtl(hwnd.into(), uflags, lpinfo)
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn ShowScrollBar<P0, P1>(hwnd: P0, wbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, bshow: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "user32.dll""system" fn ShowScrollBar ( hwnd : super::super::Foundation:: HWND , wbar : super::WindowsAndMessaging:: SCROLLBAR_CONSTANTS , bshow : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    ShowScrollBar(hwnd.into(), wbar, bshow.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Str_SetPtrW<P0>(ppsz: *mut ::windows::core::PWSTR, psz: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn Str_SetPtrW ( ppsz : *mut :: windows::core::PWSTR , psz : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    Str_SetPtrW(ppsz, psz.into().abi())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TaskDialog<P0, P1, P2, P3, P4, P5>(hwndowner: P0, hinstance: P1, pszwindowtitle: P2, pszmaininstruction: P3, pszcontent: P4, dwcommonbuttons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon: P5, pnbutton: ::core::option::Option<*mut i32>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn TaskDialog ( hwndowner : super::super::Foundation:: HWND , hinstance : super::super::Foundation:: HINSTANCE , pszwindowtitle : :: windows::core::PCWSTR , pszmaininstruction : :: windows::core::PCWSTR , pszcontent : :: windows::core::PCWSTR , dwcommonbuttons : TASKDIALOG_COMMON_BUTTON_FLAGS , pszicon : :: windows::core::PCWSTR , pnbutton : *mut i32 ) -> :: windows::core::HRESULT );
    TaskDialog(hwndowner.into(), hinstance.into(), pszwindowtitle.into().abi(), pszmaininstruction.into().abi(), pszcontent.into().abi(), dwcommonbuttons, pszicon.into().abi(), ::core::mem::transmute(pnbutton.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn TaskDialogIndirect(ptaskconfig: *const TASKDIALOGCONFIG, pnbutton: ::core::option::Option<*mut i32>, pnradiobutton: ::core::option::Option<*mut i32>, pfverificationflagchecked: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "comctl32.dll""system" fn TaskDialogIndirect ( ptaskconfig : *const TASKDIALOGCONFIG , pnbutton : *mut i32 , pnradiobutton : *mut i32 , pfverificationflagchecked : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    TaskDialogIndirect(ptaskconfig, ::core::mem::transmute(pnbutton.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnradiobutton.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfverificationflagchecked.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UninitializeFlatSB<P0>(param0: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "comctl32.dll""system" fn UninitializeFlatSB ( param0 : super::super::Foundation:: HWND ) -> :: windows::core::HRESULT );
    UninitializeFlatSB(param0.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdatePanningFeedback<P0, P1>(hwnd: P0, ltotaloverpanoffsetx: i32, ltotaloverpanoffsety: i32, fininertia: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "uxtheme.dll""system" fn UpdatePanningFeedback ( hwnd : super::super::Foundation:: HWND , ltotaloverpanoffsetx : i32 , ltotaloverpanoffsety : i32 , fininertia : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    UpdatePanningFeedback(hwnd.into(), ltotaloverpanoffsetx, ltotaloverpanoffsety, fininertia.into())
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
pub struct IImageList(::windows::core::IUnknown);
impl IImageList {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Add<P0, P1>(&self, hbmimage: P0, hbmmask: P1) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), hbmimage.into(), hbmmask.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn ReplaceIcon<P0>(&self, i: i32, hicon: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<super::WindowsAndMessaging::HICON>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReplaceIcon)(::windows::core::Vtable::as_raw(self), i, hicon.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOverlayImage)(::windows::core::Vtable::as_raw(self), iimage, ioverlay).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Replace<P0, P1>(&self, i: i32, hbmimage: P0, hbmmask: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
    {
        (::windows::core::Vtable::vtable(self).Replace)(::windows::core::Vtable::as_raw(self), i, hbmimage.into(), hbmmask.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn AddMasked<P0, P1>(&self, hbmimage: P0, crmask: P1) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
        P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddMasked)(::windows::core::Vtable::as_raw(self), hbmimage.into(), crmask.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Draw(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Draw)(::windows::core::Vtable::as_raw(self), pimldp).ok()
    }
    pub unsafe fn Remove(&self, i: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), i).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self, i: i32, flags: u32) -> ::windows::core::Result<super::WindowsAndMessaging::HICON> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIcon)(::windows::core::Vtable::as_raw(self), i, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImageInfo(&self, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetImageInfo)(::windows::core::Vtable::as_raw(self), i, pimageinfo).ok()
    }
    pub unsafe fn Copy<P0>(&self, idst: i32, punksrc: P0, isrc: i32, uflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).Copy)(::windows::core::Vtable::as_raw(self), idst, punksrc.into().abi(), isrc, uflags).ok()
    }
    pub unsafe fn Merge<P0>(&self, i1: i32, punk2: P0, i2: i32, dx: i32, dy: i32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).Merge)(::windows::core::Vtable::as_raw(self), i1, punk2.into().abi(), i2, dx, dy, riid, ppv).ok()
    }
    pub unsafe fn Clone(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), riid, ppv).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetImageRect(&self, i: i32) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImageRect)(::windows::core::Vtable::as_raw(self), i, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetIconSize)(::windows::core::Vtable::as_raw(self), cx, cy).ok()
    }
    pub unsafe fn SetIconSize(&self, cx: i32, cy: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetIconSize)(::windows::core::Vtable::as_raw(self), cx, cy).ok()
    }
    pub unsafe fn GetImageCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImageCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetImageCount(&self, unewcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetImageCount)(::windows::core::Vtable::as_raw(self), unewcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBkColor<P0>(&self, clrbk: P0) -> ::windows::core::Result<super::super::Foundation::COLORREF>
    where
        P0: ::std::convert::Into<super::super::Foundation::COLORREF>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SetBkColor)(::windows::core::Vtable::as_raw(self), clrbk.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBkColor(&self) -> ::windows::core::Result<super::super::Foundation::COLORREF> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBkColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).BeginDrag)(::windows::core::Vtable::as_raw(self), itrack, dxhotspot, dyhotspot).ok()
    }
    pub unsafe fn EndDrag(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndDrag)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DragEnter<P0>(&self, hwndlock: P0, x: i32, y: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).DragEnter)(::windows::core::Vtable::as_raw(self), hwndlock.into(), x, y).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DragLeave<P0>(&self, hwndlock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).DragLeave)(::windows::core::Vtable::as_raw(self), hwndlock.into()).ok()
    }
    pub unsafe fn DragMove(&self, x: i32, y: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DragMove)(::windows::core::Vtable::as_raw(self), x, y).ok()
    }
    pub unsafe fn SetDragCursorImage<P0>(&self, punk: P0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetDragCursorImage)(::windows::core::Vtable::as_raw(self), punk.into().abi(), idrag, dxhotspot, dyhotspot).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DragShowNolock<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).DragShowNolock)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDragImage(&self, ppt: ::core::option::Option<*mut super::super::Foundation::POINT>, ppthotspot: ::core::option::Option<*mut super::super::Foundation::POINT>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDragImage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppt.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppthotspot.unwrap_or(::std::ptr::null_mut())), riid, ppv).ok()
    }
    pub unsafe fn GetItemFlags(&self, i: i32) -> ::windows::core::Result<IMAGE_LIST_ITEM_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItemFlags)(::windows::core::Vtable::as_raw(self), i, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOverlayImage(&self, ioverlay: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOverlayImage)(::windows::core::Vtable::as_raw(self), ioverlay, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IImageList, ::windows::core::IUnknown);
impl ::core::clone::Clone for IImageList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IImageList {
    type Vtable = IImageList_Vtbl;
}
unsafe impl ::windows::core::Interface for IImageList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46eb5926_582e_4017_9fdf_e8998daa0950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, pi: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Add: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub ReplaceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, hicon: super::WindowsAndMessaging::HICON, pi: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    ReplaceIcon: usize,
    pub SetOverlayImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimage: i32, ioverlay: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Replace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Replace: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub AddMasked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: super::super::Foundation::COLORREF, pi: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    AddMasked: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    Draw: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, flags: u32, picon: *mut super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetImageInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetImageInfo: usize,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idst: i32, punksrc: *mut ::core::ffi::c_void, isrc: i32, uflags: u32) -> ::windows::core::HRESULT,
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i1: i32, punk2: *mut ::core::ffi::c_void, i2: i32, dx: i32, dy: i32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetImageRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetImageRect: usize,
    pub GetIconSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cx: *mut i32, cy: *mut i32) -> ::windows::core::HRESULT,
    pub SetIconSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cx: i32, cy: i32) -> ::windows::core::HRESULT,
    pub GetImageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pi: *mut i32) -> ::windows::core::HRESULT,
    pub SetImageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unewcount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBkColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clrbk: super::super::Foundation::COLORREF, pclr: *mut super::super::Foundation::COLORREF) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBkColor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBkColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclr: *mut super::super::Foundation::COLORREF) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBkColor: usize,
    pub BeginDrag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::HRESULT,
    pub EndDrag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DragEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DragEnter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DragLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndlock: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DragLeave: usize,
    pub DragMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT,
    pub SetDragCursorImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DragShowNolock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DragShowNolock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDragImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDragImage: usize,
    pub GetItemFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, dwflags: *mut IMAGE_LIST_ITEM_FLAGS) -> ::windows::core::HRESULT,
    pub GetOverlayImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ioverlay: i32, piindex: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
pub struct IImageList2(::windows::core::IUnknown);
impl IImageList2 {
    pub unsafe fn Resize(&self, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Resize)(::windows::core::Vtable::as_raw(self), cxnewiconsize, cynewiconsize).ok()
    }
    pub unsafe fn GetOriginalSize(&self, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetOriginalSize)(::windows::core::Vtable::as_raw(self), iimage, dwflags, pcx, pcy).ok()
    }
    pub unsafe fn SetOriginalSize(&self, iimage: i32, cx: i32, cy: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOriginalSize)(::windows::core::Vtable::as_raw(self), iimage, cx, cy).ok()
    }
    pub unsafe fn SetCallback<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetCallback)(::windows::core::Vtable::as_raw(self), punk.into().abi()).ok()
    }
    pub unsafe fn GetCallback(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCallback)(::windows::core::Vtable::as_raw(self), riid, ppv).ok()
    }
    pub unsafe fn ForceImagePresent(&self, iimage: i32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ForceImagePresent)(::windows::core::Vtable::as_raw(self), iimage, dwflags).ok()
    }
    pub unsafe fn DiscardImages(&self, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DiscardImages)(::windows::core::Vtable::as_raw(self), ifirstimage, ilastimage, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn PreloadImages(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PreloadImages)(::windows::core::Vtable::as_raw(self), pimldp).ok()
    }
    pub unsafe fn GetStatistics(&self, pils: *mut IMAGELISTSTATS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetStatistics)(::windows::core::Vtable::as_raw(self), pils).ok()
    }
    pub unsafe fn Initialize(&self, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), cx, cy, flags, cinitial, cgrow).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Replace2<P0, P1, P2>(&self, i: i32, hbmimage: P0, hbmmask: P1, punk: P2, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).Replace2)(::windows::core::Vtable::as_raw(self), i, hbmimage.into(), hbmmask.into(), punk.into().abi(), dwflags).ok()
    }
    pub unsafe fn ReplaceFromImageList<P0, P1>(&self, i: i32, pil: P0, isrc: i32, punk: P1, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IImageList>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).ReplaceFromImageList)(::windows::core::Vtable::as_raw(self), i, pil.into().abi(), isrc, punk.into().abi(), dwflags).ok()
    }
}
::windows::core::interface_hierarchy!(IImageList2, ::windows::core::IUnknown, IImageList);
impl ::core::clone::Clone for IImageList2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IImageList2 {
    type Vtable = IImageList2_Vtbl;
}
unsafe impl ::windows::core::Interface for IImageList2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x192b9d83_50fc_457b_90a0_2b82a8b5dae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList2_Vtbl {
    pub base__: IImageList_Vtbl,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows::core::HRESULT,
    pub GetOriginalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows::core::HRESULT,
    pub SetOriginalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimage: i32, cx: i32, cy: i32) -> ::windows::core::HRESULT,
    pub SetCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ForceImagePresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32) -> ::windows::core::HRESULT,
    pub DiscardImages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub PreloadImages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    PreloadImages: usize,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pils: *mut IMAGELISTSTATS) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Replace2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Replace2: usize,
    pub ReplaceFromImageList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, pil: *mut ::core::ffi::c_void, isrc: i32, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACM_ISPLAYING: u32 = 1128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACM_OPEN: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACM_OPENA: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACM_OPENW: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACM_PLAY: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACM_STOP: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACN_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACN_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACS_AUTOPLAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACS_CENTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACS_TIMER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ACS_TRANSPARENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ANIMATE_CLASS: ::windows::core::PCWSTR = ::windows::w!("SysAnimate32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ANIMATE_CLASSA: ::windows::core::PCSTR = ::windows::s!("SysAnimate32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ANIMATE_CLASSW: ::windows::core::PCWSTR = ::windows::w!("SysAnimate32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_FIRST: u32 = 5632u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_GETIDEALSIZE: u32 = 5633u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_GETIMAGELIST: u32 = 5635u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_GETNOTE: u32 = 5642u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_GETNOTELENGTH: u32 = 5643u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_GETSPLITINFO: u32 = 5640u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_GETTEXTMARGIN: u32 = 5637u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_SETDROPDOWNSTATE: u32 = 5638u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_SETIMAGELIST: u32 = 5634u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_SETNOTE: u32 = 5641u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_SETSHIELD: u32 = 5644u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_SETSPLITINFO: u32 = 5639u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCM_SETTEXTMARGIN: u32 = 5636u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCN_DROPDOWN: u32 = 4294966048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCN_FIRST: u32 = 4294966046u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCN_HOTITEMCHANGE: u32 = 4294966047u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCSIF_GLYPH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCSIF_IMAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCSIF_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCSIF_STYLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCSS_ALIGNLEFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCSS_IMAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCSS_NOSPLIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BCSS_STRETCH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BST_DROPDOWNPUSHED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BST_HOT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BS_COMMANDLINK: i32 = 14i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BS_DEFCOMMANDLINK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BS_DEFSPLITBUTTON: i32 = 13i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BS_SPLITBUTTON: i32 = 12i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BTNS_AUTOSIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BTNS_BUTTON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BTNS_CHECK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BTNS_DROPDOWN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BTNS_GROUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BTNS_NOPREFIX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BTNS_SEP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BTNS_SHOWTEXT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BTNS_WHOLEDROPDOWN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEMAXSTRLEN: u32 = 260u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_GETCOMBOCONTROL: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_GETEDITCONTROL: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_GETEXSTYLE: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_GETEXTENDEDSTYLE: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_GETIMAGELIST: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_GETITEM: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_GETITEMA: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_GETITEMW: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_HASEDITCHANGED: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_INSERTITEM: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_INSERTITEMA: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_INSERTITEMW: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_SETEXSTYLE: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_SETEXTENDEDSTYLE: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_SETIMAGELIST: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_SETITEM: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_SETITEMA: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_SETITEMW: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEM_SETWINDOWTHEME: u32 = 8203u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBENF_DROPDOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBENF_ESCAPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBENF_KILLFOCUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBENF_RETURN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBES_EX_CASESENSITIVE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBES_EX_NOEDITIMAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBES_EX_NOEDITIMAGEINDENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBES_EX_NOSIZELIMIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBES_EX_PATHWORDBREAKPROC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBES_EX_TEXTENDELLIPSIS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBM_FIRST: u32 = 5888u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CB_GETCUEBANNER: u32 = 5892u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CB_GETMINVISIBLE: u32 = 5890u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CB_SETCUEBANNER: u32 = 5891u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CB_SETMINVISIBLE: u32 = 5889u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCF_NOTEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCHCCCLASS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCHCCDESC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCHCCTEXT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_DPISCALE: u32 = 8204u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_FIRST: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_GETCOLORSCHEME: u32 = 8195u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_GETDROPTARGET: u32 = 8196u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_GETVERSION: u32 = 8200u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_LAST: u32 = 8704u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_SETBKCOLOR: u32 = 8193u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_SETCOLORSCHEME: u32 = 8194u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_SETNOTIFYWINDOW: u32 = 8201u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_SETVERSION: u32 = 8199u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCM_SETWINDOWTHEME: u32 = 8203u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCS_ADJUSTABLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCS_BOTTOM: i32 = 3i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCS_NODIVIDER: i32 = 64i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCS_NOMOVEY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCS_NOPARENTALIGN: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCS_NORESIZE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCS_TOP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CCS_VERT: i32 = 128i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDDS_ITEM: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDDS_POSTERASE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDRF_DODEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDRF_DOERASE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDRF_NEWFONT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDRF_NOTIFYITEMDRAW: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDRF_NOTIFYPOSTERASE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDRF_NOTIFYPOSTPAINT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDRF_NOTIFYSUBITEMDRAW: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDRF_SKIPDEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDRF_SKIPPOSTPAINT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CLR_DEFAULT: i32 = -16777216i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CLR_HILIGHT: i32 = -16777216i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CLR_NONE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMB_MASKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const COLORMGMTDLGORD: u32 = 1551u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const COMCTL32_VERSION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DATETIMEPICK_CLASS: ::windows::core::PCWSTR = ::windows::w!("SysDateTimePick32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DATETIMEPICK_CLASSA: ::windows::core::PCSTR = ::windows::s!("SysDateTimePick32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DATETIMEPICK_CLASSW: ::windows::core::PCWSTR = ::windows::w!("SysDateTimePick32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DA_ERR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DA_LAST: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DL_COPYCURSOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DL_CURSORSET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DL_MOVECURSOR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DL_STOPCURSOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPAM_INTERSECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPAM_NORMAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPAM_SORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPAM_UNION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPAS_INSERTAFTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPAS_INSERTBEFORE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPAS_SORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPA_APPEND: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPA_ERR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DRAGLISTMSGSTRING: ::windows::core::PCWSTR = ::windows::w!("commctrl_DragListMsg");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DSA_APPEND: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DSA_ERR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTBG_CLIPRECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTBG_COMPUTINGREGION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTBG_DRAWSOLID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTBG_MIRRORDC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTBG_NOMIRROR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTBG_OMITBORDER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTBG_OMITCONTENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_CLOSEMONTHCAL: u32 = 4109u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_FIRST: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_GETDATETIMEPICKERINFO: u32 = 4110u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_GETIDEALSIZE: u32 = 4111u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_GETMCCOLOR: u32 = 4103u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_GETMCFONT: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_GETMCSTYLE: u32 = 4108u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_GETMONTHCAL: u32 = 4104u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_GETRANGE: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_GETSYSTEMTIME: u32 = 4097u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_SETFORMAT: u32 = 4146u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_SETFORMATA: u32 = 4101u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_SETFORMATW: u32 = 4146u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_SETMCCOLOR: u32 = 4102u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_SETMCFONT: u32 = 4105u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_SETMCSTYLE: u32 = 4107u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_SETRANGE: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTM_SETSYSTEMTIME: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTS_APPCANPARSE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTS_LONGDATEFORMAT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTS_RIGHTALIGN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTS_SHORTDATECENTURYFORMAT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTS_SHORTDATEFORMAT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTS_SHOWNONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTS_TIMEFORMAT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTS_UPDOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_FLAGS2VALIDBITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_GRAYED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ECM_FIRST: u32 = 5376u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_CANUNDO: u32 = 198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_CHARFROMPOS: u32 = 215u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_EMPTYUNDOBUFFER: u32 = 205u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_ENABLEFEATURE: u32 = 218u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_ENABLESEARCHWEB: u32 = 5390u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_FILELINEFROMCHAR: u32 = 5395u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_FILELINEINDEX: u32 = 5396u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_FILELINELENGTH: u32 = 5397u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_FMTLINES: u32 = 200u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETCARETINDEX: u32 = 5394u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETCUEBANNER: u32 = 5378u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETENDOFLINE: u32 = 5389u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETEXTENDEDSTYLE: u32 = 5387u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETFILELINE: u32 = 5398u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETFILELINECOUNT: u32 = 5399u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETFIRSTVISIBLELINE: u32 = 206u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETHANDLE: u32 = 189u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETHILITE: u32 = 5382u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETIMESTATUS: u32 = 217u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETLIMITTEXT: u32 = 213u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETLINE: u32 = 196u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETLINECOUNT: u32 = 186u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETMARGINS: u32 = 212u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETMODIFY: u32 = 184u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETPASSWORDCHAR: u32 = 210u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETRECT: u32 = 178u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETSEL: u32 = 176u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETTHUMB: u32 = 190u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_GETWORDBREAKPROC: u32 = 209u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_HIDEBALLOONTIP: u32 = 5380u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_LIMITTEXT: u32 = 197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_LINEFROMCHAR: u32 = 201u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_LINEINDEX: u32 = 187u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_LINELENGTH: u32 = 193u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_LINESCROLL: u32 = 182u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_NOSETFOCUS: u32 = 5383u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_POSFROMCHAR: u32 = 214u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_REPLACESEL: u32 = 194u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SCROLL: u32 = 181u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SCROLLCARET: u32 = 183u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SEARCHWEB: u32 = 5391u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETCARETINDEX: u32 = 5393u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETCUEBANNER: u32 = 5377u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETENDOFLINE: u32 = 5388u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETEXTENDEDSTYLE: u32 = 5386u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETHANDLE: u32 = 188u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETHILITE: u32 = 5381u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETIMESTATUS: u32 = 216u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETLIMITTEXT: u32 = 197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETMARGINS: u32 = 211u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETMODIFY: u32 = 185u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETPASSWORDCHAR: u32 = 204u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETREADONLY: u32 = 207u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETRECT: u32 = 179u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETRECTNP: u32 = 180u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETSEL: u32 = 177u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETTABSTOPS: u32 = 203u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SETWORDBREAKPROC: u32 = 208u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_SHOWBALLOONTIP: u32 = 5379u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_TAKEFOCUS: u32 = 5384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EM_UNDO: u32 = 199u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ES_EX_ALLOWEOL_CR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ES_EX_ALLOWEOL_LF: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ES_EX_CONVERT_EOL_ON_PASTE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ES_EX_ZOOMABLE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETDT_DISABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETDT_ENABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETDT_USEAEROWIZARDTABTEXTURE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETDT_USETABTEXTURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FILEOPENORD: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FINDDLGORD: u32 = 1540u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FONTDLGORD: u32 = 1542u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FORMATDLGORD30: u32 = 1544u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FORMATDLGORD31: u32 = 1543u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FSB_ENCARTA_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FSB_FLAT_MODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FSB_REGULAR_MODE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GDTR_MAX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GDTR_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GDT_ERROR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GMR_DAYSTATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GMR_VISIBLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_CLEARFILTER: u32 = 4632u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_CREATEDRAGIMAGE: u32 = 4624u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_DELETEITEM: u32 = 4610u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_EDITFILTER: u32 = 4631u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_FIRST: u32 = 4608u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETBITMAPMARGIN: u32 = 4629u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETFOCUSEDITEM: u32 = 4635u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETIMAGELIST: u32 = 4617u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETITEM: u32 = 4619u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETITEMA: u32 = 4611u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETITEMCOUNT: u32 = 4608u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETITEMDROPDOWNRECT: u32 = 4633u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETITEMRECT: u32 = 4615u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETITEMW: u32 = 4619u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETORDERARRAY: u32 = 4625u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETOVERFLOWRECT: u32 = 4634u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_HITTEST: u32 = 4614u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_INSERTITEM: u32 = 4618u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_INSERTITEMA: u32 = 4609u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_INSERTITEMW: u32 = 4618u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_LAYOUT: u32 = 4613u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_ORDERTOINDEX: u32 = 4623u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_SETBITMAPMARGIN: u32 = 4628u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_SETFILTERCHANGETIMEOUT: u32 = 4630u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_SETFOCUSEDITEM: u32 = 4636u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_SETHOTDIVIDER: u32 = 4627u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_SETIMAGELIST: u32 = 4616u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_SETITEM: u32 = 4620u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_SETITEMA: u32 = 4612u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_SETITEMW: u32 = 4620u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_SETORDERARRAY: u32 = 4626u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDSIL_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDSIL_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_BUTTONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_CHECKBOXES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_DRAGDROP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_FILTERBAR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_FLAT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_FULLDRAG: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_HIDDEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_HORZ: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_HOTTRACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_NOSIZING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDS_OVERFLOW: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIST_ADDTOFAVORITES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIST_BACK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIST_FAVORITES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIST_FORWARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIST_VIEWTREE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKCOMB_A: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKCOMB_C: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKCOMB_CA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKCOMB_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKCOMB_S: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKCOMB_SA: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKCOMB_SC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKCOMB_SCA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKM_GETHOTKEY: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKM_SETHOTKEY: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HKM_SETRULES: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HOTKEYF_ALT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HOTKEYF_CONTROL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HOTKEYF_EXT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HOTKEYF_SHIFT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HOTKEY_CLASS: ::windows::core::PCWSTR = ::windows::w!("msctls_hotkey32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HOTKEY_CLASSA: ::windows::core::PCSTR = ::windows::s!("msctls_hotkey32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HOTKEY_CLASSW: ::windows::core::PCWSTR = ::windows::w!("msctls_hotkey32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HOVER_DEFAULT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDB_HIST_DISABLED: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDB_HIST_HOT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDB_HIST_LARGE_COLOR: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDB_HIST_NORMAL: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDB_HIST_PRESSED: u32 = 15u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDB_HIST_SMALL_COLOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDB_STD_LARGE_COLOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDB_STD_SMALL_COLOR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDB_VIEW_LARGE_COLOR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDB_VIEW_SMALL_COLOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IDC_MANAGE_LINK: u32 = 1592u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ID_PSRESTARTWINDOWS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILDI_PURGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILDI_QUERYACCESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILDI_RESETACCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILDI_STANDBY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILDRF_IMAGELOWQUALITY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILDRF_OVERLAYLOWQUALITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILFIP_ALWAYS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILFIP_FROMSTANDBY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILGOS_ALWAYS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILGOS_FROMSTANDBY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILGT_ASYNC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILGT_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILR_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILR_HORIZONTAL_CENTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILR_HORIZONTAL_LEFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILR_HORIZONTAL_RIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILR_SCALE_ASPECTRATIO: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILR_SCALE_CLIP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILR_VERTICAL_BOTTOM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILR_VERTICAL_CENTER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILR_VERTICAL_TOP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILS_ALPHA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILS_GLOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILS_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILS_SATURATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILS_SHADOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const INFOTIPSIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const INVALID_LINK_INDEX: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IPM_CLEARADDRESS: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IPM_GETADDRESS: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IPM_ISBLANK: u32 = 1129u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IPM_SETADDRESS: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IPM_SETFOCUS: u32 = 1128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IPM_SETRANGE: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const I_IMAGECALLBACK: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const I_IMAGENONE: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const I_INDENTCALLBACK: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ImageList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c476ba2_02b1_48f4_8048_b24619ddc058);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LM_GETIDEALHEIGHT: u32 = 1793u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LM_GETIDEALSIZE: u32 = 1793u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LM_GETITEM: u32 = 1795u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LM_HITTEST: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LM_SETITEM: u32 = 1794u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVA_ALIGNLEFT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVA_ALIGNTOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVA_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVA_SNAPTOGRID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCDRF_NOGROUPFRAME: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCDRF_NOSELECT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVFF_ITEMCOUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVFIS_FOCUSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_ALIGN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_DESCRIPTIONBOTTOM: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_DESCRIPTIONTOP: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_EXTENDEDIMAGE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_GROUPID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_ITEMS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_SUBSET: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_SUBSETITEMS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_SUBTITLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_TASK: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_TITLEIMAGE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGGR_GROUP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGGR_HEADER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGGR_LABEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGGR_SUBSETLINK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGMF_BORDERCOLOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGMF_BORDERSIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGMF_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGMF_TEXTCOLOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIR_BOUNDS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIR_ICON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIR_LABEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIR_SELECTBOUNDS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVKF_ALT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVKF_CONTROL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVKF_SHIFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_APPROXIMATEVIEWRECT: u32 = 4160u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_ARRANGE: u32 = 4118u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_CANCELEDITLABEL: u32 = 4275u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_CREATEDRAGIMAGE: u32 = 4129u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_DELETEALLITEMS: u32 = 4105u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_DELETECOLUMN: u32 = 4124u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_DELETEITEM: u32 = 4104u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_EDITLABEL: u32 = 4214u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_EDITLABELA: u32 = 4119u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_EDITLABELW: u32 = 4214u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_ENABLEGROUPVIEW: u32 = 4253u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_ENSUREVISIBLE: u32 = 4115u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_FINDITEM: u32 = 4179u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_FINDITEMA: u32 = 4109u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_FINDITEMW: u32 = 4179u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_FIRST: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETBKCOLOR: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETBKIMAGE: u32 = 4235u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETBKIMAGEA: u32 = 4165u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETBKIMAGEW: u32 = 4235u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETCALLBACKMASK: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETCOLUMN: u32 = 4191u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETCOLUMNA: u32 = 4121u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETCOLUMNORDERARRAY: u32 = 4155u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETCOLUMNW: u32 = 4191u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETCOLUMNWIDTH: u32 = 4125u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETCOUNTPERPAGE: u32 = 4136u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETEDITCONTROL: u32 = 4120u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETEMPTYTEXT: u32 = 4300u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETEXTENDEDLISTVIEWSTYLE: u32 = 4151u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETFOCUSEDGROUP: u32 = 4189u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETFOOTERINFO: u32 = 4302u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETFOOTERITEM: u32 = 4304u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETFOOTERITEMRECT: u32 = 4303u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETFOOTERRECT: u32 = 4301u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETGROUPCOUNT: u32 = 4248u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETGROUPINFO: u32 = 4245u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETGROUPINFOBYINDEX: u32 = 4249u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETGROUPMETRICS: u32 = 4252u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETGROUPRECT: u32 = 4194u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETGROUPSTATE: u32 = 4188u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETHEADER: u32 = 4127u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETHOTCURSOR: u32 = 4159u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETHOTITEM: u32 = 4157u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETHOVERTIME: u32 = 4168u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETIMAGELIST: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETINSERTMARK: u32 = 4263u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETINSERTMARKCOLOR: u32 = 4267u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETINSERTMARKRECT: u32 = 4265u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETISEARCHSTRING: u32 = 4213u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETISEARCHSTRINGA: u32 = 4148u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETISEARCHSTRINGW: u32 = 4213u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEM: u32 = 4171u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMA: u32 = 4101u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMCOUNT: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMINDEXRECT: u32 = 4305u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMPOSITION: u32 = 4112u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMRECT: u32 = 4110u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMSPACING: u32 = 4147u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMSTATE: u32 = 4140u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMTEXT: u32 = 4211u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMTEXTA: u32 = 4141u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMTEXTW: u32 = 4211u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETITEMW: u32 = 4171u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETNEXTITEM: u32 = 4108u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETNEXTITEMINDEX: u32 = 4307u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETNUMBEROFWORKAREAS: u32 = 4169u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETORIGIN: u32 = 4137u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETOUTLINECOLOR: u32 = 4272u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETSELECTEDCOLUMN: u32 = 4270u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETSELECTEDCOUNT: u32 = 4146u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETSELECTIONMARK: u32 = 4162u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETSTRINGWIDTH: u32 = 4183u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETSTRINGWIDTHA: u32 = 4113u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETSTRINGWIDTHW: u32 = 4183u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETSUBITEMRECT: u32 = 4152u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETTEXTBKCOLOR: u32 = 4133u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETTEXTCOLOR: u32 = 4131u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETTILEINFO: u32 = 4261u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETTILEVIEWINFO: u32 = 4259u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETTOOLTIPS: u32 = 4174u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETTOPINDEX: u32 = 4135u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETVIEW: u32 = 4239u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETVIEWRECT: u32 = 4130u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_GETWORKAREAS: u32 = 4166u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_HASGROUP: u32 = 4257u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_HITTEST: u32 = 4114u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_INSERTCOLUMN: u32 = 4193u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_INSERTCOLUMNA: u32 = 4123u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_INSERTCOLUMNW: u32 = 4193u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_INSERTGROUP: u32 = 4241u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_INSERTGROUPSORTED: u32 = 4255u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_INSERTITEM: u32 = 4173u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_INSERTITEMA: u32 = 4103u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_INSERTITEMW: u32 = 4173u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_INSERTMARKHITTEST: u32 = 4264u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_ISGROUPVIEWENABLED: u32 = 4271u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_ISITEMVISIBLE: u32 = 4278u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_MAPIDTOINDEX: u32 = 4277u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_MAPINDEXTOID: u32 = 4276u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_MOVEGROUP: u32 = 4247u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_MOVEITEMTOGROUP: u32 = 4250u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_REDRAWITEMS: u32 = 4117u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_REMOVEALLGROUPS: u32 = 4256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_REMOVEGROUP: u32 = 4246u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SCROLL: u32 = 4116u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETBKCOLOR: u32 = 4097u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETBKIMAGE: u32 = 4234u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETBKIMAGEA: u32 = 4164u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETBKIMAGEW: u32 = 4234u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETCALLBACKMASK: u32 = 4107u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETCOLUMN: u32 = 4192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETCOLUMNA: u32 = 4122u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETCOLUMNORDERARRAY: u32 = 4154u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETCOLUMNW: u32 = 4192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETCOLUMNWIDTH: u32 = 4126u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETEXTENDEDLISTVIEWSTYLE: u32 = 4150u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETGROUPINFO: u32 = 4243u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETGROUPMETRICS: u32 = 4251u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETHOTCURSOR: u32 = 4158u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETHOTITEM: u32 = 4156u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETHOVERTIME: u32 = 4167u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETICONSPACING: u32 = 4149u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETIMAGELIST: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETINFOTIP: u32 = 4269u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETINSERTMARK: u32 = 4262u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETINSERTMARKCOLOR: u32 = 4266u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEM: u32 = 4172u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEMA: u32 = 4102u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEMCOUNT: u32 = 4143u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEMINDEXSTATE: u32 = 4306u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEMPOSITION: u32 = 4111u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEMPOSITION32: u32 = 4145u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEMSTATE: u32 = 4139u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEMTEXT: u32 = 4212u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEMTEXTA: u32 = 4142u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEMTEXTW: u32 = 4212u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETITEMW: u32 = 4172u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETOUTLINECOLOR: u32 = 4273u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETSELECTEDCOLUMN: u32 = 4236u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETSELECTIONMARK: u32 = 4163u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETTEXTBKCOLOR: u32 = 4134u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETTEXTCOLOR: u32 = 4132u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETTILEINFO: u32 = 4260u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETTILEVIEWINFO: u32 = 4258u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETTOOLTIPS: u32 = 4170u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETVIEW: u32 = 4238u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SETWORKAREAS: u32 = 4161u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SORTGROUPS: u32 = 4254u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SORTITEMS: u32 = 4144u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SORTITEMSEX: u32 = 4177u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_SUBITEMHITTEST: u32 = 4153u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVM_UPDATE: u32 = 4138u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_ABOVE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_BELOW: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_CUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_DROPHILITED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_FOCUSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_PREVIOUS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_SAMEGROUPONLY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_SELECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_TOLEFT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_TORIGHT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_VISIBLEONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNI_VISIBLEORDER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNSCH_DEFAULT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNSCH_ERROR: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVNSCH_IGNORE: i32 = -3i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVSCW_AUTOSIZE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVSCW_AUTOSIZE_USEHEADER: i32 = -2i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVSICF_NOINVALIDATEALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVSICF_NOSCROLL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVSIL_GROUPHEADER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVSIL_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVSIL_SMALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVSIL_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_ALIGNLEFT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_ALIGNMASK: u32 = 3072u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_ALIGNTOP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_AUTOARRANGE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EDITLABELS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_AUTOAUTOARRANGE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_AUTOCHECKSELECT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_AUTOSIZECOLUMNS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_BORDERSELECT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_CHECKBOXES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_COLUMNOVERFLOW: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_COLUMNSNAPPOINTS: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_DOUBLEBUFFER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_FLATSB: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_FULLROWSELECT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_GRIDLINES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_HEADERDRAGDROP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_HEADERINALLVIEWS: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_HIDELABELS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_INFOTIP: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_JUSTIFYCOLUMNS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_LABELTIP: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_MULTIWORKAREAS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_ONECLICKACTIVATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_REGIONAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_SIMPLESELECT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_SINGLEROW: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_SNAPTOGRID: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_SUBITEMIMAGES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_TRACKSELECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_TRANSPARENTBKGND: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_TRANSPARENTSHADOWTEXT: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_TWOCLICKACTIVATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_UNDERLINECOLD: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_EX_UNDERLINEHOT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_ICON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_LIST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_NOCOLUMNHEADER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_NOLABELWRAP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_NOSCROLL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_NOSORTHEADER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_OWNERDATA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_OWNERDRAWFIXED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_REPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_SHAREIMAGELISTS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_SHOWSELALWAYS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_SINGLESEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_SMALLICON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_SORTASCENDING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_SORTDESCENDING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_TYPEMASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVS_TYPESTYLEMASK: u32 = 64512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVTVIF_EXTENDED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LV_MAX_WORKAREAS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LV_VIEW_DETAILS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LV_VIEW_ICON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LV_VIEW_LIST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LV_VIEW_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LV_VIEW_SMALLICON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LV_VIEW_TILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LWS_IGNORERETURN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LWS_NOPREFIX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LWS_RIGHT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LWS_TRANSPARENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LWS_USECUSTOMTEXT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LWS_USEVISUALSTYLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MAXPROPPAGES: u32 = 100u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MAX_INTLIST_COUNT: u32 = 402u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MAX_LINKID_TEXT: u32 = 48u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MAX_THEMECOLOR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MAX_THEMESIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_FIRST: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETCALENDARBORDER: u32 = 4127u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETCALENDARCOUNT: u32 = 4119u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETCALENDARGRIDINFO: u32 = 4120u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETCALID: u32 = 4123u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETCOLOR: u32 = 4107u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETCURRENTVIEW: u32 = 4118u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETCURSEL: u32 = 4097u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETFIRSTDAYOFWEEK: u32 = 4112u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETMAXSELCOUNT: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETMAXTODAYWIDTH: u32 = 4117u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETMINREQRECT: u32 = 4105u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETMONTHDELTA: u32 = 4115u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETMONTHRANGE: u32 = 4103u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETRANGE: u32 = 4113u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETSELRANGE: u32 = 4101u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETTODAY: u32 = 4109u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_HITTEST: u32 = 4110u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETCALENDARBORDER: u32 = 4126u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETCALID: u32 = 4124u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETCOLOR: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETCURRENTVIEW: u32 = 4128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETCURSEL: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETDAYSTATE: u32 = 4104u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETFIRSTDAYOFWEEK: u32 = 4111u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETMAXSELCOUNT: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETMONTHDELTA: u32 = 4116u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETRANGE: u32 = 4114u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETSELRANGE: u32 = 4102u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETTODAY: u32 = 4108u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCM_SIZERECTTOMIN: u32 = 4125u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCSC_BACKGROUND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCSC_MONTHBK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCSC_TEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCSC_TITLEBK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCSC_TITLETEXT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCSC_TRAILINGTEXT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCS_DAYSTATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCS_MULTISELECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCS_NOSELCHANGEONNAV: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCS_NOTODAY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCS_NOTODAYCIRCLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCS_NOTRAILINGDATES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCS_SHORTDAYSOFWEEK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCS_WEEKNUMBERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MONTHCAL_CLASS: ::windows::core::PCWSTR = ::windows::w!("SysMonthCal32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MONTHCAL_CLASSA: ::windows::core::PCSTR = ::windows::s!("SysMonthCal32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MONTHCAL_CLASSW: ::windows::core::PCWSTR = ::windows::w!("SysMonthCal32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSGF_COMMCTRL_BEGINDRAG: u32 = 16896u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSGF_COMMCTRL_DRAGSELECT: u32 = 16898u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSGF_COMMCTRL_SIZEHEADER: u32 = 16897u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSGF_COMMCTRL_TOOLBARCUST: u32 = 16899u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MULTIFILEOPENORD: u32 = 1537u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NEWFILEOPENORD: u32 = 1547u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NEWFILEOPENV2ORD: u32 = 1552u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NEWFILEOPENV3ORD: u32 = 1553u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NEWFORMATDLGWITHLINK: u32 = 1591u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NFS_ALL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NFS_BUTTON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NFS_EDIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NFS_LISTCOMBO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NFS_STATIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NFS_USEFONTASSOC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NM_GETCUSTOMSPLITRECT: u32 = 4294966049u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODT_HEADER: u32 = 100u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PAGESETUPDLGORD: u32 = 1546u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PAGESETUPDLGORDMOTIF: u32 = 1550u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_DELTAPOS: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_GETBARCOLOR: u32 = 1039u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_GETBKCOLOR: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_GETPOS: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_GETRANGE: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_GETSTATE: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_GETSTEP: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_SETBARCOLOR: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_SETBKCOLOR: u32 = 8193u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_SETMARQUEE: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_SETPOS: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_SETRANGE: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_SETRANGE32: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_SETSTATE: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_SETSTEP: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBM_STEPIT: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBST_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBST_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBST_PAUSED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBS_MARQUEE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBS_SMOOTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBS_SMOOTHREVERSE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBS_VERTICAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGB_BOTTOMORRIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGB_TOPORLEFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_DEPRESSED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_GRAYED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_HOT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_INVISIBLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_FIRST: u32 = 5120u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_FORWARDMOUSE: u32 = 5123u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_GETBKCOLOR: u32 = 5125u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_GETBORDER: u32 = 5127u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_GETBUTTONSIZE: u32 = 5131u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_GETBUTTONSTATE: u32 = 5132u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_GETDROPTARGET: u32 = 8196u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_GETPOS: u32 = 5129u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_RECALCSIZE: u32 = 5122u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_SETBKCOLOR: u32 = 5124u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_SETBORDER: u32 = 5126u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_SETBUTTONSIZE: u32 = 5130u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_SETCHILD: u32 = 5121u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_SETPOS: u32 = 5128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGM_SETSCROLLINFO: u32 = 5133u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGS_AUTOSCROLL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGS_DRAGNDROP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGS_HORZ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGS_VERT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PRINTDLGEXORD: u32 = 1549u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PRINTDLGORD: u32 = 1538u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PRNSETUPDLGORD: u32 = 1539u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PROGRESS_CLASS: ::windows::core::PCWSTR = ::windows::w!("msctls_progress32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PROGRESS_CLASSA: ::windows::core::PCSTR = ::windows::s!("msctls_progress32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PROGRESS_CLASSW: ::windows::core::PCWSTR = ::windows::w!("msctls_progress32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PROP_LG_CXDLG: u32 = 252u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PROP_LG_CYDLG: u32 = 218u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PROP_MED_CXDLG: u32 = 227u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PROP_MED_CYDLG: u32 = 215u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PROP_SM_CXDLG: u32 = 212u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PROP_SM_CYDLG: u32 = 188u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSBTN_APPLYNOW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSBTN_BACK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSBTN_CANCEL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSBTN_FINISH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSBTN_HELP: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSBTN_MAX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSBTN_NEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSBTN_OK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSCB_BUTTONPRESSED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSCB_INITIALIZED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSCB_PRECREATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_AEROWIZARD: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_HASHELP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_HEADER: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_HEADERBITMAP: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_MODELESS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_NOAPPLYNOW: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_NOCONTEXTHELP: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_NOMARGIN: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_PROPSHEETPAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_PROPTITLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_RESIZABLE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_RTLREADING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_STRETCHWATERMARK: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_USECALLBACK: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_USEHBMHEADER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_USEHBMWATERMARK: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_USEHICON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_USEHPLWATERMARK: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_USEICONID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_USEPAGELANG: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_USEPSTARTPAGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_WATERMARK: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_WIZARD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_WIZARD97: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_WIZARDCONTEXTHELP: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_WIZARDHASFINISH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSH_WIZARD_LITE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_ADDPAGE: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_APPLY: u32 = 1134u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_CANCELTOCLOSE: u32 = 1131u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_CHANGED: u32 = 1128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_ENABLEWIZBUTTONS: u32 = 1163u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_GETCURRENTPAGEHWND: u32 = 1142u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_GETRESULT: u32 = 1159u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_GETTABCONTROL: u32 = 1140u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_HWNDTOINDEX: u32 = 1153u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_IDTOINDEX: u32 = 1157u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_INDEXTOHWND: u32 = 1154u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_INDEXTOID: u32 = 1158u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_INDEXTOPAGE: u32 = 1156u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_INSERTPAGE: u32 = 1143u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_ISDIALOGMESSAGE: u32 = 1141u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_PAGETOINDEX: u32 = 1155u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_PRESSBUTTON: u32 = 1137u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_QUERYSIBLINGS: u32 = 1132u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_REBOOTSYSTEM: u32 = 1130u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_RECALCPAGESIZES: u32 = 1160u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_REMOVEPAGE: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_RESTARTWINDOWS: u32 = 1129u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETBUTTONTEXT: u32 = 1164u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETBUTTONTEXTW: u32 = 1164u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETCURSEL: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETCURSELID: u32 = 1138u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETFINISHTEXT: u32 = 1145u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETFINISHTEXTA: u32 = 1139u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETFINISHTEXTW: u32 = 1145u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETHEADERSUBTITLE: u32 = 1152u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETHEADERSUBTITLEA: u32 = 1151u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETHEADERSUBTITLEW: u32 = 1152u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETHEADERTITLE: u32 = 1150u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETHEADERTITLEA: u32 = 1149u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETHEADERTITLEW: u32 = 1150u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETNEXTTEXT: u32 = 1161u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETNEXTTEXTW: u32 = 1161u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETTITLE: u32 = 1144u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETTITLEA: u32 = 1135u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETTITLEW: u32 = 1144u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SETWIZBUTTONS: u32 = 1136u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_SHOWWIZBUTTONS: u32 = 1162u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSM_UNCHANGED: u32 = 1133u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSNRET_INVALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSNRET_INVALID_NOCHANGEPAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSNRET_MESSAGEHANDLED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSNRET_NOERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_DLGINDIRECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_HASHELP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_HIDEHEADER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_PREMATURE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_RTLREADING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_USECALLBACK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_USEFUSIONCONTEXT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_USEHEADERSUBTITLE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_USEHEADERTITLE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_USEHICON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_USEICONID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_USEREFPARENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSP_USETITLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSWIZBF_ELEVATIONREQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSWIZB_BACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSWIZB_CANCEL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSWIZB_DISABLEDFINISH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSWIZB_FINISH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSWIZB_NEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSWIZB_RESTORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSWIZB_SHOW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBAB_ADDBAND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBAB_AUTOSIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_BACKGROUND: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_CHEVRONLOCATION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_CHEVRONSTATE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_CHILD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_CHILDSIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_COLORS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_HEADERSIZE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_ID: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_IDEALSIZE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_IMAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_LPARAM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_SIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_STYLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBIM_TEXT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_BREAK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_CHILDEDGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_FIXEDBMP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_FIXEDSIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_GRIPPERALWAYS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_HIDDEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_HIDETITLE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_NOGRIPPER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_NOVERT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_TOPALIGN: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_USECHEVRON: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBBS_VARIABLEHEIGHT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBHT_CAPTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBHT_CHEVRON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBHT_CLIENT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBHT_GRABBER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBHT_NOWHERE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBHT_SPLITTER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBIM_IMAGELIST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBSTR_CHANGERECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_AUTOSIZE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_BANDBORDERS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_DBLCLKTOGGLE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_FIXEDORDER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_REGISTERDROP: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_TOOLTIPS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_VARHEIGHT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_VERTICALGRIPPER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_BEGINDRAG: u32 = 1048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_DELETEBAND: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_DRAGMOVE: u32 = 1050u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_ENDDRAG: u32 = 1049u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETBANDBORDERS: u32 = 1058u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETBANDCOUNT: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETBANDINFO: u32 = 1052u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETBANDINFOA: u32 = 1053u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETBANDINFOW: u32 = 1052u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETBANDMARGINS: u32 = 1064u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETBARHEIGHT: u32 = 1051u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETBARINFO: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETBKCOLOR: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETCOLORSCHEME: u32 = 8195u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETDROPTARGET: u32 = 8196u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETEXTENDEDSTYLE: u32 = 1066u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETPALETTE: u32 = 1062u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETRECT: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETROWCOUNT: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETROWHEIGHT: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETTEXTCOLOR: u32 = 1046u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETTOOLTIPS: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_HITTEST: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_IDTOINDEX: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_INSERTBAND: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_INSERTBANDA: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_INSERTBANDW: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_MAXIMIZEBAND: u32 = 1055u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_MINIMIZEBAND: u32 = 1054u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_MOVEBAND: u32 = 1063u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_PUSHCHEVRON: u32 = 1067u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETBANDINFO: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETBANDINFOA: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETBANDINFOW: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETBANDWIDTH: u32 = 1068u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETBARINFO: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETBKCOLOR: u32 = 1043u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETCOLORSCHEME: u32 = 8194u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETEXTENDEDSTYLE: u32 = 1065u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETPALETTE: u32 = 1061u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETPARENT: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETTEXTCOLOR: u32 = 1045u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETTOOLTIPS: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SETWINDOWTHEME: u32 = 8203u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SHOWBAND: u32 = 1059u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RB_SIZETORECT: u32 = 1047u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const REBARCLASSNAME: ::windows::core::PCWSTR = ::windows::w!("ReBarWindow32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const REBARCLASSNAMEA: ::windows::core::PCSTR = ::windows::s!("ReBarWindow32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const REBARCLASSNAMEW: ::windows::core::PCWSTR = ::windows::w!("ReBarWindow32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const REPLACEDLGORD: u32 = 1541u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RUNDLGORD: u32 = 1545u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBARS_SIZEGRIP: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBARS_TOOLTIPS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBT_NOBORDERS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBT_NOTABPARSING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBT_OWNERDRAW: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBT_POPOUT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBT_RTLREADING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBT_TOOLTIPS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETBORDERS: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETICON: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETPARTS: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETRECT: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETTEXT: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETTEXTA: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETTEXTLENGTH: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETTEXTLENGTHA: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETTEXTLENGTHW: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETTEXTW: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETTIPTEXTA: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETTIPTEXTW: u32 = 1043u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_ISSIMPLE: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SETBKCOLOR: u32 = 8193u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SETICON: u32 = 1039u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SETMINHEIGHT: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SETPARTS: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SETTEXT: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SETTEXTA: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SETTEXTW: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SETTIPTEXTA: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SETTIPTEXTW: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SIMPLE: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SB_SIMPLEID: u32 = 255u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STATUSCLASSNAME: ::windows::core::PCWSTR = ::windows::w!("msctls_statusbar32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STATUSCLASSNAMEA: ::windows::core::PCSTR = ::windows::s!("msctls_statusbar32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STATUSCLASSNAMEW: ::windows::core::PCWSTR = ::windows::w!("msctls_statusbar32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_COPY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_CUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_DELETE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_FILENEW: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_FILEOPEN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_FILESAVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_FIND: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_HELP: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_PASTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_PRINT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_PRINTPRE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_PROPERTIES: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_REDOW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_REPLACE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STD_UNDO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZ_THDOCPROP_AUTHOR: ::windows::core::PCWSTR = ::windows::w!("author");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZ_THDOCPROP_CANONICALNAME: ::windows::core::PCWSTR = ::windows::w!("ThemeName");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZ_THDOCPROP_DISPLAYNAME: ::windows::core::PCWSTR = ::windows::w!("DisplayName");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZ_THDOCPROP_TOOLTIP: ::windows::core::PCWSTR = ::windows::w!("ToolTip");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBBF_LARGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCDRF_BLENDICON: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCDRF_HILITEHOTTRACK: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCDRF_NOBACKGROUND: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCDRF_NOEDGES: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCDRF_NOETCHEDEFFECT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCDRF_NOMARK: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCDRF_NOOFFSET: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCDRF_USECDCOLORS: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCD_CHANNEL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCD_THUMB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBCD_TICS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBDDRET_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBDDRET_NODEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBDDRET_TREATPRESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBMF_BARPAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBMF_BUTTONSPACING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBMF_PAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_CLEARSEL: u32 = 1043u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_CLEARTICS: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETBUDDY: u32 = 1057u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETCHANNELRECT: u32 = 1050u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETLINESIZE: u32 = 1048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETNUMTICS: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETPAGESIZE: u32 = 1046u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETPTICS: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETRANGEMAX: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETRANGEMIN: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETSELEND: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETSELSTART: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETTHUMBLENGTH: u32 = 1052u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETTHUMBRECT: u32 = 1049u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETTIC: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETTICPOS: u32 = 1039u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETTOOLTIPS: u32 = 1054u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETBUDDY: u32 = 1056u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETLINESIZE: u32 = 1047u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETPAGESIZE: u32 = 1045u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETPOS: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETPOSNOTIFY: u32 = 1058u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETRANGE: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETRANGEMAX: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETRANGEMIN: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETSEL: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETSELEND: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETSELSTART: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETTHUMBLENGTH: u32 = 1051u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETTIC: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETTICFREQ: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETTIPSIDE: u32 = 1055u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETTOOLTIPS: u32 = 1053u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBNRF_ENDCUSTOMIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBNRF_HIDEHELP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTATE_CHECKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTATE_ELLIPSES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTATE_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTATE_HIDDEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTATE_INDETERMINATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTATE_MARKED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTATE_PRESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTATE_WRAP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_ALTDRAG: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_AUTOSIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_BUTTON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_CHECK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_CUSTOMERASE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_DROPDOWN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_EX_DOUBLEBUFFER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_EX_DRAWDDARROWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_EX_HIDECLIPPEDBUTTONS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_EX_MIXEDBUTTONS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_EX_MULTICOLUMN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_EX_VERTICAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_FLAT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_GROUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_LIST: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_NOPREFIX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_REGISTERDROP: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_SEP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_TOOLTIPS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_TRANSPARENT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBSTYLE_WRAPABLE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_AUTOTICKS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_BOTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_BOTTOM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_DOWNISLEFT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_ENABLESELRANGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_FIXEDLENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_HORZ: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_LEFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_NOTHUMB: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_NOTICKS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_NOTIFYBEFOREMOVE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_REVERSED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_RIGHT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_TOOLTIPS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_TOP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_TRANSPARENTBKGND: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBS_VERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBTS_BOTTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBTS_LEFT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBTS_RIGHT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBTS_TOP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ADDBITMAP: u32 = 1043u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ADDBUTTONS: u32 = 1092u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ADDBUTTONSA: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ADDBUTTONSW: u32 = 1092u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ADDSTRING: u32 = 1101u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ADDSTRINGA: u32 = 1052u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ADDSTRINGW: u32 = 1101u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_AUTOSIZE: u32 = 1057u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_BOTTOM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_BUTTONCOUNT: u32 = 1048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_BUTTONSTRUCTSIZE: u32 = 1054u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_CHANGEBITMAP: u32 = 1067u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_CHECKBUTTON: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_COMMANDTOINDEX: u32 = 1049u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_CUSTOMIZE: u32 = 1051u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_DELETEBUTTON: u32 = 1046u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ENABLEBUTTON: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ENDTRACK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETANCHORHIGHLIGHT: u32 = 1098u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETBITMAP: u32 = 1068u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETBITMAPFLAGS: u32 = 1065u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETBUTTON: u32 = 1047u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETBUTTONINFO: u32 = 1087u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETBUTTONINFOA: u32 = 1089u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETBUTTONINFOW: u32 = 1087u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETBUTTONSIZE: u32 = 1082u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETBUTTONTEXT: u32 = 1099u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETBUTTONTEXTA: u32 = 1069u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETBUTTONTEXTW: u32 = 1099u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETCOLORSCHEME: u32 = 8195u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETDISABLEDIMAGELIST: u32 = 1079u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETEXTENDEDSTYLE: u32 = 1109u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETHOTIMAGELIST: u32 = 1077u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETHOTITEM: u32 = 1095u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETIDEALSIZE: u32 = 1123u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETIMAGELIST: u32 = 1073u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETIMAGELISTCOUNT: u32 = 1122u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETINSERTMARK: u32 = 1103u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETINSERTMARKCOLOR: u32 = 1113u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETITEMDROPDOWNRECT: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETITEMRECT: u32 = 1053u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETMAXSIZE: u32 = 1107u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETMETRICS: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETOBJECT: u32 = 1086u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETPADDING: u32 = 1110u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETPRESSEDIMAGELIST: u32 = 1129u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETRECT: u32 = 1075u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETROWS: u32 = 1064u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETSTATE: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETSTRING: u32 = 1115u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETSTRINGA: u32 = 1116u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETSTRINGW: u32 = 1115u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETSTYLE: u32 = 1081u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETTEXTROWS: u32 = 1085u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETTOOLTIPS: u32 = 1059u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_HASACCELERATOR: u32 = 1119u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_HIDEBUTTON: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_HITTEST: u32 = 1093u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_INDETERMINATE: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_INSERTBUTTON: u32 = 1091u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_INSERTBUTTONA: u32 = 1045u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_INSERTBUTTONW: u32 = 1091u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_INSERTMARKHITTEST: u32 = 1105u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ISBUTTONCHECKED: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ISBUTTONENABLED: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ISBUTTONHIDDEN: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ISBUTTONHIGHLIGHTED: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ISBUTTONINDETERMINATE: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_ISBUTTONPRESSED: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_LINEDOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_LINEUP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_LOADIMAGES: u32 = 1074u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_MAPACCELERATOR: u32 = 1114u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_MAPACCELERATORA: u32 = 1102u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_MAPACCELERATORW: u32 = 1114u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_MARKBUTTON: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_MOVEBUTTON: u32 = 1106u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_PAGEDOWN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_PAGEUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_PRESSBUTTON: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_REPLACEBITMAP: u32 = 1070u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SAVERESTORE: u32 = 1100u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SAVERESTOREA: u32 = 1050u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SAVERESTOREW: u32 = 1100u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETANCHORHIGHLIGHT: u32 = 1097u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETBITMAPSIZE: u32 = 1056u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETBOUNDINGSIZE: u32 = 1117u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETBUTTONINFO: u32 = 1088u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETBUTTONINFOA: u32 = 1090u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETBUTTONINFOW: u32 = 1088u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETBUTTONSIZE: u32 = 1055u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETBUTTONWIDTH: u32 = 1083u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETCMDID: u32 = 1066u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETCOLORSCHEME: u32 = 8194u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETDISABLEDIMAGELIST: u32 = 1078u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETDRAWTEXTFLAGS: u32 = 1094u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETEXTENDEDSTYLE: u32 = 1108u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETHOTIMAGELIST: u32 = 1076u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETHOTITEM: u32 = 1096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETHOTITEM2: u32 = 1118u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETIMAGELIST: u32 = 1072u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETINDENT: u32 = 1071u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETINSERTMARK: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETINSERTMARKCOLOR: u32 = 1112u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETLISTGAP: u32 = 1120u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETMAXTEXTROWS: u32 = 1084u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETMETRICS: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETPADDING: u32 = 1111u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETPARENT: u32 = 1061u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETPRESSEDIMAGELIST: u32 = 1128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETROWS: u32 = 1063u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETSTATE: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETSTYLE: u32 = 1080u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETTOOLTIPS: u32 = 1060u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_SETWINDOWTHEME: u32 = 8203u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_THUMBPOSITION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_THUMBTRACK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TB_TOP: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_ADJUSTRECT: u32 = 4904u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_DELETEALLITEMS: u32 = 4873u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_DELETEITEM: u32 = 4872u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_DESELECTALL: u32 = 4914u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_FIRST: u32 = 4864u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETCURFOCUS: u32 = 4911u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETCURSEL: u32 = 4875u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETEXTENDEDSTYLE: u32 = 4917u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETIMAGELIST: u32 = 4866u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETITEM: u32 = 4924u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETITEMA: u32 = 4869u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETITEMCOUNT: u32 = 4868u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETITEMRECT: u32 = 4874u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETITEMW: u32 = 4924u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETROWCOUNT: u32 = 4908u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETTOOLTIPS: u32 = 4909u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_HIGHLIGHTITEM: u32 = 4915u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_HITTEST: u32 = 4877u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_INSERTITEM: u32 = 4926u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_INSERTITEMA: u32 = 4871u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_INSERTITEMW: u32 = 4926u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_REMOVEIMAGE: u32 = 4906u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETCURFOCUS: u32 = 4912u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETCURSEL: u32 = 4876u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETEXTENDEDSTYLE: u32 = 4916u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETIMAGELIST: u32 = 4867u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETITEM: u32 = 4925u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETITEMA: u32 = 4870u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETITEMEXTRA: u32 = 4878u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETITEMSIZE: u32 = 4905u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETITEMW: u32 = 4925u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETMINTABWIDTH: u32 = 4913u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETPADDING: u32 = 4907u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETTOOLTIPS: u32 = 4910u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_BOTTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_BUTTONS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_EX_FLATSEPARATORS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_EX_REGISTERDROP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_FIXEDWIDTH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_FLATBUTTONS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_FOCUSNEVER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_FOCUSONBUTTONDOWN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_FORCEICONLEFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_FORCELABELLEFT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_HOTTRACK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_MULTILINE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_MULTISELECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_OWNERDRAWFIXED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_RAGGEDRIGHT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_RIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_RIGHTJUSTIFY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_SCROLLOPPOSITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_SINGLELINE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_TABS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_TOOLTIPS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCS_VERTICAL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TD_ERROR_ICON: ::windows::core::PCWSTR = ::windows::core::PCWSTR(-2i32 as u16 as _);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TD_INFORMATION_ICON: ::windows::core::PCWSTR = ::windows::core::PCWSTR(-3i32 as u16 as _);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TD_SHIELD_ICON: ::windows::core::PCWSTR = ::windows::core::PCWSTR(-4i32 as u16 as _);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TD_WARNING_ICON: ::windows::core::PCWSTR = ::windows::core::PCWSTR(-1i32 as u16 as _);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMTVS_RESERVEDHIGH: u32 = 19999u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMTVS_RESERVEDLOW: u32 = 100000u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TOOLBARCLASSNAME: ::windows::core::PCWSTR = ::windows::w!("ToolbarWindow32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TOOLBARCLASSNAMEA: ::windows::core::PCSTR = ::windows::s!("ToolbarWindow32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TOOLBARCLASSNAMEW: ::windows::core::PCWSTR = ::windows::w!("ToolbarWindow32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TOOLTIPS_CLASS: ::windows::core::PCWSTR = ::windows::w!("tooltips_class32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TOOLTIPS_CLASSA: ::windows::core::PCSTR = ::windows::s!("tooltips_class32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TOOLTIPS_CLASSW: ::windows::core::PCWSTR = ::windows::w!("tooltips_class32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TRACKBAR_CLASS: ::windows::core::PCWSTR = ::windows::w!("msctls_trackbar32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TRACKBAR_CLASSA: ::windows::core::PCSTR = ::windows::s!("msctls_trackbar32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TRACKBAR_CLASSW: ::windows::core::PCWSTR = ::windows::w!("msctls_trackbar32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTDT_AUTOMATIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTDT_AUTOPOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTDT_INITIAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTDT_RESHOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_ACTIVATE: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_ADDTOOL: u32 = 1074u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_ADDTOOLA: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_ADDTOOLW: u32 = 1074u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_ADJUSTRECT: u32 = 1055u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_DELTOOL: u32 = 1075u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_DELTOOLA: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_DELTOOLW: u32 = 1075u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_ENUMTOOLS: u32 = 1082u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_ENUMTOOLSA: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_ENUMTOOLSW: u32 = 1082u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETBUBBLESIZE: u32 = 1054u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETCURRENTTOOL: u32 = 1083u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETCURRENTTOOLA: u32 = 1039u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETCURRENTTOOLW: u32 = 1083u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETDELAYTIME: u32 = 1045u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETMARGIN: u32 = 1051u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETMAXTIPWIDTH: u32 = 1049u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETTEXT: u32 = 1080u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETTEXTA: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETTEXTW: u32 = 1080u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETTIPBKCOLOR: u32 = 1046u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETTIPTEXTCOLOR: u32 = 1047u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETTITLE: u32 = 1059u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETTOOLCOUNT: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETTOOLINFO: u32 = 1077u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETTOOLINFOA: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_GETTOOLINFOW: u32 = 1077u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_HITTEST: u32 = 1079u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_HITTESTA: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_HITTESTW: u32 = 1079u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_NEWTOOLRECT: u32 = 1076u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_NEWTOOLRECTA: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_NEWTOOLRECTW: u32 = 1076u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_POP: u32 = 1052u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_POPUP: u32 = 1058u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_RELAYEVENT: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETDELAYTIME: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETMARGIN: u32 = 1050u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETMAXTIPWIDTH: u32 = 1048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETTIPBKCOLOR: u32 = 1043u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETTIPTEXTCOLOR: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETTITLE: u32 = 1057u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETTITLEA: u32 = 1056u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETTITLEW: u32 = 1057u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETTOOLINFO: u32 = 1078u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETTOOLINFOA: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETTOOLINFOW: u32 = 1078u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_SETWINDOWTHEME: u32 = 8203u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_TRACKACTIVATE: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_TRACKPOSITION: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_UPDATE: u32 = 1053u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_UPDATETIPTEXT: u32 = 1081u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_UPDATETIPTEXTA: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_UPDATETIPTEXTW: u32 = 1081u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTM_WINDOWFROMPOINT: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTS_ALWAYSTIP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTS_BALLOON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTS_CLOSE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTS_NOANIMATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTS_NOFADE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTS_NOPREFIX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTS_USEVISUALSTYLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVCDRF_NOIMAGES: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_CARET: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_CHILD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_DROPHILITE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_FIRSTVISIBLE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_LASTVISIBLE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_NEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_NEXTSELECTED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_NEXTVISIBLE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_PARENT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_PREVIOUS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_PREVIOUSVISIBLE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGN_ROOT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVI_FIRST: HTREEITEM = HTREEITEM(-65535i32 as _);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVI_LAST: HTREEITEM = HTREEITEM(-65534i32 as _);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVI_ROOT: HTREEITEM = HTREEITEM(-65536i32 as _);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVI_SORT: HTREEITEM = HTREEITEM(-65533i32 as _);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_CREATEDRAGIMAGE: u32 = 4370u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_DELETEITEM: u32 = 4353u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_EDITLABEL: u32 = 4417u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_EDITLABELA: u32 = 4366u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_EDITLABELW: u32 = 4417u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_ENDEDITLABELNOW: u32 = 4374u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_ENSUREVISIBLE: u32 = 4372u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_EXPAND: u32 = 4354u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETBKCOLOR: u32 = 4383u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETCOUNT: u32 = 4357u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETEDITCONTROL: u32 = 4367u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETEXTENDEDSTYLE: u32 = 4397u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETIMAGELIST: u32 = 4360u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETINDENT: u32 = 4358u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETINSERTMARKCOLOR: u32 = 4390u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETISEARCHSTRING: u32 = 4416u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETISEARCHSTRINGA: u32 = 4375u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETISEARCHSTRINGW: u32 = 4416u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETITEM: u32 = 4414u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETITEMA: u32 = 4364u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETITEMHEIGHT: u32 = 4380u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETITEMPARTRECT: u32 = 4424u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETITEMRECT: u32 = 4356u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETITEMSTATE: u32 = 4391u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETITEMW: u32 = 4414u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETLINECOLOR: u32 = 4393u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETNEXTITEM: u32 = 4362u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETSCROLLTIME: u32 = 4386u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETSELECTEDCOUNT: u32 = 4422u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETTEXTCOLOR: u32 = 4384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETTOOLTIPS: u32 = 4377u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_GETVISIBLECOUNT: u32 = 4368u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_HITTEST: u32 = 4369u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_INSERTITEM: u32 = 4402u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_INSERTITEMA: u32 = 4352u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_INSERTITEMW: u32 = 4402u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_MAPACCIDTOHTREEITEM: u32 = 4394u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_MAPHTREEITEMTOACCID: u32 = 4395u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SELECTITEM: u32 = 4363u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETAUTOSCROLLINFO: u32 = 4411u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETBKCOLOR: u32 = 4381u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETBORDER: u32 = 4387u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETEXTENDEDSTYLE: u32 = 4396u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETHOT: u32 = 4410u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETIMAGELIST: u32 = 4361u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETINDENT: u32 = 4359u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETINSERTMARK: u32 = 4378u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETINSERTMARKCOLOR: u32 = 4389u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETITEM: u32 = 4415u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETITEMA: u32 = 4365u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETITEMHEIGHT: u32 = 4379u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETITEMW: u32 = 4415u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETLINECOLOR: u32 = 4392u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETSCROLLTIME: u32 = 4385u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETTEXTCOLOR: u32 = 4382u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETTOOLTIPS: u32 = 4376u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SHOWINFOTIP: u32 = 4423u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SORTCHILDREN: u32 = 4371u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVM_SORTCHILDRENCB: u32 = 4373u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVNRET_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVNRET_SKIPNEW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVNRET_SKIPOLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVSBF_XBORDER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVSBF_YBORDER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVSIL_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVSIL_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVSI_NOSINGLEEXPAND: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_CHECKBOXES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_DISABLEDRAGDROP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EDITLABELS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_AUTOHSCROLL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_DIMMEDCHECKBOXES: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_DOUBLEBUFFER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_DRAWIMAGEASYNC: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_EXCLUSIONCHECKBOXES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_FADEINOUTEXPANDOS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_MULTISELECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_NOINDENTSTATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_NOSINGLECOLLAPSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_PARTIALCHECKBOXES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_EX_RICHTOOLTIP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_FULLROWSELECT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_HASBUTTONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_HASLINES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_INFOTIP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_LINESATROOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_NOHSCROLL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_NONEVENHEIGHT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_NOSCROLL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_NOTOOLTIPS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_RTLREADING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_SHOWSELALWAYS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_SINGLEEXPAND: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVS_TRACKSELECT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TV_FIRST: u32 = 4352u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_GETACCEL: u32 = 1132u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_GETBASE: u32 = 1134u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_GETBUDDY: u32 = 1130u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_GETPOS: u32 = 1128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_GETPOS32: u32 = 1138u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_GETRANGE: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_GETRANGE32: u32 = 1136u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_SETACCEL: u32 = 1131u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_SETBASE: u32 = 1133u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_SETBUDDY: u32 = 1129u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_SETPOS: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_SETPOS32: u32 = 1137u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_SETRANGE: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_SETRANGE32: u32 = 1135u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDS_ALIGNLEFT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDS_ALIGNRIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDS_ARROWKEYS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDS_AUTOBUDDY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDS_HORZ: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDS_HOTTRACK: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDS_NOTHOUSANDS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDS_SETBUDDYINT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UDS_WRAP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UD_MAXVAL: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPDOWN_CLASS: ::windows::core::PCWSTR = ::windows::w!("msctls_updown32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPDOWN_CLASSA: ::windows::core::PCSTR = ::windows::s!("msctls_updown32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPDOWN_CLASSW: ::windows::core::PCWSTR = ::windows::w!("msctls_updown32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_DETAILS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_LARGEICONS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_LIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_NETCONNECT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_NETDISCONNECT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_NEWFOLDER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_PARENTFOLDER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_SMALLICONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_SORTDATE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_SORTNAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_SORTSIZE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_SORTTYPE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VIEW_VIEWMENU: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_AEROWIZARD: ::windows::core::PCWSTR = ::windows::w!("AEROWIZARD");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_AEROWIZARDSTYLE: ::windows::core::PCWSTR = ::windows::w!("AEROWIZARDSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_BUTTON: ::windows::core::PCWSTR = ::windows::w!("BUTTON");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_BUTTONSTYLE: ::windows::core::PCWSTR = ::windows::w!("BUTTONSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_CLOCK: ::windows::core::PCWSTR = ::windows::w!("CLOCK");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_COMBOBOX: ::windows::core::PCWSTR = ::windows::w!("COMBOBOX");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_COMBOBOXSTYLE: ::windows::core::PCWSTR = ::windows::w!("COMBOBOXSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_COMMUNICATIONS: ::windows::core::PCWSTR = ::windows::w!("COMMUNICATIONS");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_COMMUNICATIONSSTYLE: ::windows::core::PCWSTR = ::windows::w!("COMMUNICATIONSSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_CONTROLPANEL: ::windows::core::PCWSTR = ::windows::w!("CONTROLPANEL");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_CONTROLPANELSTYLE: ::windows::core::PCWSTR = ::windows::w!("CONTROLPANELSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_DATEPICKER: ::windows::core::PCWSTR = ::windows::w!("DATEPICKER");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_DATEPICKERSTYLE: ::windows::core::PCWSTR = ::windows::w!("DATEPICKERSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_DRAGDROP: ::windows::core::PCWSTR = ::windows::w!("DRAGDROP");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_DRAGDROPSTYLE: ::windows::core::PCWSTR = ::windows::w!("DRAGDROPSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_EDIT: ::windows::core::PCWSTR = ::windows::w!("EDIT");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_EDITSTYLE: ::windows::core::PCWSTR = ::windows::w!("EDITSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_EMPTYMARKUP: ::windows::core::PCWSTR = ::windows::w!("EMPTYMARKUP");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_EXPLORERBAR: ::windows::core::PCWSTR = ::windows::w!("EXPLORERBAR");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_EXPLORERBARSTYLE: ::windows::core::PCWSTR = ::windows::w!("EXPLORERBARSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_FLYOUT: ::windows::core::PCWSTR = ::windows::w!("FLYOUT");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_FLYOUTSTYLE: ::windows::core::PCWSTR = ::windows::w!("FLYOUTSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_HEADER: ::windows::core::PCWSTR = ::windows::w!("HEADER");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_HEADERSTYLE: ::windows::core::PCWSTR = ::windows::w!("HEADERSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_LINK: ::windows::core::PCWSTR = ::windows::w!("LINK");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_LISTBOX: ::windows::core::PCWSTR = ::windows::w!("LISTBOX");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_LISTBOXSTYLE: ::windows::core::PCWSTR = ::windows::w!("LISTBOXSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_LISTVIEW: ::windows::core::PCWSTR = ::windows::w!("LISTVIEW");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_LISTVIEWSTYLE: ::windows::core::PCWSTR = ::windows::w!("LISTVIEWSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_MENU: ::windows::core::PCWSTR = ::windows::w!("MENU");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_MENUBAND: ::windows::core::PCWSTR = ::windows::w!("MENUBAND");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_MENUSTYLE: ::windows::core::PCWSTR = ::windows::w!("MENUSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_MONTHCAL: ::windows::core::PCWSTR = ::windows::w!("MONTHCAL");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_NAVIGATION: ::windows::core::PCWSTR = ::windows::w!("NAVIGATION");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_PAGE: ::windows::core::PCWSTR = ::windows::w!("PAGE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_PROGRESS: ::windows::core::PCWSTR = ::windows::w!("PROGRESS");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_PROGRESSSTYLE: ::windows::core::PCWSTR = ::windows::w!("PROGRESSSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_REBAR: ::windows::core::PCWSTR = ::windows::w!("REBAR");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_REBARSTYLE: ::windows::core::PCWSTR = ::windows::w!("REBARSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_SCROLLBAR: ::windows::core::PCWSTR = ::windows::w!("SCROLLBAR");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_SCROLLBARSTYLE: ::windows::core::PCWSTR = ::windows::w!("SCROLLBARSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_SPIN: ::windows::core::PCWSTR = ::windows::w!("SPIN");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_SPINSTYLE: ::windows::core::PCWSTR = ::windows::w!("SPINSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_STARTPANEL: ::windows::core::PCWSTR = ::windows::w!("STARTPANEL");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_STATIC: ::windows::core::PCWSTR = ::windows::w!("STATIC");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_STATUS: ::windows::core::PCWSTR = ::windows::w!("STATUS");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_STATUSSTYLE: ::windows::core::PCWSTR = ::windows::w!("STATUSSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TAB: ::windows::core::PCWSTR = ::windows::w!("TAB");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TABSTYLE: ::windows::core::PCWSTR = ::windows::w!("TABSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TASKBAND: ::windows::core::PCWSTR = ::windows::w!("TASKBAND");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TASKBAR: ::windows::core::PCWSTR = ::windows::w!("TASKBAR");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TASKDIALOG: ::windows::core::PCWSTR = ::windows::w!("TASKDIALOG");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TASKDIALOGSTYLE: ::windows::core::PCWSTR = ::windows::w!("TASKDIALOGSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TEXTSELECTIONGRIPPER: ::windows::core::PCWSTR = ::windows::w!("TEXTSELECTIONGRIPPER");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TEXTSTYLE: ::windows::core::PCWSTR = ::windows::w!("TEXTSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TOOLBAR: ::windows::core::PCWSTR = ::windows::w!("TOOLBAR");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TOOLBARSTYLE: ::windows::core::PCWSTR = ::windows::w!("TOOLBARSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TOOLTIP: ::windows::core::PCWSTR = ::windows::w!("TOOLTIP");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TOOLTIPSTYLE: ::windows::core::PCWSTR = ::windows::w!("TOOLTIPSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TRACKBAR: ::windows::core::PCWSTR = ::windows::w!("TRACKBAR");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TRACKBARSTYLE: ::windows::core::PCWSTR = ::windows::w!("TRACKBARSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TRAYNOTIFY: ::windows::core::PCWSTR = ::windows::w!("TRAYNOTIFY");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TREEVIEW: ::windows::core::PCWSTR = ::windows::w!("TREEVIEW");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_TREEVIEWSTYLE: ::windows::core::PCWSTR = ::windows::w!("TREEVIEWSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_USERTILE: ::windows::core::PCWSTR = ::windows::w!("USERTILE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_WINDOW: ::windows::core::PCWSTR = ::windows::w!("WINDOW");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSCLASS_WINDOWSTYLE: ::windows::core::PCWSTR = ::windows::w!("WINDOWSTYLE");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_BUTTON: ::windows::core::PCWSTR = ::windows::w!("Button");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_BUTTONA: ::windows::core::PCSTR = ::windows::s!("Button");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_BUTTONW: ::windows::core::PCWSTR = ::windows::w!("Button");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_COMBOBOX: ::windows::core::PCWSTR = ::windows::w!("ComboBox");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_COMBOBOXA: ::windows::core::PCSTR = ::windows::s!("ComboBox");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_COMBOBOXEX: ::windows::core::PCWSTR = ::windows::w!("ComboBoxEx32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_COMBOBOXEXA: ::windows::core::PCSTR = ::windows::s!("ComboBoxEx32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_COMBOBOXEXW: ::windows::core::PCWSTR = ::windows::w!("ComboBoxEx32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_COMBOBOXW: ::windows::core::PCWSTR = ::windows::w!("ComboBox");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_EDIT: ::windows::core::PCWSTR = ::windows::w!("Edit");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_EDITA: ::windows::core::PCSTR = ::windows::s!("Edit");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_EDITW: ::windows::core::PCWSTR = ::windows::w!("Edit");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_HEADER: ::windows::core::PCWSTR = ::windows::w!("SysHeader32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_HEADERA: ::windows::core::PCSTR = ::windows::s!("SysHeader32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_HEADERW: ::windows::core::PCWSTR = ::windows::w!("SysHeader32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_IPADDRESS: ::windows::core::PCWSTR = ::windows::w!("SysIPAddress32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_IPADDRESSA: ::windows::core::PCSTR = ::windows::s!("SysIPAddress32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_IPADDRESSW: ::windows::core::PCWSTR = ::windows::w!("SysIPAddress32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_LINK: ::windows::core::PCWSTR = ::windows::w!("SysLink");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_LISTBOX: ::windows::core::PCWSTR = ::windows::w!("ListBox");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_LISTBOXA: ::windows::core::PCSTR = ::windows::s!("ListBox");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_LISTBOXW: ::windows::core::PCWSTR = ::windows::w!("ListBox");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_LISTVIEW: ::windows::core::PCWSTR = ::windows::w!("SysListView32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_LISTVIEWA: ::windows::core::PCSTR = ::windows::s!("SysListView32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_LISTVIEWW: ::windows::core::PCWSTR = ::windows::w!("SysListView32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_NATIVEFONTCTL: ::windows::core::PCWSTR = ::windows::w!("NativeFontCtl");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_NATIVEFONTCTLA: ::windows::core::PCSTR = ::windows::s!("NativeFontCtl");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_NATIVEFONTCTLW: ::windows::core::PCWSTR = ::windows::w!("NativeFontCtl");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_PAGESCROLLER: ::windows::core::PCWSTR = ::windows::w!("SysPager");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_PAGESCROLLERA: ::windows::core::PCSTR = ::windows::s!("SysPager");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_PAGESCROLLERW: ::windows::core::PCWSTR = ::windows::w!("SysPager");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_SCROLLBAR: ::windows::core::PCWSTR = ::windows::w!("ScrollBar");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_SCROLLBARA: ::windows::core::PCSTR = ::windows::s!("ScrollBar");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_SCROLLBARW: ::windows::core::PCWSTR = ::windows::w!("ScrollBar");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_STATIC: ::windows::core::PCWSTR = ::windows::w!("Static");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_STATICA: ::windows::core::PCSTR = ::windows::s!("Static");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_STATICW: ::windows::core::PCWSTR = ::windows::w!("Static");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_TABCONTROL: ::windows::core::PCWSTR = ::windows::w!("SysTabControl32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_TABCONTROLA: ::windows::core::PCSTR = ::windows::s!("SysTabControl32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_TABCONTROLW: ::windows::core::PCWSTR = ::windows::w!("SysTabControl32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_TREEVIEW: ::windows::core::PCWSTR = ::windows::w!("SysTreeView32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_TREEVIEWA: ::windows::core::PCSTR = ::windows::s!("SysTreeView32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WC_TREEVIEWW: ::windows::core::PCWSTR = ::windows::w!("SysTreeView32");
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WIZ_BODYCX: u32 = 184u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WIZ_BODYX: u32 = 92u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WIZ_CXBMP: u32 = 80u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WIZ_CXDLG: u32 = 276u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WIZ_CYDLG: u32 = 140u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WM_CTLCOLOR: u32 = 25u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WM_MOUSEHOVER: u32 = 673u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WM_MOUSELEAVE: u32 = 675u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_MASK: i32 = 4095i32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WTNCA_NODRAWCAPTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WTNCA_NODRAWICON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WTNCA_NOMIRRORHELP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WTNCA_NOSYSMENU: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx1: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx10: u32 = 1049u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx11: u32 = 1050u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx12: u32 = 1051u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx13: u32 = 1052u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx14: u32 = 1053u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx15: u32 = 1054u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx16: u32 = 1055u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx2: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx3: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx4: u32 = 1043u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx5: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx6: u32 = 1045u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx7: u32 = 1046u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx8: u32 = 1047u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const chx9: u32 = 1048u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb1: u32 = 1136u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb10: u32 = 1145u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb11: u32 = 1146u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb12: u32 = 1147u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb13: u32 = 1148u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb14: u32 = 1149u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb15: u32 = 1150u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb16: u32 = 1151u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb2: u32 = 1137u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb3: u32 = 1138u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb4: u32 = 1139u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb5: u32 = 1140u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb6: u32 = 1141u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb7: u32 = 1142u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb8: u32 = 1143u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const cmb9: u32 = 1144u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ctl1: u32 = 1184u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ctlFirst: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ctlLast: u32 = 1279u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt1: u32 = 1152u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt10: u32 = 1161u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt11: u32 = 1162u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt12: u32 = 1163u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt13: u32 = 1164u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt14: u32 = 1165u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt15: u32 = 1166u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt16: u32 = 1167u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt2: u32 = 1153u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt3: u32 = 1154u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt4: u32 = 1155u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt5: u32 = 1156u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt6: u32 = 1157u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt7: u32 = 1158u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt8: u32 = 1159u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const edt9: u32 = 1160u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const frm1: u32 = 1076u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const frm2: u32 = 1077u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const frm3: u32 = 1078u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const frm4: u32 = 1079u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const grp1: u32 = 1072u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const grp2: u32 = 1073u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const grp3: u32 = 1074u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const grp4: u32 = 1075u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ico1: u32 = 1084u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ico2: u32 = 1085u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ico3: u32 = 1086u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ico4: u32 = 1087u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst1: u32 = 1120u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst10: u32 = 1129u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst11: u32 = 1130u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst12: u32 = 1131u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst13: u32 = 1132u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst14: u32 = 1133u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst15: u32 = 1134u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst16: u32 = 1135u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst2: u32 = 1121u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst3: u32 = 1122u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst4: u32 = 1123u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst5: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst6: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst7: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst8: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const lst9: u32 = 1128u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh1: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh10: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh11: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh12: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh13: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh14: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh15: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh16: u32 = 1039u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh2: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh3: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh4: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh5: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh6: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh7: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh8: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const psh9: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const pshHelp: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad1: u32 = 1056u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad10: u32 = 1065u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad11: u32 = 1066u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad12: u32 = 1067u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad13: u32 = 1068u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad14: u32 = 1069u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad15: u32 = 1070u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad16: u32 = 1071u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad2: u32 = 1057u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad3: u32 = 1058u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad4: u32 = 1059u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad5: u32 = 1060u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad6: u32 = 1061u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad7: u32 = 1062u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad8: u32 = 1063u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rad9: u32 = 1064u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rct1: u32 = 1080u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rct2: u32 = 1081u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rct3: u32 = 1082u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const rct4: u32 = 1083u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const scr1: u32 = 1168u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const scr2: u32 = 1169u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const scr3: u32 = 1170u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const scr4: u32 = 1171u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const scr5: u32 = 1172u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const scr6: u32 = 1173u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const scr7: u32 = 1174u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const scr8: u32 = 1175u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc1: u32 = 1088u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc10: u32 = 1097u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc11: u32 = 1098u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc12: u32 = 1099u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc13: u32 = 1100u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc14: u32 = 1101u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc15: u32 = 1102u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc16: u32 = 1103u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc17: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc18: u32 = 1105u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc19: u32 = 1106u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc2: u32 = 1089u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc20: u32 = 1107u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc21: u32 = 1108u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc22: u32 = 1109u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc23: u32 = 1110u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc24: u32 = 1111u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc25: u32 = 1112u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc26: u32 = 1113u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc27: u32 = 1114u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc28: u32 = 1115u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc29: u32 = 1116u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc3: u32 = 1090u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc30: u32 = 1117u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc31: u32 = 1118u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc32: u32 = 1119u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc4: u32 = 1091u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc5: u32 = 1092u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc6: u32 = 1093u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc7: u32 = 1094u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc8: u32 = 1095u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const stc9: u32 = 1096u32;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AEROWIZARDPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const AW_TITLEBAR: AEROWIZARDPARTS = AEROWIZARDPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const AW_HEADERAREA: AEROWIZARDPARTS = AEROWIZARDPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const AW_CONTENTAREA: AEROWIZARDPARTS = AEROWIZARDPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const AW_COMMANDAREA: AEROWIZARDPARTS = AEROWIZARDPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const AW_BUTTON: AEROWIZARDPARTS = AEROWIZARDPARTS(5i32);
impl ::core::marker::Copy for AEROWIZARDPARTS {}
impl ::core::clone::Clone for AEROWIZARDPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AEROWIZARDPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ARROWBTNSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_UPNORMAL: ARROWBTNSTATES = ARROWBTNSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_UPHOT: ARROWBTNSTATES = ARROWBTNSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_UPPRESSED: ARROWBTNSTATES = ARROWBTNSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_UPDISABLED: ARROWBTNSTATES = ARROWBTNSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_DOWNNORMAL: ARROWBTNSTATES = ARROWBTNSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_DOWNHOT: ARROWBTNSTATES = ARROWBTNSTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_DOWNPRESSED: ARROWBTNSTATES = ARROWBTNSTATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_DOWNDISABLED: ARROWBTNSTATES = ARROWBTNSTATES(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_LEFTNORMAL: ARROWBTNSTATES = ARROWBTNSTATES(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_LEFTHOT: ARROWBTNSTATES = ARROWBTNSTATES(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_LEFTPRESSED: ARROWBTNSTATES = ARROWBTNSTATES(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_LEFTDISABLED: ARROWBTNSTATES = ARROWBTNSTATES(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_RIGHTNORMAL: ARROWBTNSTATES = ARROWBTNSTATES(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_RIGHTHOT: ARROWBTNSTATES = ARROWBTNSTATES(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_RIGHTPRESSED: ARROWBTNSTATES = ARROWBTNSTATES(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_RIGHTDISABLED: ARROWBTNSTATES = ARROWBTNSTATES(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_UPHOVER: ARROWBTNSTATES = ARROWBTNSTATES(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_DOWNHOVER: ARROWBTNSTATES = ARROWBTNSTATES(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_LEFTHOVER: ARROWBTNSTATES = ARROWBTNSTATES(19i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ABS_RIGHTHOVER: ARROWBTNSTATES = ARROWBTNSTATES(20i32);
impl ::core::marker::Copy for ARROWBTNSTATES {}
impl ::core::clone::Clone for ARROWBTNSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ARROWBTNSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BACKGROUNDSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBS_NORMAL: BACKGROUNDSTATES = BACKGROUNDSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBS_HOT: BACKGROUNDSTATES = BACKGROUNDSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBS_DISABLED: BACKGROUNDSTATES = BACKGROUNDSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBS_FOCUSED: BACKGROUNDSTATES = BACKGROUNDSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBS_READONLY: BACKGROUNDSTATES = BACKGROUNDSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBS_ASSIST: BACKGROUNDSTATES = BACKGROUNDSTATES(6i32);
impl ::core::marker::Copy for BACKGROUNDSTATES {}
impl ::core::clone::Clone for BACKGROUNDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BACKGROUNDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BACKGROUNDWITHBORDERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBWBS_NORMAL: BACKGROUNDWITHBORDERSTATES = BACKGROUNDWITHBORDERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBWBS_HOT: BACKGROUNDWITHBORDERSTATES = BACKGROUNDWITHBORDERSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBWBS_DISABLED: BACKGROUNDWITHBORDERSTATES = BACKGROUNDWITHBORDERSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBWBS_FOCUSED: BACKGROUNDWITHBORDERSTATES = BACKGROUNDWITHBORDERSTATES(4i32);
impl ::core::marker::Copy for BACKGROUNDWITHBORDERSTATES {}
impl ::core::clone::Clone for BACKGROUNDWITHBORDERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BACKGROUNDWITHBORDERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BALLOONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTBS_NORMAL: BALLOONSTATES = BALLOONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTBS_LINK: BALLOONSTATES = BALLOONSTATES(2i32);
impl ::core::marker::Copy for BALLOONSTATES {}
impl ::core::clone::Clone for BALLOONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BALLOONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BALLOONSTEMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTBSS_POINTINGUPLEFTWALL: BALLOONSTEMSTATES = BALLOONSTEMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTBSS_POINTINGUPCENTERED: BALLOONSTEMSTATES = BALLOONSTEMSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTBSS_POINTINGUPRIGHTWALL: BALLOONSTEMSTATES = BALLOONSTEMSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTBSS_POINTINGDOWNRIGHTWALL: BALLOONSTEMSTATES = BALLOONSTEMSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTBSS_POINTINGDOWNCENTERED: BALLOONSTEMSTATES = BALLOONSTEMSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTBSS_POINTINGDOWNLEFTWALL: BALLOONSTEMSTATES = BALLOONSTEMSTATES(6i32);
impl ::core::marker::Copy for BALLOONSTEMSTATES {}
impl ::core::clone::Clone for BALLOONSTEMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BALLOONSTEMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BARBACKGROUNDSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MB_ACTIVE: BARBACKGROUNDSTATES = BARBACKGROUNDSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MB_INACTIVE: BARBACKGROUNDSTATES = BARBACKGROUNDSTATES(2i32);
impl ::core::marker::Copy for BARBACKGROUNDSTATES {}
impl ::core::clone::Clone for BARBACKGROUNDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BARBACKGROUNDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BARITEMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MBI_NORMAL: BARITEMSTATES = BARITEMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MBI_HOT: BARITEMSTATES = BARITEMSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MBI_PUSHED: BARITEMSTATES = BARITEMSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MBI_DISABLED: BARITEMSTATES = BARITEMSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MBI_DISABLEDHOT: BARITEMSTATES = BARITEMSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MBI_DISABLEDPUSHED: BARITEMSTATES = BARITEMSTATES(6i32);
impl ::core::marker::Copy for BARITEMSTATES {}
impl ::core::clone::Clone for BARITEMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BARITEMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BGTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BT_IMAGEFILE: BGTYPE = BGTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BT_BORDERFILL: BGTYPE = BGTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BT_NONE: BGTYPE = BGTYPE(2i32);
impl ::core::marker::Copy for BGTYPE {}
impl ::core::clone::Clone for BGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BGTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BODYSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FBS_NORMAL: BODYSTATES = BODYSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FBS_EMPHASIZED: BODYSTATES = BODYSTATES(2i32);
impl ::core::marker::Copy for BODYSTATES {}
impl ::core::clone::Clone for BODYSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BODYSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BORDERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBB_NORMAL: BORDERSTATES = BORDERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBB_HOT: BORDERSTATES = BORDERSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBB_FOCUSED: BORDERSTATES = BORDERSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBB_DISABLED: BORDERSTATES = BORDERSTATES(4i32);
impl ::core::marker::Copy for BORDERSTATES {}
impl ::core::clone::Clone for BORDERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BORDERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BORDERTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BT_RECT: BORDERTYPE = BORDERTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BT_ROUNDRECT: BORDERTYPE = BORDERTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BT_ELLIPSE: BORDERTYPE = BORDERTYPE(2i32);
impl ::core::marker::Copy for BORDERTYPE {}
impl ::core::clone::Clone for BORDERTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BORDERTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BORDER_HSCROLLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSH_NORMAL: BORDER_HSCROLLSTATES = BORDER_HSCROLLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSH_FOCUSED: BORDER_HSCROLLSTATES = BORDER_HSCROLLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSH_HOT: BORDER_HSCROLLSTATES = BORDER_HSCROLLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSH_DISABLED: BORDER_HSCROLLSTATES = BORDER_HSCROLLSTATES(4i32);
impl ::core::marker::Copy for BORDER_HSCROLLSTATES {}
impl ::core::clone::Clone for BORDER_HSCROLLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BORDER_HSCROLLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BORDER_HVSCROLLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSHV_NORMAL: BORDER_HVSCROLLSTATES = BORDER_HVSCROLLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSHV_FOCUSED: BORDER_HVSCROLLSTATES = BORDER_HVSCROLLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSHV_HOT: BORDER_HVSCROLLSTATES = BORDER_HVSCROLLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSHV_DISABLED: BORDER_HVSCROLLSTATES = BORDER_HVSCROLLSTATES(4i32);
impl ::core::marker::Copy for BORDER_HVSCROLLSTATES {}
impl ::core::clone::Clone for BORDER_HVSCROLLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BORDER_HVSCROLLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BORDER_NOSCROLLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSN_NORMAL: BORDER_NOSCROLLSTATES = BORDER_NOSCROLLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSN_FOCUSED: BORDER_NOSCROLLSTATES = BORDER_NOSCROLLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSN_HOT: BORDER_NOSCROLLSTATES = BORDER_NOSCROLLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSN_DISABLED: BORDER_NOSCROLLSTATES = BORDER_NOSCROLLSTATES(4i32);
impl ::core::marker::Copy for BORDER_NOSCROLLSTATES {}
impl ::core::clone::Clone for BORDER_NOSCROLLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BORDER_NOSCROLLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BORDER_VSCROLLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSV_NORMAL: BORDER_VSCROLLSTATES = BORDER_VSCROLLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSV_FOCUSED: BORDER_VSCROLLSTATES = BORDER_VSCROLLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSV_HOT: BORDER_VSCROLLSTATES = BORDER_VSCROLLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSV_DISABLED: BORDER_VSCROLLSTATES = BORDER_VSCROLLSTATES(4i32);
impl ::core::marker::Copy for BORDER_VSCROLLSTATES {}
impl ::core::clone::Clone for BORDER_VSCROLLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BORDER_VSCROLLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BP_ANIMATIONSTYLE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPAS_NONE: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPAS_LINEAR: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPAS_CUBIC: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPAS_SINE: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(3i32);
impl ::core::marker::Copy for BP_ANIMATIONSTYLE {}
impl ::core::clone::Clone for BP_ANIMATIONSTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BP_ANIMATIONSTYLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BP_BUFFERFORMAT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPBF_COMPATIBLEBITMAP: BP_BUFFERFORMAT = BP_BUFFERFORMAT(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPBF_DIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPBF_TOPDOWNDIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPBF_TOPDOWNMONODIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(3i32);
impl ::core::marker::Copy for BP_BUFFERFORMAT {}
impl ::core::clone::Clone for BP_BUFFERFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BP_BUFFERFORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BP_PAINTPARAMS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPPF_ERASE: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPPF_NOCLIP: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BPPF_NONCLIENT: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(4u32);
impl ::core::marker::Copy for BP_PAINTPARAMS_FLAGS {}
impl ::core::clone::Clone for BP_PAINTPARAMS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BP_PAINTPARAMS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BUTTONPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_PUSHBUTTON: BUTTONPARTS = BUTTONPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_RADIOBUTTON: BUTTONPARTS = BUTTONPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_CHECKBOX: BUTTONPARTS = BUTTONPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_GROUPBOX: BUTTONPARTS = BUTTONPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_USERBUTTON: BUTTONPARTS = BUTTONPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_COMMANDLINK: BUTTONPARTS = BUTTONPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_COMMANDLINKGLYPH: BUTTONPARTS = BUTTONPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_RADIOBUTTON_HCDISABLED: BUTTONPARTS = BUTTONPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_CHECKBOX_HCDISABLED: BUTTONPARTS = BUTTONPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_GROUPBOX_HCDISABLED: BUTTONPARTS = BUTTONPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BP_PUSHBUTTONDROPDOWN: BUTTONPARTS = BUTTONPARTS(11i32);
impl ::core::marker::Copy for BUTTONPARTS {}
impl ::core::clone::Clone for BUTTONPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BUTTONPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BUTTON_IMAGELIST_ALIGN(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BUTTON_IMAGELIST_ALIGN_LEFT: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BUTTON_IMAGELIST_ALIGN_RIGHT: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BUTTON_IMAGELIST_ALIGN_TOP: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BUTTON_IMAGELIST_ALIGN_BOTTOM: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BUTTON_IMAGELIST_ALIGN_CENTER: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(4u32);
impl ::core::marker::Copy for BUTTON_IMAGELIST_ALIGN {}
impl ::core::clone::Clone for BUTTON_IMAGELIST_ALIGN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BUTTON_IMAGELIST_ALIGN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CAPTIONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CS_ACTIVE: CAPTIONSTATES = CAPTIONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CS_INACTIVE: CAPTIONSTATES = CAPTIONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CS_DISABLED: CAPTIONSTATES = CAPTIONSTATES(3i32);
impl ::core::marker::Copy for CAPTIONSTATES {}
impl ::core::clone::Clone for CAPTIONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CAPTIONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHECKBOXSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_UNCHECKEDNORMAL: CHECKBOXSTATES = CHECKBOXSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_UNCHECKEDHOT: CHECKBOXSTATES = CHECKBOXSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_UNCHECKEDPRESSED: CHECKBOXSTATES = CHECKBOXSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_UNCHECKEDDISABLED: CHECKBOXSTATES = CHECKBOXSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_CHECKEDNORMAL: CHECKBOXSTATES = CHECKBOXSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_CHECKEDHOT: CHECKBOXSTATES = CHECKBOXSTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_CHECKEDPRESSED: CHECKBOXSTATES = CHECKBOXSTATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_CHECKEDDISABLED: CHECKBOXSTATES = CHECKBOXSTATES(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_MIXEDNORMAL: CHECKBOXSTATES = CHECKBOXSTATES(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_MIXEDHOT: CHECKBOXSTATES = CHECKBOXSTATES(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_MIXEDPRESSED: CHECKBOXSTATES = CHECKBOXSTATES(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_MIXEDDISABLED: CHECKBOXSTATES = CHECKBOXSTATES(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_IMPLICITNORMAL: CHECKBOXSTATES = CHECKBOXSTATES(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_IMPLICITHOT: CHECKBOXSTATES = CHECKBOXSTATES(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_IMPLICITPRESSED: CHECKBOXSTATES = CHECKBOXSTATES(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_IMPLICITDISABLED: CHECKBOXSTATES = CHECKBOXSTATES(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_EXCLUDEDNORMAL: CHECKBOXSTATES = CHECKBOXSTATES(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_EXCLUDEDHOT: CHECKBOXSTATES = CHECKBOXSTATES(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_EXCLUDEDPRESSED: CHECKBOXSTATES = CHECKBOXSTATES(19i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_EXCLUDEDDISABLED: CHECKBOXSTATES = CHECKBOXSTATES(20i32);
impl ::core::marker::Copy for CHECKBOXSTATES {}
impl ::core::clone::Clone for CHECKBOXSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CHECKBOXSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHEVRONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CHEVS_NORMAL: CHEVRONSTATES = CHEVRONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CHEVS_HOT: CHEVRONSTATES = CHEVRONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CHEVS_PRESSED: CHEVRONSTATES = CHEVRONSTATES(3i32);
impl ::core::marker::Copy for CHEVRONSTATES {}
impl ::core::clone::Clone for CHEVRONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CHEVRONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHEVRONVERTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CHEVSV_NORMAL: CHEVRONVERTSTATES = CHEVRONVERTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CHEVSV_HOT: CHEVRONVERTSTATES = CHEVRONVERTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CHEVSV_PRESSED: CHEVRONVERTSTATES = CHEVRONVERTSTATES(3i32);
impl ::core::marker::Copy for CHEVRONVERTSTATES {}
impl ::core::clone::Clone for CHEVRONVERTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CHEVRONVERTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLOCKPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CLP_TIME: CLOCKPARTS = CLOCKPARTS(1i32);
impl ::core::marker::Copy for CLOCKPARTS {}
impl ::core::clone::Clone for CLOCKPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLOCKPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLOCKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CLS_NORMAL: CLOCKSTATES = CLOCKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CLS_HOT: CLOCKSTATES = CLOCKSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CLS_PRESSED: CLOCKSTATES = CLOCKSTATES(3i32);
impl ::core::marker::Copy for CLOCKSTATES {}
impl ::core::clone::Clone for CLOCKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLOCKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLOSEBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_NORMAL: CLOSEBUTTONSTATES = CLOSEBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_HOT: CLOSEBUTTONSTATES = CLOSEBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_PUSHED: CLOSEBUTTONSTATES = CLOSEBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBS_DISABLED: CLOSEBUTTONSTATES = CLOSEBUTTONSTATES(4i32);
impl ::core::marker::Copy for CLOSEBUTTONSTATES {}
impl ::core::clone::Clone for CLOSEBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLOSEBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLOSESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTCS_NORMAL: CLOSESTATES = CLOSESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTCS_HOT: CLOSESTATES = CLOSESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTCS_PRESSED: CLOSESTATES = CLOSESTATES(3i32);
impl ::core::marker::Copy for CLOSESTATES {}
impl ::core::clone::Clone for CLOSESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLOSESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COLLAPSEBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCB_NORMAL: COLLAPSEBUTTONSTATES = COLLAPSEBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCB_HOVER: COLLAPSEBUTTONSTATES = COLLAPSEBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCB_PUSHED: COLLAPSEBUTTONSTATES = COLLAPSEBUTTONSTATES(3i32);
impl ::core::marker::Copy for COLLAPSEBUTTONSTATES {}
impl ::core::clone::Clone for COLLAPSEBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for COLLAPSEBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMBOBOXINFO_BUTTON_STATE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STATE_SYSTEM_INVISIBLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STATE_SYSTEM_PRESSED: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STATE_SYSTEM_FOCUSABLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(1048576u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STATE_SYSTEM_OFFSCREEN: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STATE_SYSTEM_UNAVAILABLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(1u32);
impl ::core::marker::Copy for COMBOBOXINFO_BUTTON_STATE {}
impl ::core::clone::Clone for COMBOBOXINFO_BUTTON_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for COMBOBOXINFO_BUTTON_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMBOBOXPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CP_DROPDOWNBUTTON: COMBOBOXPARTS = COMBOBOXPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CP_BACKGROUND: COMBOBOXPARTS = COMBOBOXPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CP_TRANSPARENTBACKGROUND: COMBOBOXPARTS = COMBOBOXPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CP_BORDER: COMBOBOXPARTS = COMBOBOXPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CP_READONLY: COMBOBOXPARTS = COMBOBOXPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CP_DROPDOWNBUTTONRIGHT: COMBOBOXPARTS = COMBOBOXPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CP_DROPDOWNBUTTONLEFT: COMBOBOXPARTS = COMBOBOXPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CP_CUEBANNER: COMBOBOXPARTS = COMBOBOXPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CP_DROPDOWNITEM: COMBOBOXPARTS = COMBOBOXPARTS(9i32);
impl ::core::marker::Copy for COMBOBOXPARTS {}
impl ::core::clone::Clone for COMBOBOXPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for COMBOBOXPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMBOBOXSTYLESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXS_NORMAL: COMBOBOXSTYLESTATES = COMBOBOXSTYLESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXS_HOT: COMBOBOXSTYLESTATES = COMBOBOXSTYLESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXS_PRESSED: COMBOBOXSTYLESTATES = COMBOBOXSTYLESTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXS_DISABLED: COMBOBOXSTYLESTATES = COMBOBOXSTYLESTATES(4i32);
impl ::core::marker::Copy for COMBOBOXSTYLESTATES {}
impl ::core::clone::Clone for COMBOBOXSTYLESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for COMBOBOXSTYLESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMBOBOX_EX_ITEM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEIF_DI_SETITEM: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEIF_IMAGE: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEIF_INDENT: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEIF_LPARAM: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEIF_OVERLAY: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEIF_SELECTEDIMAGE: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBEIF_TEXT: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(1u32);
impl ::core::marker::Copy for COMBOBOX_EX_ITEM_FLAGS {}
impl ::core::clone::Clone for COMBOBOX_EX_ITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for COMBOBOX_EX_ITEM_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMMANDLINKGLYPHSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLGS_NORMAL: COMMANDLINKGLYPHSTATES = COMMANDLINKGLYPHSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLGS_HOT: COMMANDLINKGLYPHSTATES = COMMANDLINKGLYPHSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLGS_PRESSED: COMMANDLINKGLYPHSTATES = COMMANDLINKGLYPHSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLGS_DISABLED: COMMANDLINKGLYPHSTATES = COMMANDLINKGLYPHSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLGS_DEFAULTED: COMMANDLINKGLYPHSTATES = COMMANDLINKGLYPHSTATES(5i32);
impl ::core::marker::Copy for COMMANDLINKGLYPHSTATES {}
impl ::core::clone::Clone for COMMANDLINKGLYPHSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for COMMANDLINKGLYPHSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMMANDLINKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLS_NORMAL: COMMANDLINKSTATES = COMMANDLINKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLS_HOT: COMMANDLINKSTATES = COMMANDLINKSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLS_PRESSED: COMMANDLINKSTATES = COMMANDLINKSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLS_DISABLED: COMMANDLINKSTATES = COMMANDLINKSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLS_DEFAULTED: COMMANDLINKSTATES = COMMANDLINKSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CMDLS_DEFAULTED_ANIMATING: COMMANDLINKSTATES = COMMANDLINKSTATES(6i32);
impl ::core::marker::Copy for COMMANDLINKSTATES {}
impl ::core::clone::Clone for COMMANDLINKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for COMMANDLINKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMMUNICATIONSPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CSST_TAB: COMMUNICATIONSPARTS = COMMUNICATIONSPARTS(1i32);
impl ::core::marker::Copy for COMMUNICATIONSPARTS {}
impl ::core::clone::Clone for COMMUNICATIONSPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for COMMUNICATIONSPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONTENTALIGNMENT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CA_LEFT: CONTENTALIGNMENT = CONTENTALIGNMENT(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CA_CENTER: CONTENTALIGNMENT = CONTENTALIGNMENT(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CA_RIGHT: CONTENTALIGNMENT = CONTENTALIGNMENT(2i32);
impl ::core::marker::Copy for CONTENTALIGNMENT {}
impl ::core::clone::Clone for CONTENTALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONTENTALIGNMENT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONTENTAREASTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const AW_S_CONTENTAREA_NOMARGIN: CONTENTAREASTATES = CONTENTAREASTATES(1i32);
impl ::core::marker::Copy for CONTENTAREASTATES {}
impl ::core::clone::Clone for CONTENTAREASTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONTENTAREASTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONTENTLINKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPCL_NORMAL: CONTENTLINKSTATES = CONTENTLINKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPCL_HOT: CONTENTLINKSTATES = CONTENTLINKSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPCL_PRESSED: CONTENTLINKSTATES = CONTENTLINKSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPCL_DISABLED: CONTENTLINKSTATES = CONTENTLINKSTATES(4i32);
impl ::core::marker::Copy for CONTENTLINKSTATES {}
impl ::core::clone::Clone for CONTENTLINKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONTENTLINKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONTENTPANESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLGCPS_STANDALONE: CONTENTPANESTATES = CONTENTPANESTATES(1i32);
impl ::core::marker::Copy for CONTENTPANESTATES {}
impl ::core::clone::Clone for CONTENTPANESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONTENTPANESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONTROLLABELSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_CONTROLLABEL_NORMAL: CONTROLLABELSTATES = CONTROLLABELSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_CONTROLLABEL_DISABLED: CONTROLLABELSTATES = CONTROLLABELSTATES(2i32);
impl ::core::marker::Copy for CONTROLLABELSTATES {}
impl ::core::clone::Clone for CONTROLLABELSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONTROLLABELSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONTROLPANELPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_NAVIGATIONPANE: CONTROLPANELPARTS = CONTROLPANELPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_CONTENTPANE: CONTROLPANELPARTS = CONTROLPANELPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_NAVIGATIONPANELABEL: CONTROLPANELPARTS = CONTROLPANELPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_CONTENTPANELABEL: CONTROLPANELPARTS = CONTROLPANELPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_TITLE: CONTROLPANELPARTS = CONTROLPANELPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_BODYTEXT: CONTROLPANELPARTS = CONTROLPANELPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_HELPLINK: CONTROLPANELPARTS = CONTROLPANELPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_TASKLINK: CONTROLPANELPARTS = CONTROLPANELPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_GROUPTEXT: CONTROLPANELPARTS = CONTROLPANELPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_CONTENTLINK: CONTROLPANELPARTS = CONTROLPANELPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_SECTIONTITLELINK: CONTROLPANELPARTS = CONTROLPANELPARTS(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_LARGECOMMANDAREA: CONTROLPANELPARTS = CONTROLPANELPARTS(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_SMALLCOMMANDAREA: CONTROLPANELPARTS = CONTROLPANELPARTS(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_BUTTON: CONTROLPANELPARTS = CONTROLPANELPARTS(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_MESSAGETEXT: CONTROLPANELPARTS = CONTROLPANELPARTS(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_NAVIGATIONPANELINE: CONTROLPANELPARTS = CONTROLPANELPARTS(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_CONTENTPANELINE: CONTROLPANELPARTS = CONTROLPANELPARTS(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_BANNERAREA: CONTROLPANELPARTS = CONTROLPANELPARTS(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPANEL_BODYTITLE: CONTROLPANELPARTS = CONTROLPANELPARTS(19i32);
impl ::core::marker::Copy for CONTROLPANELPARTS {}
impl ::core::clone::Clone for CONTROLPANELPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONTROLPANELPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COPYSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDCOPY_HIGHLIGHT: COPYSTATES = COPYSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDCOPY_NOHIGHLIGHT: COPYSTATES = COPYSTATES(2i32);
impl ::core::marker::Copy for COPYSTATES {}
impl ::core::clone::Clone for COPYSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for COPYSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATELINKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDCREATELINK_HIGHLIGHT: CREATELINKSTATES = CREATELINKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDCREATELINK_NOHIGHLIGHT: CREATELINKSTATES = CREATELINKSTATES(2i32);
impl ::core::marker::Copy for CREATELINKSTATES {}
impl ::core::clone::Clone for CREATELINKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CREATELINKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CUEBANNERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBCB_NORMAL: CUEBANNERSTATES = CUEBANNERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBCB_HOT: CUEBANNERSTATES = CUEBANNERSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBCB_PRESSED: CUEBANNERSTATES = CUEBANNERSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBCB_DISABLED: CUEBANNERSTATES = CUEBANNERSTATES(4i32);
impl ::core::marker::Copy for CUEBANNERSTATES {}
impl ::core::clone::Clone for CUEBANNERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CUEBANNERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DATEBORDERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPDB_NORMAL: DATEBORDERSTATES = DATEBORDERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPDB_HOT: DATEBORDERSTATES = DATEBORDERSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPDB_FOCUSED: DATEBORDERSTATES = DATEBORDERSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPDB_DISABLED: DATEBORDERSTATES = DATEBORDERSTATES(4i32);
impl ::core::marker::Copy for DATEBORDERSTATES {}
impl ::core::clone::Clone for DATEBORDERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DATEBORDERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DATEPICKERPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DP_DATETEXT: DATEPICKERPARTS = DATEPICKERPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DP_DATEBORDER: DATEPICKERPARTS = DATEPICKERPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DP_SHOWCALENDARBUTTONRIGHT: DATEPICKERPARTS = DATEPICKERPARTS(3i32);
impl ::core::marker::Copy for DATEPICKERPARTS {}
impl ::core::clone::Clone for DATEPICKERPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DATEPICKERPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DATETEXTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPDT_NORMAL: DATETEXTSTATES = DATETEXTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPDT_DISABLED: DATETEXTSTATES = DATETEXTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPDT_SELECTED: DATETEXTSTATES = DATETEXTSTATES(3i32);
impl ::core::marker::Copy for DATETEXTSTATES {}
impl ::core::clone::Clone for DATETEXTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DATETEXTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DLG_BUTTON_CHECK_STATE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BST_CHECKED: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BST_INDETERMINATE: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const BST_UNCHECKED: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(0u32);
impl ::core::marker::Copy for DLG_BUTTON_CHECK_STATE {}
impl ::core::clone::Clone for DLG_BUTTON_CHECK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DLG_BUTTON_CHECK_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DLG_DIR_LIST_FILE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDL_ARCHIVE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDL_DIRECTORY: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDL_DRIVES: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDL_EXCLUSIVE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDL_HIDDEN: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDL_READONLY: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDL_READWRITE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDL_SYSTEM: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDL_POSTMSGS: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(8192u32);
impl ::core::marker::Copy for DLG_DIR_LIST_FILE_TYPE {}
impl ::core::clone::Clone for DLG_DIR_LIST_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DLG_DIR_LIST_FILE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOWNHORZSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DNHZS_NORMAL: DOWNHORZSTATES = DOWNHORZSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DNHZS_HOT: DOWNHORZSTATES = DOWNHORZSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DNHZS_PRESSED: DOWNHORZSTATES = DOWNHORZSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DNHZS_DISABLED: DOWNHORZSTATES = DOWNHORZSTATES(4i32);
impl ::core::marker::Copy for DOWNHORZSTATES {}
impl ::core::clone::Clone for DOWNHORZSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOWNHORZSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOWNSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DNS_NORMAL: DOWNSTATES = DOWNSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DNS_HOT: DOWNSTATES = DOWNSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DNS_PRESSED: DOWNSTATES = DOWNSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DNS_DISABLED: DOWNSTATES = DOWNSTATES(4i32);
impl ::core::marker::Copy for DOWNSTATES {}
impl ::core::clone::Clone for DOWNSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOWNSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DPAMM_MESSAGE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPAMM_MERGE: DPAMM_MESSAGE = DPAMM_MESSAGE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPAMM_DELETE: DPAMM_MESSAGE = DPAMM_MESSAGE(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPAMM_INSERT: DPAMM_MESSAGE = DPAMM_MESSAGE(3u32);
impl ::core::marker::Copy for DPAMM_MESSAGE {}
impl ::core::clone::Clone for DPAMM_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DPAMM_MESSAGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAGDROPPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DD_COPY: DRAGDROPPARTS = DRAGDROPPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DD_MOVE: DRAGDROPPARTS = DRAGDROPPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DD_UPDATEMETADATA: DRAGDROPPARTS = DRAGDROPPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DD_CREATELINK: DRAGDROPPARTS = DRAGDROPPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DD_WARNING: DRAGDROPPARTS = DRAGDROPPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DD_NONE: DRAGDROPPARTS = DRAGDROPPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DD_IMAGEBG: DRAGDROPPARTS = DRAGDROPPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DD_TEXTBG: DRAGDROPPARTS = DRAGDROPPARTS(8i32);
impl ::core::marker::Copy for DRAGDROPPARTS {}
impl ::core::clone::Clone for DRAGDROPPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRAGDROPPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAGLISTINFO_NOTIFICATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DL_BEGINDRAG: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1157u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DL_CANCELDRAG: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1160u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DL_DRAGGING: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1158u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DL_DROPPED: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1159u32);
impl ::core::marker::Copy for DRAGLISTINFO_NOTIFICATION_FLAGS {}
impl ::core::clone::Clone for DRAGLISTINFO_NOTIFICATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRAGLISTINFO_NOTIFICATION_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAWITEMSTRUCT_CTL_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODT_BUTTON: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODT_COMBOBOX: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODT_LISTBOX: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODT_LISTVIEW: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(102u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODT_MENU: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODT_STATIC: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(5u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODT_TAB: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(101u32);
impl ::core::marker::Copy for DRAWITEMSTRUCT_CTL_TYPE {}
impl ::core::clone::Clone for DRAWITEMSTRUCT_CTL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRAWITEMSTRUCT_CTL_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAW_THEME_PARENT_BACKGROUND_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTPB_WINDOWDC: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTPB_USECTLCOLORSTATIC: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTPB_USEERASEBKGND: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(4u32);
impl ::core::marker::Copy for DRAW_THEME_PARENT_BACKGROUND_FLAGS {}
impl ::core::clone::Clone for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DROPDOWNBUTTONLEFTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXSL_NORMAL: DROPDOWNBUTTONLEFTSTATES = DROPDOWNBUTTONLEFTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXSL_HOT: DROPDOWNBUTTONLEFTSTATES = DROPDOWNBUTTONLEFTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXSL_PRESSED: DROPDOWNBUTTONLEFTSTATES = DROPDOWNBUTTONLEFTSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXSL_DISABLED: DROPDOWNBUTTONLEFTSTATES = DROPDOWNBUTTONLEFTSTATES(4i32);
impl ::core::marker::Copy for DROPDOWNBUTTONLEFTSTATES {}
impl ::core::clone::Clone for DROPDOWNBUTTONLEFTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DROPDOWNBUTTONLEFTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DROPDOWNBUTTONRIGHTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXSR_NORMAL: DROPDOWNBUTTONRIGHTSTATES = DROPDOWNBUTTONRIGHTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXSR_HOT: DROPDOWNBUTTONRIGHTSTATES = DROPDOWNBUTTONRIGHTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXSR_PRESSED: DROPDOWNBUTTONRIGHTSTATES = DROPDOWNBUTTONRIGHTSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBXSR_DISABLED: DROPDOWNBUTTONRIGHTSTATES = DROPDOWNBUTTONRIGHTSTATES(4i32);
impl ::core::marker::Copy for DROPDOWNBUTTONRIGHTSTATES {}
impl ::core::clone::Clone for DROPDOWNBUTTONRIGHTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DROPDOWNBUTTONRIGHTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DROPDOWNITEMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBDI_NORMAL: DROPDOWNITEMSTATES = DROPDOWNITEMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBDI_HIGHLIGHTED: DROPDOWNITEMSTATES = DROPDOWNITEMSTATES(2i32);
impl ::core::marker::Copy for DROPDOWNITEMSTATES {}
impl ::core::clone::Clone for DROPDOWNITEMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DROPDOWNITEMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DTTOPTS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_TEXTCOLOR: DTTOPTS_FLAGS = DTTOPTS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_BORDERCOLOR: DTTOPTS_FLAGS = DTTOPTS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_SHADOWCOLOR: DTTOPTS_FLAGS = DTTOPTS_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_SHADOWTYPE: DTTOPTS_FLAGS = DTTOPTS_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_SHADOWOFFSET: DTTOPTS_FLAGS = DTTOPTS_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_BORDERSIZE: DTTOPTS_FLAGS = DTTOPTS_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_FONTPROP: DTTOPTS_FLAGS = DTTOPTS_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_COLORPROP: DTTOPTS_FLAGS = DTTOPTS_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_STATEID: DTTOPTS_FLAGS = DTTOPTS_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_CALCRECT: DTTOPTS_FLAGS = DTTOPTS_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_APPLYOVERLAY: DTTOPTS_FLAGS = DTTOPTS_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_GLOWSIZE: DTTOPTS_FLAGS = DTTOPTS_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_CALLBACK: DTTOPTS_FLAGS = DTTOPTS_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_COMPOSITED: DTTOPTS_FLAGS = DTTOPTS_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DTT_VALIDBITS: DTTOPTS_FLAGS = DTTOPTS_FLAGS(12287u32);
impl ::core::marker::Copy for DTTOPTS_FLAGS {}
impl ::core::clone::Clone for DTTOPTS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DTTOPTS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EC_ENDOFLINE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EC_ENDOFLINE_DETECTFROMCONTENT: EC_ENDOFLINE = EC_ENDOFLINE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EC_ENDOFLINE_CRLF: EC_ENDOFLINE = EC_ENDOFLINE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EC_ENDOFLINE_CR: EC_ENDOFLINE = EC_ENDOFLINE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EC_ENDOFLINE_LF: EC_ENDOFLINE = EC_ENDOFLINE(3i32);
impl ::core::marker::Copy for EC_ENDOFLINE {}
impl ::core::clone::Clone for EC_ENDOFLINE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EC_ENDOFLINE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EC_SEARCHWEB_ENTRYPOINT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EC_SEARCHWEB_ENTRYPOINT_EXTERNAL: EC_SEARCHWEB_ENTRYPOINT = EC_SEARCHWEB_ENTRYPOINT(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EC_SEARCHWEB_ENTRYPOINT_CONTEXTMENU: EC_SEARCHWEB_ENTRYPOINT = EC_SEARCHWEB_ENTRYPOINT(1i32);
impl ::core::marker::Copy for EC_SEARCHWEB_ENTRYPOINT {}
impl ::core::clone::Clone for EC_SEARCHWEB_ENTRYPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EC_SEARCHWEB_ENTRYPOINT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDITBALLOONTIP_ICON(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTI_ERROR: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTI_INFO: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTI_NONE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTI_WARNING: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTI_INFO_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTI_WARNING_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(5u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTI_ERROR_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(6u32);
impl ::core::marker::Copy for EDITBALLOONTIP_ICON {}
impl ::core::clone::Clone for EDITBALLOONTIP_ICON {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EDITBALLOONTIP_ICON {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDITBORDER_HSCROLLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSH_NORMAL: EDITBORDER_HSCROLLSTATES = EDITBORDER_HSCROLLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSH_HOT: EDITBORDER_HSCROLLSTATES = EDITBORDER_HSCROLLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSH_FOCUSED: EDITBORDER_HSCROLLSTATES = EDITBORDER_HSCROLLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSH_DISABLED: EDITBORDER_HSCROLLSTATES = EDITBORDER_HSCROLLSTATES(4i32);
impl ::core::marker::Copy for EDITBORDER_HSCROLLSTATES {}
impl ::core::clone::Clone for EDITBORDER_HSCROLLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EDITBORDER_HSCROLLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDITBORDER_HVSCROLLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSHV_NORMAL: EDITBORDER_HVSCROLLSTATES = EDITBORDER_HVSCROLLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSHV_HOT: EDITBORDER_HVSCROLLSTATES = EDITBORDER_HVSCROLLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSHV_FOCUSED: EDITBORDER_HVSCROLLSTATES = EDITBORDER_HVSCROLLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSHV_DISABLED: EDITBORDER_HVSCROLLSTATES = EDITBORDER_HVSCROLLSTATES(4i32);
impl ::core::marker::Copy for EDITBORDER_HVSCROLLSTATES {}
impl ::core::clone::Clone for EDITBORDER_HVSCROLLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EDITBORDER_HVSCROLLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDITBORDER_NOSCROLLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSN_NORMAL: EDITBORDER_NOSCROLLSTATES = EDITBORDER_NOSCROLLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSN_HOT: EDITBORDER_NOSCROLLSTATES = EDITBORDER_NOSCROLLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSN_FOCUSED: EDITBORDER_NOSCROLLSTATES = EDITBORDER_NOSCROLLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSN_DISABLED: EDITBORDER_NOSCROLLSTATES = EDITBORDER_NOSCROLLSTATES(4i32);
impl ::core::marker::Copy for EDITBORDER_NOSCROLLSTATES {}
impl ::core::clone::Clone for EDITBORDER_NOSCROLLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EDITBORDER_NOSCROLLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDITBORDER_VSCROLLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSV_NORMAL: EDITBORDER_VSCROLLSTATES = EDITBORDER_VSCROLLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSV_HOT: EDITBORDER_VSCROLLSTATES = EDITBORDER_VSCROLLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSV_FOCUSED: EDITBORDER_VSCROLLSTATES = EDITBORDER_VSCROLLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EPSV_DISABLED: EDITBORDER_VSCROLLSTATES = EDITBORDER_VSCROLLSTATES(4i32);
impl ::core::marker::Copy for EDITBORDER_VSCROLLSTATES {}
impl ::core::clone::Clone for EDITBORDER_VSCROLLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EDITBORDER_VSCROLLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDITPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EP_EDITTEXT: EDITPARTS = EDITPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EP_CARET: EDITPARTS = EDITPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EP_BACKGROUND: EDITPARTS = EDITPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EP_PASSWORD: EDITPARTS = EDITPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EP_BACKGROUNDWITHBORDER: EDITPARTS = EDITPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EP_EDITBORDER_NOSCROLL: EDITPARTS = EDITPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EP_EDITBORDER_HSCROLL: EDITPARTS = EDITPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EP_EDITBORDER_VSCROLL: EDITPARTS = EDITPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EP_EDITBORDER_HVSCROLL: EDITPARTS = EDITPARTS(9i32);
impl ::core::marker::Copy for EDITPARTS {}
impl ::core::clone::Clone for EDITPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EDITPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDITTEXTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETS_NORMAL: EDITTEXTSTATES = EDITTEXTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETS_HOT: EDITTEXTSTATES = EDITTEXTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETS_SELECTED: EDITTEXTSTATES = EDITTEXTSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETS_DISABLED: EDITTEXTSTATES = EDITTEXTSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETS_FOCUSED: EDITTEXTSTATES = EDITTEXTSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETS_READONLY: EDITTEXTSTATES = EDITTEXTSTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETS_ASSIST: EDITTEXTSTATES = EDITTEXTSTATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ETS_CUEBANNER: EDITTEXTSTATES = EDITTEXTSTATES(8i32);
impl ::core::marker::Copy for EDITTEXTSTATES {}
impl ::core::clone::Clone for EDITTEXTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EDITTEXTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EMPTYMARKUPPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EMP_MARKUPTEXT: EMPTYMARKUPPARTS = EMPTYMARKUPPARTS(1i32);
impl ::core::marker::Copy for EMPTYMARKUPPARTS {}
impl ::core::clone::Clone for EMPTYMARKUPPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EMPTYMARKUPPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENABLE_SCROLL_BAR_ARROWS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ESB_DISABLE_BOTH: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ESB_DISABLE_DOWN: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ESB_DISABLE_LEFT: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ESB_DISABLE_LTUP: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ESB_DISABLE_RIGHT: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ESB_DISABLE_RTDN: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ESB_DISABLE_UP: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ESB_ENABLE_BOTH: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(0u32);
impl ::core::marker::Copy for ENABLE_SCROLL_BAR_ARROWS {}
impl ::core::clone::Clone for ENABLE_SCROLL_BAR_ARROWS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ENABLE_SCROLL_BAR_ARROWS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXPANDBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVEB_NORMAL: EXPANDBUTTONSTATES = EXPANDBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVEB_HOVER: EXPANDBUTTONSTATES = EXPANDBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVEB_PUSHED: EXPANDBUTTONSTATES = EXPANDBUTTONSTATES(3i32);
impl ::core::marker::Copy for EXPANDBUTTONSTATES {}
impl ::core::clone::Clone for EXPANDBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXPANDBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXPANDOBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLGEBS_NORMAL: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLGEBS_HOVER: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLGEBS_PRESSED: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLGEBS_EXPANDEDNORMAL: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLGEBS_EXPANDEDHOVER: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLGEBS_EXPANDEDPRESSED: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLGEBS_NORMALDISABLED: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLGEBS_EXPANDEDDISABLED: EXPANDOBUTTONSTATES = EXPANDOBUTTONSTATES(8i32);
impl ::core::marker::Copy for EXPANDOBUTTONSTATES {}
impl ::core::clone::Clone for EXPANDOBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXPANDOBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXPLORERBARPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_HEADERBACKGROUND: EXPLORERBARPARTS = EXPLORERBARPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_HEADERCLOSE: EXPLORERBARPARTS = EXPLORERBARPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_HEADERPIN: EXPLORERBARPARTS = EXPLORERBARPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_IEBARMENU: EXPLORERBARPARTS = EXPLORERBARPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_NORMALGROUPBACKGROUND: EXPLORERBARPARTS = EXPLORERBARPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_NORMALGROUPCOLLAPSE: EXPLORERBARPARTS = EXPLORERBARPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_NORMALGROUPEXPAND: EXPLORERBARPARTS = EXPLORERBARPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_NORMALGROUPHEAD: EXPLORERBARPARTS = EXPLORERBARPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_SPECIALGROUPBACKGROUND: EXPLORERBARPARTS = EXPLORERBARPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_SPECIALGROUPCOLLAPSE: EXPLORERBARPARTS = EXPLORERBARPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_SPECIALGROUPEXPAND: EXPLORERBARPARTS = EXPLORERBARPARTS(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBP_SPECIALGROUPHEAD: EXPLORERBARPARTS = EXPLORERBARPARTS(12i32);
impl ::core::marker::Copy for EXPLORERBARPARTS {}
impl ::core::clone::Clone for EXPLORERBARPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EXPLORERBARPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEEDBACK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_TOUCH_CONTACTVISUALIZATION: FEEDBACK_TYPE = FEEDBACK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_PEN_BARRELVISUALIZATION: FEEDBACK_TYPE = FEEDBACK_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_PEN_TAP: FEEDBACK_TYPE = FEEDBACK_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_PEN_DOUBLETAP: FEEDBACK_TYPE = FEEDBACK_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_PEN_PRESSANDHOLD: FEEDBACK_TYPE = FEEDBACK_TYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_PEN_RIGHTTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_TOUCH_TAP: FEEDBACK_TYPE = FEEDBACK_TYPE(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_TOUCH_DOUBLETAP: FEEDBACK_TYPE = FEEDBACK_TYPE(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_TOUCH_PRESSANDHOLD: FEEDBACK_TYPE = FEEDBACK_TYPE(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_TOUCH_RIGHTTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_GESTURE_PRESSANDTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FEEDBACK_MAX: FEEDBACK_TYPE = FEEDBACK_TYPE(-1i32);
impl ::core::marker::Copy for FEEDBACK_TYPE {}
impl ::core::clone::Clone for FEEDBACK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FEEDBACK_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBFS_NORMAL: FILLSTATES = FILLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBFS_ERROR: FILLSTATES = FILLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBFS_PAUSED: FILLSTATES = FILLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBFS_PARTIAL: FILLSTATES = FILLSTATES(4i32);
impl ::core::marker::Copy for FILLSTATES {}
impl ::core::clone::Clone for FILLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILLTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FT_SOLID: FILLTYPE = FILLTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FT_VERTGRADIENT: FILLTYPE = FILLTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FT_HORZGRADIENT: FILLTYPE = FILLTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FT_RADIALGRADIENT: FILLTYPE = FILLTYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FT_TILEIMAGE: FILLTYPE = FILLTYPE(4i32);
impl ::core::marker::Copy for FILLTYPE {}
impl ::core::clone::Clone for FILLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILLTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILLVERTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBFVS_NORMAL: FILLVERTSTATES = FILLVERTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBFVS_ERROR: FILLVERTSTATES = FILLVERTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBFVS_PAUSED: FILLVERTSTATES = FILLVERTSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBFVS_PARTIAL: FILLVERTSTATES = FILLVERTSTATES(4i32);
impl ::core::marker::Copy for FILLVERTSTATES {}
impl ::core::clone::Clone for FILLVERTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILLVERTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FLYOUTPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLYOUT_HEADER: FLYOUTPARTS = FLYOUTPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLYOUT_BODY: FLYOUTPARTS = FLYOUTPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLYOUT_LABEL: FLYOUTPARTS = FLYOUTPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLYOUT_LINK: FLYOUTPARTS = FLYOUTPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLYOUT_DIVIDER: FLYOUTPARTS = FLYOUTPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLYOUT_WINDOW: FLYOUTPARTS = FLYOUTPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLYOUT_LINKAREA: FLYOUTPARTS = FLYOUTPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLYOUT_LINKHEADER: FLYOUTPARTS = FLYOUTPARTS(8i32);
impl ::core::marker::Copy for FLYOUTPARTS {}
impl ::core::clone::Clone for FLYOUTPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FLYOUTPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FRAMEBOTTOMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FRB_ACTIVE: FRAMEBOTTOMSTATES = FRAMEBOTTOMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FRB_INACTIVE: FRAMEBOTTOMSTATES = FRAMEBOTTOMSTATES(2i32);
impl ::core::marker::Copy for FRAMEBOTTOMSTATES {}
impl ::core::clone::Clone for FRAMEBOTTOMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FRAMEBOTTOMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FRAMELEFTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FRL_ACTIVE: FRAMELEFTSTATES = FRAMELEFTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FRL_INACTIVE: FRAMELEFTSTATES = FRAMELEFTSTATES(2i32);
impl ::core::marker::Copy for FRAMELEFTSTATES {}
impl ::core::clone::Clone for FRAMELEFTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FRAMELEFTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FRAMERIGHTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FRR_ACTIVE: FRAMERIGHTSTATES = FRAMERIGHTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FRR_INACTIVE: FRAMERIGHTSTATES = FRAMERIGHTSTATES(2i32);
impl ::core::marker::Copy for FRAMERIGHTSTATES {}
impl ::core::clone::Clone for FRAMERIGHTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FRAMERIGHTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FRAMESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FS_ACTIVE: FRAMESTATES = FRAMESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FS_INACTIVE: FRAMESTATES = FRAMESTATES(2i32);
impl ::core::marker::Copy for FRAMESTATES {}
impl ::core::clone::Clone for FRAMESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FRAMESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_THEME_BITMAP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GBF_DIRECT: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GBF_COPY: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GBF_VALIDBITS: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(3u32);
impl ::core::marker::Copy for GET_THEME_BITMAP_FLAGS {}
impl ::core::clone::Clone for GET_THEME_BITMAP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GET_THEME_BITMAP_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GLYPHFONTSIZINGTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GFST_NONE: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GFST_SIZE: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GFST_DPI: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(2i32);
impl ::core::marker::Copy for GLYPHFONTSIZINGTYPE {}
impl ::core::clone::Clone for GLYPHFONTSIZINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GLYPHFONTSIZINGTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GLYPHSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GLPS_CLOSED: GLYPHSTATES = GLYPHSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GLPS_OPENED: GLYPHSTATES = GLYPHSTATES(2i32);
impl ::core::marker::Copy for GLYPHSTATES {}
impl ::core::clone::Clone for GLYPHSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GLYPHSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GLYPHTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GT_NONE: GLYPHTYPE = GLYPHTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GT_IMAGEGLYPH: GLYPHTYPE = GLYPHTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GT_FONTGLYPH: GLYPHTYPE = GLYPHTYPE(2i32);
impl ::core::marker::Copy for GLYPHTYPE {}
impl ::core::clone::Clone for GLYPHTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GLYPHTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GRIDCELLBACKGROUNDSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCB_SELECTED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCB_HOT: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCB_SELECTEDHOT: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCB_SELECTEDNOTFOCUSED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCB_TODAY: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCB_TODAYSELECTED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(6i32);
impl ::core::marker::Copy for GRIDCELLBACKGROUNDSTATES {}
impl ::core::clone::Clone for GRIDCELLBACKGROUNDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GRIDCELLBACKGROUNDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GRIDCELLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGC_HOT: GRIDCELLSTATES = GRIDCELLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGC_HASSTATE: GRIDCELLSTATES = GRIDCELLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGC_HASSTATEHOT: GRIDCELLSTATES = GRIDCELLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGC_TODAY: GRIDCELLSTATES = GRIDCELLSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGC_TODAYSELECTED: GRIDCELLSTATES = GRIDCELLSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGC_SELECTED: GRIDCELLSTATES = GRIDCELLSTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGC_SELECTEDHOT: GRIDCELLSTATES = GRIDCELLSTATES(7i32);
impl ::core::marker::Copy for GRIDCELLSTATES {}
impl ::core::clone::Clone for GRIDCELLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GRIDCELLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GRIDCELLUPPERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCU_HOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCU_HASSTATE: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCU_HASSTATEHOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCU_SELECTED: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGCU_SELECTEDHOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(5i32);
impl ::core::marker::Copy for GRIDCELLUPPERSTATES {}
impl ::core::clone::Clone for GRIDCELLUPPERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GRIDCELLUPPERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GRIPPERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TSGS_NORMAL: GRIPPERSTATES = GRIPPERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TSGS_CENTERED: GRIPPERSTATES = GRIPPERSTATES(2i32);
impl ::core::marker::Copy for GRIPPERSTATES {}
impl ::core::clone::Clone for GRIPPERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GRIPPERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GROUPBOXSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GBS_NORMAL: GROUPBOXSTATES = GROUPBOXSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GBS_DISABLED: GROUPBOXSTATES = GROUPBOXSTATES(2i32);
impl ::core::marker::Copy for GROUPBOXSTATES {}
impl ::core::clone::Clone for GROUPBOXSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GROUPBOXSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GROUPHEADERLINESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_OPEN: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_OPENHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_OPENSELECTED: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_OPENSELECTEDHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_OPENSELECTEDNOTFOCUSED: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_OPENSELECTEDNOTFOCUSEDHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_OPENMIXEDSELECTION: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_OPENMIXEDSELECTIONHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_CLOSE: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_CLOSEHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_CLOSESELECTED: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_CLOSESELECTEDHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_CLOSESELECTEDNOTFOCUSED: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_CLOSESELECTEDNOTFOCUSEDHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_CLOSEMIXEDSELECTION: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGHL_CLOSEMIXEDSELECTIONHOT: GROUPHEADERLINESTATES = GROUPHEADERLINESTATES(16i32);
impl ::core::marker::Copy for GROUPHEADERLINESTATES {}
impl ::core::clone::Clone for GROUPHEADERLINESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GROUPHEADERLINESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GROUPHEADERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_OPEN: GROUPHEADERSTATES = GROUPHEADERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_OPENHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_OPENSELECTED: GROUPHEADERSTATES = GROUPHEADERSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_OPENSELECTEDHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_OPENSELECTEDNOTFOCUSED: GROUPHEADERSTATES = GROUPHEADERSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_OPENSELECTEDNOTFOCUSEDHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_OPENMIXEDSELECTION: GROUPHEADERSTATES = GROUPHEADERSTATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_OPENMIXEDSELECTIONHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_CLOSE: GROUPHEADERSTATES = GROUPHEADERSTATES(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_CLOSEHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_CLOSESELECTED: GROUPHEADERSTATES = GROUPHEADERSTATES(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_CLOSESELECTEDHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_CLOSESELECTEDNOTFOCUSED: GROUPHEADERSTATES = GROUPHEADERSTATES(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_CLOSESELECTEDNOTFOCUSEDHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_CLOSEMIXEDSELECTION: GROUPHEADERSTATES = GROUPHEADERSTATES(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGH_CLOSEMIXEDSELECTIONHOT: GROUPHEADERSTATES = GROUPHEADERSTATES(16i32);
impl ::core::marker::Copy for GROUPHEADERSTATES {}
impl ::core::clone::Clone for GROUPHEADERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GROUPHEADERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HALIGN(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HA_LEFT: HALIGN = HALIGN(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HA_CENTER: HALIGN = HALIGN(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HA_RIGHT: HALIGN = HALIGN(2i32);
impl ::core::marker::Copy for HALIGN {}
impl ::core::clone::Clone for HALIGN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HALIGN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDI_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_WIDTH: HDI_MASK = HDI_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_HEIGHT: HDI_MASK = HDI_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_TEXT: HDI_MASK = HDI_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_FORMAT: HDI_MASK = HDI_MASK(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_LPARAM: HDI_MASK = HDI_MASK(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_BITMAP: HDI_MASK = HDI_MASK(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_IMAGE: HDI_MASK = HDI_MASK(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_DI_SETITEM: HDI_MASK = HDI_MASK(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_ORDER: HDI_MASK = HDI_MASK(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_FILTER: HDI_MASK = HDI_MASK(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDI_STATE: HDI_MASK = HDI_MASK(512u32);
impl ::core::marker::Copy for HDI_MASK {}
impl ::core::clone::Clone for HDI_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HDI_MASK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERAREASTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const AW_S_HEADERAREA_NOMARGIN: HEADERAREASTATES = HEADERAREASTATES(1i32);
impl ::core::marker::Copy for HEADERAREASTATES {}
impl ::core::clone::Clone for HEADERAREASTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERAREASTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERCLOSESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBHC_NORMAL: HEADERCLOSESTATES = HEADERCLOSESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBHC_HOT: HEADERCLOSESTATES = HEADERCLOSESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBHC_PRESSED: HEADERCLOSESTATES = HEADERCLOSESTATES(3i32);
impl ::core::marker::Copy for HEADERCLOSESTATES {}
impl ::core::clone::Clone for HEADERCLOSESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERCLOSESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERDROPDOWNFILTERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDDFS_NORMAL: HEADERDROPDOWNFILTERSTATES = HEADERDROPDOWNFILTERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDDFS_SOFTHOT: HEADERDROPDOWNFILTERSTATES = HEADERDROPDOWNFILTERSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDDFS_HOT: HEADERDROPDOWNFILTERSTATES = HEADERDROPDOWNFILTERSTATES(3i32);
impl ::core::marker::Copy for HEADERDROPDOWNFILTERSTATES {}
impl ::core::clone::Clone for HEADERDROPDOWNFILTERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERDROPDOWNFILTERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERDROPDOWNSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDDS_NORMAL: HEADERDROPDOWNSTATES = HEADERDROPDOWNSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDDS_SOFTHOT: HEADERDROPDOWNSTATES = HEADERDROPDOWNSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDDS_HOT: HEADERDROPDOWNSTATES = HEADERDROPDOWNSTATES(3i32);
impl ::core::marker::Copy for HEADERDROPDOWNSTATES {}
impl ::core::clone::Clone for HEADERDROPDOWNSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERDROPDOWNSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERITEMLEFTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HILS_NORMAL: HEADERITEMLEFTSTATES = HEADERITEMLEFTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HILS_HOT: HEADERITEMLEFTSTATES = HEADERITEMLEFTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HILS_PRESSED: HEADERITEMLEFTSTATES = HEADERITEMLEFTSTATES(3i32);
impl ::core::marker::Copy for HEADERITEMLEFTSTATES {}
impl ::core::clone::Clone for HEADERITEMLEFTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERITEMLEFTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERITEMRIGHTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIRS_NORMAL: HEADERITEMRIGHTSTATES = HEADERITEMRIGHTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIRS_HOT: HEADERITEMRIGHTSTATES = HEADERITEMRIGHTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIRS_PRESSED: HEADERITEMRIGHTSTATES = HEADERITEMRIGHTSTATES(3i32);
impl ::core::marker::Copy for HEADERITEMRIGHTSTATES {}
impl ::core::clone::Clone for HEADERITEMRIGHTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERITEMRIGHTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERITEMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_NORMAL: HEADERITEMSTATES = HEADERITEMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_HOT: HEADERITEMSTATES = HEADERITEMSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_PRESSED: HEADERITEMSTATES = HEADERITEMSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_SORTEDNORMAL: HEADERITEMSTATES = HEADERITEMSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_SORTEDHOT: HEADERITEMSTATES = HEADERITEMSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_SORTEDPRESSED: HEADERITEMSTATES = HEADERITEMSTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_ICONNORMAL: HEADERITEMSTATES = HEADERITEMSTATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_ICONHOT: HEADERITEMSTATES = HEADERITEMSTATES(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_ICONPRESSED: HEADERITEMSTATES = HEADERITEMSTATES(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_ICONSORTEDNORMAL: HEADERITEMSTATES = HEADERITEMSTATES(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_ICONSORTEDHOT: HEADERITEMSTATES = HEADERITEMSTATES(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HIS_ICONSORTEDPRESSED: HEADERITEMSTATES = HEADERITEMSTATES(12i32);
impl ::core::marker::Copy for HEADERITEMSTATES {}
impl ::core::clone::Clone for HEADERITEMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERITEMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADEROVERFLOWSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HOFS_NORMAL: HEADEROVERFLOWSTATES = HEADEROVERFLOWSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HOFS_HOT: HEADEROVERFLOWSTATES = HEADEROVERFLOWSTATES(2i32);
impl ::core::marker::Copy for HEADEROVERFLOWSTATES {}
impl ::core::clone::Clone for HEADEROVERFLOWSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADEROVERFLOWSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HP_HEADERITEM: HEADERPARTS = HEADERPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HP_HEADERITEMLEFT: HEADERPARTS = HEADERPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HP_HEADERITEMRIGHT: HEADERPARTS = HEADERPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HP_HEADERSORTARROW: HEADERPARTS = HEADERPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HP_HEADERDROPDOWN: HEADERPARTS = HEADERPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HP_HEADERDROPDOWNFILTER: HEADERPARTS = HEADERPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HP_HEADEROVERFLOW: HEADERPARTS = HEADERPARTS(7i32);
impl ::core::marker::Copy for HEADERPARTS {}
impl ::core::clone::Clone for HEADERPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERPINSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBHP_NORMAL: HEADERPINSTATES = HEADERPINSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBHP_HOT: HEADERPINSTATES = HEADERPINSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBHP_PRESSED: HEADERPINSTATES = HEADERPINSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBHP_SELECTEDNORMAL: HEADERPINSTATES = HEADERPINSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBHP_SELECTEDHOT: HEADERPINSTATES = HEADERPINSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBHP_SELECTEDPRESSED: HEADERPINSTATES = HEADERPINSTATES(6i32);
impl ::core::marker::Copy for HEADERPINSTATES {}
impl ::core::clone::Clone for HEADERPINSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERPINSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERSORTARROWSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HSAS_SORTEDUP: HEADERSORTARROWSTATES = HEADERSORTARROWSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HSAS_SORTEDDOWN: HEADERSORTARROWSTATES = HEADERSORTARROWSTATES(2i32);
impl ::core::marker::Copy for HEADERSORTARROWSTATES {}
impl ::core::clone::Clone for HEADERSORTARROWSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERSORTARROWSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADERSTYLESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HBG_DETAILS: HEADERSTYLESTATES = HEADERSTYLESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HBG_ICON: HEADERSTYLESTATES = HEADERSTYLESTATES(2i32);
impl ::core::marker::Copy for HEADERSTYLESTATES {}
impl ::core::clone::Clone for HEADERSTYLESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADERSTYLESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADER_CONTROL_FORMAT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_LEFT: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_RIGHT: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_CENTER: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_JUSTIFYMASK: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_RTLREADING: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_BITMAP: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(8192i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_STRING: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(16384i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_OWNERDRAW: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(32768i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_IMAGE: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_BITMAP_ON_RIGHT: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_SORTUP: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(1024i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_SORTDOWN: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_CHECKBOX: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_CHECKED: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_FIXEDWIDTH: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDF_SPLITBUTTON: HEADER_CONTROL_FORMAT_FLAGS = HEADER_CONTROL_FORMAT_FLAGS(16777216i32);
impl ::core::marker::Copy for HEADER_CONTROL_FORMAT_FLAGS {}
impl ::core::clone::Clone for HEADER_CONTROL_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADER_CONTROL_FORMAT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADER_CONTROL_FORMAT_STATE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDIS_FOCUSED: HEADER_CONTROL_FORMAT_STATE = HEADER_CONTROL_FORMAT_STATE(1u32);
impl ::core::marker::Copy for HEADER_CONTROL_FORMAT_STATE {}
impl ::core::clone::Clone for HEADER_CONTROL_FORMAT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADER_CONTROL_FORMAT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADER_CONTROL_FORMAT_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDFT_ISSTRING: HEADER_CONTROL_FORMAT_TYPE = HEADER_CONTROL_FORMAT_TYPE(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDFT_ISNUMBER: HEADER_CONTROL_FORMAT_TYPE = HEADER_CONTROL_FORMAT_TYPE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDFT_ISDATE: HEADER_CONTROL_FORMAT_TYPE = HEADER_CONTROL_FORMAT_TYPE(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HDFT_HASNOVALUE: HEADER_CONTROL_FORMAT_TYPE = HEADER_CONTROL_FORMAT_TYPE(32768u32);
impl ::core::marker::Copy for HEADER_CONTROL_FORMAT_TYPE {}
impl ::core::clone::Clone for HEADER_CONTROL_FORMAT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADER_CONTROL_FORMAT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADER_CONTROL_NOTIFICATION_BUTTON(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_LEFT: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_RIGHT: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_MIDDLE: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(2u32);
impl ::core::marker::Copy for HEADER_CONTROL_NOTIFICATION_BUTTON {}
impl ::core::clone::Clone for HEADER_CONTROL_NOTIFICATION_BUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADER_CONTROL_NOTIFICATION_BUTTON {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEADER_HITTEST_INFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_NOWHERE: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_ONHEADER: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_ONDIVIDER: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_ONDIVOPEN: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_ONFILTER: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_ONFILTERBUTTON: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_ABOVE: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_BELOW: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_TORIGHT: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_TOLEFT: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_ONITEMSTATEICON: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_ONDROPDOWN: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HHT_ONOVERFLOW: HEADER_HITTEST_INFO_FLAGS = HEADER_HITTEST_INFO_FLAGS(16384u32);
impl ::core::marker::Copy for HEADER_HITTEST_INFO_FLAGS {}
impl ::core::clone::Clone for HEADER_HITTEST_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEADER_HITTEST_INFO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HELPBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HBS_NORMAL: HELPBUTTONSTATES = HELPBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HBS_HOT: HELPBUTTONSTATES = HELPBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HBS_PUSHED: HELPBUTTONSTATES = HELPBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HBS_DISABLED: HELPBUTTONSTATES = HELPBUTTONSTATES(4i32);
impl ::core::marker::Copy for HELPBUTTONSTATES {}
impl ::core::clone::Clone for HELPBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HELPBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HELPLINKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPHL_NORMAL: HELPLINKSTATES = HELPLINKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPHL_HOT: HELPLINKSTATES = HELPLINKSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPHL_PRESSED: HELPLINKSTATES = HELPLINKSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPHL_DISABLED: HELPLINKSTATES = HELPLINKSTATES(4i32);
impl ::core::marker::Copy for HELPLINKSTATES {}
impl ::core::clone::Clone for HELPLINKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HELPLINKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HIT_TEST_BACKGROUND_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTTB_BACKGROUNDSEG: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTTB_FIXEDBORDER: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTTB_CAPTION: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTTB_RESIZINGBORDER_LEFT: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTTB_RESIZINGBORDER_TOP: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTTB_RESIZINGBORDER_RIGHT: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTTB_RESIZINGBORDER_BOTTOM: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTTB_RESIZINGBORDER: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(240u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTTB_SIZINGTEMPLATE: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTTB_SYSTEMSIZINGMARGINS: HIT_TEST_BACKGROUND_OPTIONS = HIT_TEST_BACKGROUND_OPTIONS(512u32);
impl ::core::marker::Copy for HIT_TEST_BACKGROUND_OPTIONS {}
impl ::core::clone::Clone for HIT_TEST_BACKGROUND_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HIT_TEST_BACKGROUND_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HORZSCROLLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HSS_NORMAL: HORZSCROLLSTATES = HORZSCROLLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HSS_HOT: HORZSCROLLSTATES = HORZSCROLLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HSS_PUSHED: HORZSCROLLSTATES = HORZSCROLLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HSS_DISABLED: HORZSCROLLSTATES = HORZSCROLLSTATES(4i32);
impl ::core::marker::Copy for HORZSCROLLSTATES {}
impl ::core::clone::Clone for HORZSCROLLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HORZSCROLLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HORZTHUMBSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTS_NORMAL: HORZTHUMBSTATES = HORZTHUMBSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTS_HOT: HORZTHUMBSTATES = HORZTHUMBSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTS_PUSHED: HORZTHUMBSTATES = HORZTHUMBSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HTS_DISABLED: HORZTHUMBSTATES = HORZTHUMBSTATES(4i32);
impl ::core::marker::Copy for HORZTHUMBSTATES {}
impl ::core::clone::Clone for HORZTHUMBSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HORZTHUMBSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HOTGLYPHSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HGLPS_CLOSED: HOTGLYPHSTATES = HOTGLYPHSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HGLPS_OPENED: HOTGLYPHSTATES = HOTGLYPHSTATES(2i32);
impl ::core::marker::Copy for HOTGLYPHSTATES {}
impl ::core::clone::Clone for HOTGLYPHSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HOTGLYPHSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HOVERBACKGROUNDSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UTS_NORMAL: HOVERBACKGROUNDSTATES = HOVERBACKGROUNDSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UTS_HOT: HOVERBACKGROUNDSTATES = HOVERBACKGROUNDSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UTS_PRESSED: HOVERBACKGROUNDSTATES = HOVERBACKGROUNDSTATES(3i32);
impl ::core::marker::Copy for HOVERBACKGROUNDSTATES {}
impl ::core::clone::Clone for HOVERBACKGROUNDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HOVERBACKGROUNDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HYPERLINKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HLS_NORMALTEXT: HYPERLINKSTATES = HYPERLINKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HLS_LINKTEXT: HYPERLINKSTATES = HYPERLINKSTATES(2i32);
impl ::core::marker::Copy for HYPERLINKSTATES {}
impl ::core::clone::Clone for HYPERLINKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HYPERLINKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HYPERLINKTEXTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_HYPERLINK_NORMAL: HYPERLINKTEXTSTATES = HYPERLINKTEXTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_HYPERLINK_HOT: HYPERLINKTEXTSTATES = HYPERLINKTEXTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_HYPERLINK_PRESSED: HYPERLINKTEXTSTATES = HYPERLINKTEXTSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_HYPERLINK_DISABLED: HYPERLINKTEXTSTATES = HYPERLINKTEXTSTATES(4i32);
impl ::core::marker::Copy for HYPERLINKTEXTSTATES {}
impl ::core::clone::Clone for HYPERLINKTEXTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HYPERLINKTEXTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ICONEFFECT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICE_NONE: ICONEFFECT = ICONEFFECT(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICE_GLOW: ICONEFFECT = ICONEFFECT(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICE_SHADOW: ICONEFFECT = ICONEFFECT(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICE_PULSE: ICONEFFECT = ICONEFFECT(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICE_ALPHA: ICONEFFECT = ICONEFFECT(4i32);
impl ::core::marker::Copy for ICONEFFECT {}
impl ::core::clone::Clone for ICONEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ICONEFFECT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IEBARMENUSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBM_NORMAL: IEBARMENUSTATES = IEBARMENUSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBM_HOT: IEBARMENUSTATES = IEBARMENUSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBM_PRESSED: IEBARMENUSTATES = IEBARMENUSTATES(3i32);
impl ::core::marker::Copy for IEBARMENUSTATES {}
impl ::core::clone::Clone for IEBARMENUSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IEBARMENUSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGELAYOUT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IL_VERTICAL: IMAGELAYOUT = IMAGELAYOUT(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IL_HORIZONTAL: IMAGELAYOUT = IMAGELAYOUT(1i32);
impl ::core::marker::Copy for IMAGELAYOUT {}
impl ::core::clone::Clone for IMAGELAYOUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGELAYOUT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGELIST_CREATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_MASK: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_COLOR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_COLORDDB: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(254u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_COLOR4: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_COLOR8: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_COLOR16: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_COLOR24: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(24u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_COLOR32: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_PALETTE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_MIRROR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_PERITEMMIRROR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_ORIGINALSIZE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILC_HIGHQUALITYSCALE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(131072u32);
impl ::core::marker::Copy for IMAGELIST_CREATION_FLAGS {}
impl ::core::clone::Clone for IMAGELIST_CREATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGELIST_CREATION_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGESELECTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IST_NONE: IMAGESELECTTYPE = IMAGESELECTTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IST_SIZE: IMAGESELECTTYPE = IMAGESELECTTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const IST_DPI: IMAGESELECTTYPE = IMAGESELECTTYPE(2i32);
impl ::core::marker::Copy for IMAGESELECTTYPE {}
impl ::core::clone::Clone for IMAGESELECTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGESELECTTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_LIST_COPY_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILCF_MOVE: IMAGE_LIST_COPY_FLAGS = IMAGE_LIST_COPY_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILCF_SWAP: IMAGE_LIST_COPY_FLAGS = IMAGE_LIST_COPY_FLAGS(1u32);
impl ::core::marker::Copy for IMAGE_LIST_COPY_FLAGS {}
impl ::core::clone::Clone for IMAGE_LIST_COPY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGE_LIST_COPY_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_LIST_DRAW_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_NORMAL: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_TRANSPARENT: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_BLEND25: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_FOCUS: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_BLEND50: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_SELECTED: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_BLEND: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_MASK: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_IMAGE: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_ROP: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_OVERLAYMASK: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(3840u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_PRESERVEALPHA: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_SCALE: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_DPISCALE: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILD_ASYNC: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(32768u32);
impl ::core::marker::Copy for IMAGE_LIST_DRAW_STYLE {}
impl ::core::clone::Clone for IMAGE_LIST_DRAW_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGE_LIST_DRAW_STYLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_LIST_ITEM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILIF_ALPHA: IMAGE_LIST_ITEM_FLAGS = IMAGE_LIST_ITEM_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILIF_LOWQUALITY: IMAGE_LIST_ITEM_FLAGS = IMAGE_LIST_ITEM_FLAGS(2u32);
impl ::core::marker::Copy for IMAGE_LIST_ITEM_FLAGS {}
impl ::core::clone::Clone for IMAGE_LIST_ITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGE_LIST_ITEM_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_LIST_WRITE_STREAM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILP_NORMAL: IMAGE_LIST_WRITE_STREAM_FLAGS = IMAGE_LIST_WRITE_STREAM_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ILP_DOWNLEVEL: IMAGE_LIST_WRITE_STREAM_FLAGS = IMAGE_LIST_WRITE_STREAM_FLAGS(1u32);
impl ::core::marker::Copy for IMAGE_LIST_WRITE_STREAM_FLAGS {}
impl ::core::clone::Clone for IMAGE_LIST_WRITE_STREAM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGE_LIST_WRITE_STREAM_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INITCOMMONCONTROLSEX_ICC(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_ANIMATE_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_BAR_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_COOL_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_DATE_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_HOTKEY_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_INTERNET_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_LINK_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_LISTVIEW_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_NATIVEFNTCTL_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_PAGESCROLLER_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_PROGRESS_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_STANDARD_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_TAB_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_TREEVIEW_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_UPDOWN_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_USEREX_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ICC_WIN95_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(255u32);
impl ::core::marker::Copy for INITCOMMONCONTROLSEX_ICC {}
impl ::core::clone::Clone for INITCOMMONCONTROLSEX_ICC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INITCOMMONCONTROLSEX_ICC {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ITEMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSI_HOT: ITEMSTATES = ITEMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSI_HOTSELECTED: ITEMSTATES = ITEMSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSI_SELECTED: ITEMSTATES = ITEMSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBPSI_SELECTEDNOTFOCUS: ITEMSTATES = ITEMSTATES(4i32);
impl ::core::marker::Copy for ITEMSTATES {}
impl ::core::clone::Clone for ITEMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ITEMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LABELSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLS_NORMAL: LABELSTATES = LABELSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLS_SELECTED: LABELSTATES = LABELSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLS_EMPHASIZED: LABELSTATES = LABELSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLS_DISABLED: LABELSTATES = LABELSTATES(4i32);
impl ::core::marker::Copy for LABELSTATES {}
impl ::core::clone::Clone for LABELSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LABELSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LINKHEADERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLH_NORMAL: LINKHEADERSTATES = LINKHEADERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLH_HOVER: LINKHEADERSTATES = LINKHEADERSTATES(2i32);
impl ::core::marker::Copy for LINKHEADERSTATES {}
impl ::core::clone::Clone for LINKHEADERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINKHEADERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LINKPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LP_HYPERLINK: LINKPARTS = LINKPARTS(1i32);
impl ::core::marker::Copy for LINKPARTS {}
impl ::core::clone::Clone for LINKPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINKPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LINKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLYOUTLINK_NORMAL: LINKSTATES = LINKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const FLYOUTLINK_HOVER: LINKSTATES = LINKSTATES(2i32);
impl ::core::marker::Copy for LINKSTATES {}
impl ::core::clone::Clone for LINKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LINKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LISTBOXPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBCP_BORDER_HSCROLL: LISTBOXPARTS = LISTBOXPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBCP_BORDER_HVSCROLL: LISTBOXPARTS = LISTBOXPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBCP_BORDER_NOSCROLL: LISTBOXPARTS = LISTBOXPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBCP_BORDER_VSCROLL: LISTBOXPARTS = LISTBOXPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LBCP_ITEM: LISTBOXPARTS = LISTBOXPARTS(5i32);
impl ::core::marker::Copy for LISTBOXPARTS {}
impl ::core::clone::Clone for LISTBOXPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LISTBOXPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LISTITEMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LISS_NORMAL: LISTITEMSTATES = LISTITEMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LISS_HOT: LISTITEMSTATES = LISTITEMSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LISS_SELECTED: LISTITEMSTATES = LISTITEMSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LISS_DISABLED: LISTITEMSTATES = LISTITEMSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LISS_SELECTEDNOTFOCUS: LISTITEMSTATES = LISTITEMSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LISS_HOTSELECTED: LISTITEMSTATES = LISTITEMSTATES(6i32);
impl ::core::marker::Copy for LISTITEMSTATES {}
impl ::core::clone::Clone for LISTITEMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LISTITEMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LISTVIEWPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVP_LISTITEM: LISTVIEWPARTS = LISTVIEWPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVP_LISTGROUP: LISTVIEWPARTS = LISTVIEWPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVP_LISTDETAIL: LISTVIEWPARTS = LISTVIEWPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVP_LISTSORTEDDETAIL: LISTVIEWPARTS = LISTVIEWPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVP_EMPTYTEXT: LISTVIEWPARTS = LISTVIEWPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVP_GROUPHEADER: LISTVIEWPARTS = LISTVIEWPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVP_GROUPHEADERLINE: LISTVIEWPARTS = LISTVIEWPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVP_EXPANDBUTTON: LISTVIEWPARTS = LISTVIEWPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVP_COLLAPSEBUTTON: LISTVIEWPARTS = LISTVIEWPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVP_COLUMNDETAIL: LISTVIEWPARTS = LISTVIEWPARTS(10i32);
impl ::core::marker::Copy for LISTVIEWPARTS {}
impl ::core::clone::Clone for LISTVIEWPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LISTVIEWPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LIST_ITEM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIF_ITEMINDEX: LIST_ITEM_FLAGS = LIST_ITEM_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIF_STATE: LIST_ITEM_FLAGS = LIST_ITEM_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIF_ITEMID: LIST_ITEM_FLAGS = LIST_ITEM_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIF_URL: LIST_ITEM_FLAGS = LIST_ITEM_FLAGS(8u32);
impl ::core::marker::Copy for LIST_ITEM_FLAGS {}
impl ::core::clone::Clone for LIST_ITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LIST_ITEM_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LIST_ITEM_STATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIS_FOCUSED: LIST_ITEM_STATE_FLAGS = LIST_ITEM_STATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIS_ENABLED: LIST_ITEM_STATE_FLAGS = LIST_ITEM_STATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIS_VISITED: LIST_ITEM_STATE_FLAGS = LIST_ITEM_STATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIS_HOTTRACK: LIST_ITEM_STATE_FLAGS = LIST_ITEM_STATE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIS_DEFAULTCOLORS: LIST_ITEM_STATE_FLAGS = LIST_ITEM_STATE_FLAGS(16u32);
impl ::core::marker::Copy for LIST_ITEM_STATE_FLAGS {}
impl ::core::clone::Clone for LIST_ITEM_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LIST_ITEM_STATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LIST_VIEW_BACKGROUND_IMAGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVBKIF_SOURCE_NONE: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVBKIF_SOURCE_HBITMAP: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVBKIF_SOURCE_URL: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVBKIF_SOURCE_MASK: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVBKIF_STYLE_NORMAL: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVBKIF_STYLE_TILE: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVBKIF_STYLE_MASK: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVBKIF_FLAG_TILEOFFSET: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVBKIF_TYPE_WATERMARK: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVBKIF_FLAG_ALPHABLEND: LIST_VIEW_BACKGROUND_IMAGE_FLAGS = LIST_VIEW_BACKGROUND_IMAGE_FLAGS(536870912u32);
impl ::core::marker::Copy for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {}
impl ::core::clone::Clone for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LIST_VIEW_BACKGROUND_IMAGE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LIST_VIEW_GROUP_ALIGN_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGA_HEADER_LEFT: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGA_HEADER_CENTER: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGA_HEADER_RIGHT: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGA_FOOTER_LEFT: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGA_FOOTER_CENTER: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGA_FOOTER_RIGHT: LIST_VIEW_GROUP_ALIGN_FLAGS = LIST_VIEW_GROUP_ALIGN_FLAGS(32u32);
impl ::core::marker::Copy for LIST_VIEW_GROUP_ALIGN_FLAGS {}
impl ::core::clone::Clone for LIST_VIEW_GROUP_ALIGN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LIST_VIEW_GROUP_ALIGN_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LIST_VIEW_GROUP_STATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGS_NORMAL: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGS_COLLAPSED: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGS_HIDDEN: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGS_NOHEADER: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGS_COLLAPSIBLE: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGS_FOCUSED: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGS_SELECTED: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGS_SUBSETED: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGS_SUBSETLINKFOCUSED: LIST_VIEW_GROUP_STATE_FLAGS = LIST_VIEW_GROUP_STATE_FLAGS(128u32);
impl ::core::marker::Copy for LIST_VIEW_GROUP_STATE_FLAGS {}
impl ::core::clone::Clone for LIST_VIEW_GROUP_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LIST_VIEW_GROUP_STATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LIST_VIEW_INSERT_MARK_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIM_AFTER: LIST_VIEW_INSERT_MARK_FLAGS = LIST_VIEW_INSERT_MARK_FLAGS(1u32);
impl ::core::marker::Copy for LIST_VIEW_INSERT_MARK_FLAGS {}
impl ::core::clone::Clone for LIST_VIEW_INSERT_MARK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LIST_VIEW_INSERT_MARK_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_LINE_BREAK: LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS = LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(1048576i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_FILL: LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS = LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(2097152i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_WRAP: LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS = LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(4194304i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_NO_TITLE: LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS = LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(8388608i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_TILE_PLACEMENTMASK: LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS = LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS(3145728i32);
impl ::core::marker::Copy for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {}
impl ::core::clone::Clone for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LIST_VIEW_ITEM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIF_TEXT: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIF_IMAGE: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIF_PARAM: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIF_STATE: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIF_INDENT: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIF_NORECOMPUTE: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIF_GROUPID: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIF_COLUMNS: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIF_COLFMT: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIF_DI_SETITEM: LIST_VIEW_ITEM_FLAGS = LIST_VIEW_ITEM_FLAGS(4096u32);
impl ::core::marker::Copy for LIST_VIEW_ITEM_FLAGS {}
impl ::core::clone::Clone for LIST_VIEW_ITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LIST_VIEW_ITEM_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LIST_VIEW_ITEM_STATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIS_FOCUSED: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIS_SELECTED: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIS_CUT: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIS_DROPHILITED: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIS_GLOW: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIS_ACTIVATING: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIS_OVERLAYMASK: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(3840u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVIS_STATEIMAGEMASK: LIST_VIEW_ITEM_STATE_FLAGS = LIST_VIEW_ITEM_STATE_FLAGS(61440u32);
impl ::core::marker::Copy for LIST_VIEW_ITEM_STATE_FLAGS {}
impl ::core::clone::Clone for LIST_VIEW_ITEM_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LIST_VIEW_ITEM_STATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOGOFFBUTTONSSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPLS_NORMAL: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPLS_HOT: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPLS_PRESSED: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(3i32);
impl ::core::marker::Copy for LOGOFFBUTTONSSTATES {}
impl ::core::clone::Clone for LOGOFFBUTTONSSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LOGOFFBUTTONSSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LVCOLUMNW_FORMAT(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_LEFT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_RIGHT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_CENTER: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_JUSTIFYMASK: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_IMAGE: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_BITMAP_ON_RIGHT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_COL_HAS_IMAGES: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_FIXED_WIDTH: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_NO_DPI_SCALE: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(262144u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_FIXED_RATIO: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(524288u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCFMT_SPLITBUTTON: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(16777216u32);
impl ::core::marker::Copy for LVCOLUMNW_FORMAT {}
impl ::core::clone::Clone for LVCOLUMNW_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVCOLUMNW_FORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LVCOLUMNW_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCF_FMT: LVCOLUMNW_MASK = LVCOLUMNW_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCF_WIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCF_TEXT: LVCOLUMNW_MASK = LVCOLUMNW_MASK(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCF_SUBITEM: LVCOLUMNW_MASK = LVCOLUMNW_MASK(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCF_IMAGE: LVCOLUMNW_MASK = LVCOLUMNW_MASK(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCF_ORDER: LVCOLUMNW_MASK = LVCOLUMNW_MASK(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCF_MINWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCF_DEFAULTWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCF_IDEALWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(256u32);
impl ::core::marker::Copy for LVCOLUMNW_MASK {}
impl ::core::clone::Clone for LVCOLUMNW_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVCOLUMNW_MASK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LVFINDINFOW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVFI_PARAM: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVFI_PARTIAL: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVFI_STRING: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVFI_SUBSTRING: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVFI_WRAP: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVFI_NEARESTXY: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(64u32);
impl ::core::marker::Copy for LVFINDINFOW_FLAGS {}
impl ::core::clone::Clone for LVFINDINFOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVFINDINFOW_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LVFOOTERITEM_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVFIF_TEXT: LVFOOTERITEM_MASK = LVFOOTERITEM_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVFIF_STATE: LVFOOTERITEM_MASK = LVFOOTERITEM_MASK(2u32);
impl ::core::marker::Copy for LVFOOTERITEM_MASK {}
impl ::core::clone::Clone for LVFOOTERITEM_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVFOOTERITEM_MASK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LVGROUP_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_NONE: LVGROUP_MASK = LVGROUP_MASK(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_HEADER: LVGROUP_MASK = LVGROUP_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_FOOTER: LVGROUP_MASK = LVGROUP_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGF_STATE: LVGROUP_MASK = LVGROUP_MASK(4u32);
impl ::core::marker::Copy for LVGROUP_MASK {}
impl ::core::clone::Clone for LVGROUP_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVGROUP_MASK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LVHITTESTINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_ABOVE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_BELOW: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_NOWHERE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_ONITEMICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_ONITEMLABEL: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_ONITEMSTATEICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_TOLEFT: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_TORIGHT: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_EX_GROUP_HEADER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_EX_GROUP_FOOTER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_EX_GROUP_COLLAPSE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_EX_GROUP_BACKGROUND: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_EX_GROUP_STATEICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_EX_GROUP_SUBSETLINK: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_EX_GROUP: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(4076863488u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_EX_ONCONTENTS: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVHT_EX_FOOTER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(134217728u32);
impl ::core::marker::Copy for LVHITTESTINFO_FLAGS {}
impl ::core::clone::Clone for LVHITTESTINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVHITTESTINFO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LVITEMA_GROUP_ID(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const I_GROUPIDCALLBACK: LVITEMA_GROUP_ID = LVITEMA_GROUP_ID(-1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const I_GROUPIDNONE: LVITEMA_GROUP_ID = LVITEMA_GROUP_ID(-2i32);
impl ::core::marker::Copy for LVITEMA_GROUP_ID {}
impl ::core::clone::Clone for LVITEMA_GROUP_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVITEMA_GROUP_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LVTILEVIEWINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVTVIF_AUTOSIZE: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVTVIF_FIXEDWIDTH: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVTVIF_FIXEDHEIGHT: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVTVIF_FIXEDSIZE: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(3u32);
impl ::core::marker::Copy for LVTILEVIEWINFO_FLAGS {}
impl ::core::clone::Clone for LVTILEVIEWINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVTILEVIEWINFO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LVTILEVIEWINFO_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVTVIM_TILESIZE: LVTILEVIEWINFO_MASK = LVTILEVIEWINFO_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVTVIM_COLUMNS: LVTILEVIEWINFO_MASK = LVTILEVIEWINFO_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVTVIM_LABELMARGIN: LVTILEVIEWINFO_MASK = LVTILEVIEWINFO_MASK(4u32);
impl ::core::marker::Copy for LVTILEVIEWINFO_MASK {}
impl ::core::clone::Clone for LVTILEVIEWINFO_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVTILEVIEWINFO_MASK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MARKUPTEXTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EMT_NORMALTEXT: MARKUPTEXTSTATES = MARKUPTEXTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EMT_LINKTEXT: MARKUPTEXTSTATES = MARKUPTEXTSTATES(2i32);
impl ::core::marker::Copy for MARKUPTEXTSTATES {}
impl ::core::clone::Clone for MARKUPTEXTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MARKUPTEXTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MAXBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MAXBS_NORMAL: MAXBUTTONSTATES = MAXBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MAXBS_HOT: MAXBUTTONSTATES = MAXBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MAXBS_PUSHED: MAXBUTTONSTATES = MAXBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MAXBS_DISABLED: MAXBUTTONSTATES = MAXBUTTONSTATES(4i32);
impl ::core::marker::Copy for MAXBUTTONSTATES {}
impl ::core::clone::Clone for MAXBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MAXBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MAXCAPTIONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MXCS_ACTIVE: MAXCAPTIONSTATES = MAXCAPTIONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MXCS_INACTIVE: MAXCAPTIONSTATES = MAXCAPTIONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MXCS_DISABLED: MAXCAPTIONSTATES = MAXCAPTIONSTATES(3i32);
impl ::core::marker::Copy for MAXCAPTIONSTATES {}
impl ::core::clone::Clone for MAXCAPTIONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MAXCAPTIONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MCGRIDINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIF_DATE: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIF_RECT: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIF_NAME: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(4u32);
impl ::core::marker::Copy for MCGRIDINFO_FLAGS {}
impl ::core::clone::Clone for MCGRIDINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCGRIDINFO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MCGRIDINFO_PART(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIP_CALENDARCONTROL: MCGRIDINFO_PART = MCGRIDINFO_PART(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIP_NEXT: MCGRIDINFO_PART = MCGRIDINFO_PART(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIP_PREV: MCGRIDINFO_PART = MCGRIDINFO_PART(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIP_FOOTER: MCGRIDINFO_PART = MCGRIDINFO_PART(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIP_CALENDAR: MCGRIDINFO_PART = MCGRIDINFO_PART(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIP_CALENDARHEADER: MCGRIDINFO_PART = MCGRIDINFO_PART(5u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIP_CALENDARBODY: MCGRIDINFO_PART = MCGRIDINFO_PART(6u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIP_CALENDARROW: MCGRIDINFO_PART = MCGRIDINFO_PART(7u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCGIP_CALENDARCELL: MCGRIDINFO_PART = MCGRIDINFO_PART(8u32);
impl ::core::marker::Copy for MCGRIDINFO_PART {}
impl ::core::clone::Clone for MCGRIDINFO_PART {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCGRIDINFO_PART {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MCHITTESTINFO_HIT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_TITLE: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_CALENDAR: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_TODAYLINK: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(196608u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_CALENDARCONTROL: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_NEXT: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_PREV: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_NOWHERE: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_TITLEBK: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_TITLEMONTH: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(65537u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_TITLEYEAR: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(65538u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_TITLEBTNNEXT: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(16842755u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_TITLEBTNPREV: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(33619971u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_CALENDARBK: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_CALENDARDATE: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131073u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_CALENDARDATENEXT: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(16908289u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_CALENDARDATEPREV: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(33685505u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_CALENDARDAY: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131074u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_CALENDARWEEKNUM: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131075u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_CALENDARDATEMIN: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131076u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCHT_CALENDARDATEMAX: MCHITTESTINFO_HIT_FLAGS = MCHITTESTINFO_HIT_FLAGS(131077u32);
impl ::core::marker::Copy for MCHITTESTINFO_HIT_FLAGS {}
impl ::core::clone::Clone for MCHITTESTINFO_HIT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCHITTESTINFO_HIT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MDICLOSEBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDCL_NORMAL: MDICLOSEBUTTONSTATES = MDICLOSEBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDCL_HOT: MDICLOSEBUTTONSTATES = MDICLOSEBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDCL_PUSHED: MDICLOSEBUTTONSTATES = MDICLOSEBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDCL_DISABLED: MDICLOSEBUTTONSTATES = MDICLOSEBUTTONSTATES(4i32);
impl ::core::marker::Copy for MDICLOSEBUTTONSTATES {}
impl ::core::clone::Clone for MDICLOSEBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MDICLOSEBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MDIMINBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDMI_NORMAL: MDIMINBUTTONSTATES = MDIMINBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDMI_HOT: MDIMINBUTTONSTATES = MDIMINBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDMI_PUSHED: MDIMINBUTTONSTATES = MDIMINBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDMI_DISABLED: MDIMINBUTTONSTATES = MDIMINBUTTONSTATES(4i32);
impl ::core::marker::Copy for MDIMINBUTTONSTATES {}
impl ::core::clone::Clone for MDIMINBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MDIMINBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MDIRESTOREBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDRE_NORMAL: MDIRESTOREBUTTONSTATES = MDIRESTOREBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDRE_HOT: MDIRESTOREBUTTONSTATES = MDIRESTOREBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDRE_PUSHED: MDIRESTOREBUTTONSTATES = MDIRESTOREBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDRE_DISABLED: MDIRESTOREBUTTONSTATES = MDIRESTOREBUTTONSTATES(4i32);
impl ::core::marker::Copy for MDIRESTOREBUTTONSTATES {}
impl ::core::clone::Clone for MDIRESTOREBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MDIRESTOREBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MENUBANDPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDP_NEWAPPBUTTON: MENUBANDPARTS = MENUBANDPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDP_SEPERATOR: MENUBANDPARTS = MENUBANDPARTS(2i32);
impl ::core::marker::Copy for MENUBANDPARTS {}
impl ::core::clone::Clone for MENUBANDPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MENUBANDPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MENUBANDSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDS_NORMAL: MENUBANDSTATES = MENUBANDSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDS_HOT: MENUBANDSTATES = MENUBANDSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDS_PRESSED: MENUBANDSTATES = MENUBANDSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDS_DISABLED: MENUBANDSTATES = MENUBANDSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDS_CHECKED: MENUBANDSTATES = MENUBANDSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MDS_HOTCHECKED: MENUBANDSTATES = MENUBANDSTATES(6i32);
impl ::core::marker::Copy for MENUBANDSTATES {}
impl ::core::clone::Clone for MENUBANDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MENUBANDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MENUPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_MENUITEM_TMSCHEMA: MENUPARTS = MENUPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_MENUDROPDOWN_TMSCHEMA: MENUPARTS = MENUPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_MENUBARITEM_TMSCHEMA: MENUPARTS = MENUPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_MENUBARDROPDOWN_TMSCHEMA: MENUPARTS = MENUPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_CHEVRON_TMSCHEMA: MENUPARTS = MENUPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_SEPARATOR_TMSCHEMA: MENUPARTS = MENUPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_BARBACKGROUND: MENUPARTS = MENUPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_BARITEM: MENUPARTS = MENUPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_POPUPBACKGROUND: MENUPARTS = MENUPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_POPUPBORDERS: MENUPARTS = MENUPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_POPUPCHECK: MENUPARTS = MENUPARTS(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_POPUPCHECKBACKGROUND: MENUPARTS = MENUPARTS(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_POPUPGUTTER: MENUPARTS = MENUPARTS(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_POPUPITEM: MENUPARTS = MENUPARTS(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_POPUPSEPARATOR: MENUPARTS = MENUPARTS(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_POPUPSUBMENU: MENUPARTS = MENUPARTS(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_SYSTEMCLOSE: MENUPARTS = MENUPARTS(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_SYSTEMMAXIMIZE: MENUPARTS = MENUPARTS(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_SYSTEMMINIMIZE: MENUPARTS = MENUPARTS(19i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MENU_SYSTEMRESTORE: MENUPARTS = MENUPARTS(20i32);
impl ::core::marker::Copy for MENUPARTS {}
impl ::core::clone::Clone for MENUPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MENUPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MINBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MINBS_NORMAL: MINBUTTONSTATES = MINBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MINBS_HOT: MINBUTTONSTATES = MINBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MINBS_PUSHED: MINBUTTONSTATES = MINBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MINBS_DISABLED: MINBUTTONSTATES = MINBUTTONSTATES(4i32);
impl ::core::marker::Copy for MINBUTTONSTATES {}
impl ::core::clone::Clone for MINBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MINBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MINCAPTIONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MNCS_ACTIVE: MINCAPTIONSTATES = MINCAPTIONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MNCS_INACTIVE: MINCAPTIONSTATES = MINCAPTIONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MNCS_DISABLED: MINCAPTIONSTATES = MINCAPTIONSTATES(3i32);
impl ::core::marker::Copy for MINCAPTIONSTATES {}
impl ::core::clone::Clone for MINCAPTIONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MINCAPTIONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MONTHCALPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_BACKGROUND: MONTHCALPARTS = MONTHCALPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_BORDERS: MONTHCALPARTS = MONTHCALPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_GRIDBACKGROUND: MONTHCALPARTS = MONTHCALPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_COLHEADERSPLITTER: MONTHCALPARTS = MONTHCALPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_GRIDCELLBACKGROUND: MONTHCALPARTS = MONTHCALPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_GRIDCELL: MONTHCALPARTS = MONTHCALPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_GRIDCELLUPPER: MONTHCALPARTS = MONTHCALPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_TRAILINGGRIDCELL: MONTHCALPARTS = MONTHCALPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_TRAILINGGRIDCELLUPPER: MONTHCALPARTS = MONTHCALPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_NAVNEXT: MONTHCALPARTS = MONTHCALPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_NAVPREV: MONTHCALPARTS = MONTHCALPARTS(11i32);
impl ::core::marker::Copy for MONTHCALPARTS {}
impl ::core::clone::Clone for MONTHCALPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONTHCALPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MONTH_CALDENDAR_MESSAGES_VIEW(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCMV_MONTH: MONTH_CALDENDAR_MESSAGES_VIEW = MONTH_CALDENDAR_MESSAGES_VIEW(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCMV_YEAR: MONTH_CALDENDAR_MESSAGES_VIEW = MONTH_CALDENDAR_MESSAGES_VIEW(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCMV_DECADE: MONTH_CALDENDAR_MESSAGES_VIEW = MONTH_CALDENDAR_MESSAGES_VIEW(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCMV_CENTURY: MONTH_CALDENDAR_MESSAGES_VIEW = MONTH_CALDENDAR_MESSAGES_VIEW(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCMV_MAX: MONTH_CALDENDAR_MESSAGES_VIEW = MONTH_CALDENDAR_MESSAGES_VIEW(3u32);
impl ::core::marker::Copy for MONTH_CALDENDAR_MESSAGES_VIEW {}
impl ::core::clone::Clone for MONTH_CALDENDAR_MESSAGES_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MONTH_CALDENDAR_MESSAGES_VIEW {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MOREPROGRAMSARROWBACKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPSB_NORMAL: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPSB_HOT: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPSB_PRESSED: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(3i32);
impl ::core::marker::Copy for MOREPROGRAMSARROWBACKSTATES {}
impl ::core::clone::Clone for MOREPROGRAMSARROWBACKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MOREPROGRAMSARROWBACKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MOREPROGRAMSARROWSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPS_NORMAL: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPS_HOT: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPS_PRESSED: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(3i32);
impl ::core::marker::Copy for MOREPROGRAMSARROWSTATES {}
impl ::core::clone::Clone for MOREPROGRAMSARROWSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MOREPROGRAMSARROWSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MOREPROGRAMSTABSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPMPT_NORMAL: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPMPT_HOT: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPMPT_SELECTED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPMPT_DISABLED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPMPT_FOCUSED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(5i32);
impl ::core::marker::Copy for MOREPROGRAMSTABSTATES {}
impl ::core::clone::Clone for MOREPROGRAMSTABSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MOREPROGRAMSTABSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MOVESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDMOVE_HIGHLIGHT: MOVESTATES = MOVESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDMOVE_NOHIGHLIGHT: MOVESTATES = MOVESTATES(2i32);
impl ::core::marker::Copy for MOVESTATES {}
impl ::core::clone::Clone for MOVESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MOVESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NAVIGATIONPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_BACKBUTTON: NAVIGATIONPARTS = NAVIGATIONPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_FORWARDBUTTON: NAVIGATIONPARTS = NAVIGATIONPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_MENUBUTTON: NAVIGATIONPARTS = NAVIGATIONPARTS(3i32);
impl ::core::marker::Copy for NAVIGATIONPARTS {}
impl ::core::clone::Clone for NAVIGATIONPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NAVIGATIONPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NAVNEXTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCNN_NORMAL: NAVNEXTSTATES = NAVNEXTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCNN_HOT: NAVNEXTSTATES = NAVNEXTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCNN_PRESSED: NAVNEXTSTATES = NAVNEXTSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCNN_DISABLED: NAVNEXTSTATES = NAVNEXTSTATES(4i32);
impl ::core::marker::Copy for NAVNEXTSTATES {}
impl ::core::clone::Clone for NAVNEXTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NAVNEXTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NAVPREVSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCNP_NORMAL: NAVPREVSTATES = NAVPREVSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCNP_HOT: NAVPREVSTATES = NAVPREVSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCNP_PRESSED: NAVPREVSTATES = NAVPREVSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCNP_DISABLED: NAVPREVSTATES = NAVPREVSTATES(4i32);
impl ::core::marker::Copy for NAVPREVSTATES {}
impl ::core::clone::Clone for NAVPREVSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NAVPREVSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NAV_BACKBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_BB_NORMAL: NAV_BACKBUTTONSTATES = NAV_BACKBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_BB_HOT: NAV_BACKBUTTONSTATES = NAV_BACKBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_BB_PRESSED: NAV_BACKBUTTONSTATES = NAV_BACKBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_BB_DISABLED: NAV_BACKBUTTONSTATES = NAV_BACKBUTTONSTATES(4i32);
impl ::core::marker::Copy for NAV_BACKBUTTONSTATES {}
impl ::core::clone::Clone for NAV_BACKBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NAV_BACKBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NAV_FORWARDBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_FB_NORMAL: NAV_FORWARDBUTTONSTATES = NAV_FORWARDBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_FB_HOT: NAV_FORWARDBUTTONSTATES = NAV_FORWARDBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_FB_PRESSED: NAV_FORWARDBUTTONSTATES = NAV_FORWARDBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_FB_DISABLED: NAV_FORWARDBUTTONSTATES = NAV_FORWARDBUTTONSTATES(4i32);
impl ::core::marker::Copy for NAV_FORWARDBUTTONSTATES {}
impl ::core::clone::Clone for NAV_FORWARDBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NAV_FORWARDBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NAV_MENUBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_MB_NORMAL: NAV_MENUBUTTONSTATES = NAV_MENUBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_MB_HOT: NAV_MENUBUTTONSTATES = NAV_MENUBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_MB_PRESSED: NAV_MENUBUTTONSTATES = NAV_MENUBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const NAV_MB_DISABLED: NAV_MENUBUTTONSTATES = NAV_MENUBUTTONSTATES(4i32);
impl ::core::marker::Copy for NAV_MENUBUTTONSTATES {}
impl ::core::clone::Clone for NAV_MENUBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NAV_MENUBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMCUSTOMDRAW_DRAW_STAGE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDDS_POSTPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDDS_PREERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDDS_PREPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDDS_ITEMPOSTERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65540u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDDS_ITEMPOSTPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65538u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDDS_ITEMPREERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65539u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDDS_ITEMPREPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65537u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDDS_SUBITEM: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(131072u32);
impl ::core::marker::Copy for NMCUSTOMDRAW_DRAW_STAGE {}
impl ::core::clone::Clone for NMCUSTOMDRAW_DRAW_STAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMCUSTOMDRAW_DRAW_STAGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMCUSTOMDRAW_DRAW_STATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_SELECTED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_GRAYED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_DISABLED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_CHECKED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_FOCUS: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_DEFAULT: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_HOT: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_MARKED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_INDETERMINATE: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_SHOWKEYBOARDCUES: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_NEARHOT: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_OTHERSIDEHOT: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CDIS_DROPHILITED: NMCUSTOMDRAW_DRAW_STATE_FLAGS = NMCUSTOMDRAW_DRAW_STATE_FLAGS(4096u32);
impl ::core::marker::Copy for NMCUSTOMDRAW_DRAW_STATE_FLAGS {}
impl ::core::clone::Clone for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMCUSTOMDRAW_DRAW_STATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMDATETIMECHANGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GDT_NONE: NMDATETIMECHANGE_FLAGS = NMDATETIMECHANGE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const GDT_VALID: NMDATETIMECHANGE_FLAGS = NMDATETIMECHANGE_FLAGS(0u32);
impl ::core::marker::Copy for NMDATETIMECHANGE_FLAGS {}
impl ::core::clone::Clone for NMDATETIMECHANGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMDATETIMECHANGE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMLVCUSTOMDRAW_ITEM_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCDI_ITEM: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCDI_GROUP: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVCDI_ITEMSLIST: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(2u32);
impl ::core::marker::Copy for NMLVCUSTOMDRAW_ITEM_TYPE {}
impl ::core::clone::Clone for NMLVCUSTOMDRAW_ITEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMLVCUSTOMDRAW_ITEM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMLVEMPTYMARKUP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EMF_CENTERED: NMLVEMPTYMARKUP_FLAGS = NMLVEMPTYMARKUP_FLAGS(1u32);
impl ::core::marker::Copy for NMLVEMPTYMARKUP_FLAGS {}
impl ::core::clone::Clone for NMLVEMPTYMARKUP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMLVEMPTYMARKUP_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMLVGETINFOTIP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGIT_UNFOLDED: NMLVGETINFOTIP_FLAGS = NMLVGETINFOTIP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LVGIT_ZERO: NMLVGETINFOTIP_FLAGS = NMLVGETINFOTIP_FLAGS(0u32);
impl ::core::marker::Copy for NMLVGETINFOTIP_FLAGS {}
impl ::core::clone::Clone for NMLVGETINFOTIP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMLVGETINFOTIP_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMPGCALCSIZE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_CALCHEIGHT: NMPGCALCSIZE_FLAGS = NMPGCALCSIZE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_CALCWIDTH: NMPGCALCSIZE_FLAGS = NMPGCALCSIZE_FLAGS(1u32);
impl ::core::marker::Copy for NMPGCALCSIZE_FLAGS {}
impl ::core::clone::Clone for NMPGCALCSIZE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMPGCALCSIZE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMPGSCROLL_DIR(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_SCROLLDOWN: NMPGSCROLL_DIR = NMPGSCROLL_DIR(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_SCROLLLEFT: NMPGSCROLL_DIR = NMPGSCROLL_DIR(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_SCROLLRIGHT: NMPGSCROLL_DIR = NMPGSCROLL_DIR(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGF_SCROLLUP: NMPGSCROLL_DIR = NMPGSCROLL_DIR(1u32);
impl ::core::marker::Copy for NMPGSCROLL_DIR {}
impl ::core::clone::Clone for NMPGSCROLL_DIR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMPGSCROLL_DIR {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMPGSCROLL_KEYS(pub u16);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGK_NONE: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(0u16);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGK_SHIFT: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(1u16);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGK_CONTROL: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(2u16);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGK_MENU: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(4u16);
impl ::core::marker::Copy for NMPGSCROLL_KEYS {}
impl ::core::clone::Clone for NMPGSCROLL_KEYS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMPGSCROLL_KEYS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMREBAR_MASK_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBNM_ID: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBNM_LPARAM: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBNM_STYLE: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(2u32);
impl ::core::marker::Copy for NMREBAR_MASK_FLAGS {}
impl ::core::clone::Clone for NMREBAR_MASK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMREBAR_MASK_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMTBDISPINFOW_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBNF_IMAGE: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBNF_TEXT: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBNF_DI_SETITEM: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(268435456u32);
impl ::core::marker::Copy for NMTBDISPINFOW_MASK {}
impl ::core::clone::Clone for NMTBDISPINFOW_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMTBDISPINFOW_MASK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NMTBHOTITEM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HICF_ACCELERATOR: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HICF_ARROWKEYS: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HICF_DUPACCEL: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HICF_ENTERING: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HICF_LEAVING: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HICF_LMOUSE: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HICF_MOUSE: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HICF_OTHER: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HICF_RESELECT: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const HICF_TOGGLEDROPDOWN: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(256u32);
impl ::core::marker::Copy for NMTBHOTITEM_FLAGS {}
impl ::core::clone::Clone for NMTBHOTITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NMTBHOTITEM_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NM_TREEVIEW_ACTION(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVE_COLLAPSE: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVE_EXPAND: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVE_TOGGLE: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVE_EXPANDPARTIAL: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVE_COLLAPSERESET: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVC_UNKNOWN: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVC_BYMOUSE: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVC_BYKEYBOARD: NM_TREEVIEW_ACTION = NM_TREEVIEW_ACTION(2u32);
impl ::core::marker::Copy for NM_TREEVIEW_ACTION {}
impl ::core::clone::Clone for NM_TREEVIEW_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NM_TREEVIEW_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NONESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDNONE_HIGHLIGHT: NONESTATES = NONESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDNONE_NOHIGHLIGHT: NONESTATES = NONESTATES(2i32);
impl ::core::marker::Copy for NONESTATES {}
impl ::core::clone::Clone for NONESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NONESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NORMALGROUPCOLLAPSESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBNGC_NORMAL: NORMALGROUPCOLLAPSESTATES = NORMALGROUPCOLLAPSESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBNGC_HOT: NORMALGROUPCOLLAPSESTATES = NORMALGROUPCOLLAPSESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBNGC_PRESSED: NORMALGROUPCOLLAPSESTATES = NORMALGROUPCOLLAPSESTATES(3i32);
impl ::core::marker::Copy for NORMALGROUPCOLLAPSESTATES {}
impl ::core::clone::Clone for NORMALGROUPCOLLAPSESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NORMALGROUPCOLLAPSESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NORMALGROUPEXPANDSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBNGE_NORMAL: NORMALGROUPEXPANDSTATES = NORMALGROUPEXPANDSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBNGE_HOT: NORMALGROUPEXPANDSTATES = NORMALGROUPEXPANDSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBNGE_PRESSED: NORMALGROUPEXPANDSTATES = NORMALGROUPEXPANDSTATES(3i32);
impl ::core::marker::Copy for NORMALGROUPEXPANDSTATES {}
impl ::core::clone::Clone for NORMALGROUPEXPANDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NORMALGROUPEXPANDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ODA_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODA_DRAWENTIRE: ODA_FLAGS = ODA_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODA_SELECT: ODA_FLAGS = ODA_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODA_FOCUS: ODA_FLAGS = ODA_FLAGS(4u32);
impl ::core::marker::Copy for ODA_FLAGS {}
impl ::core::clone::Clone for ODA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ODA_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ODS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_SELECTED: ODS_FLAGS = ODS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_GRAYED: ODS_FLAGS = ODS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_DISABLED: ODS_FLAGS = ODS_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_CHECKED: ODS_FLAGS = ODS_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_FOCUS: ODS_FLAGS = ODS_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_DEFAULT: ODS_FLAGS = ODS_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_COMBOBOXEDIT: ODS_FLAGS = ODS_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_HOTLIGHT: ODS_FLAGS = ODS_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_INACTIVE: ODS_FLAGS = ODS_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_NOACCEL: ODS_FLAGS = ODS_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ODS_NOFOCUSRECT: ODS_FLAGS = ODS_FLAGS(512u32);
impl ::core::marker::Copy for ODS_FLAGS {}
impl ::core::clone::Clone for ODS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ODS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFSETTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_TOPLEFT: OFFSETTYPE = OFFSETTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_TOPRIGHT: OFFSETTYPE = OFFSETTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_TOPMIDDLE: OFFSETTYPE = OFFSETTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_BOTTOMLEFT: OFFSETTYPE = OFFSETTYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_BOTTOMRIGHT: OFFSETTYPE = OFFSETTYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_BOTTOMMIDDLE: OFFSETTYPE = OFFSETTYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_MIDDLELEFT: OFFSETTYPE = OFFSETTYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_MIDDLERIGHT: OFFSETTYPE = OFFSETTYPE(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_LEFTOFCAPTION: OFFSETTYPE = OFFSETTYPE(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_RIGHTOFCAPTION: OFFSETTYPE = OFFSETTYPE(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_LEFTOFLASTBUTTON: OFFSETTYPE = OFFSETTYPE(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_RIGHTOFLASTBUTTON: OFFSETTYPE = OFFSETTYPE(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_ABOVELASTBUTTON: OFFSETTYPE = OFFSETTYPE(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OT_BELOWLASTBUTTON: OFFSETTYPE = OFFSETTYPE(13i32);
impl ::core::marker::Copy for OFFSETTYPE {}
impl ::core::clone::Clone for OFFSETTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OFFSETTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPENBOXSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPOB_NORMAL: OPENBOXSTATES = OPENBOXSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPOB_HOT: OPENBOXSTATES = OPENBOXSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPOB_SELECTED: OPENBOXSTATES = OPENBOXSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPOB_DISABLED: OPENBOXSTATES = OPENBOXSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPOB_FOCUSED: OPENBOXSTATES = OPENBOXSTATES(5i32);
impl ::core::marker::Copy for OPENBOXSTATES {}
impl ::core::clone::Clone for OPENBOXSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OPENBOXSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPEN_THEME_DATA_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OTD_FORCE_RECT_SIZING: OPEN_THEME_DATA_FLAGS = OPEN_THEME_DATA_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const OTD_NONCLIENT: OPEN_THEME_DATA_FLAGS = OPEN_THEME_DATA_FLAGS(2u32);
impl ::core::marker::Copy for OPEN_THEME_DATA_FLAGS {}
impl ::core::clone::Clone for OPEN_THEME_DATA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OPEN_THEME_DATA_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PAGEPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGRP_UP: PAGEPARTS = PAGEPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGRP_DOWN: PAGEPARTS = PAGEPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGRP_UPHORZ: PAGEPARTS = PAGEPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PGRP_DOWNHORZ: PAGEPARTS = PAGEPARTS(4i32);
impl ::core::marker::Copy for PAGEPARTS {}
impl ::core::clone::Clone for PAGEPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PAGEPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POINTER_DEVICE_CURSOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_DEVICE_CURSOR_TYPE_UNKNOWN: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_DEVICE_CURSOR_TYPE_TIP: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_DEVICE_CURSOR_TYPE_ERASER: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_DEVICE_CURSOR_TYPE_MAX: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(-1i32);
impl ::core::marker::Copy for POINTER_DEVICE_CURSOR_TYPE {}
impl ::core::clone::Clone for POINTER_DEVICE_CURSOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POINTER_DEVICE_CURSOR_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POINTER_DEVICE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_DEVICE_TYPE_INTEGRATED_PEN: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_DEVICE_TYPE_EXTERNAL_PEN: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_DEVICE_TYPE_TOUCH: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_DEVICE_TYPE_TOUCH_PAD: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_DEVICE_TYPE_MAX: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(-1i32);
impl ::core::marker::Copy for POINTER_DEVICE_TYPE {}
impl ::core::clone::Clone for POINTER_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POINTER_DEVICE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POINTER_FEEDBACK_MODE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_FEEDBACK_DEFAULT: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_FEEDBACK_INDIRECT: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const POINTER_FEEDBACK_NONE: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(3i32);
impl ::core::marker::Copy for POINTER_FEEDBACK_MODE {}
impl ::core::clone::Clone for POINTER_FEEDBACK_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POINTER_FEEDBACK_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POPUPCHECKBACKGROUNDSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCB_DISABLED: POPUPCHECKBACKGROUNDSTATES = POPUPCHECKBACKGROUNDSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCB_NORMAL: POPUPCHECKBACKGROUNDSTATES = POPUPCHECKBACKGROUNDSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCB_BITMAP: POPUPCHECKBACKGROUNDSTATES = POPUPCHECKBACKGROUNDSTATES(3i32);
impl ::core::marker::Copy for POPUPCHECKBACKGROUNDSTATES {}
impl ::core::clone::Clone for POPUPCHECKBACKGROUNDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POPUPCHECKBACKGROUNDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POPUPCHECKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_CHECKMARKNORMAL: POPUPCHECKSTATES = POPUPCHECKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_CHECKMARKDISABLED: POPUPCHECKSTATES = POPUPCHECKSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_BULLETNORMAL: POPUPCHECKSTATES = POPUPCHECKSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MC_BULLETDISABLED: POPUPCHECKSTATES = POPUPCHECKSTATES(4i32);
impl ::core::marker::Copy for POPUPCHECKSTATES {}
impl ::core::clone::Clone for POPUPCHECKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POPUPCHECKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POPUPITEMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MPI_NORMAL: POPUPITEMSTATES = POPUPITEMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MPI_HOT: POPUPITEMSTATES = POPUPITEMSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MPI_DISABLED: POPUPITEMSTATES = POPUPITEMSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MPI_DISABLEDHOT: POPUPITEMSTATES = POPUPITEMSTATES(4i32);
impl ::core::marker::Copy for POPUPITEMSTATES {}
impl ::core::clone::Clone for POPUPITEMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POPUPITEMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POPUPSUBMENUSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSM_NORMAL: POPUPSUBMENUSTATES = POPUPSUBMENUSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSM_DISABLED: POPUPSUBMENUSTATES = POPUPSUBMENUSTATES(2i32);
impl ::core::marker::Copy for POPUPSUBMENUSTATES {}
impl ::core::clone::Clone for POPUPSUBMENUSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POPUPSUBMENUSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROGRESSPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_BAR: PROGRESSPARTS = PROGRESSPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_BARVERT: PROGRESSPARTS = PROGRESSPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_CHUNK: PROGRESSPARTS = PROGRESSPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_CHUNKVERT: PROGRESSPARTS = PROGRESSPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_FILL: PROGRESSPARTS = PROGRESSPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_FILLVERT: PROGRESSPARTS = PROGRESSPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_PULSEOVERLAY: PROGRESSPARTS = PROGRESSPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_MOVEOVERLAY: PROGRESSPARTS = PROGRESSPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_PULSEOVERLAYVERT: PROGRESSPARTS = PROGRESSPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_MOVEOVERLAYVERT: PROGRESSPARTS = PROGRESSPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_TRANSPARENTBAR: PROGRESSPARTS = PROGRESSPARTS(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PP_TRANSPARENTBARVERT: PROGRESSPARTS = PROGRESSPARTS(12i32);
impl ::core::marker::Copy for PROGRESSPARTS {}
impl ::core::clone::Clone for PROGRESSPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROGRESSPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPERTYORIGIN(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PO_STATE: PROPERTYORIGIN = PROPERTYORIGIN(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PO_PART: PROPERTYORIGIN = PROPERTYORIGIN(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PO_CLASS: PROPERTYORIGIN = PROPERTYORIGIN(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PO_GLOBAL: PROPERTYORIGIN = PROPERTYORIGIN(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PO_NOTFOUND: PROPERTYORIGIN = PROPERTYORIGIN(4i32);
impl ::core::marker::Copy for PROPERTYORIGIN {}
impl ::core::clone::Clone for PROPERTYORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROPERTYORIGIN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PSPCB_MESSAGE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSPCB_ADDREF: PSPCB_MESSAGE = PSPCB_MESSAGE(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSPCB_CREATE: PSPCB_MESSAGE = PSPCB_MESSAGE(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSPCB_RELEASE: PSPCB_MESSAGE = PSPCB_MESSAGE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PSPCB_SI_INITDIALOG: PSPCB_MESSAGE = PSPCB_MESSAGE(1025u32);
impl ::core::marker::Copy for PSPCB_MESSAGE {}
impl ::core::clone::Clone for PSPCB_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PSPCB_MESSAGE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PUSHBUTTONDROPDOWNSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBDDS_NORMAL: PUSHBUTTONDROPDOWNSTATES = PUSHBUTTONDROPDOWNSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBDDS_DISABLED: PUSHBUTTONDROPDOWNSTATES = PUSHBUTTONDROPDOWNSTATES(2i32);
impl ::core::marker::Copy for PUSHBUTTONDROPDOWNSTATES {}
impl ::core::clone::Clone for PUSHBUTTONDROPDOWNSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PUSHBUTTONDROPDOWNSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PUSHBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBS_NORMAL: PUSHBUTTONSTATES = PUSHBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBS_HOT: PUSHBUTTONSTATES = PUSHBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBS_PRESSED: PUSHBUTTONSTATES = PUSHBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBS_DISABLED: PUSHBUTTONSTATES = PUSHBUTTONSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBS_DEFAULTED: PUSHBUTTONSTATES = PUSHBUTTONSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBS_DEFAULTED_ANIMATING: PUSHBUTTONSTATES = PUSHBUTTONSTATES(6i32);
impl ::core::marker::Copy for PUSHBUTTONSTATES {}
impl ::core::clone::Clone for PUSHBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PUSHBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RADIOBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_UNCHECKEDNORMAL: RADIOBUTTONSTATES = RADIOBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_UNCHECKEDHOT: RADIOBUTTONSTATES = RADIOBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_UNCHECKEDPRESSED: RADIOBUTTONSTATES = RADIOBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_UNCHECKEDDISABLED: RADIOBUTTONSTATES = RADIOBUTTONSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_CHECKEDNORMAL: RADIOBUTTONSTATES = RADIOBUTTONSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_CHECKEDHOT: RADIOBUTTONSTATES = RADIOBUTTONSTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_CHECKEDPRESSED: RADIOBUTTONSTATES = RADIOBUTTONSTATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_CHECKEDDISABLED: RADIOBUTTONSTATES = RADIOBUTTONSTATES(8i32);
impl ::core::marker::Copy for RADIOBUTTONSTATES {}
impl ::core::clone::Clone for RADIOBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RADIOBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct READONLYSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBRO_NORMAL: READONLYSTATES = READONLYSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBRO_HOT: READONLYSTATES = READONLYSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBRO_PRESSED: READONLYSTATES = READONLYSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBRO_DISABLED: READONLYSTATES = READONLYSTATES(4i32);
impl ::core::marker::Copy for READONLYSTATES {}
impl ::core::clone::Clone for READONLYSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for READONLYSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REBARPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RP_GRIPPER: REBARPARTS = REBARPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RP_GRIPPERVERT: REBARPARTS = REBARPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RP_BAND: REBARPARTS = REBARPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RP_CHEVRON: REBARPARTS = REBARPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RP_CHEVRONVERT: REBARPARTS = REBARPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RP_BACKGROUND: REBARPARTS = REBARPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RP_SPLITTER: REBARPARTS = REBARPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RP_SPLITTERVERT: REBARPARTS = REBARPARTS(8i32);
impl ::core::marker::Copy for REBARPARTS {}
impl ::core::clone::Clone for REBARPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REBARPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RESTOREBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_NORMAL: RESTOREBUTTONSTATES = RESTOREBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_HOT: RESTOREBUTTONSTATES = RESTOREBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_PUSHED: RESTOREBUTTONSTATES = RESTOREBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const RBS_DISABLED: RESTOREBUTTONSTATES = RESTOREBUTTONSTATES(4i32);
impl ::core::marker::Copy for RESTOREBUTTONSTATES {}
impl ::core::clone::Clone for RESTOREBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESTOREBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCROLLBARPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_ARROWBTN: SCROLLBARPARTS = SCROLLBARPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_THUMBBTNHORZ: SCROLLBARPARTS = SCROLLBARPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_THUMBBTNVERT: SCROLLBARPARTS = SCROLLBARPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_LOWERTRACKHORZ: SCROLLBARPARTS = SCROLLBARPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_UPPERTRACKHORZ: SCROLLBARPARTS = SCROLLBARPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_LOWERTRACKVERT: SCROLLBARPARTS = SCROLLBARPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_UPPERTRACKVERT: SCROLLBARPARTS = SCROLLBARPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_GRIPPERHORZ: SCROLLBARPARTS = SCROLLBARPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_GRIPPERVERT: SCROLLBARPARTS = SCROLLBARPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_SIZEBOX: SCROLLBARPARTS = SCROLLBARPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBP_SIZEBOXBKGND: SCROLLBARPARTS = SCROLLBARPARTS(11i32);
impl ::core::marker::Copy for SCROLLBARPARTS {}
impl ::core::clone::Clone for SCROLLBARPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCROLLBARPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCROLLBARSTYLESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCRBS_NORMAL: SCROLLBARSTYLESTATES = SCROLLBARSTYLESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCRBS_HOT: SCROLLBARSTYLESTATES = SCROLLBARSTYLESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCRBS_PRESSED: SCROLLBARSTYLESTATES = SCROLLBARSTYLESTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCRBS_DISABLED: SCROLLBARSTYLESTATES = SCROLLBARSTYLESTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCRBS_HOVER: SCROLLBARSTYLESTATES = SCROLLBARSTYLESTATES(5i32);
impl ::core::marker::Copy for SCROLLBARSTYLESTATES {}
impl ::core::clone::Clone for SCROLLBARSTYLESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SCROLLBARSTYLESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECTIONTITLELINKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPSTL_NORMAL: SECTIONTITLELINKSTATES = SECTIONTITLELINKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPSTL_HOT: SECTIONTITLELINKSTATES = SECTIONTITLELINKSTATES(2i32);
impl ::core::marker::Copy for SECTIONTITLELINKSTATES {}
impl ::core::clone::Clone for SECTIONTITLELINKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SECTIONTITLELINKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SET_THEME_APP_PROPERTIES_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ALLOW_NONCLIENT: SET_THEME_APP_PROPERTIES_FLAGS = SET_THEME_APP_PROPERTIES_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ALLOW_CONTROLS: SET_THEME_APP_PROPERTIES_FLAGS = SET_THEME_APP_PROPERTIES_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ALLOW_WEBCONTENT: SET_THEME_APP_PROPERTIES_FLAGS = SET_THEME_APP_PROPERTIES_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VALIDBITS: SET_THEME_APP_PROPERTIES_FLAGS = SET_THEME_APP_PROPERTIES_FLAGS(7u32);
impl ::core::marker::Copy for SET_THEME_APP_PROPERTIES_FLAGS {}
impl ::core::clone::Clone for SET_THEME_APP_PROPERTIES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SET_THEME_APP_PROPERTIES_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SHOWCALENDARBUTTONRIGHTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPSCBR_NORMAL: SHOWCALENDARBUTTONRIGHTSTATES = SHOWCALENDARBUTTONRIGHTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPSCBR_HOT: SHOWCALENDARBUTTONRIGHTSTATES = SHOWCALENDARBUTTONRIGHTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPSCBR_PRESSED: SHOWCALENDARBUTTONRIGHTSTATES = SHOWCALENDARBUTTONRIGHTSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DPSCBR_DISABLED: SHOWCALENDARBUTTONRIGHTSTATES = SHOWCALENDARBUTTONRIGHTSTATES(4i32);
impl ::core::marker::Copy for SHOWCALENDARBUTTONRIGHTSTATES {}
impl ::core::clone::Clone for SHOWCALENDARBUTTONRIGHTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SHOWCALENDARBUTTONRIGHTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SIZEBOXSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZB_RIGHTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZB_LEFTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZB_TOPRIGHTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZB_TOPLEFTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZB_HALFBOTTOMRIGHTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZB_HALFBOTTOMLEFTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZB_HALFTOPRIGHTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SZB_HALFTOPLEFTALIGN: SIZEBOXSTATES = SIZEBOXSTATES(8i32);
impl ::core::marker::Copy for SIZEBOXSTATES {}
impl ::core::clone::Clone for SIZEBOXSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SIZEBOXSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SIZINGTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ST_TRUESIZE: SIZINGTYPE = SIZINGTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ST_STRETCH: SIZINGTYPE = SIZINGTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const ST_TILE: SIZINGTYPE = SIZINGTYPE(2i32);
impl ::core::marker::Copy for SIZINGTYPE {}
impl ::core::clone::Clone for SIZINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SIZINGTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SMALLCAPTIONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCS_ACTIVE: SMALLCAPTIONSTATES = SMALLCAPTIONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCS_INACTIVE: SMALLCAPTIONSTATES = SMALLCAPTIONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCS_DISABLED: SMALLCAPTIONSTATES = SMALLCAPTIONSTATES(3i32);
impl ::core::marker::Copy for SMALLCAPTIONSTATES {}
impl ::core::clone::Clone for SMALLCAPTIONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SMALLCAPTIONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SMALLCLOSEBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCBS_NORMAL: SMALLCLOSEBUTTONSTATES = SMALLCLOSEBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCBS_HOT: SMALLCLOSEBUTTONSTATES = SMALLCLOSEBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCBS_PUSHED: SMALLCLOSEBUTTONSTATES = SMALLCLOSEBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SCBS_DISABLED: SMALLCLOSEBUTTONSTATES = SMALLCLOSEBUTTONSTATES(4i32);
impl ::core::marker::Copy for SMALLCLOSEBUTTONSTATES {}
impl ::core::clone::Clone for SMALLCLOSEBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SMALLCLOSEBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SMALLFRAMEBOTTOMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SFRB_ACTIVE: SMALLFRAMEBOTTOMSTATES = SMALLFRAMEBOTTOMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SFRB_INACTIVE: SMALLFRAMEBOTTOMSTATES = SMALLFRAMEBOTTOMSTATES(2i32);
impl ::core::marker::Copy for SMALLFRAMEBOTTOMSTATES {}
impl ::core::clone::Clone for SMALLFRAMEBOTTOMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SMALLFRAMEBOTTOMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SMALLFRAMELEFTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SFRL_ACTIVE: SMALLFRAMELEFTSTATES = SMALLFRAMELEFTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SFRL_INACTIVE: SMALLFRAMELEFTSTATES = SMALLFRAMELEFTSTATES(2i32);
impl ::core::marker::Copy for SMALLFRAMELEFTSTATES {}
impl ::core::clone::Clone for SMALLFRAMELEFTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SMALLFRAMELEFTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SMALLFRAMERIGHTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SFRR_ACTIVE: SMALLFRAMERIGHTSTATES = SMALLFRAMERIGHTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SFRR_INACTIVE: SMALLFRAMERIGHTSTATES = SMALLFRAMERIGHTSTATES(2i32);
impl ::core::marker::Copy for SMALLFRAMERIGHTSTATES {}
impl ::core::clone::Clone for SMALLFRAMERIGHTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SMALLFRAMERIGHTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SOFTWAREEXPLORERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPSE_NORMAL: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPSE_HOT: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPSE_SELECTED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPSE_DISABLED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPSE_FOCUSED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(5i32);
impl ::core::marker::Copy for SOFTWAREEXPLORERSTATES {}
impl ::core::clone::Clone for SOFTWAREEXPLORERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SOFTWAREEXPLORERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPECIALGROUPCOLLAPSESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBSGC_NORMAL: SPECIALGROUPCOLLAPSESTATES = SPECIALGROUPCOLLAPSESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBSGC_HOT: SPECIALGROUPCOLLAPSESTATES = SPECIALGROUPCOLLAPSESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBSGC_PRESSED: SPECIALGROUPCOLLAPSESTATES = SPECIALGROUPCOLLAPSESTATES(3i32);
impl ::core::marker::Copy for SPECIALGROUPCOLLAPSESTATES {}
impl ::core::clone::Clone for SPECIALGROUPCOLLAPSESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SPECIALGROUPCOLLAPSESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPECIALGROUPEXPANDSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBSGE_NORMAL: SPECIALGROUPEXPANDSTATES = SPECIALGROUPEXPANDSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBSGE_HOT: SPECIALGROUPEXPANDSTATES = SPECIALGROUPEXPANDSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const EBSGE_PRESSED: SPECIALGROUPEXPANDSTATES = SPECIALGROUPEXPANDSTATES(3i32);
impl ::core::marker::Copy for SPECIALGROUPEXPANDSTATES {}
impl ::core::clone::Clone for SPECIALGROUPEXPANDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SPECIALGROUPEXPANDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPINPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPNP_UP: SPINPARTS = SPINPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPNP_DOWN: SPINPARTS = SPINPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPNP_UPHORZ: SPINPARTS = SPINPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPNP_DOWNHORZ: SPINPARTS = SPINPARTS(4i32);
impl ::core::marker::Copy for SPINPARTS {}
impl ::core::clone::Clone for SPINPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SPINPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPLITTERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPLITS_NORMAL: SPLITTERSTATES = SPLITTERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPLITS_HOT: SPLITTERSTATES = SPLITTERSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPLITS_PRESSED: SPLITTERSTATES = SPLITTERSTATES(3i32);
impl ::core::marker::Copy for SPLITTERSTATES {}
impl ::core::clone::Clone for SPLITTERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SPLITTERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPLITTERVERTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPLITSV_NORMAL: SPLITTERVERTSTATES = SPLITTERVERTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPLITSV_HOT: SPLITTERVERTSTATES = SPLITTERVERTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPLITSV_PRESSED: SPLITTERVERTSTATES = SPLITTERVERTSTATES(3i32);
impl ::core::marker::Copy for SPLITTERVERTSTATES {}
impl ::core::clone::Clone for SPLITTERVERTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SPLITTERVERTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STANDARDSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTSS_NORMAL: STANDARDSTATES = STANDARDSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTSS_LINK: STANDARDSTATES = STANDARDSTATES(2i32);
impl ::core::marker::Copy for STANDARDSTATES {}
impl ::core::clone::Clone for STANDARDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for STANDARDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STARTPANELPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_USERPANE: STARTPANELPARTS = STARTPANELPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_MOREPROGRAMS: STARTPANELPARTS = STARTPANELPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_MOREPROGRAMSARROW: STARTPANELPARTS = STARTPANELPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_PROGLIST: STARTPANELPARTS = STARTPANELPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_PROGLISTSEPARATOR: STARTPANELPARTS = STARTPANELPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_PLACESLIST: STARTPANELPARTS = STARTPANELPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_PLACESLISTSEPARATOR: STARTPANELPARTS = STARTPANELPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_LOGOFF: STARTPANELPARTS = STARTPANELPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_LOGOFFBUTTONS: STARTPANELPARTS = STARTPANELPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_USERPICTURE: STARTPANELPARTS = STARTPANELPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_PREVIEW: STARTPANELPARTS = STARTPANELPARTS(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_MOREPROGRAMSTAB: STARTPANELPARTS = STARTPANELPARTS(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_NSCHOST: STARTPANELPARTS = STARTPANELPARTS(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_SOFTWAREEXPLORER: STARTPANELPARTS = STARTPANELPARTS(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_OPENBOX: STARTPANELPARTS = STARTPANELPARTS(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_SEARCHVIEW: STARTPANELPARTS = STARTPANELPARTS(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_MOREPROGRAMSARROWBACK: STARTPANELPARTS = STARTPANELPARTS(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_TOPMATCH: STARTPANELPARTS = STARTPANELPARTS(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SPP_LOGOFFSPLITBUTTONDROPDOWN: STARTPANELPARTS = STARTPANELPARTS(19i32);
impl ::core::marker::Copy for STARTPANELPARTS {}
impl ::core::clone::Clone for STARTPANELPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for STARTPANELPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STATICPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const STAT_TEXT: STATICPARTS = STATICPARTS(1i32);
impl ::core::marker::Copy for STATICPARTS {}
impl ::core::clone::Clone for STATICPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for STATICPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STATUSPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SP_PANE: STATUSPARTS = STATUSPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SP_GRIPPERPANE: STATUSPARTS = STATUSPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SP_GRIPPER: STATUSPARTS = STATUSPARTS(3i32);
impl ::core::marker::Copy for STATUSPARTS {}
impl ::core::clone::Clone for STATUSPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for STATUSPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSBUTTONSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBS_NORMAL: SYSBUTTONSTATES = SYSBUTTONSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBS_HOT: SYSBUTTONSTATES = SYSBUTTONSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBS_PUSHED: SYSBUTTONSTATES = SYSBUTTONSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const SBS_DISABLED: SYSBUTTONSTATES = SYSBUTTONSTATES(4i32);
impl ::core::marker::Copy for SYSBUTTONSTATES {}
impl ::core::clone::Clone for SYSBUTTONSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SYSBUTTONSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEMCLOSESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSYSC_NORMAL: SYSTEMCLOSESTATES = SYSTEMCLOSESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSYSC_DISABLED: SYSTEMCLOSESTATES = SYSTEMCLOSESTATES(2i32);
impl ::core::marker::Copy for SYSTEMCLOSESTATES {}
impl ::core::clone::Clone for SYSTEMCLOSESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SYSTEMCLOSESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEMMAXIMIZESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSYSMX_NORMAL: SYSTEMMAXIMIZESTATES = SYSTEMMAXIMIZESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSYSMX_DISABLED: SYSTEMMAXIMIZESTATES = SYSTEMMAXIMIZESTATES(2i32);
impl ::core::marker::Copy for SYSTEMMAXIMIZESTATES {}
impl ::core::clone::Clone for SYSTEMMAXIMIZESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SYSTEMMAXIMIZESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEMMINIMIZESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSYSMN_NORMAL: SYSTEMMINIMIZESTATES = SYSTEMMINIMIZESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSYSMN_DISABLED: SYSTEMMINIMIZESTATES = SYSTEMMINIMIZESTATES(2i32);
impl ::core::marker::Copy for SYSTEMMINIMIZESTATES {}
impl ::core::clone::Clone for SYSTEMMINIMIZESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SYSTEMMINIMIZESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEMRESTORESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSYSR_NORMAL: SYSTEMRESTORESTATES = SYSTEMRESTORESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MSYSR_DISABLED: SYSTEMRESTORESTATES = SYSTEMRESTORESTATES(2i32);
impl ::core::marker::Copy for SYSTEMRESTORESTATES {}
impl ::core::clone::Clone for SYSTEMRESTORESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SYSTEMRESTORESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TABITEMBOTHEDGESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIBES_NORMAL: TABITEMBOTHEDGESTATES = TABITEMBOTHEDGESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIBES_HOT: TABITEMBOTHEDGESTATES = TABITEMBOTHEDGESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIBES_SELECTED: TABITEMBOTHEDGESTATES = TABITEMBOTHEDGESTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIBES_DISABLED: TABITEMBOTHEDGESTATES = TABITEMBOTHEDGESTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIBES_FOCUSED: TABITEMBOTHEDGESTATES = TABITEMBOTHEDGESTATES(5i32);
impl ::core::marker::Copy for TABITEMBOTHEDGESTATES {}
impl ::core::clone::Clone for TABITEMBOTHEDGESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TABITEMBOTHEDGESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TABITEMLEFTEDGESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TILES_NORMAL: TABITEMLEFTEDGESTATES = TABITEMLEFTEDGESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TILES_HOT: TABITEMLEFTEDGESTATES = TABITEMLEFTEDGESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TILES_SELECTED: TABITEMLEFTEDGESTATES = TABITEMLEFTEDGESTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TILES_DISABLED: TABITEMLEFTEDGESTATES = TABITEMLEFTEDGESTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TILES_FOCUSED: TABITEMLEFTEDGESTATES = TABITEMLEFTEDGESTATES(5i32);
impl ::core::marker::Copy for TABITEMLEFTEDGESTATES {}
impl ::core::clone::Clone for TABITEMLEFTEDGESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TABITEMLEFTEDGESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TABITEMRIGHTEDGESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIRES_NORMAL: TABITEMRIGHTEDGESTATES = TABITEMRIGHTEDGESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIRES_HOT: TABITEMRIGHTEDGESTATES = TABITEMRIGHTEDGESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIRES_SELECTED: TABITEMRIGHTEDGESTATES = TABITEMRIGHTEDGESTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIRES_DISABLED: TABITEMRIGHTEDGESTATES = TABITEMRIGHTEDGESTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIRES_FOCUSED: TABITEMRIGHTEDGESTATES = TABITEMRIGHTEDGESTATES(5i32);
impl ::core::marker::Copy for TABITEMRIGHTEDGESTATES {}
impl ::core::clone::Clone for TABITEMRIGHTEDGESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TABITEMRIGHTEDGESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TABITEMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIS_NORMAL: TABITEMSTATES = TABITEMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIS_HOT: TABITEMSTATES = TABITEMSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIS_SELECTED: TABITEMSTATES = TABITEMSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIS_DISABLED: TABITEMSTATES = TABITEMSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TIS_FOCUSED: TABITEMSTATES = TABITEMSTATES(5i32);
impl ::core::marker::Copy for TABITEMSTATES {}
impl ::core::clone::Clone for TABITEMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TABITEMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TABPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_TABITEM: TABPARTS = TABPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_TABITEMLEFTEDGE: TABPARTS = TABPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_TABITEMRIGHTEDGE: TABPARTS = TABPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_TABITEMBOTHEDGE: TABPARTS = TABPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_TOPTABITEM: TABPARTS = TABPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_TOPTABITEMLEFTEDGE: TABPARTS = TABPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_TOPTABITEMRIGHTEDGE: TABPARTS = TABPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_TOPTABITEMBOTHEDGE: TABPARTS = TABPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_PANE: TABPARTS = TABPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_BODY: TABPARTS = TABPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TABP_AEROWIZARDBODY: TABPARTS = TABPARTS(11i32);
impl ::core::marker::Copy for TABPARTS {}
impl ::core::clone::Clone for TABPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TABPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TABSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CSTB_NORMAL: TABSTATES = TABSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CSTB_HOT: TABSTATES = TABSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CSTB_SELECTED: TABSTATES = TABSTATES(3i32);
impl ::core::marker::Copy for TABSTATES {}
impl ::core::clone::Clone for TABSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TABSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TAB_CONTROL_ITEM_STATE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCIS_BUTTONPRESSED: TAB_CONTROL_ITEM_STATE = TAB_CONTROL_ITEM_STATE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCIS_HIGHLIGHTED: TAB_CONTROL_ITEM_STATE = TAB_CONTROL_ITEM_STATE(2u32);
impl ::core::marker::Copy for TAB_CONTROL_ITEM_STATE {}
impl ::core::clone::Clone for TAB_CONTROL_ITEM_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TAB_CONTROL_ITEM_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKBANDPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDP_GROUPCOUNT: TASKBANDPARTS = TASKBANDPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDP_FLASHBUTTON: TASKBANDPARTS = TASKBANDPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDP_FLASHBUTTONGROUPMENU: TASKBANDPARTS = TASKBANDPARTS(3i32);
impl ::core::marker::Copy for TASKBANDPARTS {}
impl ::core::clone::Clone for TASKBANDPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKBANDPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKBARPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBP_BACKGROUNDBOTTOM: TASKBARPARTS = TASKBARPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBP_BACKGROUNDRIGHT: TASKBARPARTS = TASKBARPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBP_BACKGROUNDTOP: TASKBARPARTS = TASKBARPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBP_BACKGROUNDLEFT: TASKBARPARTS = TASKBARPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBP_SIZINGBARBOTTOM: TASKBARPARTS = TASKBARPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBP_SIZINGBARRIGHT: TASKBARPARTS = TASKBARPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBP_SIZINGBARTOP: TASKBARPARTS = TASKBARPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBP_SIZINGBARLEFT: TASKBARPARTS = TASKBARPARTS(8i32);
impl ::core::marker::Copy for TASKBARPARTS {}
impl ::core::clone::Clone for TASKBARPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKBARPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKDIALOGPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_PRIMARYPANEL: TASKDIALOGPARTS = TASKDIALOGPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_MAININSTRUCTIONPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_MAINICON: TASKDIALOGPARTS = TASKDIALOGPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_CONTENTPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_CONTENTICON: TASKDIALOGPARTS = TASKDIALOGPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_EXPANDEDCONTENT: TASKDIALOGPARTS = TASKDIALOGPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_COMMANDLINKPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_SECONDARYPANEL: TASKDIALOGPARTS = TASKDIALOGPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_CONTROLPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_BUTTONSECTION: TASKDIALOGPARTS = TASKDIALOGPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_BUTTONWRAPPER: TASKDIALOGPARTS = TASKDIALOGPARTS(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_EXPANDOTEXT: TASKDIALOGPARTS = TASKDIALOGPARTS(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_EXPANDOBUTTON: TASKDIALOGPARTS = TASKDIALOGPARTS(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_VERIFICATIONTEXT: TASKDIALOGPARTS = TASKDIALOGPARTS(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_FOOTNOTEPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_FOOTNOTEAREA: TASKDIALOGPARTS = TASKDIALOGPARTS(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_FOOTNOTESEPARATOR: TASKDIALOGPARTS = TASKDIALOGPARTS(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_EXPANDEDFOOTERAREA: TASKDIALOGPARTS = TASKDIALOGPARTS(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_PROGRESSBAR: TASKDIALOGPARTS = TASKDIALOGPARTS(19i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_IMAGEALIGNMENT: TASKDIALOGPARTS = TASKDIALOGPARTS(20i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDLG_RADIOBUTTONPANE: TASKDIALOGPARTS = TASKDIALOGPARTS(21i32);
impl ::core::marker::Copy for TASKDIALOGPARTS {}
impl ::core::clone::Clone for TASKDIALOGPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKDIALOGPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKDIALOG_COMMON_BUTTON_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDCBF_OK_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDCBF_YES_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDCBF_NO_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDCBF_CANCEL_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDCBF_RETRY_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDCBF_CLOSE_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(32i32);
impl ::core::marker::Copy for TASKDIALOG_COMMON_BUTTON_FLAGS {}
impl ::core::clone::Clone for TASKDIALOG_COMMON_BUTTON_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKDIALOG_COMMON_BUTTON_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKDIALOG_ELEMENTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDE_CONTENT: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDE_EXPANDED_INFORMATION: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDE_FOOTER: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDE_MAIN_INSTRUCTION: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(3i32);
impl ::core::marker::Copy for TASKDIALOG_ELEMENTS {}
impl ::core::clone::Clone for TASKDIALOG_ELEMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKDIALOG_ELEMENTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKDIALOG_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_ENABLE_HYPERLINKS: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_USE_HICON_MAIN: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_USE_HICON_FOOTER: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_ALLOW_DIALOG_CANCELLATION: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_USE_COMMAND_LINKS: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_USE_COMMAND_LINKS_NO_ICON: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_EXPAND_FOOTER_AREA: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_EXPANDED_BY_DEFAULT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_VERIFICATION_FLAG_CHECKED: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_SHOW_PROGRESS_BAR: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_SHOW_MARQUEE_PROGRESS_BAR: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(1024i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_CALLBACK_TIMER: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_POSITION_RELATIVE_TO_WINDOW: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_RTL_LAYOUT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(8192i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_NO_DEFAULT_RADIO_BUTTON: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16384i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_CAN_BE_MINIMIZED: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(32768i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_NO_SET_FOREGROUND: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(65536i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDF_SIZE_TO_CONTENT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16777216i32);
impl ::core::marker::Copy for TASKDIALOG_FLAGS {}
impl ::core::clone::Clone for TASKDIALOG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKDIALOG_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKDIALOG_ICON_ELEMENTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDIE_ICON_MAIN: TASKDIALOG_ICON_ELEMENTS = TASKDIALOG_ICON_ELEMENTS(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDIE_ICON_FOOTER: TASKDIALOG_ICON_ELEMENTS = TASKDIALOG_ICON_ELEMENTS(1i32);
impl ::core::marker::Copy for TASKDIALOG_ICON_ELEMENTS {}
impl ::core::clone::Clone for TASKDIALOG_ICON_ELEMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKDIALOG_ICON_ELEMENTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKDIALOG_MESSAGES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_NAVIGATE_PAGE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1125i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_CLICK_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1126i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_SET_MARQUEE_PROGRESS_BAR: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1127i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_SET_PROGRESS_BAR_STATE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1128i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_SET_PROGRESS_BAR_RANGE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1129i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_SET_PROGRESS_BAR_POS: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1130i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_SET_PROGRESS_BAR_MARQUEE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1131i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_SET_ELEMENT_TEXT: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1132i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_CLICK_RADIO_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1134i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_ENABLE_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1135i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_ENABLE_RADIO_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1136i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_CLICK_VERIFICATION: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1137i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_UPDATE_ELEMENT_TEXT: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1138i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1139i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDM_UPDATE_ICON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1140i32);
impl ::core::marker::Copy for TASKDIALOG_MESSAGES {}
impl ::core::clone::Clone for TASKDIALOG_MESSAGES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKDIALOG_MESSAGES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKDIALOG_NOTIFICATIONS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_CREATED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_NAVIGATED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_HYPERLINK_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_TIMER: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_DESTROYED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_RADIO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_DIALOG_CONSTRUCTED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_VERIFICATION_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_HELP: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TDN_EXPANDO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(10i32);
impl ::core::marker::Copy for TASKDIALOG_NOTIFICATIONS {}
impl ::core::clone::Clone for TASKDIALOG_NOTIFICATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKDIALOG_NOTIFICATIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKLINKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPTL_NORMAL: TASKLINKSTATES = TASKLINKSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPTL_HOT: TASKLINKSTATES = TASKLINKSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPTL_PRESSED: TASKLINKSTATES = TASKLINKSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPTL_DISABLED: TASKLINKSTATES = TASKLINKSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CPTL_PAGE: TASKLINKSTATES = TASKLINKSTATES(5i32);
impl ::core::marker::Copy for TASKLINKSTATES {}
impl ::core::clone::Clone for TASKLINKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKLINKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TA_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAP_FLAGS: TA_PROPERTY = TA_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAP_TRANSFORMCOUNT: TA_PROPERTY = TA_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAP_STAGGERDELAY: TA_PROPERTY = TA_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAP_STAGGERDELAYCAP: TA_PROPERTY = TA_PROPERTY(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAP_STAGGERDELAYFACTOR: TA_PROPERTY = TA_PROPERTY(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAP_ZORDER: TA_PROPERTY = TA_PROPERTY(5i32);
impl ::core::marker::Copy for TA_PROPERTY {}
impl ::core::clone::Clone for TA_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_PROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TA_PROPERTY_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAPF_NONE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAPF_HASSTAGGER: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAPF_ISRTLAWARE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAPF_ALLOWCOLLECTION: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAPF_HASBACKGROUND: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TAPF_HASPERSPECTIVE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(16u32);
impl ::core::marker::Copy for TA_PROPERTY_FLAG {}
impl ::core::clone::Clone for TA_PROPERTY_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_PROPERTY_FLAG {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TA_TIMINGFUNCTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTFT_UNDEFINED: TA_TIMINGFUNCTION_TYPE = TA_TIMINGFUNCTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTFT_CUBIC_BEZIER: TA_TIMINGFUNCTION_TYPE = TA_TIMINGFUNCTION_TYPE(1i32);
impl ::core::marker::Copy for TA_TIMINGFUNCTION_TYPE {}
impl ::core::clone::Clone for TA_TIMINGFUNCTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_TIMINGFUNCTION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TA_TRANSFORM_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TATF_NONE: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TATF_TARGETVALUES_USER: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TATF_HASINITIALVALUES: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TATF_HASORIGINVALUES: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(4i32);
impl ::core::marker::Copy for TA_TRANSFORM_FLAG {}
impl ::core::clone::Clone for TA_TRANSFORM_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_TRANSFORM_FLAG {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TA_TRANSFORM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TATT_TRANSLATE_2D: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TATT_SCALE_2D: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TATT_OPACITY: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TATT_CLIP: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(3i32);
impl ::core::marker::Copy for TA_TRANSFORM_TYPE {}
impl ::core::clone::Clone for TA_TRANSFORM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_TRANSFORM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TBBUTTONINFOW_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIF_BYINDEX: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIF_COMMAND: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIF_IMAGE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIF_LPARAM: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIF_SIZE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIF_STATE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIF_STYLE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIF_TEXT: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(2u32);
impl ::core::marker::Copy for TBBUTTONINFOW_MASK {}
impl ::core::clone::Clone for TBBUTTONINFOW_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TBBUTTONINFOW_MASK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TBINSERTMARK_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIMHT_NONE: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIMHT_AFTER: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TBIMHT_BACKGROUND: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(2u32);
impl ::core::marker::Copy for TBINSERTMARK_FLAGS {}
impl ::core::clone::Clone for TBINSERTMARK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TBINSERTMARK_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TCHITTESTINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCHT_NOWHERE: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCHT_ONITEM: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(6u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCHT_ONITEMICON: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCHT_ONITEMLABEL: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(4u32);
impl ::core::marker::Copy for TCHITTESTINFO_FLAGS {}
impl ::core::clone::Clone for TCHITTESTINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TCHITTESTINFO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TCITEMHEADERA_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCIF_IMAGE: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCIF_RTLREADING: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCIF_TEXT: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCIF_PARAM: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TCIF_STATE: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(16u32);
impl ::core::marker::Copy for TCITEMHEADERA_MASK {}
impl ::core::clone::Clone for TCITEMHEADERA_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TCITEMHEADERA_MASK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TEXTSELECTIONGRIPPERPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TSGP_GRIPPER: TEXTSELECTIONGRIPPERPARTS = TEXTSELECTIONGRIPPERPARTS(1i32);
impl ::core::marker::Copy for TEXTSELECTIONGRIPPERPARTS {}
impl ::core::clone::Clone for TEXTSELECTIONGRIPPERPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TEXTSELECTIONGRIPPERPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TEXTSHADOWTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TST_NONE: TEXTSHADOWTYPE = TEXTSHADOWTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TST_SINGLE: TEXTSHADOWTYPE = TEXTSHADOWTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TST_CONTINUOUS: TEXTSHADOWTYPE = TEXTSHADOWTYPE(2i32);
impl ::core::marker::Copy for TEXTSHADOWTYPE {}
impl ::core::clone::Clone for TEXTSHADOWTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TEXTSHADOWTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TEXTSTYLEPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TEXT_MAININSTRUCTION: TEXTSTYLEPARTS = TEXTSTYLEPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TEXT_INSTRUCTION: TEXTSTYLEPARTS = TEXTSTYLEPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TEXT_BODYTITLE: TEXTSTYLEPARTS = TEXTSTYLEPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TEXT_BODYTEXT: TEXTSTYLEPARTS = TEXTSTYLEPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TEXT_SECONDARYTEXT: TEXTSTYLEPARTS = TEXTSTYLEPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TEXT_HYPERLINKTEXT: TEXTSTYLEPARTS = TEXTSTYLEPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TEXT_EXPANDED: TEXTSTYLEPARTS = TEXTSTYLEPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TEXT_LABEL: TEXTSTYLEPARTS = TEXTSTYLEPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TEXT_CONTROLLABEL: TEXTSTYLEPARTS = TEXTSTYLEPARTS(9i32);
impl ::core::marker::Copy for TEXTSTYLEPARTS {}
impl ::core::clone::Clone for TEXTSTYLEPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TEXTSTYLEPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THEMESIZE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_MIN: THEMESIZE = THEMESIZE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_TRUE: THEMESIZE = THEMESIZE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_DRAW: THEMESIZE = THEMESIZE(2i32);
impl ::core::marker::Copy for THEMESIZE {}
impl ::core::clone::Clone for THEMESIZE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for THEMESIZE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THEME_PROPERTY_SYMBOL_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_RESERVEDLOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_RESERVEDHIGH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(7999u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_DIBDATA: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLYPHDIBDATA: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ENUM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(200u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_STRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(201u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_INT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(202u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(203u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_COLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(204u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(205u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FILENAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(206u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(207u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_POSITION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(208u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_RECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(209u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(210u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_INTLIST: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(211u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_HBITMAP: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(212u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_DISKSTREAM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(213u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_STREAM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(214u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BITMAPREF: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(215u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FLOAT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(216u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FLOATLIST: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(217u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_COLORSCHEMES: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(401u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SIZES: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(402u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CHARSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(403u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(600u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_DISPLAYNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(601u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOOLTIP: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(602u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_COMPANY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(603u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_AUTHOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(604u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_COPYRIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(605u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_URL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(606u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_VERSION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(607u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_DESCRIPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(608u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FIRST_RCSTRING_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(601u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_LAST_RCSTRING_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(608u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CAPTIONFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(801u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SMALLCAPTIONFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(802u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MENUFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(803u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_STATUSFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(804u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MSGBOXFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(805u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ICONTITLEFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(806u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_HEADING1FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(807u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_HEADING2FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(808u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BODYFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(809u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FIRSTFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(801u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_LASTFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(809u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FLATMENUS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FIRSTBOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_LASTBOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SIZINGBORDERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1201u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SCROLLBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1202u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SCROLLBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1203u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CAPTIONBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1204u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CAPTIONBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1205u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SMCAPTIONBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1206u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SMCAPTIONBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1207u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MENUBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1208u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MENUBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1209u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_PADDEDBORDERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1210u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FIRSTSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1201u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_LASTSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1210u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINCOLORDEPTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FIRSTINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_LASTINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CSSNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1401u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_XMLNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1402u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_LASTUPDATED: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1403u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ALIAS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1404u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FIRSTSTRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1401u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_LASTSTRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1404u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SCROLLBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1601u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BACKGROUND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1602u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1603u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_INACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1604u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MENU: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1605u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_WINDOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1606u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_WINDOWFRAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1607u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MENUTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1608u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_WINDOWTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1609u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CAPTIONTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1610u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ACTIVEBORDER: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1611u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_INACTIVEBORDER: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1612u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_APPWORKSPACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1613u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_HIGHLIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1614u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_HIGHLIGHTTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1615u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BTNFACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1616u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BTNSHADOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1617u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRAYTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1618u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BTNTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1619u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_INACTIVECAPTIONTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1620u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BTNHIGHLIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1621u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_DKSHADOW3D: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1622u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_LIGHT3D: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1623u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_INFOTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1624u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_INFOBK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1625u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BUTTONALTERNATEFACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1626u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_HOTTRACKING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1627u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1628u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTINACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1629u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MENUHILIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1630u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MENUBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1631u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FIRSTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1601u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_LASTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1631u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FROMHUE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1801u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FROMHUE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1802u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FROMHUE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1803u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FROMHUE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1804u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FROMHUE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1805u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOHUE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1806u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOHUE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1807u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOHUE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1808u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOHUE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1809u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOHUE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1810u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FROMCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2001u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FROMCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2002u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FROMCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2003u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FROMCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2004u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FROMCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2005u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2006u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2007u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2008u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2009u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TOCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2010u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TRANSPARENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2201u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_AUTOSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2202u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BORDERONLY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2203u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_COMPOSITED: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2204u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BGFILL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2205u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLYPHTRANSPARENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2206u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLYPHONLY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2207u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ALWAYSSHOWSIZINGBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2208u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MIRRORIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2209u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_UNIFORMSIZING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2210u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_INTEGRALSIZING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2211u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SOURCEGROW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2212u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SOURCESHRINK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2213u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_DRAWBORDERS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2214u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_NOETCHEDEFFECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2215u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTAPPLYOVERLAY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2216u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTGLOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2217u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTITALIC: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2218u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_COMPOSITEDOPAQUE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2219u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_LOCALIZEDMIRRORIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2220u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGECOUNT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2401u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ALPHALEVEL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2402u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BORDERSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2403u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ROUNDCORNERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2404u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ROUNDCORNERHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2405u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTRATIO1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2406u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTRATIO2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2407u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTRATIO3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2408u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTRATIO4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2409u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTRATIO5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2410u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_PROGRESSCHUNKSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2411u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_PROGRESSSPACESIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2412u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SATURATION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2413u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTBORDERSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2414u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ALPHATHRESHOLD: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2415u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_WIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2416u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_HEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2417u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLYPHINDEX: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2418u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TRUESIZESTRETCHMARK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2419u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINDPI1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2420u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINDPI2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2421u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINDPI3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2422u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINDPI4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2423u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINDPI5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2424u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTGLOWSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2425u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FRAMESPERSECOND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2426u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_PIXELSPERFRAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2427u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ANIMATIONDELAY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2428u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLOWINTENSITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2429u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_OPACITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2430u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_COLORIZATIONCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2431u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_COLORIZATIONOPACITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2432u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINDPI6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2433u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINDPI7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2434u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLYPHFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2601u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGEFILE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3001u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGEFILE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3002u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGEFILE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3003u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGEFILE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3004u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGEFILE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3005u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGEFILE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3006u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLYPHIMAGEFILE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3008u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGEFILE6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3009u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGEFILE7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3010u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3201u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CLASSICVALUE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3202u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_OFFSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3401u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTSHADOWOFFSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3402u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3403u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINSIZE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3404u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINSIZE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3405u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINSIZE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3406u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINSIZE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3407u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINSIZE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3408u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_NORMALSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3409u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINSIZE6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3410u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_MINSIZE7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3411u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SIZINGMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3601u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CONTENTMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3602u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CAPTIONMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3603u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BORDERCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3801u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FILLCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3802u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3803u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_EDGELIGHTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3804u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_EDGEHIGHLIGHTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3805u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_EDGESHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3806u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_EDGEDKSHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3807u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_EDGEFILLCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3808u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TRANSPARENTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3809u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3810u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3811u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3812u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3813u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GRADIENTCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3814u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3815u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3816u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTBORDERCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3817u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTSHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3818u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLYPHTEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3819u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLYPHTRANSPARENTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3820u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FILLCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3821u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BORDERCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3822u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ACCENTCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3823u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3824u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_HEADING1TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3825u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_HEADING2TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3826u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BODYTEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3827u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4001u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BORDERTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4002u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_FILLTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4003u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SIZINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4004u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_HALIGN: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4005u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CONTENTALIGNMENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4006u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_VALIGN: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4007u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_OFFSETTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4008u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ICONEFFECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4009u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TEXTSHADOWTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4010u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGELAYOUT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4011u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLYPHTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4012u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_IMAGESELECTTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4013u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_GLYPHFONTSIZINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4014u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TRUESIZESCALINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4015u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_USERPICTURE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5001u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_DEFAULTPANESIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5002u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_BLENDCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5003u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_CUSTOMSPLITRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5004u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ANIMATIONBUTTONRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5005u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ANIMATIONDURATION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5006u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_TRANSITIONDURATIONS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(6000u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_SCALEDBACKGROUND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(7001u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ATLASIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8000u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ATLASINPUTIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8001u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TMT_ATLASRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8002u32);
impl ::core::marker::Copy for THEME_PROPERTY_SYMBOL_ID {}
impl ::core::clone::Clone for THEME_PROPERTY_SYMBOL_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for THEME_PROPERTY_SYMBOL_ID {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THUMBBOTTOMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUBS_NORMAL: THUMBBOTTOMSTATES = THUMBBOTTOMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUBS_HOT: THUMBBOTTOMSTATES = THUMBBOTTOMSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUBS_PRESSED: THUMBBOTTOMSTATES = THUMBBOTTOMSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUBS_FOCUSED: THUMBBOTTOMSTATES = THUMBBOTTOMSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUBS_DISABLED: THUMBBOTTOMSTATES = THUMBBOTTOMSTATES(5i32);
impl ::core::marker::Copy for THUMBBOTTOMSTATES {}
impl ::core::clone::Clone for THUMBBOTTOMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for THUMBBOTTOMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THUMBLEFTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVLS_NORMAL: THUMBLEFTSTATES = THUMBLEFTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVLS_HOT: THUMBLEFTSTATES = THUMBLEFTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVLS_PRESSED: THUMBLEFTSTATES = THUMBLEFTSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVLS_FOCUSED: THUMBLEFTSTATES = THUMBLEFTSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVLS_DISABLED: THUMBLEFTSTATES = THUMBLEFTSTATES(5i32);
impl ::core::marker::Copy for THUMBLEFTSTATES {}
impl ::core::clone::Clone for THUMBLEFTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for THUMBLEFTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THUMBRIGHTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVRS_NORMAL: THUMBRIGHTSTATES = THUMBRIGHTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVRS_HOT: THUMBRIGHTSTATES = THUMBRIGHTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVRS_PRESSED: THUMBRIGHTSTATES = THUMBRIGHTSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVRS_FOCUSED: THUMBRIGHTSTATES = THUMBRIGHTSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVRS_DISABLED: THUMBRIGHTSTATES = THUMBRIGHTSTATES(5i32);
impl ::core::marker::Copy for THUMBRIGHTSTATES {}
impl ::core::clone::Clone for THUMBRIGHTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for THUMBRIGHTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THUMBSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUS_NORMAL: THUMBSTATES = THUMBSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUS_HOT: THUMBSTATES = THUMBSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUS_PRESSED: THUMBSTATES = THUMBSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUS_FOCUSED: THUMBSTATES = THUMBSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUS_DISABLED: THUMBSTATES = THUMBSTATES(5i32);
impl ::core::marker::Copy for THUMBSTATES {}
impl ::core::clone::Clone for THUMBSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for THUMBSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THUMBTOPSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUTS_NORMAL: THUMBTOPSTATES = THUMBTOPSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUTS_HOT: THUMBTOPSTATES = THUMBTOPSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUTS_PRESSED: THUMBTOPSTATES = THUMBTOPSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUTS_FOCUSED: THUMBTOPSTATES = THUMBTOPSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUTS_DISABLED: THUMBTOPSTATES = THUMBTOPSTATES(5i32);
impl ::core::marker::Copy for THUMBTOPSTATES {}
impl ::core::clone::Clone for THUMBTOPSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for THUMBTOPSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THUMBVERTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVS_NORMAL: THUMBVERTSTATES = THUMBVERTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVS_HOT: THUMBVERTSTATES = THUMBVERTSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVS_PRESSED: THUMBVERTSTATES = THUMBVERTSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVS_FOCUSED: THUMBVERTSTATES = THUMBVERTSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TUVS_DISABLED: THUMBVERTSTATES = THUMBVERTSTATES(5i32);
impl ::core::marker::Copy for THUMBVERTSTATES {}
impl ::core::clone::Clone for THUMBVERTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for THUMBVERTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TICSSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TSS_NORMAL: TICSSTATES = TICSSTATES(1i32);
impl ::core::marker::Copy for TICSSTATES {}
impl ::core::clone::Clone for TICSSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TICSSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TICSVERTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TSVS_NORMAL: TICSVERTSTATES = TICSVERTSTATES(1i32);
impl ::core::marker::Copy for TICSVERTSTATES {}
impl ::core::clone::Clone for TICSVERTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TICSVERTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TITLEBARSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const AW_S_TITLEBAR_ACTIVE: TITLEBARSTATES = TITLEBARSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const AW_S_TITLEBAR_INACTIVE: TITLEBARSTATES = TITLEBARSTATES(2i32);
impl ::core::marker::Copy for TITLEBARSTATES {}
impl ::core::clone::Clone for TITLEBARSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TITLEBARSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOOLBARPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TP_BUTTON: TOOLBARPARTS = TOOLBARPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TP_DROPDOWNBUTTON: TOOLBARPARTS = TOOLBARPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TP_SPLITBUTTON: TOOLBARPARTS = TOOLBARPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TP_SPLITBUTTONDROPDOWN: TOOLBARPARTS = TOOLBARPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TP_SEPARATOR: TOOLBARPARTS = TOOLBARPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TP_SEPARATORVERT: TOOLBARPARTS = TOOLBARPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TP_DROPDOWNBUTTONGLYPH: TOOLBARPARTS = TOOLBARPARTS(7i32);
impl ::core::marker::Copy for TOOLBARPARTS {}
impl ::core::clone::Clone for TOOLBARPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOOLBARPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOOLBARSTYLESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_NORMAL: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_HOT: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_PRESSED: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_DISABLED: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_CHECKED: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_HOTCHECKED: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_NEARHOT: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TS_OTHERSIDEHOT: TOOLBARSTYLESTATES = TOOLBARSTYLESTATES(8i32);
impl ::core::marker::Copy for TOOLBARSTYLESTATES {}
impl ::core::clone::Clone for TOOLBARSTYLESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOOLBARSTYLESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOOLTIPPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTP_STANDARD: TOOLTIPPARTS = TOOLTIPPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTP_STANDARDTITLE: TOOLTIPPARTS = TOOLTIPPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTP_BALLOON: TOOLTIPPARTS = TOOLTIPPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTP_BALLOONTITLE: TOOLTIPPARTS = TOOLTIPPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTP_CLOSE: TOOLTIPPARTS = TOOLTIPPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTP_BALLOONSTEM: TOOLTIPPARTS = TOOLTIPPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTP_WRENCH: TOOLTIPPARTS = TOOLTIPPARTS(7i32);
impl ::core::marker::Copy for TOOLTIPPARTS {}
impl ::core::clone::Clone for TOOLTIPPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOOLTIPPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOOLTIP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTF_IDISHWND: TOOLTIP_FLAGS = TOOLTIP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTF_CENTERTIP: TOOLTIP_FLAGS = TOOLTIP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTF_RTLREADING: TOOLTIP_FLAGS = TOOLTIP_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTF_SUBCLASS: TOOLTIP_FLAGS = TOOLTIP_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTF_TRACK: TOOLTIP_FLAGS = TOOLTIP_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTF_ABSOLUTE: TOOLTIP_FLAGS = TOOLTIP_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTF_TRANSPARENT: TOOLTIP_FLAGS = TOOLTIP_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTF_PARSELINKS: TOOLTIP_FLAGS = TOOLTIP_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTF_DI_SETITEM: TOOLTIP_FLAGS = TOOLTIP_FLAGS(32768u32);
impl ::core::marker::Copy for TOOLTIP_FLAGS {}
impl ::core::clone::Clone for TOOLTIP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOOLTIP_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOPTABITEMBOTHEDGESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIBES_NORMAL: TOPTABITEMBOTHEDGESTATES = TOPTABITEMBOTHEDGESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIBES_HOT: TOPTABITEMBOTHEDGESTATES = TOPTABITEMBOTHEDGESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIBES_SELECTED: TOPTABITEMBOTHEDGESTATES = TOPTABITEMBOTHEDGESTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIBES_DISABLED: TOPTABITEMBOTHEDGESTATES = TOPTABITEMBOTHEDGESTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIBES_FOCUSED: TOPTABITEMBOTHEDGESTATES = TOPTABITEMBOTHEDGESTATES(5i32);
impl ::core::marker::Copy for TOPTABITEMBOTHEDGESTATES {}
impl ::core::clone::Clone for TOPTABITEMBOTHEDGESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOPTABITEMBOTHEDGESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOPTABITEMLEFTEDGESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTILES_NORMAL: TOPTABITEMLEFTEDGESTATES = TOPTABITEMLEFTEDGESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTILES_HOT: TOPTABITEMLEFTEDGESTATES = TOPTABITEMLEFTEDGESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTILES_SELECTED: TOPTABITEMLEFTEDGESTATES = TOPTABITEMLEFTEDGESTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTILES_DISABLED: TOPTABITEMLEFTEDGESTATES = TOPTABITEMLEFTEDGESTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTILES_FOCUSED: TOPTABITEMLEFTEDGESTATES = TOPTABITEMLEFTEDGESTATES(5i32);
impl ::core::marker::Copy for TOPTABITEMLEFTEDGESTATES {}
impl ::core::clone::Clone for TOPTABITEMLEFTEDGESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOPTABITEMLEFTEDGESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOPTABITEMRIGHTEDGESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIRES_NORMAL: TOPTABITEMRIGHTEDGESTATES = TOPTABITEMRIGHTEDGESTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIRES_HOT: TOPTABITEMRIGHTEDGESTATES = TOPTABITEMRIGHTEDGESTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIRES_SELECTED: TOPTABITEMRIGHTEDGESTATES = TOPTABITEMRIGHTEDGESTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIRES_DISABLED: TOPTABITEMRIGHTEDGESTATES = TOPTABITEMRIGHTEDGESTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIRES_FOCUSED: TOPTABITEMRIGHTEDGESTATES = TOPTABITEMRIGHTEDGESTATES(5i32);
impl ::core::marker::Copy for TOPTABITEMRIGHTEDGESTATES {}
impl ::core::clone::Clone for TOPTABITEMRIGHTEDGESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOPTABITEMRIGHTEDGESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOPTABITEMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIS_NORMAL: TOPTABITEMSTATES = TOPTABITEMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIS_HOT: TOPTABITEMSTATES = TOPTABITEMSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIS_SELECTED: TOPTABITEMSTATES = TOPTABITEMSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIS_DISABLED: TOPTABITEMSTATES = TOPTABITEMSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTIS_FOCUSED: TOPTABITEMSTATES = TOPTABITEMSTATES(5i32);
impl ::core::marker::Copy for TOPTABITEMSTATES {}
impl ::core::clone::Clone for TOPTABITEMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TOPTABITEMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRACKBARPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKP_TRACK: TRACKBARPARTS = TRACKBARPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKP_TRACKVERT: TRACKBARPARTS = TRACKBARPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKP_THUMB: TRACKBARPARTS = TRACKBARPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKP_THUMBBOTTOM: TRACKBARPARTS = TRACKBARPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKP_THUMBTOP: TRACKBARPARTS = TRACKBARPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKP_THUMBVERT: TRACKBARPARTS = TRACKBARPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKP_THUMBLEFT: TRACKBARPARTS = TRACKBARPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKP_THUMBRIGHT: TRACKBARPARTS = TRACKBARPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKP_TICS: TRACKBARPARTS = TRACKBARPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKP_TICSVERT: TRACKBARPARTS = TRACKBARPARTS(10i32);
impl ::core::marker::Copy for TRACKBARPARTS {}
impl ::core::clone::Clone for TRACKBARPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRACKBARPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRACKBARSTYLESTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TKS_NORMAL: TRACKBARSTYLESTATES = TRACKBARSTYLESTATES(1i32);
impl ::core::marker::Copy for TRACKBARSTYLESTATES {}
impl ::core::clone::Clone for TRACKBARSTYLESTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRACKBARSTYLESTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRACKSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TRS_NORMAL: TRACKSTATES = TRACKSTATES(1i32);
impl ::core::marker::Copy for TRACKSTATES {}
impl ::core::clone::Clone for TRACKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRACKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRACKVERTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TRVS_NORMAL: TRACKVERTSTATES = TRACKVERTSTATES(1i32);
impl ::core::marker::Copy for TRACKVERTSTATES {}
impl ::core::clone::Clone for TRACKVERTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRACKVERTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRAILINGGRIDCELLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGC_HOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGC_HASSTATE: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGC_HASSTATEHOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGC_TODAY: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGC_TODAYSELECTED: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGC_SELECTED: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGC_SELECTEDHOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(7i32);
impl ::core::marker::Copy for TRAILINGGRIDCELLSTATES {}
impl ::core::clone::Clone for TRAILINGGRIDCELLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRAILINGGRIDCELLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRAILINGGRIDCELLUPPERSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGCU_HOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGCU_HASSTATE: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGCU_HASSTATEHOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGCU_SELECTED: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const MCTGCU_SELECTEDHOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(5i32);
impl ::core::marker::Copy for TRAILINGGRIDCELLUPPERSTATES {}
impl ::core::clone::Clone for TRAILINGGRIDCELLUPPERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRAILINGGRIDCELLUPPERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRANSPARENTBACKGROUNDSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBTBS_NORMAL: TRANSPARENTBACKGROUNDSTATES = TRANSPARENTBACKGROUNDSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBTBS_HOT: TRANSPARENTBACKGROUNDSTATES = TRANSPARENTBACKGROUNDSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBTBS_DISABLED: TRANSPARENTBACKGROUNDSTATES = TRANSPARENTBACKGROUNDSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const CBTBS_FOCUSED: TRANSPARENTBACKGROUNDSTATES = TRANSPARENTBACKGROUNDSTATES(4i32);
impl ::core::marker::Copy for TRANSPARENTBACKGROUNDSTATES {}
impl ::core::clone::Clone for TRANSPARENTBACKGROUNDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRANSPARENTBACKGROUNDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRANSPARENTBARSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBBS_NORMAL: TRANSPARENTBARSTATES = TRANSPARENTBARSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBBS_PARTIAL: TRANSPARENTBARSTATES = TRANSPARENTBARSTATES(2i32);
impl ::core::marker::Copy for TRANSPARENTBARSTATES {}
impl ::core::clone::Clone for TRANSPARENTBARSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRANSPARENTBARSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRANSPARENTBARVERTSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBBVS_NORMAL: TRANSPARENTBARVERTSTATES = TRANSPARENTBARVERTSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const PBBVS_PARTIAL: TRANSPARENTBARVERTSTATES = TRANSPARENTBARVERTSTATES(2i32);
impl ::core::marker::Copy for TRANSPARENTBARVERTSTATES {}
impl ::core::clone::Clone for TRANSPARENTBARVERTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRANSPARENTBARVERTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRAYNOTIFYPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TNP_BACKGROUND: TRAYNOTIFYPARTS = TRAYNOTIFYPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TNP_ANIMBACKGROUND: TRAYNOTIFYPARTS = TRAYNOTIFYPARTS(2i32);
impl ::core::marker::Copy for TRAYNOTIFYPARTS {}
impl ::core::clone::Clone for TRAYNOTIFYPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRAYNOTIFYPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TREEITEMSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TREIS_NORMAL: TREEITEMSTATES = TREEITEMSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TREIS_HOT: TREEITEMSTATES = TREEITEMSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TREIS_SELECTED: TREEITEMSTATES = TREEITEMSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TREIS_DISABLED: TREEITEMSTATES = TREEITEMSTATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TREIS_SELECTEDNOTFOCUS: TREEITEMSTATES = TREEITEMSTATES(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TREIS_HOTSELECTED: TREEITEMSTATES = TREEITEMSTATES(6i32);
impl ::core::marker::Copy for TREEITEMSTATES {}
impl ::core::clone::Clone for TREEITEMSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TREEITEMSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TREEVIEWPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVP_TREEITEM: TREEVIEWPARTS = TREEVIEWPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVP_GLYPH: TREEVIEWPARTS = TREEVIEWPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVP_BRANCH: TREEVIEWPARTS = TREEVIEWPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVP_HOTGLYPH: TREEVIEWPARTS = TREEVIEWPARTS(4i32);
impl ::core::marker::Copy for TREEVIEWPARTS {}
impl ::core::clone::Clone for TREEVIEWPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TREEVIEWPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TREE_VIEW_ITEM_STATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_SELECTED: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_CUT: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_DROPHILITED: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_BOLD: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_EXPANDED: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_EXPANDEDONCE: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_EXPANDPARTIAL: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_OVERLAYMASK: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(3840u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_STATEIMAGEMASK: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(61440u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_USERMASK: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(61440u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_EX_FLAT: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_EX_DISABLED: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIS_EX_ALL: TREE_VIEW_ITEM_STATE_FLAGS = TREE_VIEW_ITEM_STATE_FLAGS(2u32);
impl ::core::marker::Copy for TREE_VIEW_ITEM_STATE_FLAGS {}
impl ::core::clone::Clone for TREE_VIEW_ITEM_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TREE_VIEW_ITEM_STATE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRUESIZESCALINGTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TSST_NONE: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TSST_SIZE: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TSST_DPI: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(2i32);
impl ::core::marker::Copy for TRUESIZESCALINGTYPE {}
impl ::core::clone::Clone for TRUESIZESCALINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRUESIZESCALINGTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TVHITTESTINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_ABOVE: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_BELOW: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_NOWHERE: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_ONITEM: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(70u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_ONITEMBUTTON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_ONITEMICON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_ONITEMINDENT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_ONITEMLABEL: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_ONITEMRIGHT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_ONITEMSTATEICON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_TOLEFT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVHT_TORIGHT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(1024u32);
impl ::core::marker::Copy for TVHITTESTINFO_FLAGS {}
impl ::core::clone::Clone for TVHITTESTINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TVHITTESTINFO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TVITEMEXW_CHILDREN(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const I_ZERO: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const I_ONE_OR_MORE: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const I_CHILDRENCALLBACK: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(-1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const I_CHILDRENAUTO: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(-2i32);
impl ::core::marker::Copy for TVITEMEXW_CHILDREN {}
impl ::core::clone::Clone for TVITEMEXW_CHILDREN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TVITEMEXW_CHILDREN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TVITEMPART(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVGIPR_BUTTON: TVITEMPART = TVITEMPART(1i32);
impl ::core::marker::Copy for TVITEMPART {}
impl ::core::clone::Clone for TVITEMPART {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TVITEMPART {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TVITEM_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_CHILDREN: TVITEM_MASK = TVITEM_MASK(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_DI_SETITEM: TVITEM_MASK = TVITEM_MASK(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_HANDLE: TVITEM_MASK = TVITEM_MASK(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_IMAGE: TVITEM_MASK = TVITEM_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_PARAM: TVITEM_MASK = TVITEM_MASK(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_SELECTEDIMAGE: TVITEM_MASK = TVITEM_MASK(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_STATE: TVITEM_MASK = TVITEM_MASK(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_TEXT: TVITEM_MASK = TVITEM_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_EXPANDEDIMAGE: TVITEM_MASK = TVITEM_MASK(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_INTEGRAL: TVITEM_MASK = TVITEM_MASK(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TVIF_STATEEX: TVITEM_MASK = TVITEM_MASK(256u32);
impl ::core::marker::Copy for TVITEM_MASK {}
impl ::core::clone::Clone for TVITEM_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TVITEM_MASK {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UPDATEMETADATASTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDUPDATEMETADATA_HIGHLIGHT: UPDATEMETADATASTATES = UPDATEMETADATASTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDUPDATEMETADATA_NOHIGHLIGHT: UPDATEMETADATASTATES = UPDATEMETADATASTATES(2i32);
impl ::core::marker::Copy for UPDATEMETADATASTATES {}
impl ::core::clone::Clone for UPDATEMETADATASTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UPDATEMETADATASTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UPHORZSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPHZS_NORMAL: UPHORZSTATES = UPHORZSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPHZS_HOT: UPHORZSTATES = UPHORZSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPHZS_PRESSED: UPHORZSTATES = UPHORZSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPHZS_DISABLED: UPHORZSTATES = UPHORZSTATES(4i32);
impl ::core::marker::Copy for UPHORZSTATES {}
impl ::core::clone::Clone for UPHORZSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UPHORZSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UPSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPS_NORMAL: UPSTATES = UPSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPS_HOT: UPSTATES = UPSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPS_PRESSED: UPSTATES = UPSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UPS_DISABLED: UPSTATES = UPSTATES(4i32);
impl ::core::marker::Copy for UPSTATES {}
impl ::core::clone::Clone for UPSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UPSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USERTILEPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UTP_STROKEBACKGROUND: USERTILEPARTS = USERTILEPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const UTP_HOVERBACKGROUND: USERTILEPARTS = USERTILEPARTS(2i32);
impl ::core::marker::Copy for USERTILEPARTS {}
impl ::core::clone::Clone for USERTILEPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USERTILEPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VALIGN(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VA_TOP: VALIGN = VALIGN(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VA_CENTER: VALIGN = VALIGN(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VA_BOTTOM: VALIGN = VALIGN(2i32);
impl ::core::marker::Copy for VALIGN {}
impl ::core::clone::Clone for VALIGN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VALIGN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VERTSCROLLSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSS_NORMAL: VERTSCROLLSTATES = VERTSCROLLSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSS_HOT: VERTSCROLLSTATES = VERTSCROLLSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSS_PUSHED: VERTSCROLLSTATES = VERTSCROLLSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VSS_DISABLED: VERTSCROLLSTATES = VERTSCROLLSTATES(4i32);
impl ::core::marker::Copy for VERTSCROLLSTATES {}
impl ::core::clone::Clone for VERTSCROLLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VERTSCROLLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VERTTHUMBSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VTS_NORMAL: VERTTHUMBSTATES = VERTTHUMBSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VTS_HOT: VERTTHUMBSTATES = VERTTHUMBSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VTS_PUSHED: VERTTHUMBSTATES = VERTTHUMBSTATES(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const VTS_DISABLED: VERTTHUMBSTATES = VERTTHUMBSTATES(4i32);
impl ::core::marker::Copy for VERTTHUMBSTATES {}
impl ::core::clone::Clone for VERTTHUMBSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VERTTHUMBSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WARNINGSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDWARNING_HIGHLIGHT: WARNINGSTATES = WARNINGSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const DDWARNING_NOHIGHLIGHT: WARNINGSTATES = WARNINGSTATES(2i32);
impl ::core::marker::Copy for WARNINGSTATES {}
impl ::core::clone::Clone for WARNINGSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WARNINGSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINDOWPARTS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_CAPTION: WINDOWPARTS = WINDOWPARTS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLCAPTION: WINDOWPARTS = WINDOWPARTS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_MINCAPTION: WINDOWPARTS = WINDOWPARTS(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLMINCAPTION: WINDOWPARTS = WINDOWPARTS(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_MAXCAPTION: WINDOWPARTS = WINDOWPARTS(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLMAXCAPTION: WINDOWPARTS = WINDOWPARTS(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_FRAMELEFT: WINDOWPARTS = WINDOWPARTS(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_FRAMERIGHT: WINDOWPARTS = WINDOWPARTS(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_FRAMEBOTTOM: WINDOWPARTS = WINDOWPARTS(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLFRAMELEFT: WINDOWPARTS = WINDOWPARTS(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLFRAMERIGHT: WINDOWPARTS = WINDOWPARTS(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLFRAMEBOTTOM: WINDOWPARTS = WINDOWPARTS(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SYSBUTTON: WINDOWPARTS = WINDOWPARTS(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_MDISYSBUTTON: WINDOWPARTS = WINDOWPARTS(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_MINBUTTON: WINDOWPARTS = WINDOWPARTS(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_MDIMINBUTTON: WINDOWPARTS = WINDOWPARTS(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_MAXBUTTON: WINDOWPARTS = WINDOWPARTS(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_CLOSEBUTTON: WINDOWPARTS = WINDOWPARTS(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLCLOSEBUTTON: WINDOWPARTS = WINDOWPARTS(19i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_MDICLOSEBUTTON: WINDOWPARTS = WINDOWPARTS(20i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_RESTOREBUTTON: WINDOWPARTS = WINDOWPARTS(21i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_MDIRESTOREBUTTON: WINDOWPARTS = WINDOWPARTS(22i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_HELPBUTTON: WINDOWPARTS = WINDOWPARTS(23i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_MDIHELPBUTTON: WINDOWPARTS = WINDOWPARTS(24i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_HORZSCROLL: WINDOWPARTS = WINDOWPARTS(25i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_HORZTHUMB: WINDOWPARTS = WINDOWPARTS(26i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_VERTSCROLL: WINDOWPARTS = WINDOWPARTS(27i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_VERTTHUMB: WINDOWPARTS = WINDOWPARTS(28i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_DIALOG: WINDOWPARTS = WINDOWPARTS(29i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_CAPTIONSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(30i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLCAPTIONSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(31i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_FRAMELEFTSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLFRAMELEFTSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(33i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_FRAMERIGHTSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(34i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLFRAMERIGHTSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(35i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_FRAMEBOTTOMSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(36i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_SMALLFRAMEBOTTOMSIZINGTEMPLATE: WINDOWPARTS = WINDOWPARTS(37i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_FRAME: WINDOWPARTS = WINDOWPARTS(38i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WP_BORDER: WINDOWPARTS = WINDOWPARTS(39i32);
impl ::core::marker::Copy for WINDOWPARTS {}
impl ::core::clone::Clone for WINDOWPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINDOWPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINDOWTHEMEATTRIBUTETYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WTA_NONCLIENT: WINDOWTHEMEATTRIBUTETYPE = WINDOWTHEMEATTRIBUTETYPE(1i32);
impl ::core::marker::Copy for WINDOWTHEMEATTRIBUTETYPE {}
impl ::core::clone::Clone for WINDOWTHEMEATTRIBUTETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINDOWTHEMEATTRIBUTETYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WORD_BREAK_ACTION(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WB_CLASSIFY: WORD_BREAK_ACTION = WORD_BREAK_ACTION(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WB_ISDELIMITER: WORD_BREAK_ACTION = WORD_BREAK_ACTION(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WB_LEFT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WB_LEFTBREAK: WORD_BREAK_ACTION = WORD_BREAK_ACTION(6u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WB_MOVEWORDLEFT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WB_MOVEWORDRIGHT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(5u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WB_RIGHT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WB_RIGHTBREAK: WORD_BREAK_ACTION = WORD_BREAK_ACTION(7u32);
impl ::core::marker::Copy for WORD_BREAK_ACTION {}
impl ::core::clone::Clone for WORD_BREAK_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WORD_BREAK_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WRENCHSTATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTWS_NORMAL: WRENCHSTATES = WRENCHSTATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTWS_HOT: WRENCHSTATES = WRENCHSTATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const TTWS_PRESSED: WRENCHSTATES = WRENCHSTATES(3i32);
impl ::core::marker::Copy for WRENCHSTATES {}
impl ::core::clone::Clone for WRENCHSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WRENCHSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSB_PROP(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_CXHSCROLL: WSB_PROP = WSB_PROP(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_CXHTHUMB: WSB_PROP = WSB_PROP(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_CXVSCROLL: WSB_PROP = WSB_PROP(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_CYHSCROLL: WSB_PROP = WSB_PROP(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_CYVSCROLL: WSB_PROP = WSB_PROP(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_CYVTHUMB: WSB_PROP = WSB_PROP(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_HBKGCOLOR: WSB_PROP = WSB_PROP(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_HSTYLE: WSB_PROP = WSB_PROP(512i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_PALETTE: WSB_PROP = WSB_PROP(2048i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_VBKGCOLOR: WSB_PROP = WSB_PROP(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_VSTYLE: WSB_PROP = WSB_PROP(256i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const WSB_PROP_WINSTYLE: WSB_PROP = WSB_PROP(1024i32);
impl ::core::marker::Copy for WSB_PROP {}
impl ::core::clone::Clone for WSB_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WSB_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _LI_METRIC(pub i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIM_SMALL: _LI_METRIC = _LI_METRIC(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub const LIM_LARGE: _LI_METRIC = _LI_METRIC(1i32);
impl ::core::marker::Copy for _LI_METRIC {}
impl ::core::clone::Clone for _LI_METRIC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for _LI_METRIC {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct BP_ANIMATIONPARAMS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub style: BP_ANIMATIONSTYLE,
    pub dwDuration: u32,
}
impl ::core::marker::Copy for BP_ANIMATIONPARAMS {}
impl ::core::clone::Clone for BP_ANIMATIONPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BP_ANIMATIONPARAMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct BP_PAINTPARAMS {
    pub cbSize: u32,
    pub dwFlags: BP_PAINTPARAMS_FLAGS,
    pub prcExclude: *const super::super::Foundation::RECT,
    pub pBlendFunction: *const super::super::Graphics::Gdi::BLENDFUNCTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for BP_PAINTPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for BP_PAINTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for BP_PAINTPARAMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BUTTON_IMAGELIST {
    pub himl: HIMAGELIST,
    pub margin: super::super::Foundation::RECT,
    pub uAlign: BUTTON_IMAGELIST_ALIGN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BUTTON_IMAGELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BUTTON_IMAGELIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BUTTON_IMAGELIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BUTTON_SPLITINFO {
    pub mask: u32,
    pub himlGlyph: HIMAGELIST,
    pub uSplitStyle: u32,
    pub size: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BUTTON_SPLITINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BUTTON_SPLITINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BUTTON_SPLITINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CCINFOA {
    pub szClass: [super::super::Foundation::CHAR; 32],
    pub flOptions: u32,
    pub szDesc: [super::super::Foundation::CHAR; 32],
    pub cxDefault: u32,
    pub cyDefault: u32,
    pub flStyleDefault: u32,
    pub flExtStyleDefault: u32,
    pub flCtrlTypeMask: u32,
    pub szTextDefault: [super::super::Foundation::CHAR; 256],
    pub cStyleFlags: i32,
    pub aStyleFlags: *mut CCSTYLEFLAGA,
    pub lpfnStyle: LPFNCCSTYLEA,
    pub lpfnSizeToText: LPFNCCSIZETOTEXTA,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CCINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CCINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for CCINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CCINFOW {
    pub szClass: [u16; 32],
    pub flOptions: u32,
    pub szDesc: [u16; 32],
    pub cxDefault: u32,
    pub cyDefault: u32,
    pub flStyleDefault: u32,
    pub flExtStyleDefault: u32,
    pub flCtrlTypeMask: u32,
    pub cStyleFlags: i32,
    pub aStyleFlags: *mut CCSTYLEFLAGW,
    pub szTextDefault: [u16; 256],
    pub lpfnStyle: LPFNCCSTYLEW,
    pub lpfnSizeToText: LPFNCCSIZETOTEXTW,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CCINFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CCINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for CCINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CCSTYLEA {
    pub flStyle: u32,
    pub flExtStyle: u32,
    pub szText: [super::super::Foundation::CHAR; 256],
    pub lgid: u16,
    pub wReserved1: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CCSTYLEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CCSTYLEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CCSTYLEA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct CCSTYLEFLAGA {
    pub flStyle: u32,
    pub flStyleMask: u32,
    pub pszStyle: ::windows::core::PSTR,
}
impl ::core::marker::Copy for CCSTYLEFLAGA {}
impl ::core::clone::Clone for CCSTYLEFLAGA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CCSTYLEFLAGA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct CCSTYLEFLAGW {
    pub flStyle: u32,
    pub flStyleMask: u32,
    pub pszStyle: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CCSTYLEFLAGW {}
impl ::core::clone::Clone for CCSTYLEFLAGW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CCSTYLEFLAGW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct CCSTYLEW {
    pub flStyle: u32,
    pub flExtStyle: u32,
    pub szText: [u16; 256],
    pub lgid: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for CCSTYLEW {}
impl ::core::clone::Clone for CCSTYLEW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CCSTYLEW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COLORMAP {
    pub from: super::super::Foundation::COLORREF,
    pub to: super::super::Foundation::COLORREF,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COLORMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COLORMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COLORMAP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COLORSCHEME {
    pub dwSize: u32,
    pub clrBtnHighlight: super::super::Foundation::COLORREF,
    pub clrBtnShadow: super::super::Foundation::COLORREF,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COLORSCHEME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COLORSCHEME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COLORSCHEME {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMBOBOXEXITEMA {
    pub mask: COMBOBOX_EX_ITEM_FLAGS,
    pub iItem: isize,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMBOBOXEXITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMBOBOXEXITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COMBOBOXEXITEMA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMBOBOXEXITEMW {
    pub mask: COMBOBOX_EX_ITEM_FLAGS,
    pub iItem: isize,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMBOBOXEXITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMBOBOXEXITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COMBOBOXEXITEMW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMBOBOXINFO {
    pub cbSize: u32,
    pub rcItem: super::super::Foundation::RECT,
    pub rcButton: super::super::Foundation::RECT,
    pub stateButton: COMBOBOXINFO_BUTTON_STATE,
    pub hwndCombo: super::super::Foundation::HWND,
    pub hwndItem: super::super::Foundation::HWND,
    pub hwndList: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMBOBOXINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMBOBOXINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COMBOBOXINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPAREITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub hwndItem: super::super::Foundation::HWND,
    pub itemID1: u32,
    pub itemData1: usize,
    pub itemID2: u32,
    pub itemData2: usize,
    pub dwLocaleId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPAREITEMSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPAREITEMSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COMPAREITEMSTRUCT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DATETIMEPICKERINFO {
    pub cbSize: u32,
    pub rcCheck: super::super::Foundation::RECT,
    pub stateCheck: u32,
    pub rcButton: super::super::Foundation::RECT,
    pub stateButton: u32,
    pub hwndEdit: super::super::Foundation::HWND,
    pub hwndUD: super::super::Foundation::HWND,
    pub hwndDropDown: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DATETIMEPICKERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DATETIMEPICKERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DATETIMEPICKERINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DELETEITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub itemID: u32,
    pub hwndItem: super::super::Foundation::HWND,
    pub itemData: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELETEITEMSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELETEITEMSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DELETEITEMSTRUCT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct DPASTREAMINFO {
    pub iPos: i32,
    pub pvItem: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DPASTREAMINFO {}
impl ::core::clone::Clone for DPASTREAMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DPASTREAMINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRAGLISTINFO {
    pub uNotification: DRAGLISTINFO_NOTIFICATION_FLAGS,
    pub hWnd: super::super::Foundation::HWND,
    pub ptCursor: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRAGLISTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRAGLISTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRAGLISTINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DRAWITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemAction: ODA_FLAGS,
    pub itemState: ODS_FLAGS,
    pub hwndItem: super::super::Foundation::HWND,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub rcItem: super::super::Foundation::RECT,
    pub itemData: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DRAWITEMSTRUCT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DRAWITEMSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DRAWITEMSTRUCT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DTBGOPTS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub rcClip: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DTBGOPTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DTBGOPTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DTBGOPTS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DTTOPTS {
    pub dwSize: u32,
    pub dwFlags: DTTOPTS_FLAGS,
    pub crText: super::super::Foundation::COLORREF,
    pub crBorder: super::super::Foundation::COLORREF,
    pub crShadow: super::super::Foundation::COLORREF,
    pub iTextShadowType: i32,
    pub ptShadowOffset: super::super::Foundation::POINT,
    pub iBorderSize: i32,
    pub iFontPropId: i32,
    pub iColorPropId: i32,
    pub iStateId: i32,
    pub fApplyOverlay: super::super::Foundation::BOOL,
    pub iGlowSize: i32,
    pub pfnDrawTextCallback: DTT_CALLBACK_PROC,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DTTOPTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DTTOPTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DTTOPTS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct EDITBALLOONTIP {
    pub cbStruct: u32,
    pub pszTitle: ::windows::core::PCWSTR,
    pub pszText: ::windows::core::PCWSTR,
    pub ttiIcon: EDITBALLOONTIP_ICON,
}
impl ::core::marker::Copy for EDITBALLOONTIP {}
impl ::core::clone::Clone for EDITBALLOONTIP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EDITBALLOONTIP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HDHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: HEADER_HITTEST_INFO_FLAGS,
    pub iItem: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HDHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HDHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HDHITTESTINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct HDITEMA {
    pub mask: HDI_MASK,
    pub cxy: i32,
    pub pszText: ::windows::core::PSTR,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: HEADER_CONTROL_FORMAT_FLAGS,
    pub lParam: super::super::Foundation::LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub r#type: HEADER_CONTROL_FORMAT_TYPE,
    pub pvFilter: *mut ::core::ffi::c_void,
    pub state: HEADER_CONTROL_FORMAT_STATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for HDITEMA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for HDITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for HDITEMA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct HDITEMW {
    pub mask: HDI_MASK,
    pub cxy: i32,
    pub pszText: ::windows::core::PWSTR,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: HEADER_CONTROL_FORMAT_FLAGS,
    pub lParam: super::super::Foundation::LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub r#type: HEADER_CONTROL_FORMAT_TYPE,
    pub pvFilter: *mut ::core::ffi::c_void,
    pub state: HEADER_CONTROL_FORMAT_STATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for HDITEMW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for HDITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for HDITEMW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct HDLAYOUT {
    pub prc: *mut super::super::Foundation::RECT,
    pub pwpos: *mut super::WindowsAndMessaging::WINDOWPOS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for HDLAYOUT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for HDLAYOUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for HDLAYOUT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDPA(pub isize);
impl HDPA {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDPA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDPA {}
impl ::core::fmt::Debug for HDPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDPA").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HDPA>> for HDPA {
    fn from(optional: ::core::option::Option<HDPA>) -> HDPA {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HDPA {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HDSA(pub isize);
impl HDSA {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDSA {}
impl ::core::fmt::Debug for HDSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDSA").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HDSA>> for HDSA {
    fn from(optional: ::core::option::Option<HDSA>) -> HDSA {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HDSA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct HD_TEXTFILTERA {
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
}
impl ::core::marker::Copy for HD_TEXTFILTERA {}
impl ::core::clone::Clone for HD_TEXTFILTERA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HD_TEXTFILTERA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct HD_TEXTFILTERW {
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
}
impl ::core::marker::Copy for HD_TEXTFILTERW {}
impl ::core::clone::Clone for HD_TEXTFILTERW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HD_TEXTFILTERW {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HIMAGELIST(pub isize);
impl HIMAGELIST {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HIMAGELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HIMAGELIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HIMAGELIST {}
impl ::core::fmt::Debug for HIMAGELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIMAGELIST").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HIMAGELIST>> for HIMAGELIST {
    fn from(optional: ::core::option::Option<HIMAGELIST>) -> HIMAGELIST {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HIMAGELIST {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HPROPSHEETPAGE(pub isize);
impl HPROPSHEETPAGE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HPROPSHEETPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPROPSHEETPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPROPSHEETPAGE {}
impl ::core::fmt::Debug for HPROPSHEETPAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPROPSHEETPAGE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HPROPSHEETPAGE>> for HPROPSHEETPAGE {
    fn from(optional: ::core::option::Option<HPROPSHEETPAGE>) -> HPROPSHEETPAGE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HPROPSHEETPAGE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HSYNTHETICPOINTERDEVICE(pub isize);
impl HSYNTHETICPOINTERDEVICE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HSYNTHETICPOINTERDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HSYNTHETICPOINTERDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HSYNTHETICPOINTERDEVICE {}
impl ::core::fmt::Debug for HSYNTHETICPOINTERDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HSYNTHETICPOINTERDEVICE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HSYNTHETICPOINTERDEVICE>> for HSYNTHETICPOINTERDEVICE {
    fn from(optional: ::core::option::Option<HSYNTHETICPOINTERDEVICE>) -> HSYNTHETICPOINTERDEVICE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HSYNTHETICPOINTERDEVICE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTHEME(pub isize);
impl HTHEME {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for HTHEME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HTHEME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HTHEME {}
impl ::core::fmt::Debug for HTHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTHEME").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HTHEME>> for HTHEME {
    fn from(optional: ::core::option::Option<HTHEME>) -> HTHEME {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HTHEME {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTREEITEM(pub isize);
impl ::core::default::Default for HTREEITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HTREEITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HTREEITEM {}
impl ::core::fmt::Debug for HTREEITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTREEITEM").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HTREEITEM>> for HTREEITEM {
    fn from(optional: ::core::option::Option<HTREEITEM>) -> HTREEITEM {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HTREEITEM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct IMAGEINFO {
    pub hbmImage: super::super::Graphics::Gdi::HBITMAP,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub Unused1: i32,
    pub Unused2: i32,
    pub rcImage: super::super::Foundation::RECT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for IMAGEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for IMAGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for IMAGEINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct IMAGELISTDRAWPARAMS {
    pub cbSize: u32,
    pub himl: HIMAGELIST,
    pub i: i32,
    pub hdcDst: super::super::Graphics::Gdi::HDC,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub xBitmap: i32,
    pub yBitmap: i32,
    pub rgbBk: super::super::Foundation::COLORREF,
    pub rgbFg: super::super::Foundation::COLORREF,
    pub fStyle: u32,
    pub dwRop: u32,
    pub fState: u32,
    pub Frame: u32,
    pub crEffect: super::super::Foundation::COLORREF,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for IMAGELISTDRAWPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for IMAGELISTDRAWPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for IMAGELISTDRAWPARAMS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct IMAGELISTSTATS {
    pub cbSize: u32,
    pub cAlloc: i32,
    pub cUsed: i32,
    pub cStandby: i32,
}
impl ::core::marker::Copy for IMAGELISTSTATS {}
impl ::core::clone::Clone for IMAGELISTSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMAGELISTSTATS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct INITCOMMONCONTROLSEX {
    pub dwSize: u32,
    pub dwICC: INITCOMMONCONTROLSEX_ICC,
}
impl ::core::marker::Copy for INITCOMMONCONTROLSEX {}
impl ::core::clone::Clone for INITCOMMONCONTROLSEX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INITCOMMONCONTROLSEX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct INTLIST {
    pub iValueCount: i32,
    pub iValues: [i32; 402],
}
impl ::core::marker::Copy for INTLIST {}
impl ::core::clone::Clone for INTLIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INTLIST {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub item: LITEM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LHITTESTINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LITEM {
    pub mask: LIST_ITEM_FLAGS,
    pub iLink: i32,
    pub state: LIST_ITEM_STATE_FLAGS,
    pub stateMask: LIST_ITEM_STATE_FLAGS,
    pub szID: [u16; 48],
    pub szUrl: [u16; 2084],
}
impl ::core::marker::Copy for LITEM {}
impl ::core::clone::Clone for LITEM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LITEM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct LVBKIMAGEA {
    pub ulFlags: LIST_VIEW_BACKGROUND_IMAGE_FLAGS,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub pszImage: ::windows::core::PSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for LVBKIMAGEA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for LVBKIMAGEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for LVBKIMAGEA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct LVBKIMAGEW {
    pub ulFlags: LIST_VIEW_BACKGROUND_IMAGE_FLAGS,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub pszImage: ::windows::core::PWSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for LVBKIMAGEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for LVBKIMAGEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for LVBKIMAGEW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LVCOLUMNA {
    pub mask: LVCOLUMNW_MASK,
    pub fmt: LVCOLUMNW_FORMAT,
    pub cx: i32,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
impl ::core::marker::Copy for LVCOLUMNA {}
impl ::core::clone::Clone for LVCOLUMNA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVCOLUMNA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LVCOLUMNW {
    pub mask: LVCOLUMNW_MASK,
    pub fmt: LVCOLUMNW_FORMAT,
    pub cx: i32,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
impl ::core::marker::Copy for LVCOLUMNW {}
impl ::core::clone::Clone for LVCOLUMNW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVCOLUMNW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LVFINDINFOA {
    pub flags: LVFINDINFOW_FLAGS,
    pub psz: ::windows::core::PCSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub pt: super::super::Foundation::POINT,
    pub vkDirection: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LVFINDINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LVFINDINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LVFINDINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LVFINDINFOW {
    pub flags: LVFINDINFOW_FLAGS,
    pub psz: ::windows::core::PCWSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub pt: super::super::Foundation::POINT,
    pub vkDirection: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LVFINDINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LVFINDINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LVFINDINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LVFOOTERINFO {
    pub mask: u32,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub cItems: u32,
}
impl ::core::marker::Copy for LVFOOTERINFO {}
impl ::core::clone::Clone for LVFOOTERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVFOOTERINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LVFOOTERITEM {
    pub mask: LVFOOTERITEM_MASK,
    pub iItem: i32,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub state: u32,
    pub stateMask: u32,
}
impl ::core::marker::Copy for LVFOOTERITEM {}
impl ::core::clone::Clone for LVFOOTERITEM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVFOOTERITEM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LVGROUP {
    pub cbSize: u32,
    pub mask: LVGROUP_MASK,
    pub pszHeader: ::windows::core::PWSTR,
    pub cchHeader: i32,
    pub pszFooter: ::windows::core::PWSTR,
    pub cchFooter: i32,
    pub iGroupId: i32,
    pub stateMask: LIST_VIEW_GROUP_STATE_FLAGS,
    pub state: LIST_VIEW_GROUP_STATE_FLAGS,
    pub uAlign: LIST_VIEW_GROUP_ALIGN_FLAGS,
    pub pszSubtitle: ::windows::core::PWSTR,
    pub cchSubtitle: u32,
    pub pszTask: ::windows::core::PWSTR,
    pub cchTask: u32,
    pub pszDescriptionTop: ::windows::core::PWSTR,
    pub cchDescriptionTop: u32,
    pub pszDescriptionBottom: ::windows::core::PWSTR,
    pub cchDescriptionBottom: u32,
    pub iTitleImage: i32,
    pub iExtendedImage: i32,
    pub iFirstItem: i32,
    pub cItems: u32,
    pub pszSubsetTitle: ::windows::core::PWSTR,
    pub cchSubsetTitle: u32,
}
impl ::core::marker::Copy for LVGROUP {}
impl ::core::clone::Clone for LVGROUP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVGROUP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LVGROUPMETRICS {
    pub cbSize: u32,
    pub mask: u32,
    pub Left: u32,
    pub Top: u32,
    pub Right: u32,
    pub Bottom: u32,
    pub crLeft: super::super::Foundation::COLORREF,
    pub crTop: super::super::Foundation::COLORREF,
    pub crRight: super::super::Foundation::COLORREF,
    pub crBottom: super::super::Foundation::COLORREF,
    pub crHeader: super::super::Foundation::COLORREF,
    pub crFooter: super::super::Foundation::COLORREF,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LVGROUPMETRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LVGROUPMETRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LVGROUPMETRICS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LVHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: LVHITTESTINFO_FLAGS,
    pub iItem: i32,
    pub iSubItem: i32,
    pub iGroup: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LVHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LVHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LVHITTESTINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LVINSERTGROUPSORTED {
    pub pfnGroupCompare: PFNLVGROUPCOMPARE,
    pub pvData: *mut ::core::ffi::c_void,
    pub lvGroup: LVGROUP,
}
impl ::core::marker::Copy for LVINSERTGROUPSORTED {}
impl ::core::clone::Clone for LVINSERTGROUPSORTED {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVINSERTGROUPSORTED {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LVINSERTMARK {
    pub cbSize: u32,
    pub dwFlags: LIST_VIEW_INSERT_MARK_FLAGS,
    pub iItem: i32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for LVINSERTMARK {}
impl ::core::clone::Clone for LVINSERTMARK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVINSERTMARK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LVITEMA {
    pub mask: LIST_VIEW_ITEM_FLAGS,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: LIST_VIEW_ITEM_STATE_FLAGS,
    pub stateMask: LIST_VIEW_ITEM_STATE_FLAGS,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
    pub iGroupId: LVITEMA_GROUP_ID,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS,
    pub iGroup: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LVITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LVITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LVITEMA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LVITEMINDEX {
    pub iItem: i32,
    pub iGroup: i32,
}
impl ::core::marker::Copy for LVITEMINDEX {}
impl ::core::clone::Clone for LVITEMINDEX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVITEMINDEX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LVITEMW {
    pub mask: LIST_VIEW_ITEM_FLAGS,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: LIST_VIEW_ITEM_STATE_FLAGS,
    pub stateMask: LIST_VIEW_ITEM_STATE_FLAGS,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
    pub iGroupId: LVITEMA_GROUP_ID,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut LIST_VIEW_ITEM_COLUMN_FORMAT_FLAGS,
    pub iGroup: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LVITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LVITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LVITEMW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LVSETINFOTIP {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pszText: ::windows::core::PWSTR,
    pub iItem: i32,
    pub iSubItem: i32,
}
impl ::core::marker::Copy for LVSETINFOTIP {}
impl ::core::clone::Clone for LVSETINFOTIP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVSETINFOTIP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct LVTILEINFO {
    pub cbSize: u32,
    pub iItem: i32,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut i32,
}
impl ::core::marker::Copy for LVTILEINFO {}
impl ::core::clone::Clone for LVTILEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LVTILEINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LVTILEVIEWINFO {
    pub cbSize: u32,
    pub dwMask: LVTILEVIEWINFO_MASK,
    pub dwFlags: LVTILEVIEWINFO_FLAGS,
    pub sizeTile: super::super::Foundation::SIZE,
    pub cLines: i32,
    pub rcLabelMargin: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LVTILEVIEWINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LVTILEVIEWINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LVTILEVIEWINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct MARGINS {
    pub cxLeftWidth: i32,
    pub cxRightWidth: i32,
    pub cyTopHeight: i32,
    pub cyBottomHeight: i32,
}
impl ::core::marker::Copy for MARGINS {}
impl ::core::clone::Clone for MARGINS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MARGINS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCGRIDINFO {
    pub cbSize: u32,
    pub dwPart: MCGRIDINFO_PART,
    pub dwFlags: MCGRIDINFO_FLAGS,
    pub iCalendar: i32,
    pub iRow: i32,
    pub iCol: i32,
    pub bSelected: super::super::Foundation::BOOL,
    pub stStart: super::super::Foundation::SYSTEMTIME,
    pub stEnd: super::super::Foundation::SYSTEMTIME,
    pub rc: super::super::Foundation::RECT,
    pub pszName: ::windows::core::PWSTR,
    pub cchName: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCGRIDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCGRIDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCGRIDINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCHITTESTINFO {
    pub cbSize: u32,
    pub pt: super::super::Foundation::POINT,
    pub uHit: MCHITTESTINFO_HIT_FLAGS,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub rc: super::super::Foundation::RECT,
    pub iOffset: i32,
    pub iRow: i32,
    pub iCol: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCHITTESTINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct MEASUREITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemWidth: u32,
    pub itemHeight: u32,
    pub itemData: usize,
}
impl ::core::marker::Copy for MEASUREITEMSTRUCT {}
impl ::core::clone::Clone for MEASUREITEMSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MEASUREITEMSTRUCT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMBCDROPDOWN {
    pub hdr: NMHDR,
    pub rcButton: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMBCDROPDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMBCDROPDOWN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMBCDROPDOWN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMBCHOTITEM {
    pub hdr: NMHDR,
    pub dwFlags: NMTBHOTITEM_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMBCHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMBCHOTITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMBCHOTITEM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMCBEDRAGBEGINA {
    pub hdr: NMHDR,
    pub iItemid: i32,
    pub szText: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMCBEDRAGBEGINA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMCBEDRAGBEGINA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMCBEDRAGBEGINA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMCBEDRAGBEGINW {
    pub hdr: NMHDR,
    pub iItemid: i32,
    pub szText: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMCBEDRAGBEGINW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMCBEDRAGBEGINW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMCBEDRAGBEGINW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMCBEENDEDITA {
    pub hdr: NMHDR,
    pub fChanged: super::super::Foundation::BOOL,
    pub iNewSelection: i32,
    pub szText: [super::super::Foundation::CHAR; 260],
    pub iWhy: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMCBEENDEDITA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMCBEENDEDITA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMCBEENDEDITA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMCBEENDEDITW {
    pub hdr: NMHDR,
    pub fChanged: super::super::Foundation::BOOL,
    pub iNewSelection: i32,
    pub szText: [u16; 260],
    pub iWhy: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMCBEENDEDITW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMCBEENDEDITW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMCBEENDEDITW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMCHAR {
    pub hdr: NMHDR,
    pub ch: u32,
    pub dwItemPrev: u32,
    pub dwItemNext: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMCHAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMCHAR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMCHAR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMCOMBOBOXEXA {
    pub hdr: NMHDR,
    pub ceItem: COMBOBOXEXITEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMCOMBOBOXEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMCOMBOBOXEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMCOMBOBOXEXA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMCOMBOBOXEXW {
    pub hdr: NMHDR,
    pub ceItem: COMBOBOXEXITEMW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMCOMBOBOXEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMCOMBOBOXEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMCOMBOBOXEXW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct NMCUSTOMDRAW {
    pub hdr: NMHDR,
    pub dwDrawStage: NMCUSTOMDRAW_DRAW_STAGE,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub rc: super::super::Foundation::RECT,
    pub dwItemSpec: usize,
    pub uItemState: NMCUSTOMDRAW_DRAW_STATE_FLAGS,
    pub lItemlParam: super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for NMCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for NMCUSTOMDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for NMCUSTOMDRAW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMCUSTOMSPLITRECTINFO {
    pub hdr: NMHDR,
    pub rcClient: super::super::Foundation::RECT,
    pub rcButton: super::super::Foundation::RECT,
    pub rcSplit: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMCUSTOMSPLITRECTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMCUSTOMSPLITRECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMCUSTOMSPLITRECTINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct NMCUSTOMTEXT {
    pub hdr: NMHDR,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub lpString: ::windows::core::PCWSTR,
    pub nCount: i32,
    pub lpRect: *mut super::super::Foundation::RECT,
    pub uFormat: u32,
    pub fLink: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for NMCUSTOMTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for NMCUSTOMTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for NMCUSTOMTEXT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMDATETIMECHANGE {
    pub nmhdr: NMHDR,
    pub dwFlags: NMDATETIMECHANGE_FLAGS,
    pub st: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMDATETIMECHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMDATETIMECHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMDATETIMECHANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMDATETIMEFORMATA {
    pub nmhdr: NMHDR,
    pub pszFormat: ::windows::core::PCSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub pszDisplay: ::windows::core::PCSTR,
    pub szDisplay: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMDATETIMEFORMATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMDATETIMEFORMATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMDATETIMEFORMATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMDATETIMEFORMATQUERYA {
    pub nmhdr: NMHDR,
    pub pszFormat: ::windows::core::PCSTR,
    pub szMax: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMDATETIMEFORMATQUERYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMDATETIMEFORMATQUERYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMDATETIMEFORMATQUERYA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMDATETIMEFORMATQUERYW {
    pub nmhdr: NMHDR,
    pub pszFormat: ::windows::core::PCWSTR,
    pub szMax: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMDATETIMEFORMATQUERYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMDATETIMEFORMATQUERYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMDATETIMEFORMATQUERYW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMDATETIMEFORMATW {
    pub nmhdr: NMHDR,
    pub pszFormat: ::windows::core::PCWSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub pszDisplay: ::windows::core::PCWSTR,
    pub szDisplay: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMDATETIMEFORMATW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMDATETIMEFORMATW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMDATETIMEFORMATW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMDATETIMESTRINGA {
    pub nmhdr: NMHDR,
    pub pszUserString: ::windows::core::PCSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMDATETIMESTRINGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMDATETIMESTRINGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMDATETIMESTRINGA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMDATETIMESTRINGW {
    pub nmhdr: NMHDR,
    pub pszUserString: ::windows::core::PCWSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMDATETIMESTRINGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMDATETIMESTRINGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMDATETIMESTRINGW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMDATETIMEWMKEYDOWNA {
    pub nmhdr: NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: ::windows::core::PCSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMDATETIMEWMKEYDOWNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMDATETIMEWMKEYDOWNA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMDATETIMEWMKEYDOWNA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMDATETIMEWMKEYDOWNW {
    pub nmhdr: NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: ::windows::core::PCWSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMDATETIMEWMKEYDOWNW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMDATETIMEWMKEYDOWNW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMDATETIMEWMKEYDOWNW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMDAYSTATE {
    pub nmhdr: NMHDR,
    pub stStart: super::super::Foundation::SYSTEMTIME,
    pub cDayState: i32,
    pub prgDayState: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMDAYSTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMDAYSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMDAYSTATE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMHDDISPINFOA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub mask: HDI_MASK,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMHDDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMHDDISPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMHDDISPINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMHDDISPINFOW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub mask: HDI_MASK,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMHDDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMHDDISPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMHDDISPINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMHDFILTERBTNCLICK {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMHDFILTERBTNCLICK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMHDFILTERBTNCLICK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMHDFILTERBTNCLICK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMHDR {
    pub hwndFrom: super::super::Foundation::HWND,
    pub idFrom: usize,
    pub code: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMHDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMHDR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct NMHEADERA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iButton: HEADER_CONTROL_NOTIFICATION_BUTTON,
    pub pitem: *mut HDITEMA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for NMHEADERA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for NMHEADERA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for NMHEADERA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct NMHEADERW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iButton: HEADER_CONTROL_NOTIFICATION_BUTTON,
    pub pitem: *mut HDITEMW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for NMHEADERW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for NMHEADERW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for NMHEADERW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMIPADDRESS {
    pub hdr: NMHDR,
    pub iField: i32,
    pub iValue: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMIPADDRESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMIPADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMIPADDRESS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMITEMACTIVATE {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: super::super::Foundation::POINT,
    pub lParam: super::super::Foundation::LPARAM,
    pub uKeyFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMITEMACTIVATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMITEMACTIVATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMITEMACTIVATE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMKEY {
    pub hdr: NMHDR,
    pub nVKey: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMKEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMKEY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLINK {
    pub hdr: NMHDR,
    pub item: LITEM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLINK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLINK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLISTVIEW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: LIST_VIEW_ITEM_FLAGS,
    pub ptAction: super::super::Foundation::POINT,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLISTVIEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLISTVIEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLISTVIEW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVCACHEHINT {
    pub hdr: NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVCACHEHINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVCACHEHINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVCACHEHINT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct NMLVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: super::super::Foundation::COLORREF,
    pub clrTextBk: super::super::Foundation::COLORREF,
    pub iSubItem: i32,
    pub dwItemType: NMLVCUSTOMDRAW_ITEM_TYPE,
    pub clrFace: super::super::Foundation::COLORREF,
    pub iIconEffect: i32,
    pub iIconPhase: i32,
    pub iPartId: i32,
    pub iStateId: i32,
    pub rcText: super::super::Foundation::RECT,
    pub uAlign: LIST_VIEW_GROUP_ALIGN_FLAGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for NMLVCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for NMLVCUSTOMDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for NMLVCUSTOMDRAW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVDISPINFOA {
    pub hdr: NMHDR,
    pub item: LVITEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVDISPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVDISPINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVDISPINFOW {
    pub hdr: NMHDR,
    pub item: LVITEMW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVDISPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVDISPINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVEMPTYMARKUP {
    pub hdr: NMHDR,
    pub dwFlags: NMLVEMPTYMARKUP_FLAGS,
    pub szMarkup: [u16; 2084],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVEMPTYMARKUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVEMPTYMARKUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVEMPTYMARKUP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVFINDITEMA {
    pub hdr: NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVFINDITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVFINDITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVFINDITEMA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVFINDITEMW {
    pub hdr: NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVFINDITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVFINDITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVFINDITEMW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVGETINFOTIPA {
    pub hdr: NMHDR,
    pub dwFlags: NMLVGETINFOTIP_FLAGS,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVGETINFOTIPA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVGETINFOTIPA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVGETINFOTIPW {
    pub hdr: NMHDR,
    pub dwFlags: NMLVGETINFOTIP_FLAGS,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVGETINFOTIPW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVGETINFOTIPW {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVKEYDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVKEYDOWN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVKEYDOWN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVLINK {
    pub hdr: NMHDR,
    pub link: LITEM,
    pub iItem: i32,
    pub iSubItem: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVLINK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVLINK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVODSTATECHANGE {
    pub hdr: NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
    pub uNewState: LIST_VIEW_ITEM_STATE_FLAGS,
    pub uOldState: LIST_VIEW_ITEM_STATE_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVODSTATECHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVODSTATECHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVODSTATECHANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMLVSCROLL {
    pub hdr: NMHDR,
    pub dx: i32,
    pub dy: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMLVSCROLL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMLVSCROLL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMLVSCROLL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMMOUSE {
    pub hdr: NMHDR,
    pub dwItemSpec: usize,
    pub dwItemData: usize,
    pub pt: super::super::Foundation::POINT,
    pub dwHitInfo: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMMOUSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMMOUSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMMOUSE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMOBJECTNOTIFY {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub piid: *const ::windows::core::GUID,
    pub pObject: *mut ::core::ffi::c_void,
    pub hResult: ::windows::core::HRESULT,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMOBJECTNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMOBJECTNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMOBJECTNOTIFY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMPGCALCSIZE {
    pub hdr: NMHDR,
    pub dwFlag: NMPGCALCSIZE_FLAGS,
    pub iWidth: i32,
    pub iHeight: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMPGCALCSIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMPGCALCSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMPGCALCSIZE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMPGHOTITEM {
    pub hdr: NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMPGHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMPGHOTITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMPGHOTITEM {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMPGSCROLL {
    pub hdr: NMHDR,
    pub fwKeys: NMPGSCROLL_KEYS,
    pub rcParent: super::super::Foundation::RECT,
    pub iDir: NMPGSCROLL_DIR,
    pub iXpos: i32,
    pub iYpos: i32,
    pub iScroll: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMPGSCROLL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMPGSCROLL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMPGSCROLL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMRBAUTOSIZE {
    pub hdr: NMHDR,
    pub fChanged: super::super::Foundation::BOOL,
    pub rcTarget: super::super::Foundation::RECT,
    pub rcActual: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMRBAUTOSIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMRBAUTOSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMRBAUTOSIZE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMREBAR {
    pub hdr: NMHDR,
    pub dwMask: NMREBAR_MASK_FLAGS,
    pub uBand: u32,
    pub fStyle: u32,
    pub wID: u32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMREBAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMREBAR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMREBAR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMREBARAUTOBREAK {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub uMsg: u32,
    pub fStyleCurrent: u32,
    pub fAutoBreak: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMREBARAUTOBREAK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMREBARAUTOBREAK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMREBARAUTOBREAK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMREBARCHEVRON {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub rc: super::super::Foundation::RECT,
    pub lParamNM: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMREBARCHEVRON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMREBARCHEVRON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMREBARCHEVRON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMREBARCHILDSIZE {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub rcChild: super::super::Foundation::RECT,
    pub rcBand: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMREBARCHILDSIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMREBARCHILDSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMREBARCHILDSIZE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMREBARSPLITTER {
    pub hdr: NMHDR,
    pub rcSizing: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMREBARSPLITTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMREBARSPLITTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMREBARSPLITTER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMSEARCHWEB {
    pub hdr: NMHDR,
    pub entrypoint: EC_SEARCHWEB_ENTRYPOINT,
    pub hasQueryText: super::super::Foundation::BOOL,
    pub invokeSucceeded: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMSEARCHWEB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMSEARCHWEB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMSEARCHWEB {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMSELCHANGE {
    pub nmhdr: NMHDR,
    pub stSelStart: super::super::Foundation::SYSTEMTIME,
    pub stSelEnd: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMSELCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMSELCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMSELCHANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct NMTBCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub hbrMonoDither: super::super::Graphics::Gdi::HBRUSH,
    pub hbrLines: super::super::Graphics::Gdi::HBRUSH,
    pub hpenLines: super::super::Graphics::Gdi::HPEN,
    pub clrText: super::super::Foundation::COLORREF,
    pub clrMark: super::super::Foundation::COLORREF,
    pub clrTextHighlight: super::super::Foundation::COLORREF,
    pub clrBtnFace: super::super::Foundation::COLORREF,
    pub clrBtnHighlight: super::super::Foundation::COLORREF,
    pub clrHighlightHotTrack: super::super::Foundation::COLORREF,
    pub rcText: super::super::Foundation::RECT,
    pub nStringBkMode: i32,
    pub nHLStringBkMode: i32,
    pub iListGap: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for NMTBCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for NMTBCUSTOMDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for NMTBCUSTOMDRAW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTBDISPINFOA {
    pub hdr: NMHDR,
    pub dwMask: NMTBDISPINFOW_MASK,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: ::windows::core::PSTR,
    pub cchText: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTBDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTBDISPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTBDISPINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTBDISPINFOW {
    pub hdr: NMHDR,
    pub dwMask: NMTBDISPINFOW_MASK,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: ::windows::core::PWSTR,
    pub cchText: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTBDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTBDISPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTBDISPINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTBGETINFOTIPA {
    pub hdr: NMHDR,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTBGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTBGETINFOTIPA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTBGETINFOTIPA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTBGETINFOTIPW {
    pub hdr: NMHDR,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTBGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTBGETINFOTIPW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTBGETINFOTIPW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTBHOTITEM {
    pub hdr: NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: NMTBHOTITEM_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTBHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTBHOTITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTBHOTITEM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTBRESTORE {
    pub hdr: NMHDR,
    pub pData: *mut u32,
    pub pCurrent: *mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub cbBytesPerRecord: i32,
    pub tbButton: TBBUTTON,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTBRESTORE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTBRESTORE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTBRESTORE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTBSAVE {
    pub hdr: NMHDR,
    pub pData: *mut u32,
    pub pCurrent: *mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub tbButton: TBBUTTON,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTBSAVE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTBSAVE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTBSAVE {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTCKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTCKEYDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTCKEYDOWN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTCKEYDOWN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTOOLBARA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: ::windows::core::PSTR,
    pub rcButton: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTOOLBARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTOOLBARA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTOOLBARA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTOOLBARW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: ::windows::core::PWSTR,
    pub rcButton: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTOOLBARW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTOOLBARW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTOOLBARW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTOOLTIPSCREATED {
    pub hdr: NMHDR,
    pub hwndToolTips: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTOOLTIPSCREATED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTOOLTIPSCREATED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTOOLTIPSCREATED {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTRBTHUMBPOSCHANGING {
    pub hdr: NMHDR,
    pub dwPos: u32,
    pub nReason: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTRBTHUMBPOSCHANGING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTRBTHUMBPOSCHANGING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTRBTHUMBPOSCHANGING {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTREEVIEWA {
    pub hdr: NMHDR,
    pub action: NM_TREEVIEW_ACTION,
    pub itemOld: TVITEMA,
    pub itemNew: TVITEMA,
    pub ptDrag: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTREEVIEWA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTREEVIEWA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTREEVIEWA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTREEVIEWW {
    pub hdr: NMHDR,
    pub action: NM_TREEVIEW_ACTION,
    pub itemOld: TVITEMW,
    pub itemNew: TVITEMW,
    pub ptDrag: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTREEVIEWW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTREEVIEWW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTREEVIEWW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct NMTTCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub uDrawFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for NMTTCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for NMTTCUSTOMDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for NMTTCUSTOMDRAW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTTDISPINFOA {
    pub hdr: NMHDR,
    pub lpszText: ::windows::core::PSTR,
    pub szText: [super::super::Foundation::CHAR; 80],
    pub hinst: super::super::Foundation::HINSTANCE,
    pub uFlags: TOOLTIP_FLAGS,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTTDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTTDISPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTTDISPINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTTDISPINFOW {
    pub hdr: NMHDR,
    pub lpszText: ::windows::core::PWSTR,
    pub szText: [u16; 80],
    pub hinst: super::super::Foundation::HINSTANCE,
    pub uFlags: TOOLTIP_FLAGS,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTTDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTTDISPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTTDISPINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct NMTVASYNCDRAW {
    pub hdr: NMHDR,
    pub pimldp: *mut IMAGELISTDRAWPARAMS,
    pub hr: ::windows::core::HRESULT,
    pub hItem: HTREEITEM,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwRetFlags: u32,
    pub iRetImageIndex: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for NMTVASYNCDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for NMTVASYNCDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for NMTVASYNCDRAW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct NMTVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: super::super::Foundation::COLORREF,
    pub clrTextBk: super::super::Foundation::COLORREF,
    pub iLevel: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for NMTVCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for NMTVCUSTOMDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for NMTVCUSTOMDRAW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTVDISPINFOA {
    pub hdr: NMHDR,
    pub item: TVITEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTVDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTVDISPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTVDISPINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTVDISPINFOEXA {
    pub hdr: NMHDR,
    pub item: TVITEMEXA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTVDISPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTVDISPINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTVDISPINFOEXA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTVDISPINFOEXW {
    pub hdr: NMHDR,
    pub item: TVITEMEXW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTVDISPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTVDISPINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTVDISPINFOEXW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTVDISPINFOW {
    pub hdr: NMHDR,
    pub item: TVITEMW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTVDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTVDISPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTVDISPINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTVGETINFOTIPA {
    pub hdr: NMHDR,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTVGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTVGETINFOTIPA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTVGETINFOTIPA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTVGETINFOTIPW {
    pub hdr: NMHDR,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTVGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTVGETINFOTIPW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTVGETINFOTIPW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTVITEMCHANGE {
    pub hdr: NMHDR,
    pub uChanged: u32,
    pub hItem: HTREEITEM,
    pub uStateNew: u32,
    pub uStateOld: u32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTVITEMCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTVITEMCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTVITEMCHANGE {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTVKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTVKEYDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTVKEYDOWN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTVKEYDOWN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMTVSTATEIMAGECHANGING {
    pub hdr: NMHDR,
    pub hti: HTREEITEM,
    pub iOldStateImageIndex: i32,
    pub iNewStateImageIndex: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMTVSTATEIMAGECHANGING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMTVSTATEIMAGECHANGING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMTVSTATEIMAGECHANGING {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMUPDOWN {
    pub hdr: NMHDR,
    pub iPos: i32,
    pub iDelta: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMUPDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMUPDOWN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMUPDOWN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NMVIEWCHANGE {
    pub nmhdr: NMHDR,
    pub dwOldView: MONTH_CALDENDAR_MESSAGES_VIEW,
    pub dwNewView: MONTH_CALDENDAR_MESSAGES_VIEW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NMVIEWCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NMVIEWCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NMVIEWCHANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct PBRANGE {
    pub iLow: i32,
    pub iHigh: i32,
}
impl ::core::marker::Copy for PBRANGE {}
impl ::core::clone::Clone for PBRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PBRANGE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct POINTER_DEVICE_CURSOR_INFO {
    pub cursorId: u32,
    pub cursor: POINTER_DEVICE_CURSOR_TYPE,
}
impl ::core::marker::Copy for POINTER_DEVICE_CURSOR_INFO {}
impl ::core::clone::Clone for POINTER_DEVICE_CURSOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POINTER_DEVICE_CURSOR_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct POINTER_DEVICE_INFO {
    pub displayOrientation: u32,
    pub device: super::super::Foundation::HANDLE,
    pub pointerDeviceType: POINTER_DEVICE_TYPE,
    pub monitor: super::super::Graphics::Gdi::HMONITOR,
    pub startingCursorId: u32,
    pub maxActiveContacts: u16,
    pub productString: [u16; 520],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for POINTER_DEVICE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for POINTER_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for POINTER_DEVICE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct POINTER_DEVICE_PROPERTY {
    pub logicalMin: i32,
    pub logicalMax: i32,
    pub physicalMin: i32,
    pub physicalMax: i32,
    pub unit: u32,
    pub unitExponent: u32,
    pub usagePageId: u16,
    pub usageId: u16,
}
impl ::core::marker::Copy for POINTER_DEVICE_PROPERTY {}
impl ::core::clone::Clone for POINTER_DEVICE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POINTER_DEVICE_PROPERTY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Input_Pointer\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct POINTER_TYPE_INFO {
    pub r#type: super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub Anonymous: POINTER_TYPE_INFO_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for POINTER_TYPE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for POINTER_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for POINTER_TYPE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Input_Pointer\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
pub union POINTER_TYPE_INFO_0 {
    pub touchInfo: super::Input::Pointer::POINTER_TOUCH_INFO,
    pub penInfo: super::Input::Pointer::POINTER_PEN_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for POINTER_TYPE_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for POINTER_TYPE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for POINTER_TYPE_INFO_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETHEADERA_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERA_V1_0,
    pub pszCaption: ::windows::core::PCSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERA_V1_1,
    pub Anonymous3: PROPSHEETHEADERA_V1_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERA_V1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V1_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERA_V1_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V1_1 {
    pub nStartPage: u32,
    pub pStartPage: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V1_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERA_V1_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V1_2 {
    pub ppsp: *mut PROPSHEETPAGEA,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V1_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERA_V1_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETHEADERA_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERA_V2_0,
    pub pszCaption: ::windows::core::PCSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERA_V2_1,
    pub Anonymous3: PROPSHEETHEADERA_V2_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
    pub Anonymous4: PROPSHEETHEADERA_V2_3,
    pub hplWatermark: super::super::Graphics::Gdi::HPALETTE,
    pub Anonymous5: PROPSHEETHEADERA_V2_4,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERA_V2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V2_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERA_V2_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V2_1 {
    pub nStartPage: u32,
    pub pStartPage: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERA_V2_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V2_2 {
    pub ppsp: *mut PROPSHEETPAGEA,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERA_V2_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V2_3 {
    pub hbmWatermark: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmWatermark: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERA_V2_3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V2_4 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERA_V2_4 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETHEADERW_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERW_V1_0,
    pub pszCaption: ::windows::core::PCWSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERW_V1_1,
    pub Anonymous3: PROPSHEETHEADERW_V1_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERW_V1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V1_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERW_V1_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V1_1 {
    pub nStartPage: u32,
    pub pStartPage: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V1_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERW_V1_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V1_2 {
    pub ppsp: *mut PROPSHEETPAGEW,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V1_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERW_V1_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETHEADERW_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERW_V2_0,
    pub pszCaption: ::windows::core::PCWSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERW_V2_1,
    pub Anonymous3: PROPSHEETHEADERW_V2_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
    pub Anonymous4: PROPSHEETHEADERW_V2_3,
    pub hplWatermark: super::super::Graphics::Gdi::HPALETTE,
    pub Anonymous5: PROPSHEETHEADERW_V2_4,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERW_V2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V2_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERW_V2_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V2_1 {
    pub nStartPage: u32,
    pub pStartPage: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERW_V2_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V2_2 {
    pub ppsp: *mut PROPSHEETPAGEW,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERW_V2_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V2_3 {
    pub hbmWatermark: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmWatermark: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERW_V2_3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V2_4 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETHEADERW_V2_4 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_0,
    pub Anonymous2: PROPSHEETPAGEA_1,
    pub pszTitle: ::windows::core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows::core::PCSTR,
    pub pszHeaderSubTitle: ::windows::core::PCSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub Anonymous3: PROPSHEETPAGEA_2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_0 {
    pub pszTemplate: ::windows::core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_2 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEA_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V1_0,
    pub Anonymous2: PROPSHEETPAGEA_V1_1,
    pub pszTitle: ::windows::core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_V1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V1_0 {
    pub pszTemplate: ::windows::core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_V1_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V1_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V1_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_V1_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEA_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V2_0,
    pub Anonymous2: PROPSHEETPAGEA_V2_1,
    pub pszTitle: ::windows::core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows::core::PCSTR,
    pub pszHeaderSubTitle: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_V2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V2_0 {
    pub pszTemplate: ::windows::core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_V2_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V2_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_V2_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEA_V3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V3_0,
    pub Anonymous2: PROPSHEETPAGEA_V3_1,
    pub pszTitle: ::windows::core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows::core::PCSTR,
    pub pszHeaderSubTitle: ::windows::core::PCSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_V3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V3_0 {
    pub pszTemplate: ::windows::core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_V3_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V3_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V3_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEA_V3_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_0,
    pub Anonymous2: PROPSHEETPAGEW_1,
    pub pszTitle: ::windows::core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows::core::PCWSTR,
    pub pszHeaderSubTitle: ::windows::core::PCWSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub Anonymous3: PROPSHEETPAGEW_2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_0 {
    pub pszTemplate: ::windows::core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_2 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEW_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V1_0,
    pub Anonymous2: PROPSHEETPAGEW_V1_1,
    pub pszTitle: ::windows::core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_V1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V1_0 {
    pub pszTemplate: ::windows::core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_V1_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V1_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V1_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_V1_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEW_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V2_0,
    pub Anonymous2: PROPSHEETPAGEW_V2_1,
    pub pszTitle: ::windows::core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows::core::PCWSTR,
    pub pszHeaderSubTitle: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_V2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V2_0 {
    pub pszTemplate: ::windows::core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_V2_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V2_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_V2_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEW_V3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V3_0,
    pub Anonymous2: PROPSHEETPAGEW_V3_1,
    pub pszTitle: ::windows::core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows::core::PCWSTR,
    pub pszHeaderSubTitle: ::windows::core::PCWSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_V3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V3_0 {
    pub pszTemplate: ::windows::core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_V3_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V3_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V3_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PROPSHEETPAGEW_V3_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PSHNOTIFY {
    pub hdr: NMHDR,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSHNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSHNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PSHNOTIFY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RBHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: u32,
    pub iBand: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RBHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RBHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RBHITTESTINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct REBARBANDINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: super::super::Foundation::COLORREF,
    pub clrBack: super::super::Foundation::COLORREF,
    pub lpText: ::windows::core::PSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: super::super::Foundation::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: super::super::Graphics::Gdi::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: super::super::Foundation::RECT,
    pub uChevronState: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for REBARBANDINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for REBARBANDINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for REBARBANDINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct REBARBANDINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: super::super::Foundation::COLORREF,
    pub clrBack: super::super::Foundation::COLORREF,
    pub lpText: ::windows::core::PWSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: super::super::Foundation::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: super::super::Graphics::Gdi::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: super::super::Foundation::RECT,
    pub uChevronState: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for REBARBANDINFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for REBARBANDINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for REBARBANDINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct REBARINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub himl: HIMAGELIST,
}
impl ::core::marker::Copy for REBARINFO {}
impl ::core::clone::Clone for REBARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REBARINFO {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct TASKDIALOGCONFIG {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub dwFlags: TASKDIALOG_FLAGS,
    pub dwCommonButtons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pub pszWindowTitle: ::windows::core::PCWSTR,
    pub Anonymous1: TASKDIALOGCONFIG_0,
    pub pszMainInstruction: ::windows::core::PCWSTR,
    pub pszContent: ::windows::core::PCWSTR,
    pub cButtons: u32,
    pub pButtons: *const TASKDIALOG_BUTTON,
    pub nDefaultButton: i32,
    pub cRadioButtons: u32,
    pub pRadioButtons: *const TASKDIALOG_BUTTON,
    pub nDefaultRadioButton: i32,
    pub pszVerificationText: ::windows::core::PCWSTR,
    pub pszExpandedInformation: ::windows::core::PCWSTR,
    pub pszExpandedControlText: ::windows::core::PCWSTR,
    pub pszCollapsedControlText: ::windows::core::PCWSTR,
    pub Anonymous2: TASKDIALOGCONFIG_1,
    pub pszFooter: ::windows::core::PCWSTR,
    pub pfCallback: PFTASKDIALOGCALLBACK,
    pub lpCallbackData: isize,
    pub cxWidth: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for TASKDIALOGCONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for TASKDIALOGCONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for TASKDIALOGCONFIG {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union TASKDIALOGCONFIG_0 {
    pub hMainIcon: super::WindowsAndMessaging::HICON,
    pub pszMainIcon: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for TASKDIALOGCONFIG_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for TASKDIALOGCONFIG_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for TASKDIALOGCONFIG_0 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub union TASKDIALOGCONFIG_1 {
    pub hFooterIcon: super::WindowsAndMessaging::HICON,
    pub pszFooterIcon: ::windows::core::PCWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for TASKDIALOGCONFIG_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for TASKDIALOGCONFIG_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for TASKDIALOGCONFIG_1 {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TASKDIALOG_BUTTON {
    pub nButtonID: i32,
    pub pszButtonText: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for TASKDIALOG_BUTTON {}
impl ::core::clone::Clone for TASKDIALOG_BUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASKDIALOG_BUTTON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TA_CUBIC_BEZIER {
    pub header: TA_TIMINGFUNCTION,
    pub rX0: f32,
    pub rY0: f32,
    pub rX1: f32,
    pub rY1: f32,
}
impl ::core::marker::Copy for TA_CUBIC_BEZIER {}
impl ::core::clone::Clone for TA_CUBIC_BEZIER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_CUBIC_BEZIER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TA_TIMINGFUNCTION {
    pub eTimingFunctionType: TA_TIMINGFUNCTION_TYPE,
}
impl ::core::marker::Copy for TA_TIMINGFUNCTION {}
impl ::core::clone::Clone for TA_TIMINGFUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_TIMINGFUNCTION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TA_TRANSFORM {
    pub eTransformType: TA_TRANSFORM_TYPE,
    pub dwTimingFunctionId: u32,
    pub dwStartTime: u32,
    pub dwDurationTime: u32,
    pub eFlags: TA_TRANSFORM_FLAG,
}
impl ::core::marker::Copy for TA_TRANSFORM {}
impl ::core::clone::Clone for TA_TRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_TRANSFORM {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TA_TRANSFORM_2D {
    pub header: TA_TRANSFORM,
    pub rX: f32,
    pub rY: f32,
    pub rInitialX: f32,
    pub rInitialY: f32,
    pub rOriginX: f32,
    pub rOriginY: f32,
}
impl ::core::marker::Copy for TA_TRANSFORM_2D {}
impl ::core::clone::Clone for TA_TRANSFORM_2D {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_TRANSFORM_2D {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TA_TRANSFORM_CLIP {
    pub header: TA_TRANSFORM,
    pub rLeft: f32,
    pub rTop: f32,
    pub rRight: f32,
    pub rBottom: f32,
    pub rInitialLeft: f32,
    pub rInitialTop: f32,
    pub rInitialRight: f32,
    pub rInitialBottom: f32,
}
impl ::core::marker::Copy for TA_TRANSFORM_CLIP {}
impl ::core::clone::Clone for TA_TRANSFORM_CLIP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_TRANSFORM_CLIP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TA_TRANSFORM_OPACITY {
    pub header: TA_TRANSFORM,
    pub rOpacity: f32,
    pub rInitialOpacity: f32,
}
impl ::core::marker::Copy for TA_TRANSFORM_OPACITY {}
impl ::core::clone::Clone for TA_TRANSFORM_OPACITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TA_TRANSFORM_OPACITY {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TBADDBITMAP {
    pub hInst: super::super::Foundation::HINSTANCE,
    pub nID: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TBADDBITMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TBADDBITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TBADDBITMAP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct TBBUTTON {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 6],
    pub dwData: usize,
    pub iString: isize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for TBBUTTON {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for TBBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for TBBUTTON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
#[cfg(target_arch = "x86")]
pub struct TBBUTTON {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 2],
    pub dwData: usize,
    pub iString: isize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for TBBUTTON {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for TBBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for TBBUTTON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TBBUTTONINFOA {
    pub cbSize: u32,
    pub dwMask: TBBUTTONINFOW_MASK,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: ::windows::core::PSTR,
    pub cchText: i32,
}
impl ::core::marker::Copy for TBBUTTONINFOA {}
impl ::core::clone::Clone for TBBUTTONINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TBBUTTONINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TBBUTTONINFOW {
    pub cbSize: u32,
    pub dwMask: TBBUTTONINFOW_MASK,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: ::windows::core::PWSTR,
    pub cchText: i32,
}
impl ::core::marker::Copy for TBBUTTONINFOW {}
impl ::core::clone::Clone for TBBUTTONINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TBBUTTONINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TBINSERTMARK {
    pub iButton: i32,
    pub dwFlags: TBINSERTMARK_FLAGS,
}
impl ::core::marker::Copy for TBINSERTMARK {}
impl ::core::clone::Clone for TBINSERTMARK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TBINSERTMARK {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TBMETRICS {
    pub cbSize: u32,
    pub dwMask: u32,
    pub cxPad: i32,
    pub cyPad: i32,
    pub cxBarPad: i32,
    pub cyBarPad: i32,
    pub cxButtonSpacing: i32,
    pub cyButtonSpacing: i32,
}
impl ::core::marker::Copy for TBMETRICS {}
impl ::core::clone::Clone for TBMETRICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TBMETRICS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TBREPLACEBITMAP {
    pub hInstOld: super::super::Foundation::HINSTANCE,
    pub nIDOld: usize,
    pub hInstNew: super::super::Foundation::HINSTANCE,
    pub nIDNew: usize,
    pub nButtons: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TBREPLACEBITMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TBREPLACEBITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TBREPLACEBITMAP {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub struct TBSAVEPARAMSA {
    pub hkr: super::super::System::Registry::HKEY,
    pub pszSubKey: ::windows::core::PCSTR,
    pub pszValueName: ::windows::core::PCSTR,
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::marker::Copy for TBSAVEPARAMSA {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::clone::Clone for TBSAVEPARAMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Registry")]
unsafe impl ::windows::core::Abi for TBSAVEPARAMSA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub struct TBSAVEPARAMSW {
    pub hkr: super::super::System::Registry::HKEY,
    pub pszSubKey: ::windows::core::PCWSTR,
    pub pszValueName: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::marker::Copy for TBSAVEPARAMSW {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::clone::Clone for TBSAVEPARAMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Registry")]
unsafe impl ::windows::core::Abi for TBSAVEPARAMSW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: TCHITTESTINFO_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCHITTESTINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCITEMA {
    pub mask: TCITEMHEADERA_MASK,
    pub dwState: TAB_CONTROL_ITEM_STATE,
    pub dwStateMask: TAB_CONTROL_ITEM_STATE,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCITEMA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TCITEMHEADERA {
    pub mask: TCITEMHEADERA_MASK,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
impl ::core::marker::Copy for TCITEMHEADERA {}
impl ::core::clone::Clone for TCITEMHEADERA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TCITEMHEADERA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TCITEMHEADERW {
    pub mask: TCITEMHEADERA_MASK,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
impl ::core::marker::Copy for TCITEMHEADERW {}
impl ::core::clone::Clone for TCITEMHEADERW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TCITEMHEADERW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCITEMW {
    pub mask: TCITEMHEADERA_MASK,
    pub dwState: TAB_CONTROL_ITEM_STATE,
    pub dwStateMask: TAB_CONTROL_ITEM_STATE,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCITEMW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOUCH_HIT_TESTING_INPUT {
    pub pointerId: u32,
    pub point: super::super::Foundation::POINT,
    pub boundingBox: super::super::Foundation::RECT,
    pub nonOccludedBoundingBox: super::super::Foundation::RECT,
    pub orientation: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOUCH_HIT_TESTING_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOUCH_HIT_TESTING_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TOUCH_HIT_TESTING_INPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    pub score: u16,
    pub adjustedPoint: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct TTGETTITLE {
    pub dwSize: u32,
    pub uTitleBitmap: u32,
    pub cch: u32,
    pub pszTitle: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for TTGETTITLE {}
impl ::core::clone::Clone for TTGETTITLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TTGETTITLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TTHITTESTINFOA {
    pub hwnd: super::super::Foundation::HWND,
    pub pt: super::super::Foundation::POINT,
    pub ti: TTTOOLINFOA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TTHITTESTINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TTHITTESTINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TTHITTESTINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TTHITTESTINFOW {
    pub hwnd: super::super::Foundation::HWND,
    pub pt: super::super::Foundation::POINT,
    pub ti: TTTOOLINFOW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TTHITTESTINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TTHITTESTINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TTHITTESTINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TTTOOLINFOA {
    pub cbSize: u32,
    pub uFlags: TOOLTIP_FLAGS,
    pub hwnd: super::super::Foundation::HWND,
    pub uId: usize,
    pub rect: super::super::Foundation::RECT,
    pub hinst: super::super::Foundation::HINSTANCE,
    pub lpszText: ::windows::core::PSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpReserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TTTOOLINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TTTOOLINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TTTOOLINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TTTOOLINFOW {
    pub cbSize: u32,
    pub uFlags: TOOLTIP_FLAGS,
    pub hwnd: super::super::Foundation::HWND,
    pub uId: usize,
    pub rect: super::super::Foundation::RECT,
    pub hinst: super::super::Foundation::HINSTANCE,
    pub lpszText: ::windows::core::PWSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpReserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TTTOOLINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TTTOOLINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TTTOOLINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TVGETITEMPARTRECTINFO {
    pub hti: HTREEITEM,
    pub prc: *mut super::super::Foundation::RECT,
    pub partID: TVITEMPART,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVGETITEMPARTRECTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVGETITEMPARTRECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVGETITEMPARTRECTINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TVHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: TVHITTESTINFO_FLAGS,
    pub hItem: HTREEITEM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVHITTESTINFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TVINSERTSTRUCTA {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub Anonymous: TVINSERTSTRUCTA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVINSERTSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVINSERTSTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVINSERTSTRUCTA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union TVINSERTSTRUCTA_0 {
    pub itemex: TVITEMEXA,
    pub item: TVITEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVINSERTSTRUCTA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVINSERTSTRUCTA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVINSERTSTRUCTA_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TVINSERTSTRUCTW {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub Anonymous: TVINSERTSTRUCTW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVINSERTSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVINSERTSTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVINSERTSTRUCTW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union TVINSERTSTRUCTW_0 {
    pub itemex: TVITEMEXW,
    pub item: TVITEMW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVINSERTSTRUCTW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVINSERTSTRUCTW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVINSERTSTRUCTW_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TVITEMA {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: TREE_VIEW_ITEM_STATE_FLAGS,
    pub stateMask: TREE_VIEW_ITEM_STATE_FLAGS,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVITEMA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TVITEMEXA {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: ::windows::core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVITEMEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVITEMEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVITEMEXA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TVITEMEXW {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVITEMEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVITEMEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVITEMEXW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TVITEMW {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: TREE_VIEW_ITEM_STATE_FLAGS,
    pub stateMask: TREE_VIEW_ITEM_STATE_FLAGS,
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVITEMW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TVSORTCB {
    pub hParent: HTREEITEM,
    pub lpfnCompare: PFNTVCOMPARE,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TVSORTCB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TVSORTCB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TVSORTCB {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct UDACCEL {
    pub nSec: u32,
    pub nInc: u32,
}
impl ::core::marker::Copy for UDACCEL {}
impl ::core::clone::Clone for UDACCEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UDACCEL {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct USAGE_PROPERTIES {
    pub level: u16,
    pub page: u16,
    pub usage: u16,
    pub logicalMinimum: i32,
    pub logicalMaximum: i32,
    pub unit: u16,
    pub exponent: u16,
    pub count: u8,
    pub physicalMinimum: i32,
    pub physicalMaximum: i32,
}
impl ::core::marker::Copy for USAGE_PROPERTIES {}
impl ::core::clone::Clone for USAGE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USAGE_PROPERTIES {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub struct WTA_OPTIONS {
    pub dwFlags: u32,
    pub dwMask: u32,
}
impl ::core::marker::Copy for WTA_OPTIONS {}
impl ::core::clone::Clone for WTA_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WTA_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type DTT_CALLBACK_PROC = ::core::option::Option<unsafe extern "system" fn(hdc: super::super::Graphics::Gdi::HDC, psztext: ::windows::core::PWSTR, cchtext: i32, prc: *mut super::super::Foundation::RECT, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub type EDITWORDBREAKPROCA = ::core::option::Option<unsafe extern "system" fn(lpch: ::windows::core::PCSTR, ichcurrent: i32, cch: i32, code: WORD_BREAK_ACTION) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub type EDITWORDBREAKPROCW = ::core::option::Option<unsafe extern "system" fn(lpch: ::windows::core::PCWSTR, ichcurrent: i32, cch: i32, code: WORD_BREAK_ACTION) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNADDPROPSHEETPAGES = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: LPFNSVADDPROPSHEETPAGE, param2: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPFNCCINFOA = ::core::option::Option<unsafe extern "system" fn(acci: *mut CCINFOA) -> u32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPFNCCINFOW = ::core::option::Option<unsafe extern "system" fn(acci: *mut CCINFOW) -> u32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type LPFNCCSIZETOTEXTA = ::core::option::Option<unsafe extern "system" fn(flstyle: u32, flextstyle: u32, hfont: super::super::Graphics::Gdi::HFONT, psztext: ::windows::core::PCSTR) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type LPFNCCSIZETOTEXTW = ::core::option::Option<unsafe extern "system" fn(flstyle: u32, flextstyle: u32, hfont: super::super::Graphics::Gdi::HFONT, psztext: ::windows::core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNCCSTYLEA = ::core::option::Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pccs: *mut CCSTYLEA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNCCSTYLEW = ::core::option::Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pccs: *mut CCSTYLEW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPFNPSPCALLBACKA = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: PSPCB_MESSAGE, ppsp: *mut PROPSHEETPAGEA) -> u32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPFNPSPCALLBACKW = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: PSPCB_MESSAGE, ppsp: *mut PROPSHEETPAGEW) -> u32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNSVADDPROPSHEETPAGE = ::core::option::Option<unsafe extern "system" fn(param0: HPROPSHEETPAGE, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNDACOMPARE = ::core::option::Option<unsafe extern "system" fn(p1: *const ::core::ffi::c_void, p2: *const ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNDACOMPARECONST = ::core::option::Option<unsafe extern "system" fn(p1: *const ::core::ffi::c_void, p2: *const ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub type PFNDAENUMCALLBACK = ::core::option::Option<unsafe extern "system" fn(p: *const ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub type PFNDAENUMCALLBACKCONST = ::core::option::Option<unsafe extern "system" fn(p: *const ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNDPAMERGE = ::core::option::Option<unsafe extern "system" fn(umsg: DPAMM_MESSAGE, pvdest: *const ::core::ffi::c_void, pvsrc: *const ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNDPAMERGECONST = ::core::option::Option<unsafe extern "system" fn(umsg: DPAMM_MESSAGE, pvdest: *const ::core::ffi::c_void, pvsrc: *const ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type PFNDPASTREAM = ::core::option::Option<unsafe extern "system" fn(pinfo: *const DPASTREAMINFO, pstream: ::core::option::Option<super::super::System::Com::IStream>, pvinstdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNLVCOMPARE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::LPARAM, param1: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
pub type PFNLVGROUPCOMPARE = ::core::option::Option<unsafe extern "system" fn(param0: i32, param1: i32, param2: *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNPROPSHEETCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNTVCOMPARE = ::core::option::Option<unsafe extern "system" fn(lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM, lparamsort: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFTASKDIALOGCALLBACK = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lprefdata: isize) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
