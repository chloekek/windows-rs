#[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn PdfCreateRenderer<P0>(pdevice: P0) -> ::windows::core::Result<IPdfRendererNative>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::super::Graphics::Dxgi::IDXGIDevice>>,
{
    ::windows::core::link ! ( "windows.data.pdf.dll""system" fn PdfCreateRenderer ( pdevice : * mut::core::ffi::c_void , pprenderer : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PdfCreateRenderer(pdevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`*"]
#[repr(transparent)]
pub struct IPdfRendererNative(::windows::core::IUnknown);
impl IPdfRendererNative {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    pub unsafe fn RenderPageToSurface<P0, P1>(&self, pdfpage: P0, psurface: P1, offset: super::super::super::Foundation::POINT, prenderparams: ::core::option::Option<*const PDF_RENDER_PARAMS>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::Graphics::Dxgi::IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).RenderPageToSurface)(::windows::core::Vtable::as_raw(self), pdfpage.into().abi(), psurface.into().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(prenderparams.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn RenderPageToDeviceContext<P0, P1>(&self, pdfpage: P0, pd2ddevicecontext: P1, prenderparams: ::core::option::Option<*const PDF_RENDER_PARAMS>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::Graphics::Direct2D::ID2D1DeviceContext>>,
    {
        (::windows::core::Vtable::vtable(self).RenderPageToDeviceContext)(::windows::core::Vtable::as_raw(self), pdfpage.into().abi(), pd2ddevicecontext.into().abi(), ::core::mem::transmute(prenderparams.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::core::interface_hierarchy!(IPdfRendererNative, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPdfRendererNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IPdfRendererNative {
    type Vtable = IPdfRendererNative_Vtbl;
}
unsafe impl ::windows::core::Interface for IPdfRendererNative {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d9dcd91_d277_4947_8527_07a0daeda94a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfRendererNative_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    pub RenderPageToSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, psurface: *mut ::core::ffi::c_void, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi")))]
    RenderPageToSurface: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub RenderPageToDeviceContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, pd2ddevicecontext: *mut ::core::ffi::c_void, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    RenderPageToDeviceContext: usize,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: super::super::super::Graphics::Direct2D::Common::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: super::super::super::Graphics::Direct2D::Common::D2D_COLOR_F,
    pub IgnoreHighContrast: super::super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for PDF_RENDER_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for PDF_RENDER_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
unsafe impl ::windows::core::Abi for PDF_RENDER_PARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGIDevice>, param1: *mut ::core::option::Option<IPdfRendererNative>) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
